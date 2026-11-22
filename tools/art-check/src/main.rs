//! Checks the masters in `manifest.json` against two rules (hill,
//! 2026-11-21):
//!
//! - Use: a picture is outside `archive/` only if something uses it: it
//!   is the master of a pose, launcher icon or moment picture in
//!   `poses.json`, or one of the few pictures the exporters use that it
//!   does not list (`UNLISTED_ICONS`, `UNLISTED_MOMENTS`). Every other
//!   picture is in `archive/`.
//! - Framing: `face` and the launcher icons are headshots; every other
//!   master shows the companion's full body.
//!
//! It also checks `poses.json` itself: every fallback names a known pose
//! and none forms a cycle, and `missing` in the manifest lists only core
//! poses. And no two pictures outside `docs/` and `tools/` have the same
//! bytes: poses that share a picture name one file in the manifest.
//!
//! Usage: `cargo run --manifest-path tools/art-check/Cargo.toml -- .`
//! (the argument is the repository root).

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Launcher icons the exporters use that `poses.json` does not list:
/// the monochrome layer, which the Android exporter makes from the face's
/// line art unless a companion has its own master.
const UNLISTED_ICONS: &[&str] = &["icon_mono"];

/// Moment pictures the exporters export that `poses.json` does not list,
/// having no slot or fallback of their own.
const UNLISTED_MOMENTS: &[&str] = &["study"];

/// Picture file extensions the duplicate check compares.
const PICTURE_EXTENSIONS: &[&str] = &["png", "webp", "jpg", "jpeg", "gif"];

/// Folders the duplicate check skips: git's own and the records.
const NOT_COMPARED: &[&str] = &[".git", "docs", "tools"];

/// Folders whose pictures are not masters: the archive and the records.
const NOT_MASTERS: &[&str] = &["archive", "docs", "tools", ".git", ".github"];

/// How much of the companion a picture shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Framing {
    /// The whole body, head to feet.
    FullBody,
    /// Head and shoulders, or down to the waist.
    Bust,
    /// The head only.
    Headshot,
}

/// `kind` in the manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Kind {
    Core,
    Extension,
    Picture,
}

/// What uses a master, worked out from its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Use {
    Core,
    Extension,
    LauncherIcon,
    Moment,
    Unused,
}

#[derive(Deserialize)]
struct Master {
    companion: String,
    pose: String,
    kind: Kind,
    file: String,
    framing: Framing,
}

#[derive(Deserialize)]
struct Shared {
    file: String,
    framing: Framing,
}

#[derive(Deserialize)]
struct Manifest {
    masters: Vec<Master>,
    shared: Vec<Shared>,
}

/// `poses.json`: the pose names and their fallbacks.
#[derive(Deserialize)]
struct Poses {
    core: Vec<String>,
    extensions: BTreeMap<String, String>,
    pictures: BTreeMap<String, String>,
    icons: BTreeMap<String, String>,
    #[serde(default)]
    core_fallbacks: BTreeMap<String, String>,
}

impl Poses {
    /// The groups that map each name to its fallback, with their names.
    fn groups(&self) -> [(&'static str, &BTreeMap<String, String>); 3] {
        [
            ("extension", &self.extensions),
            ("picture", &self.pictures),
            ("icon", &self.icons),
        ]
    }

    /// The pose, picture or icon a companion without `name` shows.
    fn fallback(&self, name: &str) -> Option<&String> {
        self.extensions
            .get(name)
            .or(self.pictures.get(name))
            .or(self.icons.get(name))
            .or(self.core_fallbacks.get(name))
    }
}

/// An entry of `missing` in the manifest.
#[derive(Deserialize)]
struct MissingPose {
    companion: String,
    pose: String,
}

/// `missing` in the manifest, read apart from [`Manifest`].
#[derive(Deserialize)]
struct Missing {
    #[serde(default)]
    missing: Vec<MissingPose>,
}

/// What uses a picture with this name.
fn use_of(name: &str, poses: &Poses) -> Use {
    if poses.core.iter().any(|p| p == name) {
        return Use::Core;
    }
    if poses.extensions.contains_key(name) {
        return Use::Extension;
    }
    if poses.icons.contains_key(name) || UNLISTED_ICONS.contains(&name) {
        return Use::LauncherIcon;
    }
    if poses.pictures.contains_key(name) || UNLISTED_MOMENTS.contains(&name) {
        return Use::Moment;
    }
    Use::Unused
}

/// The manifest kind a picture with this use has.
fn kind_for(used: Use) -> Option<Kind> {
    match used {
        Use::Core => Some(Kind::Core),
        Use::Extension => Some(Kind::Extension),
        Use::LauncherIcon | Use::Moment => Some(Kind::Picture),
        Use::Unused => None,
    }
}

/// The framing a picture with this use and name must have.
fn required(used: Use, name: &str) -> Framing {
    if used == Use::LauncherIcon || name == "face" {
        return Framing::Headshot;
    }
    Framing::FullBody
}

/// One line per master that breaks a rule.
fn violations(manifest: &Manifest, poses: &Poses) -> Vec<String> {
    let mut out = Vec::new();

    // Per-companion masters: used, of the right kind and framing.
    for m in &manifest.masters {
        let what = format!("{} ({} {})", m.file, m.companion, m.pose);
        let used = use_of(&m.pose, poses);
        let Some(kind) = kind_for(used) else {
            out.push(format!("{what}: nothing uses it; move it to archive/"));
            continue;
        };
        if m.kind != kind {
            out.push(format!("{what}: kind {:?}, needs {kind:?}", m.kind));
        }
        let want = required(used, &m.pose);
        if m.framing != want {
            out.push(format!("{what}: {:?}, needs {want:?}", m.framing));
        }
    }

    // Pictures of several companions are never faces or icons.
    for s in &manifest.shared {
        if s.framing != Framing::FullBody {
            out.push(format!("{}: {:?}, needs FullBody", s.file, s.framing));
        }
    }
    out
}

/// One line per fallback in `poses.json` that names an unknown pose or
/// forms a cycle, and per name listed twice.
fn pose_violations(poses: &Poses) -> Vec<String> {
    let mut out = Vec::new();
    let is_core = |p: &str| poses.core.iter().any(|c| c == p);

    // Every name once, in one group.
    let mut seen: BTreeMap<&str, &str> = BTreeMap::new();
    let names = poses.core.iter().map(|p| ("core", p)).chain(
        poses
            .groups()
            .into_iter()
            .flat_map(|(group, map)| map.keys().map(move |p| (group, p))),
    );
    for (group, name) in names {
        if let Some(before) = seen.insert(name, group) {
            out.push(format!(
                "poses.json: {name} is listed as {before} and {group}"
            ));
        }
    }

    for (group, map) in poses.groups() {
        for (name, to) in map {
            if !seen.contains_key(to.as_str()) {
                out.push(format!(
                    "poses.json: {group} {name} falls back to unknown pose {to}"
                ));
            }
        }
    }
    for (from, to) in &poses.core_fallbacks {
        if !is_core(from) {
            out.push(format!(
                "poses.json: core_fallbacks names {from}, not a core pose"
            ));
        }
        if !is_core(to) {
            out.push(format!(
                "poses.json: core_fallbacks {from} falls back to {to}, not a core pose"
            ));
        }
    }

    // Follow each chain until it ends. A chain longer than the number of
    // names has a cycle.
    let starts = poses
        .groups()
        .into_iter()
        .flat_map(|(_, map)| map.keys())
        .chain(poses.core_fallbacks.keys());
    for start in starts {
        let mut at = start.as_str();
        let mut steps = 0;
        while let Some(to) = poses.fallback(at) {
            at = to;
            steps += 1;
            if steps > seen.len() {
                out.push(format!(
                    "poses.json: the fallbacks from {start} form a cycle"
                ));
                break;
            }
        }
    }
    out
}

/// One line per `missing` entry that is not a core pose.
fn missing_violations(missing: &Missing, poses: &Poses) -> Vec<String> {
    missing
        .missing
        .iter()
        .filter(|m| !poses.core.contains(&m.pose))
        .map(|m| {
            format!(
                "manifest.json: missing lists {} {}, not a core pose in poses.json",
                m.companion, m.pose
            )
        })
        .collect()
}

/// Reads and parses a JSON file.
fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|text| serde_json::from_str(&text).map_err(|e| e.to_string()))
        .map_err(|e| format!("{}: {e}", path.display()))
}

/// Every PNG under `dir`, skipping the folders that hold no masters.
fn pictures(root: &Path, dir: &Path, out: &mut Vec<String>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let rel = path.strip_prefix(root).unwrap_or(&path);
        let rel = rel.to_string_lossy().replace('\\', "/");

        // Recurse into folders that can hold masters.
        if path.is_dir() {
            let top = rel.split('/').next().unwrap_or_default();
            if !NOT_MASTERS.contains(&top) {
                pictures(root, &path, out)?;
            }
            continue;
        }
        if rel.ends_with(".png") {
            out.push(rel);
        }
    }
    Ok(())
}

/// Files on disk and in the manifest that don't match.
fn stray(root: &Path, manifest: &Manifest) -> std::io::Result<Vec<String>> {
    let listed: Vec<&str> = manifest
        .masters
        .iter()
        .map(|m| m.file.as_str())
        .chain(manifest.shared.iter().map(|s| s.file.as_str()))
        .collect();
    let mut found = Vec::new();
    pictures(root, root, &mut found)?;

    let mut out = Vec::new();
    for file in &found {
        if !listed.contains(&file.as_str()) {
            out.push(format!(
                "{file}: not in manifest.json; list it or move it to archive/"
            ));
        }
    }
    for file in &listed {
        if !root.join(file).is_file() {
            out.push(format!("{file}: in manifest.json but not on disk"));
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

/// Every picture under `dir`, skipping `NOT_COMPARED` at the top.
fn all_pictures(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let rel = path.strip_prefix(root).unwrap_or(&path);
            let top = rel.components().next();
            let skip =
                top.is_some_and(|c| NOT_COMPARED.contains(&&*c.as_os_str().to_string_lossy()));
            if !skip {
                all_pictures(root, &path, out)?;
            }
            continue;
        }
        let ext = path
            .extension()
            .map(|e| e.to_string_lossy().to_ascii_lowercase())
            .unwrap_or_default();
        if PICTURE_EXTENSIONS.contains(&ext.as_str()) {
            out.push(path);
        }
    }
    Ok(())
}

/// One line per pair of pictures with identical bytes: grouped by length,
/// then by hash, then compared byte for byte.
fn duplicates(root: &Path) -> std::io::Result<Vec<String>> {
    use std::collections::HashMap;
    use std::hash::{DefaultHasher, Hash, Hasher};

    let mut found = Vec::new();
    all_pictures(root, root, &mut found)?;
    found.sort();

    let mut by_len: BTreeMap<u64, Vec<PathBuf>> = BTreeMap::new();
    for path in found {
        by_len
            .entry(std::fs::metadata(&path)?.len())
            .or_default()
            .push(path);
    }

    let rel = |p: &Path| {
        p.strip_prefix(root)
            .unwrap_or(p)
            .to_string_lossy()
            .replace('\\', "/")
    };
    let mut out = Vec::new();
    for paths in by_len.into_values().filter(|p| p.len() > 1) {
        let mut by_hash: HashMap<u64, Vec<(PathBuf, Vec<u8>)>> = HashMap::new();
        for path in paths {
            let bytes = std::fs::read(&path)?;
            let mut hasher = DefaultHasher::new();
            bytes.hash(&mut hasher);
            by_hash
                .entry(hasher.finish())
                .or_default()
                .push((path, bytes));
        }
        for group in by_hash.values() {
            for (i, (a, bytes_a)) in group.iter().enumerate() {
                for (b, bytes_b) in &group[i + 1..] {
                    if bytes_a == bytes_b {
                        out.push(format!(
                            "{} and {}: the same picture; poses that share a picture name one file in manifest.json",
                            rel(a),
                            rel(b)
                        ));
                    }
                }
            }
        }
    }
    out.sort();
    Ok(out)
}

fn main() -> ExitCode {
    let root = PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| ".".into()));
    let path = root.join("manifest.json");
    let read = read_json::<Manifest>(&path).and_then(|manifest| {
        let missing = read_json::<Missing>(&path)?;
        let poses = read_json::<Poses>(&root.join("poses.json"))?;
        Ok((manifest, missing, poses))
    });
    let (manifest, missing, poses) = match read {
        Ok(read) => read,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    let mut found = pose_violations(&poses);
    found.extend(missing_violations(&missing, &poses));
    found.extend(violations(&manifest, &poses));
    match stray(&root, &manifest).and_then(|mut lines| {
        lines.extend(duplicates(&root)?);
        Ok(lines)
    }) {
        Ok(lines) => found.extend(lines),
        Err(e) => {
            eprintln!("{}: {e}", root.display());
            return ExitCode::FAILURE;
        }
    }
    if found.is_empty() {
        println!("Every master is used and framed as its pose needs.");
        return ExitCode::SUCCESS;
    }
    for line in &found {
        println!("{line}");
    }
    println!("{} problems.", found.len());
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This repository's `poses.json`.
    fn poses() -> Poses {
        serde_json::from_str(include_str!("../../../poses.json")).unwrap()
    }

    fn check(manifest: &Manifest) -> Vec<String> {
        violations(manifest, &poses())
    }

    fn manifest(pose: &str, kind: &str, framing: &str) -> Manifest {
        let json = format!(
            r#"{{"masters": [{{"companion": "aki", "pose": "{pose}", "kind": "{kind}", "file": "aki/{pose}.png", "framing": "{framing}"}}], "shared": []}}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn face_and_icons_are_headshots() {
        assert!(check(&manifest("face", "core", "headshot")).is_empty());
        assert!(check(&manifest("icon_mono", "picture", "headshot")).is_empty());
        assert_eq!(check(&manifest("face", "core", "full_body")).len(), 1);
        assert_eq!(check(&manifest("icon_happy", "picture", "bust")).len(), 1);
    }

    #[test]
    fn other_pictures_are_full_body() {
        assert!(check(&manifest("sad", "core", "full_body")).is_empty());
        assert!(check(&manifest("winner", "extension", "full_body")).is_empty());
        assert!(check(&manifest("place_1", "picture", "full_body")).is_empty());
        assert_eq!(check(&manifest("sad", "core", "headshot")).len(), 1);
        assert_eq!(check(&manifest("frozen", "core", "bust")).len(), 1);
    }

    #[test]
    fn unused_pictures_belong_in_the_archive() {
        let found = check(&manifest("icon_fire", "picture", "headshot"));
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("archive/"));
    }

    #[test]
    fn kind_follows_the_pose_list() {
        assert_eq!(check(&manifest("welcome", "core", "full_body")).len(), 1);
        assert_eq!(check(&manifest("goal", "core", "full_body")).len(), 1);
    }

    #[test]
    fn unknown_framing_is_rejected() {
        let json = r#"{"masters": [{"companion": "aki", "pose": "sad", "kind": "core", "file": "aki/sad.png", "framing": "portrait"}], "shared": []}"#;
        assert!(serde_json::from_str::<Manifest>(json).is_err());
    }

    #[test]
    fn poses_outside_the_list_are_rejected() {
        let found = check(&manifest("streak_mythic", "extension", "full_body"));
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("archive/"));
    }

    fn pose_list(extensions: &str, core_fallbacks: &str) -> Poses {
        slot_list(extensions, "", "", core_fallbacks)
    }

    fn slot_list(extensions: &str, pictures: &str, icons: &str, core_fallbacks: &str) -> Poses {
        let json = format!(
            r#"{{"core": ["face", "happy", "hopeful"], "extensions": {{{extensions}}}, "pictures": {{{pictures}}}, "icons": {{{icons}}}, "core_fallbacks": {{{core_fallbacks}}}}}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn the_pose_list_is_sound() {
        assert_eq!(pose_violations(&poses()), Vec::<String>::new());
        let ok = pose_list(
            r#""winner": "happy", "champion": "winner""#,
            r#""happy": "hopeful""#,
        );
        assert!(pose_violations(&ok).is_empty());
    }

    #[test]
    fn fallbacks_to_unknown_poses_are_rejected() {
        let found = pose_violations(&pose_list(r#""winner": "jolly""#, ""));
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("unknown pose jolly"));

        let found = pose_violations(&pose_list("", r#""happy": "jolly""#));
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("not a core pose"));

        let found = pose_violations(&pose_list("", r#""sad": "hopeful""#));
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("core_fallbacks names sad"));
    }

    #[test]
    fn fallback_cycles_are_rejected() {
        let found = pose_violations(&pose_list(
            r#""winner": "champion", "champion": "winner""#,
            "",
        ));
        assert_eq!(found.len(), 2);
        assert!(found.iter().all(|f| f.contains("cycle")));

        let found = pose_violations(&pose_list("", r#""happy": "hopeful", "hopeful": "happy""#));
        assert_eq!(found.len(), 2);
        assert!(found.iter().all(|f| f.contains("cycle")));
    }

    #[test]
    fn picture_and_icon_fallbacks_are_checked() {
        let ok = slot_list(
            r#""winner": "happy""#,
            r#""place_1": "winner", "goal": "happy""#,
            r#""icon": "face", "icon_happy": "icon""#,
            "",
        );
        assert!(pose_violations(&ok).is_empty());

        let found = pose_violations(&slot_list("", r#""goal": "jolly""#, "", ""));
        assert_eq!(
            found,
            ["poses.json: picture goal falls back to unknown pose jolly"]
        );

        let found = pose_violations(&slot_list(
            "",
            "",
            r#""icon": "icon_happy", "icon_happy": "icon""#,
            "",
        ));
        assert_eq!(found.len(), 2);
        assert!(found.iter().all(|f| f.contains("cycle")));

        // A cycle through several groups.
        let found = pose_violations(&slot_list(
            r#""winner": "goal""#,
            r#""goal": "icon""#,
            r#""icon": "winner""#,
            "",
        ));
        assert_eq!(found.len(), 3);
    }

    #[test]
    fn pictures_and_icons_must_be_slots() {
        assert!(check(&manifest("place_1", "picture", "full_body")).is_empty());
        assert!(check(&manifest("icon_sleepy", "picture", "headshot")).is_empty());
        // Used by the exporters but not slots.
        assert!(check(&manifest("study", "picture", "full_body")).is_empty());
        assert!(check(&manifest("icon_mono", "picture", "headshot")).is_empty());

        let found = check(&manifest("place_4", "picture", "full_body"));
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("archive/"));
    }

    #[test]
    fn poses_are_listed_once() {
        let found = pose_violations(&pose_list(r#""happy": "hopeful""#, ""));
        assert!(found
            .iter()
            .any(|f| f.contains("listed as core and extension")));
        let found = pose_violations(&slot_list(
            r#""goal": "happy""#,
            r#""goal": "happy""#,
            "",
            "",
        ));
        assert_eq!(
            found,
            ["poses.json: goal is listed as extension and picture"]
        );
    }

    #[test]
    fn missing_lists_only_core_poses() {
        let missing = |pose: &str| -> Missing {
            let json = format!(r#"{{"missing": [{{"companion": "aki", "pose": "{pose}"}}]}}"#);
            serde_json::from_str(&json).unwrap()
        };
        assert!(missing_violations(&missing("streak"), &poses()).is_empty());
        assert_eq!(missing_violations(&missing("jolly"), &poses()).len(), 1);
        assert_eq!(missing_violations(&missing("winner"), &poses()).len(), 1);
    }

    /// A fresh, empty folder for one test.
    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("art-check-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("aki")).unwrap();
        dir
    }

    #[test]
    fn identical_pictures_are_rejected() {
        let dir = scratch("dup");
        std::fs::create_dir_all(dir.join("archive/aki")).unwrap();
        std::fs::create_dir_all(dir.join("docs")).unwrap();
        // Master and master, archive and archive.
        std::fs::write(dir.join("aki/happy.png"), b"happy").unwrap();
        std::fs::write(dir.join("aki/smile.png"), b"happy").unwrap();
        std::fs::write(dir.join("archive/aki/a.gif"), b"old").unwrap();
        std::fs::write(dir.join("archive/aki/b.jpg"), b"old").unwrap();
        // Different bytes of the same length, and records, pass.
        std::fs::write(dir.join("aki/sad.png"), b"hapPy").unwrap();
        std::fs::write(dir.join("docs/happy.png"), b"other").unwrap();
        std::fs::write(dir.join("docs/copy.png"), b"other").unwrap();
        let found = duplicates(&dir).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(found[0].contains("aki/happy.png and aki/smile.png"));
        assert!(found[0].contains("name one file in manifest.json"));
        assert!(found[1].contains("archive/aki/a.gif and archive/aki/b.jpg"));
    }

    #[test]
    fn archive_copies_of_masters_are_rejected() {
        let dir = scratch("archived");
        std::fs::create_dir_all(dir.join("archive/aki")).unwrap();
        std::fs::write(dir.join("aki/happy.png"), b"same bytes").unwrap();
        std::fs::write(dir.join("archive/aki/old.webp"), b"same bytes").unwrap();
        let found = duplicates(&dir).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("aki/happy.png and archive/aki/old.webp"));
    }

    #[test]
    fn shared_poses_naming_one_file_pass() {
        let dir = scratch("shared");
        std::fs::write(dir.join("aki/idle.png"), b"one picture").unwrap();
        let json = r#"{"masters": [
            {"companion": "aki", "pose": "sad", "kind": "core", "file": "aki/idle.png", "framing": "full_body"},
            {"companion": "aki", "pose": "hopeful", "kind": "core", "file": "aki/idle.png", "framing": "full_body"}
        ], "shared": []}"#;
        let shared: Manifest = serde_json::from_str(json).unwrap();
        let dups = duplicates(&dir).unwrap();
        let strays = stray(&dir, &shared).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(dups.is_empty(), "{dups:?}");
        assert!(strays.is_empty(), "{strays:?}");
        assert!(check(&shared).is_empty());
    }

    #[test]
    fn lists_match_each_other() {
        // A name has one use: the lists never overlap.
        let poses = poses();
        let mut all: Vec<&str> = poses.core.iter().map(String::as_str).collect();
        for (_, map) in poses.groups() {
            all.extend(map.keys().map(String::as_str));
        }
        all.extend(UNLISTED_ICONS.iter().chain(UNLISTED_MOMENTS));
        let mut unique = all.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(all.len(), unique.len());
    }
}

//! Checks the masters in `manifest.json` against two rules (hill,
//! 2026-11-21):
//!
//! - Use: a picture is outside `archive/` only if something uses it: it
//!   is the master of a pose in `poses.json`, a launcher icon, or a
//!   moment picture an exporter exports. Every other picture is in
//!   `archive/`.
//! - Framing: `face` and the launcher icons are headshots; every other
//!   master shows the companion's full body.
//!
//! It also checks `poses.json` itself: every fallback names a known pose
//! and none forms a cycle, and `missing` in the manifest lists only core
//! poses.
//!
//! Usage: `cargo run --manifest-path tools/art-check/Cargo.toml -- .`
//! (the argument is the repository root).

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Launcher icons, as AnkiQuest-Server's exporter names them
/// (`crates/xtask/src/art.rs`: `<companion>_icon*`, used by
/// AnkiQuest-Android's launcher).
const LAUNCHER_ICONS: &[&str] = &[
    "icon",
    "icon_mono",
    "icon_happy",
    "icon_worried",
    "icon_frozen",
    "icon_sleepy",
];

/// Moment, notification and leaderboard pictures AnkiQuest-Server's
/// exporter exports (`crates/xtask/src/art.rs`) for the add-on and the
/// Android app.
const MOMENTS: &[&str] = &[
    "achievement",
    "at_risk",
    "backlog",
    "challenge",
    "challenge_lost",
    "challenge_won",
    "comeback",
    "deck_conquered",
    "freeze",
    "freeze_used",
    "goal",
    "level_up",
    "minimum",
    "nudge",
    "place_1",
    "place_2",
    "place_3",
    "study",
];

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
    #[serde(default)]
    core_fallbacks: BTreeMap<String, String>,
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
    if LAUNCHER_ICONS.contains(&name) {
        return Use::LauncherIcon;
    }
    if MOMENTS.contains(&name) {
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
/// forms a cycle, and per pose listed twice.
fn pose_violations(poses: &Poses) -> Vec<String> {
    let mut out = Vec::new();
    let is_core = |p: &str| poses.core.iter().any(|c| c == p);
    let known = |p: &str| is_core(p) || poses.extensions.contains_key(p);

    for (i, p) in poses.core.iter().enumerate() {
        if poses.core[..i].contains(p) {
            out.push(format!("poses.json: core pose {p} is listed twice"));
        }
        if poses.extensions.contains_key(p) {
            out.push(format!("poses.json: {p} is both core and an extension"));
        }
    }
    for (ext, to) in &poses.extensions {
        if !known(to) {
            out.push(format!(
                "poses.json: extension {ext} falls back to unknown pose {to}"
            ));
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

    // Follow each chain: extensions first, then core_fallbacks. A chain
    // longer than the number of poses has a cycle.
    let next = |p: &str| poses.extensions.get(p).or(poses.core_fallbacks.get(p));
    let limit = poses.core.len() + poses.extensions.len();
    for start in poses.extensions.keys().chain(poses.core_fallbacks.keys()) {
        let mut at = start.as_str();
        let mut steps = 0;
        while let Some(to) = next(at) {
            at = to;
            steps += 1;
            if steps > limit {
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
    match stray(&root, &manifest) {
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
        let json = format!(
            r#"{{"core": ["face", "happy", "hopeful"], "extensions": {{{extensions}}}, "core_fallbacks": {{{core_fallbacks}}}}}"#
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
    fn poses_are_listed_once() {
        let found = pose_violations(&pose_list(r#""happy": "hopeful""#, ""));
        assert!(found
            .iter()
            .any(|f| f.contains("both core and an extension")));
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

    #[test]
    fn lists_match_each_other() {
        // A name has one use: the lists never overlap.
        let poses = poses();
        let mut all: Vec<&str> = poses.core.iter().map(String::as_str).collect();
        all.extend(poses.extensions.keys().map(String::as_str));
        all.extend(LAUNCHER_ICONS.iter().chain(MOMENTS));
        let mut unique = all.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(all.len(), unique.len());
    }
}

//! Checks the masters in `manifest.json` against two rules:
//!
//! - Use: a picture is outside `archive/` only if something uses it: it
//!   is the master of a pose in AnkiQuest-Server's
//!   `tests/fixtures/companion_poses.json`, a launcher icon, or a moment
//!   picture an exporter exports. Every other picture is in `archive/`.
//! - Framing: `face` and the launcher icons are headshots; every other
//!   master shows the companion's full body.
//!
//! Usage: `cargo run --manifest-path tools/art-check/Cargo.toml -- .`
//! (the argument is the repository root).

use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Core poses, from `companion_poses.json` (`core`).
const CORE: &[&str] = &[
    "face",
    "asleep",
    "happy",
    "streak",
    "sad",
    "hopeful",
    "frozen",
    "neglected",
];

/// Extensions, from `companion_poses.json` (`extensions`).
const EXTENSIONS: &[&str] = &[
    "streak_hot",
    "streak_blazing",
    "streak_legend",
    "streak_milestone",
    "frozen_2",
    "frozen_3",
    "welcome",
    "review",
    "hurt",
    "abandoned",
    "dead",
    "leech_defeated",
    "winner",
];

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

/// What uses a picture with this name.
fn use_of(name: &str) -> Use {
    if CORE.contains(&name) {
        return Use::Core;
    }
    if EXTENSIONS.contains(&name) {
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
fn violations(manifest: &Manifest) -> Vec<String> {
    let mut out = Vec::new();

    // Per-companion masters: used, of the right kind and framing.
    for m in &manifest.masters {
        let what = format!("{} ({} {})", m.file, m.companion, m.pose);
        let used = use_of(&m.pose);
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
    let manifest: Manifest = match std::fs::read_to_string(&path)
        .map_err(|e| e.to_string())
        .and_then(|text| serde_json::from_str(&text).map_err(|e| e.to_string()))
    {
        Ok(manifest) => manifest,
        Err(e) => {
            eprintln!("{}: {e}", path.display());
            return ExitCode::FAILURE;
        }
    };

    let mut found = violations(&manifest);
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

    fn manifest(pose: &str, kind: &str, framing: &str) -> Manifest {
        let json = format!(
            r#"{{"masters": [{{"companion": "aki", "pose": "{pose}", "kind": "{kind}", "file": "aki/{pose}.png", "framing": "{framing}"}}], "shared": []}}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn face_and_icons_are_headshots() {
        assert!(violations(&manifest("face", "core", "headshot")).is_empty());
        assert!(violations(&manifest("icon_mono", "picture", "headshot")).is_empty());
        assert_eq!(violations(&manifest("face", "core", "full_body")).len(), 1);
        assert_eq!(
            violations(&manifest("icon_happy", "picture", "bust")).len(),
            1
        );
    }

    #[test]
    fn other_pictures_are_full_body() {
        assert!(violations(&manifest("sad", "core", "full_body")).is_empty());
        assert!(violations(&manifest("winner", "extension", "full_body")).is_empty());
        assert!(violations(&manifest("place_1", "picture", "full_body")).is_empty());
        assert_eq!(violations(&manifest("sad", "core", "headshot")).len(), 1);
        assert_eq!(violations(&manifest("frozen", "core", "bust")).len(), 1);
    }

    #[test]
    fn unused_pictures_belong_in_the_archive() {
        let found = violations(&manifest("icon_fire", "picture", "headshot"));
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("archive/"));
    }

    #[test]
    fn kind_follows_the_pose_list() {
        assert_eq!(
            violations(&manifest("welcome", "core", "full_body")).len(),
            1
        );
        assert_eq!(violations(&manifest("goal", "core", "full_body")).len(), 1);
    }

    #[test]
    fn unknown_framing_is_rejected() {
        let json = r#"{"masters": [{"companion": "aki", "pose": "sad", "kind": "core", "file": "aki/sad.png", "framing": "portrait"}], "shared": []}"#;
        assert!(serde_json::from_str::<Manifest>(json).is_err());
    }

    #[test]
    fn lists_match_each_other() {
        // A name has one use: the lists never overlap.
        let all: Vec<&str> = [CORE, EXTENSIONS, LAUNCHER_ICONS, MOMENTS].concat();
        let mut unique = all.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(all.len(), unique.len());
    }
}

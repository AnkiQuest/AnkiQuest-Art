//! Checks the framing of every master in `manifest.json` (hill,
//! 2026-11-21): `face` and the launcher icons are headshots; every other
//! master shows the companion's full body.
//!
//! Usage: `cargo run --manifest-path tools/framing/Cargo.toml -- manifest.json`

use serde::Deserialize;
use std::process::ExitCode;

/// The launcher icon pictures, as AnkiQuest-Server's exporters name them
/// (`crates/xtask/src/art.rs`, `art/coverage.rs`).
const LAUNCHER_ICONS: &[&str] = &[
    "icon",
    "icon_mono",
    "icon_happy",
    "icon_worried",
    "icon_frozen",
    "icon_sleepy",
];

/// The pose that is a headshot for every companion.
const FACE: &str = "face";

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

#[derive(Deserialize)]
struct Master {
    companion: String,
    pose: String,
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

/// The framing a picture with this pose name must have.
fn required(pose: &str) -> Framing {
    if pose == FACE || LAUNCHER_ICONS.contains(&pose) {
        return Framing::Headshot;
    }
    Framing::FullBody
}

/// One line per master whose framing breaks the rule.
fn violations(manifest: &Manifest) -> Vec<String> {
    let mut out = Vec::new();

    // Per-companion masters follow their pose name.
    for m in &manifest.masters {
        let want = required(&m.pose);
        if m.framing != want {
            out.push(format!(
                "{} ({} {}): {:?}, needs {:?}",
                m.file, m.companion, m.pose, m.framing, want
            ));
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

fn main() -> ExitCode {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "manifest.json".into());
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::FAILURE;
        }
    };
    let manifest: Manifest = match serde_json::from_str(&text) {
        Ok(manifest) => manifest,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::FAILURE;
        }
    };

    let found = violations(&manifest);
    if found.is_empty() {
        println!("Every master has the framing its pose needs.");
        return ExitCode::SUCCESS;
    }
    for line in &found {
        println!("{line}");
    }
    println!("{} masters have the wrong framing.", found.len());
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(pose: &str, framing: &str) -> Manifest {
        let json = format!(
            r#"{{"masters": [{{"companion": "aki", "pose": "{pose}", "file": "aki/{pose}.png", "framing": "{framing}"}}], "shared": []}}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn face_and_icons_are_headshots() {
        assert_eq!(required("face"), Framing::Headshot);
        assert_eq!(required("icon_mono"), Framing::Headshot);
        assert!(violations(&manifest("face", "headshot")).is_empty());
        assert_eq!(violations(&manifest("face", "full_body")).len(), 1);
        assert_eq!(violations(&manifest("icon_happy", "bust")).len(), 1);
    }

    #[test]
    fn other_poses_are_full_body() {
        assert!(violations(&manifest("sad", "full_body")).is_empty());
        assert_eq!(violations(&manifest("sad", "headshot")).len(), 1);
        assert_eq!(violations(&manifest("frozen", "bust")).len(), 1);
        assert_eq!(violations(&manifest("icon_fire", "headshot")).len(), 1);
    }

    #[test]
    fn unknown_framing_is_rejected() {
        let json = r#"{"masters": [{"companion": "aki", "pose": "sad", "file": "aki/sad.png", "framing": "portrait"}], "shared": []}"#;
        assert!(serde_json::from_str::<Manifest>(json).is_err());
    }
}

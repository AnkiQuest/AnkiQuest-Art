//! Finding an export's master through `manifest.json`.

use std::path::{Path, PathBuf};

use art_check::Manifest;

/// Retired pictures; nothing in it is ever exported.
const ARCHIVE: &str = "archive/";

/// The manifest entry an export is made from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Source {
    /// A companion's master for a pose or picture: companion id, then pose.
    Pose(&'static str, &'static str),
    /// A picture of more than one companion, by its `shared` name.
    Shared(&'static str),
}

/// A companion's master for `pose`.
pub const fn pose(companion: &'static str, pose: &'static str) -> Source {
    Source::Pose(companion, pose)
}

/// The repository root: this tool lives in `tools/art-export`.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The master file of `source`, relative to the repository root. A master
/// in `archive/` is refused.
pub fn file(manifest: &Manifest, source: Source) -> Result<&str, String> {
    let file = match source {
        Source::Pose(companion, pose) => manifest
            .master(companion, pose)
            .ok_or(format!("manifest.json has no {companion} {pose}"))?,
        Source::Shared(name) => manifest
            .shared(name)
            .ok_or(format!("manifest.json has no shared picture {name}"))?,
    };
    if file.starts_with(ARCHIVE) {
        return Err(format!("{file} is retired; archive/ is never exported"));
    }
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(file: &str) -> Manifest {
        let json = format!(
            r#"{{"masters": [{{"companion": "aki", "pose": "sad", "kind": "core", "file": "{file}", "framing": "full_body"}}],
            "shared": [{{"name": "friends", "file": "duo/friends.png", "framing": "full_body"}}]}}"#
        );
        Manifest::parse(&json).unwrap()
    }

    #[test]
    fn resolves_poses_and_shared_pictures() {
        let manifest = manifest("aki/sad.png");
        assert_eq!(file(&manifest, pose("aki", "sad")), Ok("aki/sad.png"));
        assert_eq!(
            file(&manifest, Source::Shared("friends")),
            Ok("duo/friends.png")
        );
        assert!(file(&manifest, pose("aki", "happy")).is_err());
        assert!(file(&manifest, pose("cow", "sad")).is_err());
        assert!(file(&manifest, Source::Shared("trio")).is_err());
    }

    #[test]
    fn the_archive_is_never_exported() {
        let manifest = manifest("archive/aki/sad.png");
        let error = file(&manifest, pose("aki", "sad")).unwrap_err();
        assert!(error.contains("never exported"), "{error}");
    }
}

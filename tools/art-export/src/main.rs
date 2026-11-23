//! Writes the clients' copies of the masters in `manifest.json`:
//!
//! - `android <res-dir>`: AnkiQuest-Android's drawables, as
//!   `<res-dir>/drawable-nodpi/<name>.webp`.
//! - `addon <addon-dir>`: AnkiQuest-Desktop's add-on pictures, as
//!   `<addon-dir>/art/<name>.png` and `<addon-dir>/<companion>_face.png`.
//!
//! Every export names a manifest entry (companion and pose, or a shared
//! picture), so a renamed or retired master fails the tests instead of
//! exporting stale art; nothing in `archive/` is exported. The same master
//! gives the same bytes on every run. Linux builds are canonical: another
//! platform or compiler may encode a few pixels differently.
//!
//! Usage: `cargo run --locked --release --manifest-path
//! tools/art-export/Cargo.toml -- android|addon <out-dir>`.

mod addon;
mod android;
mod masters;
mod resample;

use std::path::PathBuf;
use std::process::ExitCode;

use art_check::Manifest;

const USAGE: &str = "usage: art-export android <res-dir>\n       art-export addon <addon-dir>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [client, out] = args.as_slice() else {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    };
    let out = PathBuf::from(out);
    let result = Manifest::read(&masters::root()).and_then(|manifest| match client.as_str() {
        "android" => android::export(&manifest, &out),
        "addon" => addon::export(&manifest, &out),
        _ => Err(USAGE.to_string()),
    });
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::masters::Source;

    /// A new empty folder for one test's output.
    pub fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("art-export-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Every export's master, by client and name.
    fn sources() -> Vec<(String, Source)> {
        let android = android::ART
            .iter()
            .map(|art| (format!("android {}", art.name), art.source));
        let addon = addon::PICTURES
            .iter()
            .map(|p| (format!("addon {}", p.file), p.source));
        android.chain(addon).collect()
    }

    #[test]
    fn every_export_names_a_master_outside_the_archive() {
        let root = masters::root();
        let manifest = Manifest::read(&root).unwrap();
        for (name, source) in sources() {
            let file = masters::file(&manifest, source).unwrap_or_else(|e| panic!("{name}: {e}"));
            assert!(root.join(file).is_file(), "{name}: {file} is not on disk");
        }
    }

    #[test]
    fn names_are_unique() {
        let mut names: Vec<String> = sources().into_iter().map(|(name, _)| name).collect();
        let count = names.len();
        names.sort();
        names.dedup();
        assert_eq!(count, names.len());
    }
}

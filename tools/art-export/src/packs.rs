//! Companion packs, as `docs/packs.md` describes them: one
//! `pack-<id>.zip` per companion in `companions.json`, holding
//! `companion.json` and one WebP per picture, and the index `packs.json`
//! that lists every pack with its size and SHA-256.
//!
//! A pack carries every master of its companion that `poses.json` names
//! (core poses, extensions, moment pictures and launcher icons); the apps
//! resolve a missing name through the fallbacks in `packs.json`. A
//! companion without every core pose fails the build.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use art_check::{Manifest, Poses};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::android::encode;
use crate::masters;
use crate::resample::{load, shrink, target};
use crate::zip;

/// The pack format this tool writes; the apps refuse a higher one.
pub const FORMAT: u32 = 1;

/// The index of the release, next to the packs.
pub const INDEX: &str = "packs.json";

/// The index inside each pack.
pub const PACK_INDEX: &str = "companion.json";

/// Largest edge of a pose or moment picture: the large streak widget's.
const POSE_SIZE: u32 = 1024;

/// Largest edge of a launcher icon.
const ICON_SIZE: u32 = 432;

/// WebP quality of every picture in a pack.
const QUALITY: f32 = 88.0;

/// What `packs.json` says about itself.
const INDEX_ABOUT: &str = "The companion packs of this AnkiQuest-Art release \
(docs/packs.md). core lists the poses every pack has; fallbacks maps every \
other name to the name shown when a pack has no picture for it.";

/// `companions.json`: the companions to pack, in the apps' order.
#[derive(Deserialize)]
struct Companions {
    companions: Vec<Companion>,
}

/// One companion to pack.
#[derive(Deserialize)]
struct Companion {
    id: String,
    name: String,
    /// A licence text to carry, as a path from the repository root.
    #[serde(default)]
    notice: Option<String>,
}

/// `companion.json` inside a pack.
#[derive(Serialize)]
struct PackIndex<'a> {
    format: u32,
    id: &'a str,
    name: &'a str,
    version: u32,
    licence: String,
    credit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    notice: Option<String>,
    poses: BTreeMap<String, String>,
}

/// `packs.json`, the release's index.
#[derive(Serialize)]
struct ReleaseIndex<'a> {
    #[serde(rename = "_about")]
    about: &'a str,
    format: u32,
    tag: &'a str,
    commit: &'a str,
    version: u32,
    core: &'a [String],
    fallbacks: BTreeMap<String, String>,
    companions: Vec<Entry>,
}

/// One pack in `packs.json`.
#[derive(Serialize)]
struct Entry {
    id: String,
    name: String,
    version: u32,
    file: String,
    size: u64,
    sha256: String,
}

/// Writes every pack and `packs.json` into `out` for release `tag`
/// (`art-<N>`) built from `commit`.
pub fn export(manifest: &Manifest, tag: &str, commit: &str, out: &Path) -> Result<(), String> {
    let version = version(tag)?;
    let root = masters::root();
    let poses = Poses::read(&root)?;
    let companions = read_companions(&root)?;
    fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;

    // One pack per companion, then the index that lists them.
    let mut entries = Vec::new();
    for companion in &companions.companions {
        let bytes = pack(manifest, &poses, companion, version)?;
        let file = format!("pack-{}.zip", companion.id);
        let path = out.join(&file);
        fs::write(&path, &bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        println!("{file}: {} bytes", bytes.len());
        entries.push(Entry {
            id: companion.id.clone(),
            name: companion.name.clone(),
            version,
            file,
            size: bytes.len() as u64,
            sha256: hex(&Sha256::digest(&bytes)),
        });
    }
    let index = ReleaseIndex {
        about: INDEX_ABOUT,
        format: FORMAT,
        tag,
        commit,
        version,
        core: &poses.core,
        fallbacks: fallbacks(&poses),
        companions: entries,
    };
    let path = out.join(INDEX);
    fs::write(&path, json(&index)?).map_err(|e| format!("{}: {e}", path.display()))
}

/// The release number of `tag`, `art-<N>`.
fn version(tag: &str) -> Result<u32, String> {
    tag.strip_prefix("art-")
        .and_then(|n| n.parse::<u32>().ok())
        .filter(|n| *n > 0)
        .ok_or(format!("{tag}: a tag is art-<N>"))
}

/// Reads and checks `companions.json`.
fn read_companions(root: &Path) -> Result<Companions, String> {
    let path = root.join("companions.json");
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let companions: Companions =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    for companion in &companions.companions {
        if !valid_id(&companion.id) {
            return Err(format!(
                "companions.json: {} is not a valid id",
                companion.id
            ));
        }
    }
    Ok(companions)
}

/// Whether `id` is a pack id: a lower-case letter, then up to 31 lower-case
/// letters, digits or underscores.
pub fn valid_id(id: &str) -> bool {
    let mut chars = id.chars();
    let first = chars.next().is_some_and(|c| c.is_ascii_lowercase());
    let rest = chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
    first && rest && id.len() <= 32
}

/// Every non-core name mapped to its fallback, from all groups of `poses`.
fn fallbacks(poses: &Poses) -> BTreeMap<String, String> {
    poses
        .groups()
        .into_iter()
        .flat_map(|(_, map)| map.clone())
        .collect()
}

/// The bytes of `companion`'s pack.
fn pack(
    manifest: &Manifest,
    poses: &Poses,
    companion: &Companion,
    version: u32,
) -> Result<Vec<u8>, String> {
    let id = companion.id.as_str();
    let names = names(manifest, poses, id)?;

    // Encode each master file once; poses that share a master share its picture.
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let mut by_master: BTreeMap<String, String> = BTreeMap::new();
    let mut map = BTreeMap::new();
    for name in &names {
        let master = masters::pose_file(manifest, id, name)?.to_string();
        if let Some(file) = by_master.get(&master) {
            map.insert(name.clone(), file.clone());
            continue;
        }
        let file = picture_name(&master)?;
        if files.contains_key(&file) {
            return Err(format!("{id}: two masters would be packed as {file}"));
        }
        let size = if poses.icons.contains_key(name) {
            ICON_SIZE
        } else {
            POSE_SIZE
        };
        files.insert(file.clone(), render(&master, size)?);
        by_master.insert(master, file.clone());
        map.insert(name.clone(), file);
    }

    // The pack's index, then every picture, sorted.
    let index = PackIndex {
        format: FORMAT,
        id,
        name: &companion.name,
        version,
        licence: one(manifest, id, |m| m.licence.as_deref(), "licence")?,
        credit: credit(manifest, id)?,
        notice: notice(companion)?,
        poses: map,
    };
    files.insert(PACK_INDEX.to_string(), json(&index)?);
    Ok(zip::stored(&files))
}

/// The names `poses.json` lists that `id` has a master for, failing when a
/// core pose has none.
fn names(manifest: &Manifest, poses: &Poses, id: &str) -> Result<Vec<String>, String> {
    let known = |name: &str| {
        poses.core.iter().any(|c| c == name)
            || poses.groups().iter().any(|(_, map)| map.contains_key(name))
    };
    let mut names: Vec<String> = manifest
        .masters
        .iter()
        .filter(|m| m.companion == id && known(&m.pose))
        .map(|m| m.pose.clone())
        .collect();
    names.sort();
    names.dedup();
    if let Some(core) = poses.core.iter().find(|c| !names.contains(c)) {
        return Err(format!("{id} has no master for core pose {core}"));
    }
    Ok(names)
}

/// The file name a master gets in a pack: its own name as WebP.
fn picture_name(master: &str) -> Result<String, String> {
    Path::new(master)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| format!("{stem}.webp"))
        .ok_or(format!("{master}: no file name"))
}

/// The master at `file` shrunk to fit `size` and encoded as WebP.
fn render(file: &str, size: u32) -> Result<Vec<u8>, String> {
    let master = load(&masters::root().join(file))?;
    let (width, height) = target(master.width(), master.height(), size);
    encode(&shrink(master, width, height), QUALITY)
}

/// The single value `field` has across `id`'s masters.
fn one(
    manifest: &Manifest,
    id: &str,
    field: impl Fn(&art_check::Master) -> Option<&str>,
    what: &str,
) -> Result<String, String> {
    let mut values: Vec<&str> = Vec::new();
    for master in manifest.masters.iter().filter(|m| m.companion == id) {
        let value = field(master).ok_or(format!("{}: no {what}", master.file))?;
        if !values.contains(&value) {
            values.push(value);
        }
    }
    match values.as_slice() {
        [value] => Ok((*value).to_string()),
        [] => Err(format!("{id} has no masters")),
        _ => Err(format!("{id}: masters differ in {what}: {values:?}")),
    }
}

/// Everyone `id`'s masters credit, in order of first appearance.
fn credit(manifest: &Manifest, id: &str) -> Result<String, String> {
    let mut authors: Vec<&str> = Vec::new();
    for master in manifest.masters.iter().filter(|m| m.companion == id) {
        let author = master
            .author
            .as_deref()
            .ok_or(format!("{}: no author", master.file))?;
        if !authors.contains(&author) {
            authors.push(author);
        }
    }
    Ok(authors.join("; "))
}

/// The licence text `companion` carries, if any.
fn notice(companion: &Companion) -> Result<Option<String>, String> {
    let Some(file) = &companion.notice else {
        return Ok(None);
    };
    let path = masters::root().join(file);
    fs::read_to_string(&path)
        .map(Some)
        .map_err(|e| format!("{}: {e}", path.display()))
}

/// `value` as pretty JSON with a final newline.
fn json(value: &impl Serialize) -> Result<Vec<u8>, String> {
    let mut text = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    text.push('\n');
    Ok(text.into_bytes())
}

/// Lower-case hex of `bytes`.
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_give_versions() {
        assert_eq!(version("art-8"), Ok(8));
        assert!(version("art-0").is_err());
        assert!(version("art-x").is_err());
        assert!(version("8").is_err());
    }

    #[test]
    fn ids_are_short_lower_case_words() {
        assert!(valid_id("aki"));
        assert!(valid_id("anki_tan"));
        assert!(valid_id("quest_chan2"));
        assert!(!valid_id(""));
        assert!(!valid_id("Aki"));
        assert!(!valid_id("1aki"));
        assert!(!valid_id("../aki"));
        assert!(!valid_id("anki-tan"));
        assert!(!valid_id(&"a".repeat(33)));
    }

    #[test]
    fn every_listed_companion_has_its_core_poses() {
        let root = masters::root();
        let manifest = Manifest::read(&root).unwrap();
        let poses = Poses::read(&root).unwrap();
        for companion in read_companions(&root).unwrap().companions {
            let names = names(&manifest, &poses, &companion.id).unwrap();
            assert!(
                poses.core.iter().all(|c| names.contains(c)),
                "{}",
                companion.id
            );
            one(
                &manifest,
                &companion.id,
                |m| m.licence.as_deref(),
                "licence",
            )
            .unwrap();
            credit(&manifest, &companion.id).unwrap();
            notice(&companion).unwrap();
        }
    }

    #[test]
    fn a_companion_without_a_core_pose_fails() {
        let manifest = Manifest::parse(
            r#"{"masters": [{"companion": "aki", "pose": "face", "kind": "core", "file": "aki/face.png", "framing": "headshot"}], "shared": []}"#,
        )
        .unwrap();
        let poses = Poses::read(&masters::root()).unwrap();
        let error = names(&manifest, &poses, "aki").unwrap_err();
        assert!(error.contains("core pose"), "{error}");
    }

    #[test]
    fn fallbacks_cover_every_non_core_name() {
        let poses = Poses::read(&masters::root()).unwrap();
        let map = fallbacks(&poses);
        assert_eq!(map.get("streak_hot").map(String::as_str), Some("streak"));
        assert_eq!(map.get("place_1").map(String::as_str), Some("winner"));
        assert_eq!(map.get("icon").map(String::as_str), Some("face"));
        assert!(poses.core.iter().all(|c| !map.contains_key(c)));
    }
}

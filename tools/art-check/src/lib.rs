//! The types of `manifest.json`, shared by the check (`src/main.rs`) and
//! the exporters in `tools/art-export`.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

/// How much of the companion a picture shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Framing {
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
pub enum Kind {
    Core,
    Extension,
    Picture,
}

/// One companion's master for one pose or picture.
#[derive(Debug, Deserialize)]
pub struct Master {
    pub companion: String,
    pub pose: String,
    pub kind: Kind,
    pub file: String,
    pub framing: Framing,
    /// SPDX licence of the picture.
    #[serde(default)]
    pub licence: Option<String>,
    /// Who to credit for the picture.
    #[serde(default)]
    pub author: Option<String>,
}

/// A picture of more than one companion.
#[derive(Debug, Deserialize)]
pub struct Shared {
    pub name: String,
    pub file: String,
    pub framing: Framing,
}

/// The parts of `manifest.json` the tools read.
#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub masters: Vec<Master>,
    pub shared: Vec<Shared>,
}

impl Manifest {
    /// Reads `manifest.json` in the repository at `root`.
    pub fn read(root: &Path) -> Result<Manifest, String> {
        let path = root.join("manifest.json");
        std::fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|text| Manifest::parse(&text))
            .map_err(|e| format!("{}: {e}", path.display()))
    }

    /// Parses the text of a manifest.
    pub fn parse(text: &str) -> Result<Manifest, String> {
        serde_json::from_str(text).map_err(|e| e.to_string())
    }

    /// The file of `companion`'s master for `pose`.
    pub fn master(&self, companion: &str, pose: &str) -> Option<&str> {
        self.masters
            .iter()
            .find(|m| m.companion == companion && m.pose == pose)
            .map(|m| m.file.as_str())
    }

    /// The file of the shared picture `name`.
    pub fn shared(&self, name: &str) -> Option<&str> {
        self.shared
            .iter()
            .find(|s| s.name == name)
            .map(|s| s.file.as_str())
    }
}

/// `poses.json`: the pose names and their fallbacks.
#[derive(Deserialize)]
pub struct Poses {
    pub core: Vec<String>,
    pub extensions: BTreeMap<String, String>,
    pub pictures: BTreeMap<String, String>,
    pub icons: BTreeMap<String, String>,
    #[serde(default)]
    pub core_fallbacks: BTreeMap<String, String>,
}

impl Poses {
    /// Reads `poses.json` in the repository at `root`.
    pub fn read(root: &Path) -> Result<Poses, String> {
        let path = root.join("poses.json");
        std::fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|text| serde_json::from_str(&text).map_err(|e| e.to_string()))
            .map_err(|e| format!("{}: {e}", path.display()))
    }

    /// The groups that map each name to its fallback, with their names.
    pub fn groups(&self) -> [(&'static str, &BTreeMap<String, String>); 3] {
        [
            ("extension", &self.extensions),
            ("picture", &self.pictures),
            ("icon", &self.icons),
        ]
    }

    /// The pose, picture or icon a companion without `name` shows.
    pub fn fallback(&self, name: &str) -> Option<&String> {
        self.extensions
            .get(name)
            .or(self.pictures.get(name))
            .or(self.icons.get(name))
            .or(self.core_fallbacks.get(name))
    }
}

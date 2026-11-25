"""Packs a release's assets from the exporters' output.

Usage: python3 tools/release/bundle.py <tag> <commit> <android-res-dir> <addon-dir> <out-dir>

<tag> and <commit> are the release's tag (art-<N>) and the Art commit it is
built from. <android-res-dir> and <addon-dir> are what `art-export android`
and `art-export addon` wrote. <out-dir> receives:

- android-drawables.zip: drawable-nodpi/*.webp, and Anki-tan's licence
  as LICENSE-Anki-tan.md at the root (Android's resource build accepts
  only pictures in drawable-nodpi/)
- addon-art.zip: art/*.png and *_face.png, and Anki-tan's licence as
  art/LICENSE-Anki-tan.md
- masters.zip: every master manifest.json lists, at its path here, and
  the licences
- archive.zip: everything under archive/, at its path here
- manifest.json, and poses.json when this repository has one
- ankiquest-art.json: the pin file consumers commit, naming the tag, the
  commit and the SHA-256 of every asset above and of the companion packs
  (packs.json and pack-<id>.zip, which `art-export packs` wrote to
  <out-dir> before)
- SHA256SUMS: the SHA-256 of every other asset

The same input gives the same bytes: entries are sorted, stored
uncompressed (the pictures are compressed already) with a fixed time and
mode, and carry no extra fields or comments.
"""

import hashlib
import json
import re
import shutil
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

# Licence texts that go with the masters.
LICENCES = ["LICENSE", "anki-tan/LICENSE"]

# Anki-tan's licence (CC BY-SA 4.0), which goes with every copy of her art.
ANKI_TAN_LICENCE = ROOT / "anki-tan/LICENSE"

# Where each export zip carries it.
ANDROID_LICENCE = "LICENSE-Anki-tan.md"
ADDON_LICENCE = "art/LICENSE-Anki-tan.md"

# What the pin file says about itself.
PIN_ABOUT = (
    "The AnkiQuest-Art release this repository uses. AnkiQuest-Sync updates "
    "it when Art publishes a release; don't edit it by hand."
)

# The assets the pin file lists, in its order; poses.json only when present.
PINNED = [
    "android-drawables.zip",
    "addon-art.zip",
    "masters.zip",
    "archive.zip",
    "manifest.json",
    "poses.json",
]

# The earliest time a zip entry can hold.
EPOCH = (1980, 1, 1, 0, 0, 0)


def files_under(folder):
    """Every file under `folder`, as (path inside the zip, path on disk)."""
    return [
        (path.relative_to(folder).as_posix(), path)
        for path in folder.rglob("*")
        if path.is_file()
    ]


def write_zip(target, entries):
    """Writes `entries` (path inside the zip, path on disk) to `target`."""
    names = [name for name, _ in entries]
    if len(set(names)) != len(names):
        sys.exit(f"{target.name}: an entry is listed twice")
    with zipfile.ZipFile(target, "w", zipfile.ZIP_STORED) as archive:
        for name, path in sorted(entries):
            info = zipfile.ZipInfo(name, date_time=EPOCH)
            info.create_system = 3
            info.external_attr = 0o100644 << 16
            info.compress_type = zipfile.ZIP_STORED
            archive.writestr(info, path.read_bytes())


def masters():
    """Every master file manifest.json lists, and the licences."""
    manifest = json.loads((ROOT / "manifest.json").read_text(encoding="utf-8"))
    files = {entry["file"] for entry in manifest["masters"] + manifest["shared"]}
    files.update(LICENCES)
    for file in files:
        if file.startswith(("archive/", "docs/")):
            sys.exit(f"masters.zip: {file} is not a master")
    return [(file, ROOT / file) for file in files]


def sha256(path):
    """Lower-case hex SHA-256 of the file at `path`."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def pin(out, tag, commit):
    """The pin file's text for the assets in `out`."""
    packs = ["packs.json"] + sorted(path.name for path in out.glob("pack-*.zip"))
    names = [name for name in PINNED + packs if (out / name).is_file()]
    assets = {name: sha256(out / name) for name in names}
    value = {"_about": PIN_ABOUT, "tag": tag, "commit": commit, "assets": assets}
    return json.dumps(value, indent=2) + "\n"


def main():
    if len(sys.argv) != 6:
        sys.exit(__doc__.strip().splitlines()[2])
    tag, commit = sys.argv[1:3]
    if not re.fullmatch(r"art-[1-9][0-9]*", tag):
        sys.exit(f"{tag}: a tag is art-<N>")
    if not re.fullmatch(r"[0-9a-f]{40}", commit):
        sys.exit(f"{commit}: a commit is 40 hex digits")
    android, addon, out = (Path(arg) for arg in sys.argv[3:])
    out.mkdir(parents=True, exist_ok=True)

    # Each export zip carries the licence of the art it holds.
    write_zip(
        out / "android-drawables.zip",
        files_under(android) + [(ANDROID_LICENCE, ANKI_TAN_LICENCE)],
    )
    write_zip(
        out / "addon-art.zip",
        files_under(addon) + [(ADDON_LICENCE, ANKI_TAN_LICENCE)],
    )
    for name, entry in [
        ("android-drawables.zip", ANDROID_LICENCE),
        ("addon-art.zip", ADDON_LICENCE),
    ]:
        with zipfile.ZipFile(out / name) as archive:
            if archive.read(entry) != ANKI_TAN_LICENCE.read_bytes():
                sys.exit(f"{name}: {entry} is not anki-tan/LICENSE")
    write_zip(out / "masters.zip", masters())
    archive = [("archive/" + name, path) for name, path in files_under(ROOT / "archive")]
    write_zip(out / "archive.zip", archive)
    shutil.copyfile(ROOT / "manifest.json", out / "manifest.json")
    if (ROOT / "poses.json").is_file():
        shutil.copyfile(ROOT / "poses.json", out / "poses.json")

    pin_path = out / "ankiquest-art.json"
    pin_path.write_text(pin(out, tag, commit), encoding="utf-8", newline="\n")

    # One line per asset, sorted, in sha256sum's format.
    lines = [
        f"{sha256(path)}  {path.name}\n"
        for path in sorted(out.iterdir())
        if path.name != "SHA256SUMS"
    ]
    (out / "SHA256SUMS").write_text("".join(lines), encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()

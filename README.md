# AnkiQuest-Art

The companion art of [AnkiQuest](https://github.com/AnkiQuest): every
master picture of Aki, Ankilope, Bluma and Anki-tan, the archive of
pictures that are no longer masters, and the records of how they were
made. AnkiQuest-Server, AnkiQuest-Desktop, AnkiQuest-Android and
AnkiQuest.github.io take the art from a pinned [release](#releases) of
this repository, so no code repository checks in art.

## Layout

| Path | What |
|---|---|
| `aki/`, `ankilope/` | Aki's and Ankilope's masters. |
| `cow/` | Bluma's masters (companion id `cow`). |
| `anki-tan/` | Anki-tan's masters (companion id `anki_tan`). |
| `anki-tan/chibi/` | Anki-tan chibi's masters (companion id `anki_tan_chibi`). |
| `duo/` | Pictures of more than one companion: `friends.png`, Aki and Ankilope. |
| `archive/` | Pictures that are no longer masters; never exported to a client, released whole for the gallery ([README](archive/README.md)). |
| `docs/` | Generation records, prompts, measurements and review sheets. |
| `manifest.json` | Every master (companion, pose, kind, framing, file, licence, origin and author), the core poses still missing, and what the archive holds. |
| `poses.json` | Every slot a companion has: the core poses, extensions, moment pictures and launcher icons, each with its fallback, and the core poses' stopgap fallbacks; checked by `tools/art-check`. |
| `RENAMES.md` | Where each file of AnkiQuest-Server's `static/` art is now. |
| `tools/art-check/` | The check of `manifest.json`: every picture outside `archive/` is used, and `face` and launcher icons are headshots while every other master is full body (`cargo run --manifest-path tools/art-check/Cargo.toml -- .`). |
| `tools/art-export/` | The exporter: Android drawables and the desktop add-on's pictures, made from the masters `manifest.json` names ([Exports](#exports)). |
| `tools/release/` | Builds a release's assets ([Releases](#releases)). |

A master is `<companion>/<pose>.png`, a transparent PNG. Pose names and
their fallbacks are in [`poses.json`](poses.json), which the server and
clients read from this submodule, together with the moment pictures
(`pictures`: `achievement`, `place_1` and so on) and launcher icons
(`icons`); `manifest.json` marks those `picture`. A companion with no
master for a name shows a fallback: an extension, picture or icon follows
its chain until something is drawn, reaching a core pose; a core pose with
no master shows its `core_fallbacks` entry (`happy` and `streak` show
`hopeful`). The manifest's `origin` tells Shigeyuki's
pack files (`shigeyuki`, with their path in the pack) from art AnkiQuest
made: derivatives of his Anki-tan (`derivative`) and AnkiQuest's own
companions (`ankiquest`).

Three rules, checked by `tools/art-check`:

- A picture is outside `archive/` only if something uses it: it is a
  pose master, a launcher icon or a moment picture an exporter exports.
  Everything else is in `archive/`.
- `face` and the launcher icons (`icon`, `icon_mono`, `icon_happy`,
  `icon_worried`, `icon_frozen`, `icon_sleepy`) are headshots of the
  face; every other master shows the full body. `framing` in the
  manifest records it.
- No two pictures outside `docs/` and `tools/`, `archive/` included,
  have the same bytes: poses that share a picture name one file in
  `manifest.json`.

Where one picture shows several poses, their manifest entries name the
same file instead of copies: `anki-tan/chibi/asleep.png` is ten of Anki-tan
chibi's poses.

Nothing here is generated: smaller sizes and other copies are made by
the release workflow ([Releases](#releases)) and are not committed here or
in the consumers.

## Exports

`tools/art-export` writes each client's copies of the masters into a
folder it is given; the release workflow runs it and publishes the result. Every copy names a manifest entry (companion and
pose, or a shared picture), never a path, and nothing in `archive/` is
exported; its tests fail when an entry is renamed or retired.

| Client | Command | Writes |
|---|---|---|
| AnkiQuest-Android | `cargo run --locked --release --manifest-path tools/art-export/Cargo.toml -- android <res-dir>` | `<res-dir>/drawable-nodpi/<name>.webp`: square lossy WebPs (alpha quality 100), Lanczos-downscaled to 1024 px for the streak widget's poses, 432 px for launcher icons and 336 or 168 px for moment pictures, never upscaled; the themed icons' monochrome layers are the master's line art. |
| AnkiQuest-Desktop | `cargo run --locked --release --manifest-path tools/art-export/Cargo.toml -- addon <addon-dir>` | `<addon-dir>/art/<name>.png` (the poses) and `<addon-dir>/<companion>_face.png` (the share card's and toolbar's faces): 256 px, 256-colour palette PNGs. |

The tables of what each client gets are in
[`android.rs`](tools/art-export/src/android.rs) and
[`addon.rs`](tools/art-export/src/addon.rs). The same master gives the
same bytes on every run. Linux builds are canonical: another platform or
C compiler can encode a few pixels differently, which is why consumers
take the release's copies instead of building the exporter themselves.

## Releases

On every push to master that changes the art (a companion folder,
`duo/`, `archive/`, `LICENSE`), `manifest.json`, `poses.json` or the tools
under `tools/`, the [release workflow](.github/workflows/release.yml)
builds on Linux and publishes a release; it can also be run by hand.
Tags are `art-<N>`: one more than the highest `art-*` tag, starting at
`art-1`. A commit is released once, and releases are published one at a
time. After publishing, the workflow sends `art-released` with the tag to
AnkiQuest-Sync, which opens a pull request in each consumer that moves its
pin to the new release.

| Asset | Holds |
|---|---|
| `android-drawables.zip` | `drawable-nodpi/<name>.webp`, as `art-export android` writes them, and Anki-tan's licence (`anki-tan/LICENSE`, CC BY-SA 4.0) as `LICENSE-Anki-tan.md` at the root, outside `drawable-nodpi/`, which Android's resource build keeps for pictures. |
| `addon-art.zip` | `art/<name>.png` and `<companion>_face.png`, as `art-export addon` writes them, and Anki-tan's licence as `art/LICENSE-Anki-tan.md`. |
| `masters.zip` | Every master `manifest.json` lists, at its path here, with `LICENSE` and `anki-tan/LICENSE`: for AnkiQuest-Server's share cards and web pictures. |
| `archive.zip` | Everything under `archive/` at its path here (pictures, READMEs and licences), for the gallery. |
| `manifest.json`, `poses.json` | This repository's files; `poses.json` once it exists. |
| `ankiquest-art.json` | The pin file for this release (below). |
| `SHA256SUMS` | The SHA-256 of every other asset. |

Every export zip carries the licence of the art it holds. The release
notes name the Art commit and every asset's SHA-256. Each
zip has sorted entries, stored uncompressed with a fixed time and mode and
no extra metadata, so building the same commit again gives the same
bytes; the workflow builds every asset twice and compares them before
publishing, and the pull request check does the same.

### Pinning a release

A consumer commits `ankiquest-art.json` at its repository root, a copy of
the release's asset of the same name. AnkiQuest-Sync updates it; nobody
edits it by hand.

```json
{
  "_about": "The AnkiQuest-Art release this repository uses. AnkiQuest-Sync updates it when Art publishes a release; don't edit it by hand.",
  "tag": "art-12",
  "commit": "<the Art commit the release was built from>",
  "assets": {
    "android-drawables.zip": "<sha256 hex>",
    "addon-art.zip": "<sha256 hex>",
    "masters.zip": "<sha256 hex>",
    "archive.zip": "<sha256 hex>",
    "manifest.json": "<sha256 hex>",
    "poses.json": "<sha256 hex>"
  }
}
```

Every consumer has the same file and reads only the assets it needs, from
`https://github.com/AnkiQuest/AnkiQuest-Art/releases/download/<tag>/<asset>`.
It checks each download against the SHA-256 in the file before using it
and caches by hash, so a build works offline after one download. A Nix
flake reads the file with `builtins.fromJSON (builtins.readFile
./ankiquest-art.json)` and passes the hex hash to `pkgs.fetchurl`, so it
keeps no second copy. Before a consumer's first release, its file has
`"tag": null` and the consumer keeps what it uses now. Consumers never
build the exporter or take this repository as a submodule.

## The matrix

The art is a matrix of companions by poses. The goal is a full matrix:
every pose drawn for every live companion, with no pose unique to one
companion. Missing drawings are tracked in AnkiQuest-Server:
[#57](https://github.com/AnkiQuest/AnkiQuest-Server/issues/57) for poses
and [#116](https://github.com/AnkiQuest/AnkiQuest-Server/issues/116) for
icons and moment pictures. Until they are drawn, a missing pose shows the
fallback `poses.json` gives it.

## Licences

| Path | Licence |
|---|---|
| everything except the rows below | AGPL-3.0-or-later, [LICENSE](LICENSE) |
| `anki-tan/` (with `chibi/`) | CC BY-SA 4.0, by Shigeyuki, [anki-tan/LICENSE](anki-tan/LICENSE) |
| `archive/anki-tan/` | CC BY-SA 4.0, by Shigeyuki, [archive/anki-tan/LICENSE](archive/anki-tan/LICENSE) |
| `archive/anki-tan-pack/` | CC BY-SA 4.0, by Shigeyuki, [archive/anki-tan-pack/LICENSE](archive/anki-tan-pack/LICENSE) |

Anki-tan is by Shigeyuki (https://www.patreon.com/Shigeyuki); every
derivative of her pictures is CC BY-SA 4.0 too. She is an unofficial
character, not related to the official Anki project.

The history of every master up to the move is kept: this repository was
split from AnkiQuest-Server with its commits.

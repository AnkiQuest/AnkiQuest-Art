# Companion packs

A companion pack is one companion's pictures in one zip, with an index the
apps read. Every release publishes one pack per companion in
[`companions.json`](../companions.json) and the release index
`packs.json`, written by `art-export packs` (`tools/art-export/src/packs.rs`).

The apps draw companions only from packs: a new companion ships as a new
pack, without an app update, and players can import packs of their own.
No app has code for one companion.

## Release index: `packs.json`

```json
{
  "_about": "...",
  "format": 1,
  "tag": "art-8",
  "commit": "<the Art commit the release was built from>",
  "version": 8,
  "core": ["face", "asleep", "happy", "streak", "sad", "hopeful", "frozen", "neglected"],
  "fallbacks": { "streak_hot": "streak", "place_1": "winner", "icon": "face", "...": "..." },
  "companions": [
    {
      "id": "aki",
      "name": "Aki",
      "version": 8,
      "file": "pack-aki.zip",
      "size": 4036076,
      "sha256": "<sha256 hex of pack-aki.zip>"
    }
  ]
}
```

| Field | Meaning |
|---|---|
| `format` | The format of the index and its packs. An app refuses a format higher than it knows. |
| `tag`, `commit` | The release and the Art commit it was built from. |
| `version` | `N` of the tag `art-<N>`. |
| `core` | The core poses, from `poses.json`. Every pack has a picture for each. |
| `fallbacks` | Every other name (extensions, moment pictures, launcher icons) mapped to the name shown when a pack has no picture for it, from `poses.json`. Each chain ends at a core pose. |
| `companions` | One entry per pack, in the order the apps list companions. `file` is the release asset; `size` and `sha256` are its byte count and SHA-256. |

The names an app knows are `core` and the keys of `fallbacks` of the
newest index it has: the one in its own build, or the one it downloaded
last if that is newer.

## Pack: `pack-<id>.zip`

A zip with no folders: `companion.json` and the pictures.

```json
{
  "format": 1,
  "id": "anki_tan",
  "name": "Anki-tan",
  "version": 8,
  "licence": "CC-BY-SA-4.0",
  "credit": "Shigeyuki; AnkiQuest, after Shigeyuki",
  "notice": "<the text of anki-tan/LICENSE>",
  "poses": { "face": "face.webp", "happy": "happy.webp", "welcome": "happy.webp", "...": "..." }
}
```

| Field | Meaning |
|---|---|
| `format` | As in `packs.json`. |
| `id` | The companion id, as the server stores the choice: a lower-case letter, then up to 31 lower-case letters, digits or underscores. |
| `name` | Shown in the apps; not translated. |
| `version` | The release number for Art's packs; any positive integer for an imported pack. |
| `licence` | SPDX identifier of the pictures. |
| `credit` | Who to credit, shown with the licence. |
| `notice` | Optional. A licence text the pictures' licence asks to carry; the apps show it with the credit. |
| `poses` | Every name the pack has a picture for, mapped to a file in the zip. Several names may share a file. |

Art's packs hold every master of the companion that `poses.json` names,
as lossy WebP with full-quality alpha: poses and moment pictures at most
1024 px on the longer side, launcher icons at most 432 px, never
upscaled. Each master is in the pack once, named after its master file.
The zip is stored uncompressed with sorted entries, a fixed time and mode
and no extra fields, so the same commit gives the same bytes.

## What an app accepts

The same rules apply to a downloaded and an imported pack. An app refuses
the whole pack if any rule fails, and keeps what it had.

- The zip is at most 32 MiB, has at most 128 entries, and no entry is
  larger than 8 MiB once read. The app reads at most 64 MiB in total and
  stops at the limit, whatever the zip's headers claim.
- Entry names are `companion.json` or a file name of lower-case letters,
  digits and underscores ending in `.webp` or `.png`. No folders, no `..`,
  no absolute paths, no backslashes, no name twice.
- A picture's bytes start with its format's signature, and it is at most
  2048 px on each side.
- `companion.json` is at most 64 KiB of UTF-8 JSON; `format` is one the
  app knows; `id` follows the rule above; `name` has 1 to 40 characters;
  `version` is positive; `notice` is at most 16 KiB.
- Every name in `poses` is one the app knows (above), and every core pose
  is there. Every file `poses` names is in the zip, and every picture in
  the zip is named by `poses`.

A name the pack has no picture for shows its fallback: the app follows
`fallbacks` until it reaches a name the pack has, at the latest a core
pose.

## Where packs come from

| Source | How | Replaces |
|---|---|---|
| Bundled | The full APK carries the packs and `packs.json` of the Art release it pins (`ankiquest-art.json`), checked against the pin's SHA-256 at build time. The slim APK carries none. | Nothing. |
| Downloaded | "Download companions" fetches `https://github.com/AnkiQuest/AnkiQuest-Art/releases/latest/download/packs.json`, then each pack from `https://github.com/AnkiQuest/AnkiQuest-Art/releases/download/<tag>/<file>`. Only HTTPS to `github.com` and GitHub's release download hosts, redirects included. Each pack must match the `size` and `sha256` in the index before it is read. | An installed pack of the same id with a lower `version`. |
| Imported | The player picks a zip. | Nothing official: an imported pack may not use the id of a companion in `packs.json`. It replaces an imported pack with the same id. |

The server knows only Art's companions; a companion from an imported pack
is shown on this device only, and the server keeps the player's last
official choice.

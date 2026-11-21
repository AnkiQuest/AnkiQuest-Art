# AnkiQuest-Art

The companion art of [AnkiQuest](https://github.com/AnkiQuest): every
master picture of Aki, Ankilope, Bluma and Anki-tan, the archive of
pictures that are no longer masters, and the records of how they were
made. AnkiQuest-Server, AnkiQuest-Desktop and AnkiQuest-Android take this
repository as a git submodule and generate their exported copies (share
cards, add-on art, Android drawables, website icons) from it at build
time, so no code repository checks in art.

## Layout

| Path | What |
|---|---|
| `aki/`, `ankilope/` | Aki's and Ankilope's masters. |
| `cow/` | Bluma's masters (companion id `cow`). |
| `anki-tan/` | Anki-tan's masters (companion id `anki_tan`). |
| `anki-tan/chibi/` | Anki-tan chibi's masters (companion id `anki_tan_chibi`). |
| `duo/` | Pictures of more than one companion: `friends.png`, Aki and Ankilope. |
| `archive/` | Pictures that are no longer masters; never exported ([README](archive/README.md)). |
| `docs/` | Generation records, prompts, measurements and review sheets. |
| `manifest.json` | Every master (companion, pose, kind, framing, file, licence, origin and author), the core poses still missing, and what the archive holds. |
| `RENAMES.md` | Where each file of AnkiQuest-Server's `static/` art is now. |
| `tools/art-check/` | The check of `manifest.json`: every picture outside `archive/` is used, and `face` and launcher icons are headshots while every other master is full body (`cargo run --manifest-path tools/art-check/Cargo.toml -- .`). |

A master is `<companion>/<pose>.png`, a transparent PNG. Pose names and
their fallbacks come from
[`tests/fixtures/companion_poses.json`](https://github.com/AnkiQuest/AnkiQuest-Server/blob/master/tests/fixtures/companion_poses.json)
in AnkiQuest-Server, which the clients pin; this repository does not copy
it. Files with other names (`achievement.png`, `icon_happy.png`,
`place_1.png` and so on) are moment, notification and icon pictures;
`manifest.json` marks them `picture`. Its `origin` tells Shigeyuki's
pack files (`shigeyuki`, with their path in the pack) from art AnkiQuest
made: derivatives of his Anki-tan (`derivative`) and AnkiQuest's own
companions (`ankiquest`).

Two rules (hill, 2026-11-21), checked by `tools/art-check`:

- A picture is outside `archive/` only if something uses it: it is a
  pose master, a launcher icon or a moment picture an exporter exports.
  Everything else is in `archive/`.
- `face` and the launcher icons (`icon`, `icon_mono`, `icon_happy`,
  `icon_worried`, `icon_frozen`, `icon_sleepy`) are headshots of the
  face; every other master shows the full body. `framing` in the
  manifest records it.

Where one picture shows several poses, their manifest entries name the
same file instead of copies: `anki-tan/chibi/idle.png` is ten of Anki-tan
chibi's poses.

Nothing here is generated: smaller sizes and other copies are made by the
code repositories' exporters and are not committed here.

## Use as a submodule

```sh
git submodule add https://github.com/AnkiQuest/AnkiQuest-Art art
```

The exporters read `art/manifest.json` to find each pose's file; most
are `art/<companion>/<pose>.png`. A new or changed master lands here
first; a code repository picks it up by moving its submodule to the new
commit.

## The matrix

The art is a matrix of companions by poses. The goal is a full matrix:
every pose drawn for every live companion, with no pose unique to one
companion. Missing drawings are tracked in AnkiQuest-Server:
[#57](https://github.com/AnkiQuest/AnkiQuest-Server/issues/57) for poses
and [#116](https://github.com/AnkiQuest/AnkiQuest-Server/issues/116) for
icons and moment pictures. Until they are drawn, a missing pose shows the
fallback the fixture gives it.

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

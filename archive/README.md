# Archive

Pictures that are no longer masters and are not a copy of one, and
pictures nothing uses (outside the archive is only what is
used). Nothing here is exported to a client.

Every companion folder follows one layout: `retired/` for pictures that
were masters and were replaced, `addon/` (when the companion has them)
for the add-on's hand-made copies from before `export-addon-art`, and
`found/` (Aki only, so far) for posted candidates that never became
masters.

- `aki/`, `ankilope/`: in `retired/` earlier versions of Aki's and
  Ankilope's masters, replaced in AnkiQuest-Server before the split; in
  `aki/found/` candidates that never became masters; see their READMEs.
- `anki-tan/`: in `retired/` Anki-tan's neglected pose before one
  headband star was unlit, her face with the tile cut away, and the
  generated chibi frozen bust; see its README. CC BY-SA 4.0 under its
  LICENSE.
- `anki-tan-pack/`: the rest of Shigeyuki's Anki-tan asset pack; see its
  README. CC BY-SA 4.0 under its LICENSE.

## Removed duplicates

On 2026-11-21 five chibi masters that had the same bytes as an Anki-tan
launcher icon or pack file went: `anki-tan/chibi/sad.png`
(`anki-tan/icon_worried.png`), `chibi/asleep.png` (`icon_sleepy.png`),
`chibi/streak.png` (`icon_happy.png`, was `icon_fire.png`),
`chibi/hurt.png` and `chibi/dead.png` (pack `emoji/Ankitan_28_spiral_eyes.png`
and `emoji/Ankitan_39_crossed_out_eyes.png`, here).

Each file below was the same picture as the file it maps to: the same
bytes, or the same picture at a smaller size (an export). The kept file
is the original. Duplicates were found by comparing every archive file
with every master and every other archive file, byte for byte and by
pixels scaled to 128 px, and each pair was checked by eye. The removed
files are in this repository's history.

| Removed | Kept | Why |
|---|---|---|
| `aki/cards/aki-celebrate.png` | `aki/happy.png` | 512 px copy |
| `aki/cards/aki-freeze.png` | `aki/frozen.png` | 512 px copy |
| `aki/cards/aki-streak.png` | `aki/streak.png` | 512 px copy |
| `aki/cards/aki-winner.png` | `aki/winner.png` | 512 px copy |
| `ankilope/cards/ankilope-celebrate.png` | `ankilope/happy.png` | 512 px copy |
| `cow/celebrate.png` | `cow/happy.png` | same bytes |
| `cow/face.png` | `cow/face.png` | same bytes |
| `cow/frozen.png` | `cow/frozen.png` | same bytes |
| `cow/hopeful.png` | `cow/hopeful.png` | same bytes |
| `cow/neglected.png` | `cow/neglected.png` | same bytes |
| `cow/sad.png` | `cow/sad.png` | same bytes |
| `cow/sleeping.png` | `cow/asleep.png` | same bytes |
| `cow/streak.png` | `cow/streak.png` | same bytes |
| `cow/addon/bluma_celebrate.png` | `cow/happy.png` | 256 px copy |
| `cow/addon/bluma_sleeping.png` | `cow/asleep.png` | 256 px copy |
| `anki-tan/asleep-full.png` | `anki-tan/asleep.png` | same bytes |
| `anki-tan/sad-full.png` | `anki-tan/sad.png` | same bytes |
| `anki-tan/generated/frozen-full.png` | `anki-tan/frozen.png` | same bytes |
| `anki-tan/generated/frozen-chibi.png` | `anki-tan/chibi/frozen.png` | same bytes |
| `anki-tan/addon/anki_tan_sleeping.png` | `anki-tan/asleep.png` | 256 px copy |
| `anki-tan/addon/anki_tan_neglected-three-blue-stars.png` | `archive/anki-tan/retired/neglected-three-blue-stars.png` | 256 px copy |

Removed paths are relative to `archive/`; kept paths to the repository.
`archive/anki-tan/retired/neglected-three-blue-stars.png` was
`archive/anki-tan/neglected-full.png`; it took the more descriptive name
of its removed copy. It stays: all three headband stars are lit, while
`anki-tan/neglected.png` has one unlit star.

Similar pictures that stay because they differ: emoji with another
expression (`Ankitan_23_savoring` and `Ankitan_26_smiling`,
`Ankitan_37_tear` and `Ankitan_00_smile`), and the pack's animation
frames, which play in sequence (the last frames of `sprite/hello` and
`sprite/idle` are nearly the same rest pose).

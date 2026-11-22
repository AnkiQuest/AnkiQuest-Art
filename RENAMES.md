# Renames

Where each file of AnkiQuest-Server's companion art (`static/aki`,
`static/ankilope`, `static/cow`, `static/anki-tan`, `static/duo` and
`assets/cards` at AnkiQuest-Server commit 7a34c6f) is in this
repository. Code that reads a master by its old path reads it here.

Every file not listed keeps its path without the `static/` prefix:
`static/aki/face.png` is `aki/face.png`. Pictures were moved, never
edited: each one has the same bytes as before.

## Masters renamed to their pose

| Old path | New path |
|---|---|
| `static/aki/celebrate.png` | `aki/happy.png` |
| `static/aki/leech.png` | `aki/leech_defeated.png` |
| `static/anki-tan/emoji/Ankitan_00_smile.png` | `anki-tan/chibi/face.png` |
| `static/anki-tan/emoji/Ankitan_02_cry.png` | `anki-tan/icon_worried.png` |
| `static/anki-tan/emoji/Ankitan_09_fire_eyes.png` | `anki-tan/icon_happy.png` |
| `static/anki-tan/emoji/Ankitan_35_sleeping.png` | `anki-tan/icon_sleepy.png` |
| `static/anki-tan/sticker/ankitan_lets_study.png` | `anki-tan/hopeful.png` |
| `static/ankilope/celebrate.png` | `ankilope/happy.png` |

AnkiQuest-Server exports Anki-tan's poses as follows (the rules:
`face` and launcher icons are headshots, every other pose full body, and
a picture nothing uses is in `archive/`):

| Export | Was | Is |
|---|---|---|
| `anki_tan_face` | `static/anki-tan/sprite/idle/anki-tan_idle_0001.png` | `anki-tan/face.png` |
| `anki_tan_happy` | `static/anki-tan/sticker/anki_tan_good.png` | missing; falls back |
| `anki_tan_streak` | `static/anki-tan/emoji/Ankitan_09_fire_eyes.png` | missing; falls back |
| `anki_tan_welcome` | `static/anki-tan/sticker/anki_tan_hello.png` | falls back to happy |
| `anki_tan_review` | `static/anki-tan/sticker/press_spacebar.png` | falls back to happy |
| `anki_tan_streak_legend` | `static/anki-tan/emoji/Ankitan_43_crown.png` | falls back |
| `anki_tan_streak_milestone` | `static/anki-tan/emoji/Ankitan_48_perfect_streaks.png` | falls back |
| `anki_tan_hurt` | `static/anki-tan/emoji/Ankitan_28_spiral_eyes.png` | falls back to sad |
| `anki_tan_dead` | `static/anki-tan/emoji/Ankitan_39_crossed_out_eyes.png` | falls back to neglected |
| `anki_tan_chibi_frozen` | `static/anki-tan/chibi/frozen.png` | `anki-tan/chibi/idle.png` |
| `anki_tan_icon_mono` | `static/anki-tan/emoji/Ankitan_00_smile.png` | `anki-tan/chibi/face.png` |

Anki-tan chibi's poses: `face` is `anki-tan/chibi/face.png`, `happy` is
`anki-tan/chibi/happy.png` and the others are `anki-tan/chibi/idle.png`;
`welcome` and `review` fall back to happy.

## Moved to the archive

| Old path | New path |
|---|---|
| `static/anki-tan/archive/README.md` | `archive/anki-tan/README.md` |
| `static/anki-tan/archive/neglected-full.png` | `archive/anki-tan/neglected-three-blue-stars.png` |
| `static/anki-tan/chibi/frozen.png` | `archive/anki-tan/chibi/frozen-bust.png` |
| `static/anki-tan/emoji/Ankitan_06_pleading.png` | `archive/anki-tan-pack/emoji/Ankitan_06_pleading.png` |
| `static/anki-tan/emoji/Ankitan_10_party.png` | `archive/anki-tan-pack/emoji/Ankitan_10_party.png` |
| `static/anki-tan/emoji/Ankitan_21_saluting.png` | `archive/anki-tan-pack/emoji/Ankitan_21_saluting.png` |
| `static/anki-tan/emoji/Ankitan_28_spiral_eyes.png` | `archive/anki-tan-pack/emoji/Ankitan_28_spiral_eyes.png` |
| `static/anki-tan/emoji/Ankitan_39_crossed_out_eyes.png` | `archive/anki-tan-pack/emoji/Ankitan_39_crossed_out_eyes.png` |
| `static/anki-tan/emoji/Ankitan_43_crown.png` | `archive/anki-tan-pack/emoji/Ankitan_43_crown.png` |
| `static/anki-tan/emoji/Ankitan_48_perfect_streaks.png` | `archive/anki-tan-pack/emoji/Ankitan_48_perfect_streaks.png` |
| `static/anki-tan/emoji/Ankitan_49_lost_streaks.png` | `archive/anki-tan-pack/emoji/Ankitan_49_lost_streaks.png` |
| `static/anki-tan/sprite/idle/anki-tan_idle_0001.png` | `archive/anki-tan-pack/sprite/idle/anki-tan_idle_0001.png` |
| `static/anki-tan/sticker/anki_tan_good.png` | `archive/anki-tan-pack/sticker/anki_tan_good.png` |
| `static/anki-tan/sticker/anki_tan_hello.png` | `archive/anki-tan-pack/sticker/anki_tan_hello.png` |
| `static/anki-tan/sticker/press_spacebar.png` | `archive/anki-tan-pack/sticker/press_spacebar.png` |
| `static/anki-tan/{emoji,illust,sprite,sticker}/*` (the rest) | `archive/anki-tan-pack/{emoji,illust,sprite,sticker}/*` |

## Duplicates, now the file they copied

These were in the archive and were removed as the same picture as
another file: the same bytes, or a smaller export of it
([archive README](archive/README.md#removed-duplicates)). Code that read
one reads the kept file.

| Old path | Kept file |
|---|---|
| `assets/cards/aki-celebrate.png` | `aki/happy.png` |
| `assets/cards/aki-freeze.png` | `aki/freeze.png` |
| `assets/cards/aki-streak.png` | `aki/streak.png` |
| `assets/cards/aki-winner.png` | `aki/winner.png` |
| `assets/cards/ankilope-celebrate.png` | `ankilope/happy.png` |
| `static/anki-tan/archive/addon/anki_tan_neglected-three-blue-stars.png` | `archive/anki-tan/neglected-three-blue-stars.png` |
| `static/anki-tan/archive/addon/anki_tan_sleeping.png` | `anki-tan/asleep.png` |
| `static/anki-tan/archive/asleep-full.png` | `anki-tan/asleep.png` |
| `static/anki-tan/archive/sad-full.png` | `anki-tan/sad.png` |
| `static/anki-tan/generated/frozen-chibi.png` | `archive/anki-tan/chibi/frozen-bust.png` |
| `static/anki-tan/generated/frozen-full.png` | `anki-tan/frozen.png` |
| `static/cow/archive/addon/bluma_celebrate.png` | `cow/happy.png` |
| `static/cow/archive/addon/bluma_sleeping.png` | `cow/asleep.png` |
| `static/cow/archive/celebrate.png` | `cow/happy.png` |
| `static/cow/archive/face.png` | `cow/face.png` |
| `static/cow/archive/frozen.png` | `cow/frozen.png` |
| `static/cow/archive/hopeful.png` | `cow/hopeful.png` |
| `static/cow/archive/neglected.png` | `cow/neglected.png` |
| `static/cow/archive/sad.png` | `cow/sad.png` |
| `static/cow/archive/sleeping.png` | `cow/asleep.png` |
| `static/cow/archive/streak.png` | `cow/streak.png` |

`static/anki-tan/generated/README.md` and `static/cow/archive/README.md`
described only those copies; their records are in the
[archive README](archive/README.md), [cow/README.md](cow/README.md) and
[anki-tan/README.md](anki-tan/README.md).

## Deleted

| Old path | Why |
|---|---|
| `static/aki/face-192.png`, `static/aki/face-32.png`, `static/ankilope/face-192.png` | Smaller sizes of `face.png`; AnkiQuest-Server's `build.rs` generates them. |
| `static/anki-tan/poses.json`, `static/anki-tan/chibi/poses.json` | Each pose is now `<pose>.png`; `manifest.json` lists every master. |

## Not moved

`static/icon.svg` (the website's icon, not companion art),
`docs/branding/mascot-source.md` (it describes AnkiQuest-Server's
exporters) and `crates/xtask/art-exports.json` stay in AnkiQuest-Server.
`docs/` here holds the rest of `docs/branding/`.

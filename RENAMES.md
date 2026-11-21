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
| `static/anki-tan/emoji/Ankitan_02_cry.png` | `anki-tan/chibi/sad.png` |
| `static/anki-tan/emoji/Ankitan_06_pleading.png` | `anki-tan/chibi/hopeful.png` |
| `static/anki-tan/emoji/Ankitan_09_fire_eyes.png` | `anki-tan/streak.png` |
| `static/anki-tan/emoji/Ankitan_10_party.png` | `anki-tan/chibi/happy.png` |
| `static/anki-tan/emoji/Ankitan_21_saluting.png` | `anki-tan/chibi/welcome.png` |
| `static/anki-tan/emoji/Ankitan_28_spiral_eyes.png` | `anki-tan/hurt.png` |
| `static/anki-tan/emoji/Ankitan_35_sleeping.png` | `anki-tan/chibi/asleep.png` |
| `static/anki-tan/emoji/Ankitan_39_crossed_out_eyes.png` | `anki-tan/dead.png` |
| `static/anki-tan/emoji/Ankitan_43_crown.png` | `anki-tan/streak_legend.png` |
| `static/anki-tan/emoji/Ankitan_48_perfect_streaks.png` | `anki-tan/streak_milestone.png` |
| `static/anki-tan/emoji/Ankitan_49_lost_streaks.png` | `anki-tan/chibi/neglected.png` |
| `static/anki-tan/sprite/idle/anki-tan_idle_0001.png` | `anki-tan/face.png` |
| `static/anki-tan/sticker/anki_tan_good.png` | `anki-tan/happy.png` |
| `static/anki-tan/sticker/anki_tan_hello.png` | `anki-tan/welcome.png` |
| `static/anki-tan/sticker/ankitan_lets_study.png` | `anki-tan/hopeful.png` |
| `static/anki-tan/sticker/press_spacebar.png` | `anki-tan/review.png` |
| `static/ankilope/celebrate.png` | `ankilope/happy.png` |

Anki-tan chibi uses the same pack file as Anki-tan for these poses, so
`anki-tan/chibi/` has a byte-identical copy of each: `streak`, `review`,
`streak_legend`, `streak_milestone`, `hurt` and `dead`. An exporter that
read one of those pack files for the chibi reads `anki-tan/chibi/<pose>.png`.

The pack files AnkiQuest-Server's Anki-tan launcher icons read map like
the rest: `emoji/Ankitan_09_fire_eyes.png` (icon_happy) is
`anki-tan/streak.png`, `emoji/Ankitan_02_cry.png` (icon_worried) is
`anki-tan/chibi/sad.png`, `emoji/Ankitan_35_sleeping.png` (icon_sleepy)
is `anki-tan/chibi/asleep.png`, `emoji/Ankitan_00_smile.png` (icon_mono)
is `anki-tan/chibi/face.png`.

## Moved to the archive

| Old path | New path |
|---|---|
| `assets/cards/aki-celebrate.png` | `archive/aki/cards/aki-celebrate.png` |
| `assets/cards/aki-freeze.png` | `archive/aki/cards/aki-freeze.png` |
| `assets/cards/aki-streak.png` | `archive/aki/cards/aki-streak.png` |
| `assets/cards/aki-winner.png` | `archive/aki/cards/aki-winner.png` |
| `assets/cards/ankilope-celebrate.png` | `archive/ankilope/cards/ankilope-celebrate.png` |
| `static/anki-tan/archive/README.md` | `archive/anki-tan/README.md` |
| `static/anki-tan/archive/addon/anki_tan_neglected-three-blue-stars.png` | `archive/anki-tan/addon/anki_tan_neglected-three-blue-stars.png` |
| `static/anki-tan/archive/addon/anki_tan_sleeping.png` | `archive/anki-tan/addon/anki_tan_sleeping.png` |
| `static/anki-tan/archive/asleep-full.png` | `archive/anki-tan/asleep-full.png` |
| `static/anki-tan/archive/neglected-full.png` | `archive/anki-tan/neglected-full.png` |
| `static/anki-tan/archive/sad-full.png` | `archive/anki-tan/sad-full.png` |
| `static/anki-tan/generated/README.md` | `archive/anki-tan/generated/README.md` |
| `static/anki-tan/generated/frozen-chibi.png` | `archive/anki-tan/generated/frozen-chibi.png` |
| `static/anki-tan/generated/frozen-full.png` | `archive/anki-tan/generated/frozen-full.png` |
| `static/cow/archive/README.md` | `archive/cow/README.md` |
| `static/cow/archive/addon/bluma_celebrate.png` | `archive/cow/addon/bluma_celebrate.png` |
| `static/cow/archive/addon/bluma_sleeping.png` | `archive/cow/addon/bluma_sleeping.png` |
| `static/cow/archive/celebrate.png` | `archive/cow/celebrate.png` |
| `static/cow/archive/face.png` | `archive/cow/face.png` |
| `static/cow/archive/frozen.png` | `archive/cow/frozen.png` |
| `static/cow/archive/hopeful.png` | `archive/cow/hopeful.png` |
| `static/cow/archive/neglected.png` | `archive/cow/neglected.png` |
| `static/cow/archive/sad.png` | `archive/cow/sad.png` |
| `static/cow/archive/sleeping.png` | `archive/cow/sleeping.png` |
| `static/cow/archive/streak.png` | `archive/cow/streak.png` |
| `static/anki-tan/{emoji,illust,sprite,sticker}/*` (the rest) | `archive/anki-tan-pack/{emoji,illust,sprite,sticker}/*` |

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

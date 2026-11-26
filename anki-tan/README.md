# Anki-tan

Anki-tan's masters, companion ids `anki_tan` (this folder) and
`anki_tan_chibi` (`chibi/`). Licence and credit: [LICENSE](LICENSE)
(CC BY-SA 4.0, by Shigeyuki).

Most masters are files from Shigeyuki's Anki-tan asset pack v1.0, moved
here unchanged. The rest of the pack is in
[`../archive/anki-tan-pack/`](../archive/anki-tan-pack/README.md).

The two versions share no full-size pose. `face` and the launcher icons
are headshots; every other pose is full body. The pack's emoji are the
chibi head design: headshots, so they serve only as faces and launcher
icons. The masters:

| Pose | Anki-tan | Anki-tan chibi |
|---|---|---|
| face | `chibi/face.png`, the chibi's face | `chibi/face.png`: pack `emoji/Ankitan_00_smile.png` |
| asleep | generated | `chibi/asleep.png` |
| happy | generated | `chibi/happy.png`: pack `sprite/hello/anki-tan-hello_0006.png` (waving) |
| streak | generated | `chibi/asleep.png` |
| streak_hot | [generated](../docs/anki-tan-fire-2026-11-26/README.md) | falls back |
| streak_blazing | [generated](../docs/anki-tan-fire-2026-11-26/README.md) | falls back |
| sad | generated | `chibi/asleep.png` |
| hopeful | pack `sticker/ankitan_lets_study.png` | `chibi/asleep.png` |
| frozen | generated | `chibi/asleep.png` |
| frozen_2 | [generated](../docs/anki-tan-freeze-2026-11-26/README.md) | falls back |
| frozen_3 | [generated](../docs/anki-tan-freeze-2026-11-26/README.md) | falls back |
| neglected | generated | `chibi/asleep.png` |
| abandoned | [generated](../docs/anki-tan-away-2026-11-26/README.md) | falls back |
| welcome | `hopeful.png` | falls back |
| review | falls back | falls back |
| streak_legend | falls back | `chibi/asleep.png` |
| streak_milestone | falls back | `chibi/asleep.png` |
| hurt | falls back | `chibi/asleep.png` |
| dead | falls back | `chibi/asleep.png` |

`chibi/asleep.png` is pack `sprite/idle/anki-tan_idle_0013.png`, the
neutral standing chibi; `manifest.json` names it for each of those poses.

Anki-tan's launcher icons (AnkiQuest-Art exports them as
`anki_tan_icon*`):

| Icon | Source |
|---|---|
| default | `chibi/face.png` through the icons chain: the pack app icon did not match the rest of her set, so its master is retired and `icon` stays empty |
| `icon_frozen.png` | generated |
| `icon_happy.png` | pack `emoji/Ankitan_09_fire_eyes.png`, the icon while today counts |
| `icon_sleepy.png` | pack `emoji/Ankitan_35_sleeping.png` |
| `icon_worried.png` | pack `emoji/Ankitan_02_cry.png` |

The monochrome launcher icon is made from `chibi/face.png`.

Anki-tan's extensions show their fallbacks until drawn.

## Face

Her face is the chibi's face, `chibi/face.png` (pack
`emoji/Ankitan_00_smile.png`): both are Shigeyuki's Anki-tan, and
`manifest.json` names the file for both companions. The earlier face,
the pack icon with the tile cut away, is in
[`../archive/anki-tan/`](../archive/anki-tan/README.md#face-with-the-tile-cut-away).
Credit: Anki-tan by Shigeyuki (https://www.patreon.com/Shigeyuki).

## Generated masters

`asleep.png`, `sad.png`, `neglected.png`, `frozen.png`, `happy.png`,
`streak.png` and `icon_frozen.png` are derivatives of the pack, generated
with OpenAI `image_gen` (model identifier unavailable). They are
CC BY-SA 4.0 under [LICENSE](LICENSE).
Anki-tan by Shigeyuki (https://www.patreon.com/Shigeyuki).
See the generation records for the
[core poses](../archive/docs/core-poses-2026-11/README.md),
[full-size additions](../archive/docs/ankitan-full-2026-11/README.md),
[happy and streak](../archive/docs/core-poses-2026-11-24/README.md), and
[frozen launcher icons](../archive/docs/frozen-launcher-icons-2026-11-24/README.md).
The generated chibi frozen bust is in
[`../archive/anki-tan/chibi/`](../archive/anki-tan/README.md).

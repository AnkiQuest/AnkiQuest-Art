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
| face | `icon.png`: the pack's app icon, tile and all, also her launcher icon | `chibi/face.png`: pack `emoji/Ankitan_00_smile.png` |
| asleep | generated | `chibi/idle.png` |
| happy | missing | `chibi/happy.png`: pack `sprite/hello/anki-tan-hello_0006.png` (waving) |
| streak | missing | `chibi/idle.png` |
| sad | generated | `chibi/idle.png` |
| hopeful | pack `sticker/ankitan_lets_study.png` | `chibi/idle.png` |
| frozen | generated | `chibi/idle.png` |
| neglected | generated | `chibi/idle.png` |
| welcome | falls back | falls back |
| review | falls back | falls back |
| streak_legend | falls back | `chibi/idle.png` |
| streak_milestone | falls back | `chibi/idle.png` |
| hurt | falls back | `chibi/idle.png` |
| dead | falls back | `chibi/idle.png` |

`chibi/idle.png` is pack `sprite/idle/anki-tan_idle_0013.png`, the
neutral standing chibi; `manifest.json` names it for each of those poses.

Anki-tan's launcher icons (AnkiQuest-Server exports them as
`anki_tan_icon*`):

| Icon | Source |
|---|---|
| `icon.png` | pack app icon, the launcher icon |
| `icon_happy.png` | pack `emoji/Ankitan_09_fire_eyes.png`, the icon while today counts |
| `icon_sleepy.png` | pack `emoji/Ankitan_35_sleeping.png` |
| `icon_worried.png` | pack `emoji/Ankitan_02_cry.png` |

The monochrome launcher icon is made from `chibi/face.png`.

Anki-tan's `happy` and `streak` need drawing (AnkiQuest-Server#57);
`manifest.json` lists them under `missing`. Her extensions show their
fallbacks until drawn.

## Face

Her face is the pack's `icon.png`, the same file as her launcher icon:
`manifest.json` names it for both poses. The earlier face, the icon with
the tile cut away, is in
[`../archive/anki-tan/`](../archive/anki-tan/README.md#face-with-the-tile-cut-away).
Credit: Anki-tan by Shigeyuki (https://www.patreon.com/Shigeyuki).

## Generated masters

`asleep.png`, `sad.png`, `neglected.png` and `frozen.png` are derivatives
of the pack, generated with OpenAI `image_gen` (model identifier not
exposed). They are CC BY-SA 4.0 under [LICENSE](LICENSE). Credit:
Anki-tan by Shigeyuki (https://www.patreon.com/Shigeyuki). Records:
[core poses](../docs/core-poses-2026-11/README.md) and
[full-size additions](../docs/ankitan-full-2026-11/README.md). The
generated chibi frozen bust is in
[`../archive/anki-tan/chibi/`](../archive/anki-tan/README.md).

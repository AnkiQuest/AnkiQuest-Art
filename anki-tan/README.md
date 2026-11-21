# Anki-tan

Anki-tan's masters, companion ids `anki_tan` (this folder) and
`anki_tan_chibi` (`chibi/`), one file per pose. Licence and credit:
[LICENSE](LICENSE) (CC BY-SA 4.0, by Shigeyuki).

Most masters are files from Shigeyuki's Anki-tan asset pack v1.0, moved
here unchanged and named after their pose. The rest of the pack is in
[`../archive/anki-tan-pack/`](../archive/anki-tan-pack/README.md). Where
both versions use the same pack file, `chibi/` has a byte-identical copy.

| Pose | Anki-tan | Anki-tan chibi |
|---|---|---|
| face | pack `sprite/idle/anki-tan_idle_0001.png` | pack `emoji/Ankitan_00_smile.png` |
| asleep | generated | pack `emoji/Ankitan_35_sleeping.png` |
| happy | pack `sticker/anki_tan_good.png` | pack `emoji/Ankitan_10_party.png` |
| streak | pack `emoji/Ankitan_09_fire_eyes.png` | same file |
| sad | generated | pack `emoji/Ankitan_02_cry.png` |
| hopeful | pack `sticker/ankitan_lets_study.png` | pack `emoji/Ankitan_06_pleading.png` |
| frozen | generated | generated |
| neglected | generated | pack `emoji/Ankitan_49_lost_streaks.png` |
| welcome | pack `sticker/anki_tan_hello.png` | pack `emoji/Ankitan_21_saluting.png` |
| review | pack `sticker/press_spacebar.png` | same file |
| streak_legend | pack `emoji/Ankitan_43_crown.png` | same file |
| streak_milestone | pack `emoji/Ankitan_48_perfect_streaks.png` | same file |
| hurt | pack `emoji/Ankitan_28_spiral_eyes.png` | same file |
| dead | pack `emoji/Ankitan_39_crossed_out_eyes.png` | same file |

`icon.png` is the pack's app icon, the launcher icon's master.

Missing extensions (`streak_hot`, `streak_blazing`, `frozen_2`,
`frozen_3`, `abandoned`, `leech_defeated`, `winner`) show their fallbacks
until they are drawn (AnkiQuest-Server#57).

## Generated masters

`asleep.png`, `sad.png`, `neglected.png`, `frozen.png` and
`chibi/frozen.png` are derivatives of the pack, generated with OpenAI
`image_gen` (model identifier not exposed). They are CC BY-SA 4.0 under
[LICENSE](LICENSE). Credit: Anki-tan by Shigeyuki
(https://www.patreon.com/Shigeyuki). Records:
[core poses](../docs/core-poses-2026-11/README.md) and
[full-size additions](../docs/ankitan-full-2026-11/README.md).

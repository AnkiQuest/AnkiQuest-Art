# Anki-tan

Anki-tan's masters, companion ids `anki_tan` (this folder) and
`anki_tan_chibi` (`chibi/`), one file per pose. Licence and credit:
[LICENSE](LICENSE) (CC BY-SA 4.0, by Shigeyuki).

Most masters are files from Shigeyuki's Anki-tan asset pack v1.0, moved
here unchanged. The rest of the pack is in
[`../archive/anki-tan-pack/`](../archive/anki-tan-pack/README.md).

The two versions share no full-size pose (hill, 2026-11-21). The pack's
emoji are the chibi head design and are chibi's poses. Regular Anki-tan's
full-size poses are `asleep`, `sad`, `frozen`, `neglected` and `hopeful`;
her other pictures are icons only. Head-only icons may repeat a chibi
pose.

| Pose | Anki-tan | Anki-tan chibi |
|---|---|---|
| face | missing | pack `sprite/idle/anki-tan_idle_0001.png` |
| asleep | generated | pack `emoji/Ankitan_35_sleeping.png` |
| happy | missing | pack `emoji/Ankitan_10_party.png` |
| streak | missing | pack `emoji/Ankitan_09_fire_eyes.png` |
| sad | generated | pack `emoji/Ankitan_02_cry.png` |
| hopeful | pack `sticker/ankitan_lets_study.png` | pack `emoji/Ankitan_06_pleading.png` |
| frozen | generated | generated |
| neglected | generated | pack `emoji/Ankitan_49_lost_streaks.png` |
| welcome | falls back | pack `emoji/Ankitan_21_saluting.png` |
| review | falls back | missing |
| streak_legend | falls back | pack `emoji/Ankitan_43_crown.png` |
| streak_milestone | falls back | pack `emoji/Ankitan_48_perfect_streaks.png` |
| hurt | falls back | pack `emoji/Ankitan_28_spiral_eyes.png` |
| dead | falls back | pack `emoji/Ankitan_39_crossed_out_eyes.png` |

Anki-tan's icons:

| Icon | Source |
|---|---|
| `icon.png` | pack app icon, the launcher icon's master |
| `icon_mono.png` | pack `emoji/Ankitan_00_smile.png`, the monochrome (themed) launcher icon |
| `icon_fire.png` | pack `emoji/Ankitan_09_fire_eyes.png`, the streak icon; was `streak.png` and the old `icon_happy.png` |
| `icon_happy.png` | pack `sticker/anki_tan_good.png`; was `happy.png` |
| `icon_sleepy.png` | pack `emoji/Ankitan_35_sleeping.png` |
| `icon_worried.png` | pack `emoji/Ankitan_02_cry.png` |
| `icon_hello.png` | pack `sticker/anki_tan_hello.png`; was `welcome.png` |
| `icon_review.png` | pack `sticker/press_spacebar.png`; was `review.png` |
| `icon_dead.png` | pack `emoji/Ankitan_39_crossed_out_eyes.png`; was `dead.png` |
| `icon_hurt.png` | pack `emoji/Ankitan_28_spiral_eyes.png`; was `hurt.png` |

Anki-tan's `face`, `happy` and `streak` need drawing
(AnkiQuest-Server#57); `manifest.json` lists them under `missing`. The
sprite that was her face is the chibi design and is `chibi/face.png`.
Her extensions show their fallbacks until drawn.

## Generated masters

`asleep.png`, `sad.png`, `neglected.png`, `frozen.png` and
`chibi/frozen.png` are derivatives of the pack, generated with OpenAI
`image_gen` (model identifier not exposed). They are CC BY-SA 4.0 under
[LICENSE](LICENSE). Credit: Anki-tan by Shigeyuki
(https://www.patreon.com/Shigeyuki). Records:
[core poses](../docs/core-poses-2026-11/README.md) and
[full-size additions](../docs/ankitan-full-2026-11/README.md).

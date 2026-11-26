# Archive

Pictures that are no longer masters and are not a copy of one, and
pictures nothing uses (outside the archive is only what is
used). Nothing here is exported to a client.

Every companion folder follows one layout: `retired/` for pictures that
were masters and were replaced, `addon/` (when the companion has them)
for the add-on's hand-made copies from before `export-addon-art`,
`cards/` (Aki and Ankilope) for hand-made share cards from before the
cards were exported, `found/` (Aki only, so far) for posted
candidates that never became masters, and `reviews/` (Aki and Ankilope)
for review sheets other repositories committed.

- `aki/`, `ankilope/`: in `retired/` earlier versions of Aki's and
  Ankilope's masters, replaced in AnkiQuest-Server before the split; in
  `aki/found/` candidates that never became masters; see their READMEs.
- `anki-tan/`: in `retired/` Anki-tan's neglected pose before one
  headband star was unlit, her face with the tile cut away, and the
  generated chibi frozen bust; see its README. CC BY-SA 4.0 under its
  LICENSE.
- `anki-tan-pack/`: the rest of Shigeyuki's Anki-tan asset pack; see its
  README. CC BY-SA 4.0 under its LICENSE.
- `cow/`: in `addon/` the add-on's hand-made Bluma copies from before
  `export-addon-art`; see its README.
- Aki's and Ankilope's share cards from AnkiQuest-Server's
  `assets/cards/`, hand-made before the cards were exported, moved to
  `aki/cards/` and `ankilope/cards/`; see their READMEs.

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

## AnkiQuest-Android's exported copies

AnkiQuest-Android kept four superseded drawables in `docs/art-archive/`.
They are not archived here: `art-export android` writes each one again,
byte for byte, from a file this repository keeps. The first three are
the current drawables; the fourth comes from the retired master when it
is exported in place of `anki-tan/neglected.png`.

| AnkiQuest-Android file | Added in | SHA-256 | Exported from |
|---|---|---|---|
| `anki-tan/anki_tan_sleeping.webp` | d584c942c5707bcf4f46c4f780b78ccc7fcda437 | 3c9876172542efa032c932e583f64fc1bcee2e5c70abad5e8f565e167c0b6f4c | `anki-tan/asleep.png` (`anki_tan_asleep`) |
| `anki-tan/anki_tan_neglected-three-blue-stars.webp` | d584c942c5707bcf4f46c4f780b78ccc7fcda437 | 4f66fc71f4cf373f911f95151a73bd00b4d84669543444b5bef9b285924a5bfc | `archive/anki-tan/retired/neglected-three-blue-stars.png` (`anki_tan_neglected`) |
| `cow/bluma_sleeping.webp` | df9701e46629d9ea3cb0d6b58b3901e230ad0c92 | c12da35455cdeb36b1802f09fadf833807a5d30b81cabd969fa0d73461d8c7af | `cow/asleep.png` (`bluma_asleep`) |
| `cow/bluma_celebrate.webp` | df9701e46629d9ea3cb0d6b58b3901e230ad0c92 | 44dde7c166fe83abea67f59f8ba76796e1de3634dbd1d66ca87e200248c9e485 | `cow/happy.png` (`bluma_happy`) |

The files were added under `AnkiDroid/src/main/res/drawable-nodpi/` and
moved to `docs/art-archive/` in
99532b8affa4929e4f72d85a99812a1e2e86ba59; they stay in AnkiQuest-Android's
history.

## AnkiQuest-Android's review sheets

AnkiQuest-Android also removes five review sheets from
`docs/development/ankiquest-ui/`. Three differ from every picture here
and moved to `aki/reviews/` and `ankilope/reviews/`; see
[aki/README.md](aki/README.md#android-review-sheets) and
[ankilope/README.md](ankilope/README.md#android-review-sheets). Two have
the same bytes as review sheets this repository keeps:

| AnkiQuest-Android file | Added in | SHA-256 | Same bytes as |
|---|---|---|---|
| `backlog-art/review.png` | 37d0930b5b0ac09a8bb99e10a641eab09157d20a | a2a5530737002428ff9f69dcdedbd2e1bf5a9e73e0453d1c6358fa5472765ac4 | `docs/backlog-art/review.png` |
| `podium-art/podium-review.png` | f3f1b0a5f0f6a646ef429f4c76774657faf13768 | d8f805d12349673c0da5b5878ab9a57d24811c06432cf4964c39f531c2ad844c | `docs/podium-review.png` |

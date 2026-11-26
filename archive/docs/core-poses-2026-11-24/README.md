# Ankilope and Anki-tan core poses

Closes the three core gaps for live companions listed in
[AnkiQuest-Art #15](https://github.com/AnkiQuest/AnkiQuest-Art/issues/15).

| Pose | Master | Candidate |
| --- | --- | --- |
| Ankilope streak | [`ankilope/streak.png`](../../../ankilope/streak.png) | B |
| Anki-tan happy | [`anki-tan/happy.png`](../../../anki-tan/happy.png) | B |
| Anki-tan streak | [`anki-tan/streak.png`](../../../anki-tan/streak.png) | C |

Three candidates per pose used OpenAI's built-in image generation.
The model identifier was unavailable. The [generation record](generation.json)
contains each prompt and its references.

Each master is a 1254 × 1254 transparent RGBA PNG. The
[alpha cleanup](../alpha-cleanup/README.md) clears pixels with alpha 1–4.
[Measurements](measurements.json) record the hashes and confirm every
pixel above alpha 4 stayed identical.

Anki-tan by Shigeyuki (https://www.patreon.com/Shigeyuki).
The two Anki-tan derivatives are CC BY-SA 4.0 under
[her licence](../../../anki-tan/LICENSE). Ankilope is AGPL-3.0-or-later.

//! AnkiQuest-Android's companion drawables: `drawable-nodpi/<name>.webp`
//! in the resource folder the build passes.
//!
//! Each drawable is a square lossy WebP with full-quality alpha, downscaled
//! with Lanczos to about three times its largest place in the app and never
//! upscaled.

use std::fs;
use std::path::Path;

use art_check::Manifest;
use image::RgbaImage;

use crate::masters::{self, Source, pose};
use crate::resample::{load, shrink, target};

/// Drawable folder inside the resource folder.
pub const DRAWABLES: &str = "drawable-nodpi";

/// WebP quality for most drawables.
const QUALITY: f32 = 88.0;

/// WebP quality for the streak widget poses, which are shown large.
const WIDGET_QUALITY: f32 = 95.0;

/// Alpha plane quality; edges must stay clean on any background.
const ALPHA_QUALITY: i32 = 100;

/// Slowest, smallest libwebp encoding method.
const METHOD: i32 = 6;

/// Large and huge streak widgets show the pose full-width, up to about
/// 1100 px (AnkiQuest-Android#168), so smaller poses were upscaled.
const WIDGET: u32 = 1024;

/// Luminance below which a pixel counts as line art in [`Look::Outline`].
const LINE_LUMA: f32 = 90.0;

/// Alpha above which a pixel is part of the picture in [`Look::Outline`].
const LINE_ALPHA: u8 = 128;

/// How far [`Look::BoldOutline`] widens each line, in thousandths of the
/// master's width: 5 px on a 1254 px master. Thinner lines fade out when a
/// themed launcher icon is drawn at 48 dp.
const BOLD_SPREAD: u32 = 4;

/// How a drawable shows its master.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Look {
    /// The master as drawn.
    Colour,
    /// Only the master's dark line art, in black on transparent: the
    /// monochrome layer of a themed launcher icon, which keeps alpha only.
    Outline,
    /// [`Look::Outline`] with every line widened by [`BOLD_SPREAD`], for
    /// masters drawn with thin lines.
    BoldOutline,
}

/// One drawable: name, master, square size, WebP quality and look.
pub struct Art {
    pub name: &'static str,
    pub source: Source,
    pub size: u32,
    quality: f32,
    look: Look,
}

/// A drawable at the default quality.
const fn art(name: &'static str, source: Source, size: u32) -> Art {
    Art {
        name,
        source,
        size,
        quality: QUALITY,
        look: Look::Colour,
    }
}

/// A themed icon's monochrome layer, from the master's line art.
const fn outline(name: &'static str, source: Source, size: u32) -> Art {
    Art {
        name,
        source,
        size,
        quality: QUALITY,
        look: Look::Outline,
    }
}

/// A themed icon's monochrome layer from a master with thin line art.
const fn bold_outline(name: &'static str, source: Source, size: u32) -> Art {
    Art {
        name,
        source,
        size,
        quality: QUALITY,
        look: Look::BoldOutline,
    }
}

/// A pose the big streak widget shows, for any companion.
const fn widget(name: &'static str, source: Source) -> Art {
    Art {
        name,
        source,
        size: WIDGET,
        quality: WIDGET_QUALITY,
        look: Look::Colour,
    }
}

/// Every drawable. Names are the app's; `cow` is Bluma, whose drawables
/// are `bluma_*` except her launcher icons.
pub const ART: &[Art] = &[
    widget("aki_face", pose("aki", "face")),
    widget("aki_sad", pose("aki", "sad")),
    widget("aki_hopeful", pose("aki", "hopeful")),
    widget("aki_frozen", pose("aki", "frozen")),
    widget("aki_neglected", pose("aki", "neglected")),
    widget("ankilope_face", pose("ankilope", "face")),
    widget("ankilope_sad", pose("ankilope", "sad")),
    widget("ankilope_hopeful", pose("ankilope", "hopeful")),
    widget("ankilope_frozen", pose("ankilope", "frozen")),
    widget("ankilope_asleep", pose("ankilope", "asleep")),
    widget("ankilope_neglected", pose("ankilope", "neglected")),
    widget("bluma_face", pose("cow", "face")),
    widget("bluma_asleep", pose("cow", "asleep")),
    widget("bluma_happy", pose("cow", "happy")),
    widget("bluma_streak", pose("cow", "streak")),
    widget("bluma_sad", pose("cow", "sad")),
    widget("bluma_hopeful", pose("cow", "hopeful")),
    widget("bluma_frozen", pose("cow", "frozen")),
    widget("bluma_neglected", pose("cow", "neglected")),
    widget("aki_celebrate", pose("aki", "happy")),
    widget("aki_streak", pose("aki", "streak")),
    widget("ankilope_celebrate", pose("ankilope", "happy")),
    widget("ankilope_streak", pose("ankilope", "streak")),
    widget("ankilope_study", pose("ankilope", "study")),
    widget("aki_welcome", pose("aki", "welcome")),
    widget("aki_review", pose("aki", "review")),
    widget("aki_freeze", pose("aki", "freeze")),
    widget("ankilope_welcome", pose("ankilope", "welcome")),
    art("aki_winner", pose("aki", "winner"), 168),
    // Moment art from AnkiQuest-Server#88: app icon moods, streak widget,
    // celebrations.
    art("aki_icon_happy", pose("aki", "icon_happy"), 432),
    art("aki_icon_worried", pose("aki", "icon_worried"), 432),
    art("aki_icon_frozen", pose("aki", "icon_frozen"), 432),
    art("aki_icon_sleepy", pose("aki", "icon_sleepy"), 432),
    widget("aki_streak_hot", pose("aki", "streak_hot")),
    widget("aki_streak_blazing", pose("aki", "streak_blazing")),
    widget("aki_streak_legend", pose("aki", "streak_legend")),
    widget("aki_streak_milestone", pose("aki", "streak_milestone")),
    widget("aki_freeze_used", pose("aki", "freeze_used")),
    widget("aki_comeback", pose("aki", "comeback")),
    widget("aki_minimum", pose("aki", "minimum")),
    widget("aki_goal", pose("aki", "goal")),
    art("aki_achievement", pose("aki", "achievement"), 336),
    art("aki_level_up", pose("aki", "level_up"), 336),
    art("aki_deck_conquered", pose("aki", "deck_conquered"), 336),
    art("aki_leech", pose("aki", "leech_defeated"), 336),
    widget("aki_asleep", pose("aki", "asleep")),
    widget("aki_at_risk", pose("aki", "at_risk")),
    art("aki_backlog", pose("aki", "backlog"), 336),
    art("ankilope_backlog", pose("ankilope", "backlog"), 336),
    art("aki_place_1", pose("aki", "place_1"), 336),
    art("aki_place_2", pose("aki", "place_2"), 336),
    art("aki_place_3", pose("aki", "place_3"), 336),
    art("ankilope_place_1", pose("ankilope", "place_1"), 336),
    art("ankilope_place_2", pose("ankilope", "place_2"), 336),
    art("ankilope_place_3", pose("ankilope", "place_3"), 336),
    art("aki_nudge", pose("aki", "nudge"), 336),
    art("aki_challenge", pose("aki", "challenge"), 336),
    art("aki_challenge_won", pose("aki", "challenge_won"), 336),
    art("aki_challenge_lost", pose("aki", "challenge_lost"), 336),
    art("ankilope_nudge", pose("ankilope", "nudge"), 336),
    art("ankilope_challenge", pose("ankilope", "challenge"), 336),
    art(
        "ankilope_challenge_won",
        pose("ankilope", "challenge_won"),
        336,
    ),
    art(
        "ankilope_challenge_lost",
        pose("ankilope", "challenge_lost"),
        336,
    ),
    art("duo_friends", Source::Shared("friends"), 336),
    widget("anki_tan_face", pose("anki_tan", "face")),
    widget("anki_tan_frozen", pose("anki_tan", "frozen")),
    widget("anki_tan_asleep", pose("anki_tan", "asleep")),
    widget("anki_tan_happy", pose("anki_tan", "happy")),
    widget("anki_tan_sad", pose("anki_tan", "sad")),
    widget("anki_tan_hopeful", pose("anki_tan", "hopeful")),
    widget("anki_tan_streak", pose("anki_tan", "streak")),
    widget("anki_tan_neglected", pose("anki_tan", "neglected")),
    widget("anki_tan_chibi_frozen", pose("anki_tan_chibi", "frozen")),
    // Anki-tan's launcher icons (AnkiQuest-Android AnkiquestIcon), CC BY-SA
    // 4.0. The pack's icons are 256 px and are kept at that size.
    art("anki_tan_icon", pose("anki_tan", "icon"), 432),
    art("anki_tan_icon_happy", pose("anki_tan", "icon_happy"), 432),
    art(
        "anki_tan_icon_worried",
        pose("anki_tan", "icon_worried"),
        432,
    ),
    art("anki_tan_icon_sleepy", pose("anki_tan", "icon_sleepy"), 432),
    outline("anki_tan_icon_mono", pose("anki_tan_chibi", "face"), 432),
    art("ankilope_icon", pose("ankilope", "face"), 432),
    bold_outline("ankilope_icon_mono", pose("ankilope", "face"), 432),
    art("cow_icon", pose("cow", "face"), 432),
    bold_outline("cow_icon_mono", pose("cow", "face"), 432),
    art("ankilope_icon_happy", pose("ankilope", "icon_happy"), 432),
    art("ankilope_icon_sleepy", pose("ankilope", "icon_sleepy"), 432),
    art(
        "ankilope_icon_worried",
        pose("ankilope", "icon_worried"),
        432,
    ),
    art("cow_icon_happy", pose("cow", "icon_happy"), 432),
    art("cow_icon_sleepy", pose("cow", "icon_sleepy"), 432),
    art("cow_icon_worried", pose("cow", "icon_worried"), 432),
];

/// Writes every drawable into `res/drawable-nodpi`.
pub fn export(manifest: &Manifest, res: &Path) -> Result<(), String> {
    write(manifest, res, &ART.iter().collect::<Vec<_>>())
}

/// Writes `arts` into `res/drawable-nodpi`.
fn write(manifest: &Manifest, res: &Path, arts: &[&Art]) -> Result<(), String> {
    let folder = res.join(DRAWABLES);
    fs::create_dir_all(&folder).map_err(|e| format!("{}: {e}", folder.display()))?;
    for art in arts {
        let ((width, height), bytes) = render(manifest, art)?;
        let path = folder.join(format!("{}.webp", art.name));
        fs::write(&path, &bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        println!(
            "{DRAWABLES}/{}.webp: {width}x{height}, {} bytes",
            art.name,
            bytes.len()
        );
    }
    Ok(())
}

/// What export writes for `art`: its pixel size and WebP bytes.
fn render(manifest: &Manifest, art: &Art) -> Result<((u32, u32), Vec<u8>), String> {
    let file = masters::file(manifest, art.source)?;
    let master = look(load(&masters::root().join(file))?, art.look);
    let (width, height) = target(master.width(), master.height(), art.size);
    if (width, height) != (art.size, art.size) {
        println!(
            "{}: {file} is {}x{}, kept at {width}x{height}",
            art.name,
            master.width(),
            master.height()
        );
    }

    // Downscale and encode.
    let image = shrink(master, width, height);
    Ok(((width, height), encode(&image, art.quality)?))
}

/// The master as [`Look`] asks for it.
fn look(master: RgbaImage, look: Look) -> RgbaImage {
    if look == Look::Colour {
        return master;
    }

    // Keep opaque dark pixels as black line art; drop everything else.
    let mut lines = master;
    for pixel in lines.pixels_mut() {
        let [r, g, b, a] = pixel.0;
        let luma = 0.299 * f32::from(r) + 0.587 * f32::from(g) + 0.114 * f32::from(b);
        let line = a > LINE_ALPHA && luma < LINE_LUMA;
        pixel.0 = if line { [0, 0, 0, 255] } else { [0, 0, 0, 0] };
    }
    if look == Look::Outline {
        return lines;
    }

    // Widen the lines so they survive the downscale to launcher size.
    let spread = (lines.width() * BOLD_SPREAD).div_ceil(1000);
    let wide = widen(&lines, spread, Axis::Across);
    widen(&wide, spread, Axis::Down)
}

/// Direction of one [`widen`] pass.
#[derive(Clone, Copy, PartialEq)]
enum Axis {
    Across,
    Down,
}

/// Makes every pixel opaque that has an opaque pixel within `spread`
/// along `axis`; two passes widen lines by a square of that radius.
fn widen(lines: &RgbaImage, spread: u32, axis: Axis) -> RgbaImage {
    let (width, height) = lines.dimensions();
    RgbaImage::from_fn(width, height, |x, y| {
        let (at, end) = if axis == Axis::Across {
            (x, width)
        } else {
            (y, height)
        };
        let from = at.saturating_sub(spread);
        let to = (at + spread).min(end - 1);
        let hit = (from..=to).any(|i| {
            let (px, py) = if axis == Axis::Across { (i, y) } else { (x, i) };
            lines.get_pixel(px, py)[3] > 0
        });
        if hit {
            image::Rgba([0, 0, 0, 255])
        } else {
            image::Rgba([0, 0, 0, 0])
        }
    })
}

/// Encodes lossy WebP with full-quality alpha.
fn encode(image: &RgbaImage, quality: f32) -> Result<Vec<u8>, String> {
    let mut config = webp::WebPConfig::new().map_err(|()| "libwebp config failed")?;
    config.lossless = 0;
    config.quality = quality;
    config.alpha_quality = ALPHA_QUALITY;
    config.method = METHOD;
    let encoder = webp::Encoder::from_rgba(image, image.width(), image.height());
    let memory = encoder
        .encode_advanced(&config)
        .map_err(|e| format!("WebP encoding failed: {e:?}"))?;
    Ok(memory.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_poses_are_large() {
        // Large widget poses, including core art awaiting companion
        // selection wiring.
        let poses = [
            "aki_face",
            "aki_sad",
            "aki_hopeful",
            "aki_frozen",
            "aki_neglected",
            "ankilope_face",
            "ankilope_sad",
            "ankilope_hopeful",
            "ankilope_frozen",
            "ankilope_asleep",
            "ankilope_neglected",
            "bluma_face",
            "bluma_asleep",
            "bluma_happy",
            "bluma_streak",
            "bluma_sad",
            "bluma_hopeful",
            "bluma_frozen",
            "bluma_neglected",
            "aki_celebrate",
            "aki_streak",
            "ankilope_celebrate",
            "ankilope_streak",
            "ankilope_study",
            "aki_welcome",
            "aki_review",
            "aki_freeze",
            "ankilope_welcome",
            "aki_streak_hot",
            "aki_streak_blazing",
            "aki_streak_legend",
            "aki_streak_milestone",
            "aki_freeze_used",
            "aki_comeback",
            "aki_minimum",
            "aki_goal",
            "aki_asleep",
            "aki_at_risk",
            "anki_tan_face",
            "anki_tan_frozen",
            "anki_tan_asleep",
            "anki_tan_happy",
            "anki_tan_sad",
            "anki_tan_hopeful",
            "anki_tan_streak",
            "anki_tan_neglected",
            "anki_tan_chibi_frozen",
        ];
        let large: Vec<_> = ART.iter().filter(|art| art.size == WIDGET).collect();
        let names: Vec<_> = large.iter().map(|art| art.name).collect();
        assert_eq!(poses.to_vec(), names);
        assert!(large.iter().all(|art| art.quality == WIDGET_QUALITY));
    }

    #[test]
    fn asleep_has_one_name() {
        // AnkiQuest-Server#32: every companion's asleep drawable uses the
        // pose name.
        let asleep: Vec<_> = ART
            .iter()
            .filter(|art| matches!(art.source, Source::Pose(_, "asleep")))
            .collect();
        assert_eq!(4, asleep.len(), "aki, ankilope, bluma, anki_tan");
        for art in asleep {
            assert!(art.name.ends_with("_asleep"), "{}", art.name);
        }
    }

    #[test]
    fn encodes_lossy_webp() {
        let image = RgbaImage::from_pixel(64, 64, image::Rgba([200, 40, 40, 128]));
        let bytes = encode(&shrink(image, 32, 32), QUALITY).unwrap();
        let features = webp::BitstreamFeatures::new(&bytes).unwrap();
        assert_eq!((32, 32), (features.width(), features.height()));
        assert!(features.has_alpha());
        assert!(matches!(
            features.format(),
            Some(webp::BitstreamFormat::Lossy)
        ));
    }

    #[test]
    fn export_writes_the_same_bytes_every_time() {
        // The smallest drawable keeps the test fast.
        let manifest = Manifest::read(&masters::root()).unwrap();
        let art = ART.iter().min_by_key(|art| art.size).unwrap();
        let res = crate::tests::scratch("android");
        write(&manifest, &res, &[art]).unwrap();
        let path = res.join(DRAWABLES).join(format!("{}.webp", art.name));
        let first = fs::read(&path).unwrap();
        write(&manifest, &res, &[art]).unwrap();
        assert_eq!(first, fs::read(&path).unwrap());
        assert_eq!(first, render(&manifest, art).unwrap().1);
        fs::remove_dir_all(&res).unwrap();
    }

    #[test]
    fn outline_keeps_only_dark_lines() {
        let mut image = RgbaImage::new(3, 1);
        image.put_pixel(0, 0, image::Rgba([10, 10, 10, 255]));
        image.put_pixel(1, 0, image::Rgba([250, 200, 180, 255]));
        image.put_pixel(2, 0, image::Rgba([10, 10, 10, 40]));
        let lines = look(image, Look::Outline);
        assert_eq!([0, 0, 0, 255], lines.get_pixel(0, 0).0, "black line");
        assert_eq!(0, lines.get_pixel(1, 0)[3], "skin");
        assert_eq!(0, lines.get_pixel(2, 0)[3], "faint dark pixel");
    }

    #[test]
    fn bold_outline_widens_lines() {
        // One dark pixel in the middle of a 1000 px row spreads 4 px each way.
        let mut image = RgbaImage::from_pixel(1000, 9, image::Rgba([250, 200, 180, 255]));
        image.put_pixel(500, 4, image::Rgba([10, 10, 10, 255]));
        let lines = look(image, Look::BoldOutline);
        assert_eq!(255, lines.get_pixel(504, 8)[3], "within the spread");
        assert_eq!(0, lines.get_pixel(505, 4)[3], "beyond the spread");
        assert_eq!(0, lines.get_pixel(0, 0)[3], "skin");
    }

    #[test]
    fn every_companion_has_launcher_icons() {
        // AnkiQuest-Android AnkiquestIcon shows these; a missing one fell
        // back to Aki (AnkiQuest-Android#313).
        for name in [
            "aki_icon_happy",
            "anki_tan_icon",
            "anki_tan_icon_mono",
            "ankilope_icon",
            "ankilope_icon_mono",
            "cow_icon",
            "cow_icon_mono",
        ] {
            let art = ART.iter().find(|art| art.name == name);
            assert!(art.is_some_and(|art| art.size == 432), "{name}");
        }
    }
}

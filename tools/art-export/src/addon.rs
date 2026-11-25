//! The desktop add-on's companion pictures, written into the folder the
//! build passes, laid out as AnkiQuest-Desktop's `addon/` folder: the poses
//! in `art/<name>.png` and the faces of the share card and toolbar in
//! `<name>.png`.
//!
//! Each picture is a 256 px square, 256-colour palette PNG with
//! transparency.

use std::fs;
use std::path::Path;

use art_check::Manifest;
use color_quant::NeuQuant;
use image::RgbaImage;

use crate::masters::{self, Source, pose};
use crate::resample::{load, shrink, target};

/// Picture edge in pixels: the Today panel shows 96 px at up to 2x, and
/// moments 112 px.
const SIZE: u32 = 256;

/// Palette size, the most an indexed PNG holds.
const COLOURS: usize = 256;

/// NeuQuant sampling factor: 1 looks at every pixel, the slowest and best.
const SAMPLING: i32 = 1;

/// One picture: its path in the add-on folder and its master.
pub struct Picture {
    pub file: &'static str,
    pub source: Source,
}

/// A picture at `file`.
const fn picture(file: &'static str, source: Source) -> Picture {
    Picture { file, source }
}

/// Every picture. Names are the add-on's; `cow` is Bluma.
pub const PICTURES: &[Picture] = &[
    // The faces the share card and the toolbar show.
    picture("aki_face.png", pose("aki", "face")),
    picture("ankilope_face.png", pose("ankilope", "face")),
    picture("anki_tan_face.png", pose("anki_tan", "face")),
    // The poses.
    picture("art/aki_face.png", pose("aki", "face")),
    picture("art/aki_welcome.png", pose("aki", "welcome")),
    picture("art/aki_review.png", pose("aki", "review")),
    picture("art/aki_celebrate.png", pose("aki", "happy")),
    picture("art/aki_winner.png", pose("aki", "winner")),
    picture("art/aki_streak.png", pose("aki", "streak")),
    picture("art/aki_streak_hot.png", pose("aki", "streak_hot")),
    picture("art/aki_streak_blazing.png", pose("aki", "streak_blazing")),
    picture("art/aki_streak_legend.png", pose("aki", "streak_legend")),
    picture(
        "art/aki_streak_milestone.png",
        pose("aki", "streak_milestone"),
    ),
    picture("art/aki_freeze.png", pose("aki", "freeze")),
    picture("art/aki_freeze_used.png", pose("aki", "freeze_used")),
    picture("art/aki_asleep.png", pose("aki", "asleep")),
    picture("art/aki_at_risk.png", pose("aki", "at_risk")),
    picture("art/aki_sad.png", pose("aki", "sad")),
    picture("art/aki_hopeful.png", pose("aki", "hopeful")),
    picture("art/aki_frozen.png", pose("aki", "frozen")),
    picture("art/aki_neglected.png", pose("aki", "neglected")),
    picture("art/ankilope_face.png", pose("ankilope", "face")),
    picture("art/ankilope_sad.png", pose("ankilope", "sad")),
    picture("art/ankilope_hopeful.png", pose("ankilope", "hopeful")),
    picture("art/ankilope_frozen.png", pose("ankilope", "frozen")),
    picture("art/ankilope_asleep.png", pose("ankilope", "asleep")),
    picture("art/ankilope_neglected.png", pose("ankilope", "neglected")),
    picture("art/ankilope_welcome.png", pose("ankilope", "welcome")),
    picture("art/ankilope_study.png", pose("ankilope", "study")),
    picture("art/ankilope_celebrate.png", pose("ankilope", "happy")),
    picture("art/ankilope_streak.png", pose("ankilope", "streak")),
    picture("art/anki_tan_face.png", pose("anki_tan", "face")),
    picture("art/anki_tan_frozen.png", pose("anki_tan", "frozen")),
    picture(
        "art/anki_tan_chibi_frozen.png",
        pose("anki_tan_chibi", "frozen"),
    ),
    picture("art/anki_tan_asleep.png", pose("anki_tan", "asleep")),
    picture("art/anki_tan_happy.png", pose("anki_tan", "happy")),
    picture("art/anki_tan_sad.png", pose("anki_tan", "sad")),
    picture("art/anki_tan_hopeful.png", pose("anki_tan", "hopeful")),
    picture("art/anki_tan_neglected.png", pose("anki_tan", "neglected")),
    picture("art/anki_tan_streak.png", pose("anki_tan", "streak")),
    picture("art/bluma_face.png", pose("cow", "face")),
    picture("art/bluma_asleep.png", pose("cow", "asleep")),
    picture("art/bluma_happy.png", pose("cow", "happy")),
    picture("art/bluma_streak.png", pose("cow", "streak")),
    picture("art/bluma_sad.png", pose("cow", "sad")),
    picture("art/bluma_hopeful.png", pose("cow", "hopeful")),
    picture("art/bluma_frozen.png", pose("cow", "frozen")),
    picture("art/bluma_neglected.png", pose("cow", "neglected")),
    picture(
        "art/ankilope_deck_conquered.png",
        pose("ankilope", "deck_conquered"),
    ),
    picture(
        "art/ankilope_freeze_used.png",
        pose("ankilope", "freeze_used"),
    ),
    picture(
        "art/bluma_deck_conquered.png",
        pose("cow", "deck_conquered"),
    ),
    picture("art/bluma_freeze_used.png", pose("cow", "freeze_used")),
    picture(
        "art/anki_tan_deck_conquered.png",
        pose("anki_tan", "deck_conquered"),
    ),
    picture(
        "art/anki_tan_freeze_used.png",
        pose("anki_tan", "freeze_used"),
    ),
];

/// Writes every picture into `out`.
pub fn export(manifest: &Manifest, out: &Path) -> Result<(), String> {
    write(manifest, out, &PICTURES.iter().collect::<Vec<_>>())
}

/// Writes `pictures` into `out`.
fn write(manifest: &Manifest, out: &Path, pictures: &[&Picture]) -> Result<(), String> {
    for picture in pictures {
        let bytes = render(manifest, picture)?;
        let path = out.join(picture.file);
        if let Some(folder) = path.parent() {
            fs::create_dir_all(folder).map_err(|e| format!("{}: {e}", folder.display()))?;
        }
        fs::write(&path, &bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        println!("{}: {} bytes", picture.file, bytes.len());
    }
    Ok(())
}

/// The picture made from its master.
fn render(manifest: &Manifest, picture: &Picture) -> Result<Vec<u8>, String> {
    let file = masters::file(manifest, picture.source)?;
    let master = load(&masters::root().join(file))?;
    let (width, height) = target(master.width(), master.height(), SIZE);
    encode(&shrink(master, width, height))
}

/// Encodes `image` as an indexed PNG with a 256-colour RGBA palette.
fn encode(image: &RgbaImage) -> Result<Vec<u8>, String> {
    let quant = NeuQuant::new(SAMPLING, COLOURS, image.as_raw());
    let (pixels, _) = image.as_raw().as_chunks::<4>();
    let indices: Vec<u8> = pixels
        .iter()
        .map(|pixel| quant.index_of(pixel) as u8)
        .collect();

    // Split the RGBA palette into PNG's colour and transparency chunks.
    let rgba = quant.color_map_rgba();
    let (palette, _) = rgba.as_chunks::<4>();
    let colours: Vec<u8> = palette.iter().flat_map(|&[r, g, b, _]| [r, g, b]).collect();
    let alpha: Vec<u8> = palette.iter().map(|&[_, _, _, a]| a).collect();

    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, image.width(), image.height());
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_palette(colours);
    encoder.set_trns(alpha);
    encoder.set_compression(png::Compression::High);
    let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
    writer
        .write_image_data(&indices)
        .map_err(|e| e.to_string())?;
    writer.finish().map_err(|e| e.to_string())?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn faces_and_poses_are_where_the_add_on_reads_them() {
        for picture in PICTURES {
            let depth = picture.file.matches('/').count();
            assert!(picture.file.ends_with(".png"), "{}", picture.file);
            assert!(
                (depth == 0 && picture.file.ends_with("_face.png"))
                    || (depth == 1 && picture.file.starts_with("art/")),
                "{}",
                picture.file
            );
        }
    }

    #[test]
    fn export_writes_the_same_palette_png_every_time() {
        let manifest = Manifest::read(&masters::root()).unwrap();
        let winner = PICTURES
            .iter()
            .find(|p| p.file == "art/aki_winner.png")
            .unwrap();
        let out = crate::tests::scratch("addon");
        write(&manifest, &out, &[winner]).unwrap();
        let path = out.join(winner.file);
        let first = fs::read(&path).unwrap();
        write(&manifest, &out, &[winner]).unwrap();
        assert_eq!(first, fs::read(&path).unwrap());

        // A 256 px square, 8-bit palette picture with transparency.
        let decoder = png::Decoder::new(std::io::Cursor::new(&first));
        let reader = decoder.read_info().unwrap();
        let info = reader.info();
        assert_eq!((SIZE, SIZE), (info.width, info.height));
        assert_eq!(png::ColorType::Indexed, info.color_type);
        assert!(info.trns.is_some());
        fs::remove_dir_all(&out).unwrap();
    }

    #[test]
    fn encoding_keeps_every_pixel_close() {
        let red = RgbaImage::from_pixel(8, 8, image::Rgba([255, 0, 0, 255]));
        let bytes = encode(&red).unwrap();
        let decoded = image::load_from_memory(&bytes).unwrap().into_rgba8();
        assert!(decoded.pixels().all(|p| p.0[0] > 240 && p.0[3] == 255));
    }
}

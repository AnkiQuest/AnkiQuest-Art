//! Loading a master and shrinking it, the same way for every export.

use std::path::Path;

use image::imageops::{self, FilterType};
use image::{Rgba32FImage, RgbaImage};

/// Decodes a PNG master as 8-bit RGBA.
pub fn load(path: &Path) -> Result<RgbaImage, String> {
    let image = image::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(image.into_rgba8())
}

/// The size that fits `width` by `height` into `size` square, never upscaled.
pub fn target(width: u32, height: u32, size: u32) -> (u32, u32) {
    let scale = (f64::from(size) / f64::from(width.max(height))).min(1.0);
    let side = |n: u32| (f64::from(n) * scale).round_ties_even() as u32;
    (side(width), side(height))
}

/// Lanczos downscale in premultiplied alpha, as Pillow does, so fully
/// transparent pixels do not bleed their colour into the edges.
pub fn shrink(image: RgbaImage, width: u32, height: u32) -> RgbaImage {
    if image.dimensions() == (width, height) {
        return image;
    }
    let mut linear = Rgba32FImage::from(image::DynamicImage::ImageRgba8(image));
    for pixel in linear.pixels_mut() {
        let alpha = pixel[3];
        pixel.0[..3].iter_mut().for_each(|c| *c *= alpha);
    }
    let mut small = imageops::resize(&linear, width, height, FilterType::Lanczos3);
    for pixel in small.pixels_mut() {
        unpremultiply(&mut pixel.0);
    }
    image::DynamicImage::ImageRgba32F(small).into_rgba8()
}

/// Divides colour by alpha, clamping Lanczos overshoot.
fn unpremultiply(pixel: &mut [f32; 4]) {
    let alpha = pixel[3].clamp(0.0, 1.0);
    pixel[3] = alpha;
    if alpha == 0.0 {
        pixel[..3].fill(0.0);
        return;
    }
    pixel[..3]
        .iter_mut()
        .for_each(|c| *c = (*c / alpha).clamp(0.0, 1.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_never_upscales() {
        assert_eq!((336, 336), target(1254, 1254, 336));
        assert_eq!((200, 100), target(200, 100, 336));
        assert_eq!((336, 168), target(1254, 627, 336));
    }

    #[test]
    fn transparent_edges_stay_clean() {
        // A red opaque half next to transparent green must not turn green.
        let mut image = RgbaImage::new(64, 64);
        for (x, _, pixel) in image.enumerate_pixels_mut() {
            *pixel = if x < 32 {
                image::Rgba([255, 0, 0, 255])
            } else {
                image::Rgba([0, 255, 0, 0])
            };
        }
        let small = shrink(image, 16, 16);
        let edge = small.get_pixel(7, 8);
        assert!(edge[1] < 8, "{edge:?}");
    }
}

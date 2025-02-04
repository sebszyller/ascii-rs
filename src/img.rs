use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView, GrayImage, ImageReader, Pixels};

pub fn read_image(img_path: &str) -> Result<DynamicImage> {
    let file = File::open(img_path)
        .with_context(|| format!("Failed to read image from img_path: {}", img_path))?;

    let reader = BufReader::new(file);
    let img = ImageReader::new(reader)
        .with_guessed_format()
        .with_context(|| format!("Could not guess format"))?
        .decode()
        .with_context(|| format!("Failed to decode image"))?;

    Ok(img)
}

pub fn downsize(img: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
    if width > img.width() {
        return Err(anyhow::anyhow!(
            "Provided width larger than the image width! ({} > {})",
            width,
            img.width()
        ))?;
    }
    if height > img.height() {
        return Err(anyhow::anyhow!(
            "Provided height larger than the image height! ({} > {})",
            height,
            img.height()
        ))?;
    }
    Ok(img.resize(width, height, image::imageops::FilterType::Lanczos3))
}

pub fn print_pixel_values(pixels: Pixels<DynamicImage>) {
    for (x, y, pixel) in pixels {
        println!("Pixel at ({}, {}): {:?}", x, y, pixel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downsize() {
        let img = DynamicImage::new_rgb8(100, 100);
        let result = downsize(&img, 50, 50);
        assert!(result.is_ok());
        let resized = result.unwrap();
        assert_eq!(resized.width(), 50);
        assert_eq!(resized.height(), 50);
    }

    #[test]
    fn test_downsize_long_edge() {
        // image::resize preserves the ratio and respects the lower dimension
        // 20x20 img resized to 5x10 should become 5x5
        let img = DynamicImage::new_rgb8(100, 100);
        let result = downsize(&img, 50, 100);
        assert!(result.is_ok());
        let resized = result.unwrap();
        assert_eq!(resized.width(), 50);
        assert_eq!(resized.height(), 50);
    }

    #[test]
    fn test_downsize_width_error() {
        let img = DynamicImage::new_rgb8(100, 100);
        let result = downsize(&img, img.width() + 1, img.height());
        assert!(result.is_err());
    }

    #[test]
    fn test_downsize_height_error() {
        let img = DynamicImage::new_rgb8(100, 100);
        let result = downsize(&img, img.width(), img.height() + 1);
        assert!(result.is_err());
    }
}

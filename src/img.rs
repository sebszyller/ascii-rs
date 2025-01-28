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

// TODO: new tests for this
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

pub fn print_img_details(img: &DynamicImage) {
    let width = img.width();
    let height = img.height();

    println!("Image loaded successfully!");
    println!("Dimensions: {}x{}", width, height);
}

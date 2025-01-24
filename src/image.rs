use image::{DynamicImage, ImageReader};
use std::fs::File;
use std::io::BufReader;

use anyhow::Result;

fn read_image(file_path: &str) -> Result<DynamicImage, image::ImageError> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let image = ImageReader::new(reader).with_guessed_format()?.decode()?;

    Ok(image)
}

pub fn print_img_details(img_path: &str) -> Result<()> {
    let image = read_image(img_path)?;

    // Get image dimensions
    let width = image.width();
    let height = image.height();

    println!("Image loaded successfully!");
    println!("Dimensions: {}x{}", width, height);
    Ok(())
}

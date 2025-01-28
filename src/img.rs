use image::{DynamicImage, ImageReader, Pixels};
use rand::{Rng, SeedableRng};
use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Result};
use colored::Colorize;
use image::GenericImageView;
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;

pub fn read_image(img_path: &str) -> Result<DynamicImage> {
    let file = File::open(img_path)
        .with_context(|| format!("Failed to read image from img_path: {}", img_path))?; // NOTE: we don't want to recover from this

    let reader = BufReader::new(file);
    let img = ImageReader::new(reader)
        .with_guessed_format()
        .with_context(|| format!("Could not guess format"))? // NOTE: we don't want to recover from this
        .decode()
        .with_context(|| format!("Failed to decode image"))?; // NOTE: we don't want to recover from this

    Ok(img)
}

pub fn to_ascii(img: &DynamicImage, seed: u64) {
    let line_width = img.width() - 1;
    let mut prng = StdRng::seed_from_u64(seed);
    // TODO: customise chars higher up
    let chars = vec![':', ';', '+', '*', '?', '%', 'S', '#', '@'];

    for (x, _, pixel) in img.pixels() {
        let ch = random_char_from(&chars, &mut prng).to_string();
        let colored = ch.truecolor(pixel[0], pixel[1], pixel[2]);
        print!("{}", colored);
        if x == line_width {
            print!("\n");
        }
    }
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

fn random_char_from(options: &Vec<char>, rng: &mut StdRng) -> char {
    options.choose(rng).unwrap().to_owned()
}

pub fn print_pixel_values(pixels: Pixels<DynamicImage>) {
    for (x, y, pixel) in pixels {
        let formatted = format!("Pixel at ({}, {}): {:?}", x, y, pixel);
        let colored = formatted.on_truecolor(pixel[0], pixel[1], pixel[2]);
        println!("{}", colored);
    }
}

pub fn print_img_details(img: &DynamicImage) {
    let width = img.width();
    let height = img.height();

    println!("Image loaded successfully!");
    println!("Dimensions: {}x{}", width, height);
}

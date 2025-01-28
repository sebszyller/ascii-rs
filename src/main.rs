mod ascii;
mod img;

use anyhow::{Context, Result};
use clap::Parser;
use image::GenericImageView;

// https://docs.rs/clap/4.5.27/clap/_cookbook/typed_derive/index.html
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, value_name = "IMAGE_PATH", value_hint = clap::ValueHint::FilePath)]
    image: String,

    #[arg(long, value_name = "MAX_WIDTH")]
    width: Option<u32>,

    #[arg(long, value_name = "MAX_HEIGHT")]
    height: Option<u32>,

    #[arg(long, value_name = "ASCII_CHARS", default_value = ":;+*?%S#@")]
    chars: String,

    #[arg(long, value_name = "SEED", default_value = "1234567890")]
    seed: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{args:?}");

    let mut img = img::read_image(&args.image)?;
    if args.width.is_some() || args.height.is_some() {
        let nwidth = args.width.unwrap_or(img.width());
        let nheight = args.height.unwrap_or(img.height());
        img = img::downsize(&img, nwidth, nheight).with_context(|| "Failed to downsize image")?;
    }

    let mut ascii_img = ascii::ASCIIimg::init(img, args.chars, args.seed);
    ascii_img.to_ascii();
    // img::print_pixel_values(img.pixels());
    // img::print_img_details(&img);
    Ok(())
}

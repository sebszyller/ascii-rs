mod ascii;
mod img;

use anyhow::{Context, Result};
use clap::Parser;
use image::{DynamicImage, GenericImageView};
use imageproc::edges::canny;

// https://docs.rs/clap/4.5.27/clap/_cookbook/typed_derive/index.html
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, value_name = "IMAGE_PATH", value_hint = clap::ValueHint::FilePath)]
    image: String,

    #[arg(long, value_name = "MAX_WIDTH")]
    width: Option<u32>,

    #[arg(long, value_name = "MAX_HEIGHT")]
    height: Option<u32>,

    #[arg(long, value_name = "LIGHT_CHARS", default_value = "?%#@")]
    light_chars: String,

    #[arg(long, value_name = "HEAVY_CHARS", default_value = ".,o")]
    heavy_chars: String,

    #[arg(long, value_name = "edge_char", default_value = "+/\\")]
    edge_char: String,

    #[arg(long, value_name = "LUMINANCE_THRESHOLD", default_value_t = 50.0)]
    luma_threshold: f32,

    #[arg(long, value_name = "CANNY_LOW_THRESHOLD", default_value_t = 10.0)]
    canny_low_threshold: f32,

    #[arg(long, value_name = "CANNY_HIGH_THRESHOLD", default_value_t = 50.0)]
    canny_high_threshold: f32,

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

    let edges = canny(
        &img.to_luma8(),
        args.canny_low_threshold,
        args.canny_high_threshold,
    );
    let edges_img = DynamicImage::from(edges);

    let mut ascii_img = ascii::ASCIIimg::init(
        img,
        args.light_chars,
        args.heavy_chars,
        args.edge_char,
        args.luma_threshold,
        args.seed,
    );
    ascii_img.to_ascii(&edges_img);

    // edges.save("edges.png")?;
    // img::print_pixel_values(img.pixels());
    // img::print_pixel_values(edges.pixels());
    // img::print_img_details(&img);
    Ok(())
}

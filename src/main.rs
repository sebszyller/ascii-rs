mod img;

use anyhow::Result;
use clap::Parser;
use image::GenericImageView;

// https://docs.rs/clap/4.5.27/clap/_cookbook/typed_derive/index.html
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, value_name = "IMAGE_PATH", value_hint = clap::ValueHint::DirPath)]
    image: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{args:?}");

    let img = img::read_image(&args.image)?;

    img::to_ascii(&img);
    // img::print_pixel_values(img.pixels());
    // img::print_img_details(&img);
    Ok(())
}

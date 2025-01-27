mod img;

use anyhow::Result;
use clap::Parser;
use image::GenericImageView;

// https://docs.rs/clap/4.5.27/clap/_cookbook/typed_derive/index.html
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, value_name = "IMAGE_PATH", value_hint = clap::ValueHint::DirPath)]
    image: String,

    #[arg(long, value_name = "MAX_WIDTH")]
    width: Option<u32>,

    #[arg(long, value_name = "MAX_HEIGHT")]
    height: Option<u32>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{args:?}");

    let mut img = img::read_image(&args.image)?;
    if args.width.is_some() || args.height.is_some() {
        let nwidth = args.width.unwrap_or(img.width());
        let nheight = args.height.unwrap_or(img.height());
        img = img::downsize(img, nwidth, nheight);
    }

    img::to_ascii(&img);
    // img::print_pixel_values(img.pixels());
    // img::print_img_details(&img);
    Ok(())
}

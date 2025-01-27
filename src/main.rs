mod img;

use ::image::GenericImageView;
use anyhow::Result;

fn main() -> Result<()> {
    let path = "test_inputs/church.png";
    let img = img::read_image(path)?;

    img::print_pixels(img.pixels());
    img::print_img_details(&img);
    Ok(())
}

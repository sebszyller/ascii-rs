mod image;

use anyhow::Result;

fn main() -> Result<()> {
    let path = "test_inputs/church.png";
    image::print_img_details(path)?;
    Ok(())
}

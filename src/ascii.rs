use colored::Colorize;
use image::{DynamicImage, GenericImageView};
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;
use rand::{Rng, SeedableRng};

pub fn to_ascii(img: &DynamicImage, seed: u64) {
    let line_width = img.width() - 1;
    let mut prng = StdRng::seed_from_u64(seed);
    // TODO: customise chars higher up?
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

fn random_char_from(options: &Vec<char>, rng: &mut StdRng) -> char {
    options.choose(rng).unwrap().to_owned()
}

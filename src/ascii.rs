use colored::Colorize;
use image::{DynamicImage, GenericImageView};
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;
use rand::{Rng, SeedableRng};

pub struct ASCIIimg {
    img: DynamicImage,
    chars: Vec<char>,
    prng: StdRng,
}

impl ASCIIimg {
    pub fn init(img: DynamicImage, chars: String, seed: u64) -> ASCIIimg {
        let chars = chars.chars().collect();
        let prng = StdRng::seed_from_u64(seed);
        ASCIIimg { img, chars, prng }
    }

    pub fn to_ascii(&mut self) {
        let line_width = self.img.width() - 1;
        let pixels: Vec<_> = self.img.pixels().collect();

        for (x, _, pixel) in pixels {
            let ch = self.random_char().to_string();
            let colored = ch.truecolor(pixel[0], pixel[1], pixel[2]);
            print!("{}", colored);
            if x == line_width {
                print!("\n");
            }
        }
    }

    fn random_char(&mut self) -> char {
        self.chars.choose(&mut self.prng).unwrap().to_owned()
    }
}

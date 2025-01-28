use colored::Colorize;
use image::{DynamicImage, GenericImageView};
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;
use rand::{Rng, SeedableRng};

pub struct ASCIIimg {
    img: DynamicImage,
    chars: Vec<String>,
    edge_char: String,
    prng: StdRng,
}

impl ASCIIimg {
    pub fn init(img: DynamicImage, chars: String, edge_char: String, seed: u64) -> ASCIIimg {
        let chars = chars.chars().map(|c| c.to_string()).collect();
        let edge_char = edge_char;
        let prng = StdRng::seed_from_u64(seed);
        ASCIIimg {
            img,
            chars,
            edge_char,
            prng,
        }
    }

    pub fn to_ascii(&mut self, edges: &DynamicImage) {
        let line_width = self.img.width() - 1;
        let pixels: Vec<_> = self.img.pixels().collect();

        for (x, y, pixel) in pixels {
            let p = edges.get_pixel(x, y);
            let ch = if p[0] == 255 {
                self.edge_char.clone()
            } else {
                self.random_char()
            };

            let colored = ch.truecolor(pixel[0], pixel[1], pixel[2]);
            print!("{}", colored);
            if x == line_width {
                print!("\n");
            }
        }
    }

    fn random_char(&mut self) -> String {
        self.chars.choose(&mut self.prng).unwrap().to_owned()
    }
}

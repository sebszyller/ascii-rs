use colored::Colorize;
use image::{DynamicImage, GenericImageView, Rgb};
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;
use rand::{Rng, SeedableRng};

pub struct ASCIIimg {
    img: DynamicImage,
    light_chars: Vec<String>,
    heavy_chars: Vec<String>,
    edge_chars: Vec<String>,
    luma_threshold: f32,
    prng: StdRng,
}

impl ASCIIimg {
    pub fn init(
        img: DynamicImage,
        light_chars: String,
        heavy_chars: String,
        edge_chars: String,
        luma_threshold: f32,
        seed: u64,
    ) -> ASCIIimg {
        ASCIIimg {
            img,
            light_chars: light_chars.chars().map(|c| c.to_string()).collect(),
            heavy_chars: heavy_chars.chars().map(|c| c.to_string()).collect(),
            edge_chars: edge_chars.chars().map(|c| c.to_string()).collect(),
            luma_threshold,
            prng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn to_ascii(&mut self, edges: &DynamicImage) {
        let line_width = self.img.width() - 1;
        let rgb = self.img.to_rgb8();
        let edge_rgb = edges.to_rgb8();

        for (x, y, pixel) in rgb.enumerate_pixels() {
            let edge_pixel = edge_rgb.get_pixel(x, y);
            let ch = self.choose_char(pixel, &edge_pixel);

            let colored = ch.truecolor(pixel[0], pixel[1], pixel[2]);
            print!("{}", colored);
            if x == line_width {
                print!("\n");
            }
        }
    }

    fn choose_char(&mut self, pixel: &Rgb<u8>, edge_pixel: &Rgb<u8>) -> String {
        let char_set = if edge_pixel[0] == 255 {
            self.edge_chars.clone()
        } else {
            let luminance =
                0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;

            if luminance > self.luma_threshold {
                // FIXME: this cloning is no bueno
                self.light_chars.clone()
            } else {
                self.heavy_chars.clone()
            }
        };
        self.random_char(char_set)
    }

    fn random_char(&mut self, options: Vec<String>) -> String {
        options.choose(&mut self.prng).unwrap().to_owned()
    }
}

use colored::Colorize;
use image::{DynamicImage, GenericImageView, Rgb};
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;

pub struct ASCIIimg {
    img: DynamicImage,
    light_chars: Vec<String>,
    medium_chars: Vec<String>,
    heavy_chars: Vec<String>,
    edge_chars: Vec<String>,
    luma_threshold_mid: f32,
    luma_threshold_high: f32,
    no_colour: bool,
}

impl ASCIIimg {
    pub fn init(
        img: DynamicImage,
        light_chars: String,
        medium_chars: String,
        heavy_chars: String,
        edge_chars: String,
        luma_threshold_mid: f32,
        luma_threshold_high: f32,
        no_colour: bool,
    ) -> ASCIIimg {
        ASCIIimg {
            img,
            light_chars: light_chars.chars().map(|c| c.to_string()).collect(),
            medium_chars: medium_chars.chars().map(|c| c.to_string()).collect(),
            heavy_chars: heavy_chars.chars().map(|c| c.to_string()).collect(),
            edge_chars: edge_chars.chars().map(|c| c.to_string()).collect(),
            luma_threshold_mid,
            luma_threshold_high,
            no_colour,
        }
    }

    pub fn to_ascii(self, edges: &DynamicImage, prng: &mut StdRng) {
        let line_width = self.img.width() - 1;
        let rgb = self.img.to_rgb8();
        let edge_rgb = edges.to_rgb8();

        for (x, y, pixel) in rgb.enumerate_pixels() {
            let edge_pixel = edge_rgb.get_pixel(x, y);
            let ch = self.choose_char(pixel, &edge_pixel, prng);

            let colored = if self.no_colour {
                ch.white()
            } else {
                ch.truecolor(pixel[0], pixel[1], pixel[2])
            };
            print!("{}", colored);
            if x == line_width {
                print!("\n");
            }
        }
    }

    fn choose_char(&self, pixel: &Rgb<u8>, edge_pixel: &Rgb<u8>, prng: &mut StdRng) -> String {
        let char_set = if edge_pixel[0] == 255 {
            self.edge_chars.clone()
        } else {
            // ITU BT.601
            let luminance =
                0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;

            if luminance > self.luma_threshold_high {
                self.light_chars.clone()
            } else if luminance > self.luma_threshold_mid {
                self.medium_chars.clone()
            } else {
                self.heavy_chars.clone()
            }
        };
        self.random_char(&char_set, prng)
    }

    fn random_char(&self, options: &Vec<String>, prng: &mut StdRng) -> String {
        options.choose(prng).unwrap().to_owned()
    }
}

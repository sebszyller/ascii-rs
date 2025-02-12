use clap::Parser;

// https://docs.rs/clap/4.5.27/clap/_cookbook/typed_derive/index.html
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(required = true, value_name = "IMAGE_PATH", index = 1, value_hint = clap::ValueHint::FilePath)]
    pub image: String,

    #[arg(long, value_name = "MAX_WIDTH")]
    pub width: Option<u32>,

    #[arg(long, value_name = "MAX_HEIGHT")]
    pub height: Option<u32>,

    #[arg(long, value_name = "LIGHT_CHARS", default_value = "?%#@")]
    pub light_chars: String,

    #[arg(long, value_name = "MEDIUM_CHARS", default_value = "DOS")]
    pub medium_chars: String,

    #[arg(long, value_name = "DARK_CHARS", default_value = ".,")]
    pub dark_chars: String,

    #[arg(long, value_name = "EDGE_CHARS", default_value = "/\\")]
    pub edge_chars: String,

    #[arg(long, value_name = "LUMINANCE_THRESHOLD_MID", default_value_t = 50.0)]
    pub luma_threshold_mid: f32,

    #[arg(long, value_name = "LUMINANCE_THRESHOLD_HIGH", default_value_t = 95.0)]
    pub luma_threshold_high: f32,

    #[arg(long, value_name = "CANNY_THRESHOLD_LOW", default_value_t = 10.0)]
    pub canny_threshold_low: f32,

    #[arg(long, value_name = "CANNY_THRESHOLD_HIGH", default_value_t = 50.0)]
    pub canny_threshold_high: f32,

    #[arg(long, value_name = "NO_COLOUR", default_value_t = false)]
    pub no_colour: bool,

    #[arg(long, value_name = "OUTPUT_EDGES", default_value_t = false)]
    pub output_edges: bool,

    #[arg(long, value_name = "SEED", default_value = "1234567890")]
    pub seed: u64,
}

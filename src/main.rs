mod ascii;
mod img;

use anyhow::{Context, Result};
use clap::Parser;
use image::{DynamicImage, GenericImageView};
use imageproc::edges::canny;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use tracing::debug;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// https://docs.rs/clap/4.5.27/clap/_cookbook/typed_derive/index.html
#[derive(Parser, Debug)]
struct Args {
    #[arg(required = true, value_name = "IMAGE_PATH", index = 1, value_hint = clap::ValueHint::FilePath)]
    image: String,

    #[arg(long, value_name = "MAX_WIDTH")]
    width: Option<u32>,

    #[arg(long, value_name = "MAX_HEIGHT")]
    height: Option<u32>,

    #[arg(long, value_name = "LIGHT_CHARS", default_value = "?%#@")]
    light_chars: String,

    #[arg(long, value_name = "MEDIUM_CHARS", default_value = "DOS")]
    medium_chars: String,

    #[arg(long, value_name = "DARK_CHARS", default_value = ".,")]
    dark_chars: String,

    #[arg(long, value_name = "EDGE_CHARS", default_value = "/\\")]
    edge_chars: String,

    #[arg(long, value_name = "LUMINANCE_THRESHOLD_MID", default_value_t = 50.0)]
    luma_threshold_mid: f32,

    #[arg(long, value_name = "LUMINANCE_THRESHOLD_HIGH", default_value_t = 95.0)]
    luma_threshold_high: f32,

    #[arg(long, value_name = "CANNY_THRESHOLD_LOW", default_value_t = 10.0)]
    canny_threshold_low: f32,

    #[arg(long, value_name = "CANNY_THRESHOLD_HIGH", default_value_t = 50.0)]
    canny_threshold_high: f32,

    #[arg(long, value_name = "NO_COLOUR", default_value_t = false)]
    no_colour: bool,

    #[arg(long, value_name = "OUTPUT_EDGES", default_value_t = false)]
    output_edges: bool,

    #[arg(long, value_name = "SEED", default_value = "1234567890")]
    seed: u64,
}

fn main() -> Result<()> {
    configure_tracing().with_context(|| "Failed to configure tracing")?;

    let args = Args::parse();
    debug!("{args:?}");

    let mut img = img::read_image(&args.image)?;
    if args.width.is_some() || args.height.is_some() {
        let nwidth = args.width.unwrap_or(img.width());
        let nheight = args.height.unwrap_or(img.height());
        debug!(
            "Dimensions before downsizig: {}x{}",
            img.width(),
            img.height()
        );
        img = img::downsize(&img, nwidth, nheight).with_context(|| "Failed to downsize image")?;
        debug!(
            "Dimensions after downsizig: {}x{}",
            img.width(),
            img.height()
        );
    }

    let edges = canny(
        &img.to_luma8(),
        args.canny_threshold_low,
        args.canny_threshold_high,
    );
    let edges_img = DynamicImage::from(edges);

    let mut ascii_img = ascii::ASCIIimg::init(
        img,
        args.light_chars,
        args.medium_chars,
        args.dark_chars,
        args.edge_chars,
        args.luma_threshold_mid,
        args.luma_threshold_high,
        args.no_colour,
    );

    let mut prng = StdRng::seed_from_u64(args.seed);
    ascii_img.to_ascii(&edges_img, &mut prng);

    if args.output_edges {
        edges_img
            .save("edges.png")
            .with_context(|| "Failed to save edges.png")?;
    }
    Ok(())
}

fn configure_tracing() -> Result<()> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .compact()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

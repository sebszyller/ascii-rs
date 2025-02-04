mod ascii;
mod img;

use anyhow::{Context, Result};
use clap::Parser;
use image::{DynamicImage, GenericImageView};
use imageproc::edges::canny;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// https://docs.rs/clap/4.5.27/clap/_cookbook/typed_derive/index.html
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, value_name = "IMAGE_PATH", value_hint = clap::ValueHint::FilePath)]
    image: String,

    #[arg(long, value_name = "MAX_WIDTH")]
    width: Option<u32>,

    #[arg(long, value_name = "MAX_HEIGHT")]
    height: Option<u32>,

    #[arg(long, value_name = "LIGHT_CHARS", default_value = "?%#@")]
    light_chars: String,

    #[arg(long, value_name = "HEAVY_CHARS", default_value = ".,o")]
    heavy_chars: String,

    #[arg(long, value_name = "edge_char", default_value = "+/\\")]
    edge_char: String,

    #[arg(long, value_name = "LUMINANCE_THRESHOLD", default_value_t = 50.0)]
    luma_threshold: f32,

    #[arg(long, value_name = "CANNY_LOW_THRESHOLD", default_value_t = 10.0)]
    canny_low_threshold: f32,

    #[arg(long, value_name = "CANNY_HIGH_THRESHOLD", default_value_t = 50.0)]
    canny_high_threshold: f32,

    #[arg(long, value_name = "SEED", default_value = "1234567890")]
    seed: u64,
}

fn main() -> Result<()> {
    configure_tracing()?;

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
        args.canny_low_threshold,
        args.canny_high_threshold,
    );
    let edges_img = DynamicImage::from(edges);

    let mut ascii_img = ascii::ASCIIimg::init(
        img,
        args.light_chars,
        args.heavy_chars,
        args.edge_char,
        args.luma_threshold,
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

mod args;
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

fn main() -> Result<()> {
    configure_tracing().with_context(|| "Failed to configure tracing")?;

    let args = args::Args::parse();
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

    let mut ascii_transform = ascii::AsciiTransform::init(
        args.light_chars,
        args.medium_chars,
        args.dark_chars,
        args.edge_chars,
        args.luma_threshold_mid,
        args.luma_threshold_high,
        args.no_colour,
    );

    let mut prng = StdRng::seed_from_u64(args.seed);
    ascii_transform.to_ascii(&img, &edges_img, &mut prng); // TODO: this should return a string not print

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

    Ok(tracing::subscriber::set_global_default(subscriber)?)
}

use std::path::PathBuf;

use clap::Parser;
use image::{ImageBuffer, Rgb};

#[derive(Parser)]
struct Args {
    /// input grayscale image
    #[arg(short, long)]
    input: PathBuf,
    /// output distance field image
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let input = image::io::Reader::open(args.input)?.decode()?;

    let sdf = signed_distance_field(input.into());

    sdf.save(args.output)?;

    Ok(())
}

fn signed_distance_field(image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let sigma = 5.0;
    let blurred = image::imageops::blur(&image, sigma);
    blurred
}

use std::path::PathBuf;

use clap::Parser;
use image::{ImageBuffer, Rgb};

#[derive(Parser)]
struct Args {
    /// output image
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let size = 1000;

    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(size, size);
    // make every pixel white
    for pixel in image.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    image.save(args.output)?;

    Ok(())
}

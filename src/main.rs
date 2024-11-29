use std::{fs, path::PathBuf, process::ExitCode};

use clap::{Args, Parser};
use image::{GenericImageView, RgbImage};
use thiserror::Error;

#[derive(Parser)]
#[clap(author, version)]
pub enum Cli {
    Stitch(StitchArgs),
    Compare(CompareArgs),
}

#[derive(Args)]
pub struct StitchArgs {}

#[derive(Args)]
pub struct CompareArgs {
    a: PathBuf,
    b: PathBuf,
    output: Option<PathBuf>,

    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("IO Error {0}")]
    IoError(#[from] std::io::Error),

    #[error("Image Error {0}")]
    ImageError(#[from] image::error::ImageError),

    #[error("Input images have different dimensions")]
    DifferentDimensions,
}

fn main() -> impl std::process::Termination {
    if let Err(err) = match Cli::parse() {
        Cli::Stitch(_) => panic!("Not implemented"),
        Cli::Compare(compare_args) => compare(&compare_args),
    } {
        eprintln!("{err}");
        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;
}

fn compare(args: &CompareArgs) -> Result<(), CliError> {
    let a = image::open(&args.a)?;
    let b = image::open(&args.b)?;

    if a.dimensions() != b.dimensions() {
        return Err(CliError::DifferentDimensions);
    }

    let mut output = RgbImage::new(a.width(), a.height());
    let mut diff_pixels = 0usize;

    for ((a, b), out) in a.pixels().zip(b.pixels()).zip(output.pixels_mut()) {
        let a = a.2 .0;
        let b = b.2 .0;

        let diff = a != b;
        diff_pixels += diff as usize;
        out.0 = [0xff, 0xff * !diff as u8, 0xff * !diff as u8];
    }

    if let Some(name) = &args.name {
        print!("{name}: ");
    }

    println!(
        "Imgages have {diff_pixels} different pixels. ({}%)",
        diff_pixels as f32 / (output.width() * output.height()) as f32 * 100f32
    );

    if let Some(output_path) = &args.output {
        output.save(output_path)?;
    }

    Ok(())
}

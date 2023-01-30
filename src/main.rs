use std::{fs, path::PathBuf};

use clap::Parser;
use image::{imageops, DynamicImage, GenericImageView};

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    input: PathBuf,

    #[clap(short, long, default_value = "output.png")]
    output: PathBuf,

    #[clap(long)]
    identity: Option<PathBuf>,

    #[clap(long, short)]
    max: Option<usize>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut files = fs::read_dir(&args.input)
        .expect("Could not read dir")
        .map(|entry| entry.expect("Could not read dir"))
        .filter(|e| e.file_type().expect("Could not get File type").is_file())
        .map(|e| e.path())
        .collect::<Vec<PathBuf>>();

    alphanumeric_sort::sort_path_slice(&mut files);

    let identity = image::open(match args.identity {
        Some(path) => path,
        None => files.get(0).unwrap().clone(),
    })
    .unwrap();

    let chunk_size = args.max.unwrap_or(files.len());
    let chunks = files.chunks(chunk_size);

    let mut blocks = Vec::new();
    for chunk in chunks {
        let mut block =
            DynamicImage::new_rgb8(identity.width() * chunk.len() as u32, identity.height());

        for (i, path) in chunk.iter().enumerate() {
            let img = image::open(path).unwrap();
            imageops::overlay(&mut block, &img, i as u32 * identity.width(), 0);
        }
        blocks.push(block);
    }

    let mut image = DynamicImage::new_rgb8(
        identity.width() * chunk_size as u32,
        identity.height() * blocks.len() as u32,
    );

    for (i, block) in blocks.iter().enumerate() {
        imageops::overlay(&mut image, block, 0, i as u32 * identity.height());
    }

    image.save(args.output).expect("Could not save image");

    Ok(())
}

extern crate image;

mod dimension;
mod img;
mod write_direction;

use clap::{Clap, ValueHint};
use dimension::Dimensions;
use image::GenericImageView;
use image::{ImageBuffer, ImageFormat};
use img::{process_images, save_image_buffer};
use std::path::PathBuf;
use std::time::Instant;
use std::{fs, process::exit};
use write_direction::WriteDirection;

#[derive(Clap, Debug)]
#[clap(name = "image-stitch", version = "0.2.0")]
pub struct Opt {
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, default_value = "output/output.png")]
    pub output: PathBuf,

    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, default_value = ".")]
    pub input: PathBuf,

    #[clap(short, long)]
    pub max: Option<u32>,

    #[clap(short, long, default_value = "X")]
    pub direction: WriteDirection,
}

fn main() {
    let opt = Opt::parse();
    let global_instant = Instant::now();
    let files = &mut index_images(fs::read_dir(&opt.input).unwrap());
    let input_dir: PathBuf = opt.input;

    println!("Input dir: {:?}", input_dir.into_os_string());

    if files.len() <= 0 {
        println!("No images were found");
        exit(1);
    }

    alphanumeric_sort::sort_path_slice(files);

    let img_dims =
        Dimensions::from_tuple(image::open(files.first().unwrap()).unwrap().dimensions());
    let max_length: u32 = opt.max.unwrap_or(0);

    let mut buffer_dim: (u32, u32) = match opt.direction {
        WriteDirection::X => (files.len() as u32 * img_dims.x, img_dims.y),
        WriteDirection::Y => (img_dims.x, files.len() as u32 * img_dims.y),
    };

    if max_length != 0 {
        buffer_dim = match opt.direction {
            WriteDirection::X => calc_buffer_dims(max_length, img_dims.x, img_dims.y, files.len()),
            WriteDirection::Y => calc_buffer_dims(max_length, img_dims.y, img_dims.x, files.len()),
        };
    }

    println!("Images: {}", files.len());
    println!("Row Length: {}", opt.max.unwrap_or(1));
    println!("Buffer Dimensions: {}x{}", buffer_dim.0, buffer_dim.1);

    let mut image_buffer = ImageBuffer::new(buffer_dim.0, buffer_dim.1);
    process_images(files, img_dims, &mut image_buffer, opt.direction);
    save_image_buffer(&opt.output, image_buffer, ImageFormat::Png);

    println!("finished process in {:?}", global_instant.elapsed())
}

fn calc_buffer_dims(max_length: u32, dim_a: u32, dim_b: u32, size: usize) -> (u32, u32) {
    let x_count = max_length / dim_a;
    let y_count = (size as f32 / x_count as f32).ceil() as u32;

    (x_count * dim_a, y_count * dim_b)
}

fn index_images(dir: std::fs::ReadDir) -> Vec<PathBuf> {
    let mut file_list: Vec<PathBuf> = vec![];
    for file in dir {
        match file {
            Ok(file) => {
                let path = file.path();
                match ImageFormat::from_path(path.as_path()) {
                    Ok(p) => {
                        if p.can_read() {
                            file_list.push(path);
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    return file_list;
}

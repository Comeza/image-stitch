extern crate image;

mod dimension;
mod img;
mod write_direction;

use clap::Clap;
use dimension::Dimension;
use image::{GenericImageView, ImageBuffer, ImageFormat};
use loading::Loading;
use opt::Opt;
use std::path::PathBuf;
use std::time::Instant;
use std::{fs, process};
use write_direction::WriteDirection;

mod opt;
fn main() {
    let global_instant = Instant::now();
    let opt = Opt::parse();
    let max_length: u32 = opt.max.unwrap_or(0);

    let mut files = index_dir(&opt.input);
    if files.len() <= 0 {
        println!("No files indexed.");
        process::exit(0)
    }

    alphanumeric_sort::sort_path_slice(&mut files);

    let first_file = files
        .first()
        .expect("Could not unwrap first file to measure dimensions");

    let img_dims = Dimension::from(
        image::open(first_file)
            .expect("Could not parse first image´to measure dimensions")
            .dimensions(),
    );

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
    img::process_images(&mut files, img_dims, &mut image_buffer, opt.direction);
    img::save_image_buffer(&opt.output, image_buffer, ImageFormat::Png);

    // file_dim.
    println!("Finished in {:?}", global_instant.elapsed())
}

fn index_dir(dir: &PathBuf) -> Vec<PathBuf> {
    let mut loading = Loading::new();

    let read_dir = fs::read_dir(dir).expect(format!("Could not read dir {:?}", dir).as_str());
    let mut file_list = Vec::<PathBuf>::new();

    for (index, item) in read_dir.into_iter().enumerate() {
        let item = item.expect(format!("Could not iterate through dir: {:?}", dir).as_str());
        let path = item.path();
        let format = ImageFormat::from_path(&path)
            .expect(format!("Could not get image format: {:?}", path).as_str());
        if format.can_read() {
            file_list.push(path);
        }
    }

    file_list
}

fn calc_buffer_dims(max_length: u32, dim_a: u32, dim_b: u32, size: usize) -> (u32, u32) {
    let x_count = max_length / dim_a;
    let y_count = (size as f32 / x_count as f32).ceil() as u32;

    (x_count * dim_a, y_count * dim_b)
}

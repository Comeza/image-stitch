use crate::Dimensions;
use crate::WriteDirection;
use image::imageops;
use image::ImageFormat;
use image::RgbaImage;
use indicatif::ProgressBar;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn process_images(
    files: &Vec<PathBuf>,
    img_dim: Dimensions<u32>,
    image_buffer: &mut RgbaImage,
    direction: WriteDirection,
) {
    let progress_bar = ProgressBar::new(files.len() as u64);
    let (buffer_dim_x, buffer_dim_y) = image_buffer.dimensions();
    let (img_dim_x, img_dim_y) = img_dim.to_tuple();

    let mut index = 0_usize;

    match direction {
        WriteDirection::Y => {
            for x in 0..(buffer_dim_x / img_dim_x) {
                for y in 0..(buffer_dim_y / img_dim_y) {
                    overlay_image(index, files, image_buffer, &img_dim, x, y);
                    index += 1;
                    progress_bar.inc(1);
                }
            }
        }
        WriteDirection::X => {
            for y in 0..(buffer_dim_y / img_dim_y) {
                for x in 0..(buffer_dim_x / img_dim_x) {
                    overlay_image(index, files, image_buffer, &img_dim, x, y);
                    index += 1;
                    progress_bar.inc(1);
                }
            }
        }
    }
    progress_bar.finish_and_clear();
}

fn overlay_image(
    index: usize,
    files: &Vec<PathBuf>,
    image_buffer: &mut RgbaImage,
    img_dim: &Dimensions<u32>,
    x: u32,
    y: u32,
) {
    match files.get(index) {
        Some(path) => {
            imageops::overlay(
                image_buffer,
                &mut image::open(path).unwrap(),
                x * img_dim.x,
                y * img_dim.y,
            );
        }
        None => (),
    }
}

pub fn save_image_buffer(file: &Path, image: RgbaImage, format: ImageFormat) {
    let parent_dir = file.parent().unwrap();

    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).unwrap();
    }

    match image.save_with_format(file, format) {
        Ok(_) => println!("Saved file {:?}", file),
        Err(e) => panic!(e),
    }
}

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
                    match files.get(index) {
                        Some(path) => {
                            let mut image = image::open(path).unwrap();
                            imageops::overlay(
                                image_buffer,
                                &mut image,
                                x * img_dim_x,
                                y * img_dim_y,
                            );
                            index += 1;
                            progress_bar.inc(1);
                        }
                        None => return,
                    }
                }
            }
        }
        WriteDirection::X => {
            for y in 0..(buffer_dim_y / img_dim_y) {
                for x in 0..(buffer_dim_x / img_dim_x) {
                    match files.get(index) {
                        Some(path) => {
                            let mut image = image::open(path).unwrap();
                            imageops::overlay(
                                image_buffer,
                                &mut image,
                                x * img_dim_x,
                                y * img_dim_y,
                            );
                            index += 1;
                            progress_bar.inc(1);
                        }
                        None => return,
                    }
                }
            }
        }
    }
    progress_bar.finish_and_clear();
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

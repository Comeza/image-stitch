use crate::Dimension;
use crate::WriteDirection;
use image::imageops;
use image::ImageFormat;
use image::RgbaImage;
use loading::Loading;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn process_images(
    files: &mut Vec<PathBuf>,
    img_dim: Dimension<u32>,
    image_buffer: &mut RgbaImage,
    direction: WriteDirection,
) {
    let (buffer_dim_x, buffer_dim_y) = image_buffer.dimensions();
    let (img_dim_x, img_dim_y) = img_dim.to_tuple();

    let mut index = 0usize;
    let files_count = files.len();

    let mut loader = Loading::new();
    loader.start();

    match direction {
        WriteDirection::Y => {
            for x in 0..(buffer_dim_x / img_dim_x) {
                for y in 0..(buffer_dim_y / img_dim_y) {
                    overlay_image(index, files, image_buffer, &img_dim, x, y);
                    loader.text(format!("Processing image {} / {}", index + 1, files_count));
                    drop(files.get_mut(index));
                    index += 1;
                }
            }
        }
        WriteDirection::X => {
            for y in 0..(buffer_dim_y / img_dim_y) {
                for x in 0..(buffer_dim_x / img_dim_x) {
                    overlay_image(index, files, image_buffer, &img_dim, x, y);
                    loader.text(format!("Processing image {} / {}", index + 1, files_count));
                    drop(files.get_mut(index));
                    index += 1;
                }
            }
        }
    }

    loader.success(format!("Processed {} images.", files_count));
    loader.end();
}

fn overlay_image(
    index: usize,
    files: &Vec<PathBuf>,
    image_buffer: &mut RgbaImage,
    img_dim: &Dimension<u32>,
    x: u32,
    y: u32,
) {
    if let Some(path) = files.get(index) {
        imageops::overlay(
            image_buffer,
            &mut image::open(path).unwrap(),
            x * img_dim.x,
            y * img_dim.y,
        );
    }
}

pub fn save_image_buffer(file: &Path, image: RgbaImage, format: ImageFormat) {
    let parent_dir = file.parent().unwrap();

    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).unwrap();
    }

    image
        .save_with_format(file, format)
        .expect(format!("Could not save Image with {:?} format", format).as_str());

    println!("Saved file to {:?}", file);
}

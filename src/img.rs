use crate::Dimensions;
use image::ImageBuffer;
use image::ImageFormat;
use image::RgbaImage;
use indicatif::ProgressBar;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn process_images(files: &Vec<PathBuf>, dimensions: Dimensions) -> RgbaImage {
  let mut image_buffer: RgbaImage =
    ImageBuffer::new(dimensions.0, dimensions.1 * files.len() as u32);
  let progress_bar = ProgressBar::new(files.len() as u64);

  for (i, file) in files.iter().enumerate() {
    image::imageops::overlay(
      &mut image_buffer,
      &mut image::open(file).unwrap(),
      0,
      dimensions.1 * i as u32,
    );
    progress_bar.inc(1);
  }

  progress_bar.finish_and_clear();

  return image_buffer;
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

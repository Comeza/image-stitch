extern crate image;

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, RgbaImage};
use regex::Regex;
use std::{fs, path::Path, time::Instant};

fn main() {
  let dir = &mut fs::read_dir(".").unwrap();
  let regex = Regex::new(r".*\.(png|jpeg|bmp|ico|tiff|webp|avif|pnm|dds|tga|farbfeld)$").unwrap();
  let mut count = 0;

  let (mut dim_x, mut dim_y): (u32, u32) = (0, 0);
  let mut images: Vec<DynamicImage> = vec![];

  for file in dir {
    match file {
      Ok(file) => {
        let file_name = file.file_name();
        print!("loading {:?}", file_name);
        if !regex.is_match(file_name.to_str().unwrap()) {
          println!("[unsporrted file format]");
          continue;
        }

        let timer = Instant::now();
        let image = image::open(file.path()).unwrap();
        let dims = image.dimensions();
        images.push(image);
        dim_x += dims.0;
        dim_y += dims.1;
        count += 1;
        println!(" [{:?}]", timer.elapsed());
      }
      Err(_) => continue,
    }
  }

  dim_x /= count;
  dim_y /= count;
  println!("dims {}x{} starting to process..", dim_x, dim_y);

  let mut buffer: RgbaImage = ImageBuffer::new(dim_x, dim_y * count);
  let mut offset = 0;

  for (i, image) in images.iter_mut().enumerate() {
    image::imageops::overlay(&mut buffer, image, 0, (offset) as u32);
    offset += image.dimensions().1;
    println!("{}/{}", i + 1, count);
  }

  print!("saving file...");
  let file_path = Path::new("./output");
  if !file_path.exists() {
    fs::create_dir_all(file_path).unwrap();
  }
  let out_path = file_path.join("output.png");
  println!("save file: {:?}", out_path);
  match buffer.save_with_format(out_path, ImageFormat::Png) {
    Ok(_) => println!(" done."),
    Err(e) => println!("error\n{:?}", e),
  }
}

extern crate image;

use image::{GenericImageView, ImageBuffer, ImageFormat, RgbaImage};
use regex::Regex;
use std::{
	fs,
	path::{Path, PathBuf},
	time::Instant,
};

type Dimensions = (u32, u32);
fn main() {
	let global_instant = Instant::now();
	let dir = &mut fs::read_dir(".").unwrap();
	let regex = Regex::new(r".*\.(png|jpeg|bmp|ico|tiff|webp|avif|pnm|dds|tga|farbfeld)$").unwrap();

	let mut file_list: Vec<PathBuf> = vec![];
	let dimensions: Dimensions;
	let output_dir = Path::new("output");

	for file in dir {
		match file {
			Ok(file) => {
				let path = file.path();
				println!("indexing file {:?}", path);
				if regex.is_match(path.to_str().unwrap()) {
					file_list.push(path);
				}
			}
			Err(_) => {}
		}
	}

	dimensions = image::open(file_list.first().unwrap())
		.unwrap()
		.dimensions();

	let mut image_buffer: RgbaImage =
		ImageBuffer::new(dimensions.0, dimensions.1 * file_list.len() as u32);

	for (i, file) in file_list.iter().enumerate() {
		let load_time = Instant::now();
		print!("processing {:?}", file);
		let mut image = image::open(file).unwrap();
		image::imageops::overlay(&mut image_buffer, &mut image, 0, dimensions.1 * i as u32);
		println!(" done [{:?}]", load_time.elapsed());
	}

	if !output_dir.exists() {
		fs::create_dir_all(output_dir).unwrap();
		println!("creatd output dir")
	}

	let output_file = output_dir.join("output.png");
	match image_buffer.save_with_format(output_file, ImageFormat::Png) {
		Ok(_) => println!("Saved file {:?}", output_dir),
		Err(e) => panic!(e),
	}
	println!("Finished in {:?}", global_instant.elapsed());
}

extern crate image;

mod img;

use clap::{Clap, ValueHint};
use image::GenericImageView;
use image::ImageFormat;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

type Dimensions = (u32, u32);

#[derive(Clap, Debug)]
#[clap(name = "image-stitch", version = "0.2.0")]
struct Opt {
	#[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, default_value = "output/output.png")]
	output: PathBuf,

	#[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, default_value = ".")]
	input: PathBuf,
}

fn main() {
	let opt = Opt::parse();
	let global_instant = Instant::now();
	let file_list = &mut index_images(fs::read_dir(opt.input).unwrap());

	alphanumeric_sort::sort_path_slice(file_list);

	let dimensions = image::open(file_list.first().unwrap())
		.unwrap()
		.dimensions();
	img::save_image_buffer(
		&opt.output,
		img::process_images(file_list, dimensions),
		ImageFormat::Png,
	);
	println!("Finished in {:?}", global_instant.elapsed());
}

fn index_images(dir: std::fs::ReadDir) -> Vec<PathBuf> {
	let mut file_list: Vec<PathBuf> = vec![];
	let regex = Regex::new(r".*\.(png|jpeg|bmp|ico|tiff|webp|avif|pnm|dds|tga|farbfeld)$").unwrap();
	for file in dir {
		match file {
			Ok(file) => {
				let path = file.path();
				if regex.is_match(path.to_str().unwrap()) {
					file_list.push(path);
				}
			}
			Err(_) => {}
		}
	}

	return file_list;
}

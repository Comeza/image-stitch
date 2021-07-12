use clap::{Clap, ValueHint};
use std::path::PathBuf;

use crate::write_direction::WriteDirection;

#[derive(Clap, Debug)]
#[clap(name = "image-stitch", version = "2.0.0")]
pub struct Opt {
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, default_value = "output.png")]
    pub output: PathBuf,

    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, default_value = ".")]
    pub input: PathBuf,

    #[clap(short, long)]
    pub max: Option<u32>,

    #[clap(short, long, default_value = "X")]
    pub direction: WriteDirection,
}

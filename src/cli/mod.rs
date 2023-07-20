use crate::generate_canvas_file_content;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None )]
pub struct Args {
    #[clap(
        short,
        long,
        help = r#"Path to directory containing Sherlock judging issues."#
    )]
    pub input_path: String,

    #[clap(
        short,
        long,
        help = "Output path, where you want to save the created canvas file.\nFor ex. ~/Documents/Judging/Contest.canvas"
    )]
    pub output_path: String,
}

impl Args {
    pub fn new() -> Result<Self, String> {
        let args = Self::parse();
        Ok(args)
    }
}

pub fn create_canvas_file(input_path: &str, output_path: &str) -> Result<(), String> {
    let canvas_file_content = generate_canvas_file_content(input_path)?;
    fs::write(output_path, canvas_file_content).map_err(|e| e.to_string())
}

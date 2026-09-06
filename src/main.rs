use std::{env, error::Error, time::Instant};

mod cli;
mod detector;
mod error;
mod reader;
mod signatures;

use crate::{
    cli::command_line_handler,
    detector::option_handle,
    reader::{file_path_handler, read_footer, read_header},
};

fn main() {
    let start = Instant::now();
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
    let duration = start.elapsed();
    println!("Execution Time: {:?}", duration);
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    command_line_handler(&args)?;

    let mut file = file_path_handler(&args)?;

    let header_buffer: Vec<u8> = read_header(&mut file)?;

    let footer_buffer: Vec<u8> = read_footer(&mut file)?;

    option_handle(&header_buffer, &footer_buffer);

    Ok(())
}

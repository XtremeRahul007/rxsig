use std::{env, time::Instant};

mod cli;
mod detector;
mod reader;
mod signatures;

use crate::{
    cli::command_line_handler,
    detector::{detect_signature, option_handle},
    reader::{file_path_handler, read_header},
};

fn main() {
    let start = Instant::now();
    run();
    let duration = start.elapsed();
    println!("Execution Time: {:?}", duration);
}

fn run() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    command_line_handler(&args);

    let file = file_path_handler(&args);

    let buffer: [u8; 16] = read_header(file);

    detect_signature(&buffer);

    option_handle(&buffer);
}

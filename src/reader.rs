use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
    process,
};

pub fn file_path_handler(args: &[String]) -> File {
    let path: &Path = Path::new(&args[1]);

    let file = File::open(path);

    match file {
        Ok(file) => file,

        Err(error) => {
            println!(
                "Error: Failed to open file.\nReason: {}.\nUsage: rxsig <FILE>\nFor more information, try 'rxsig --help' or 'rxsig -h'",
                error
            );
            process::exit(1);
        }
    }
}

pub fn read_header(file: &mut File) -> [u8; 576] {
    let mut buffer: [u8; 576] = [0u8; 576];

    match file.read_exact(&mut buffer) {
        Ok(_) => {
            println!("Header: {:?}", buffer);
            buffer
        }
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    }
}

pub fn read_footer(file: &mut File) -> [u8; 16] {
    match file.seek(SeekFrom::End(-16)) {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    }
    let mut buffer: [u8; 16] = [0u8; 16];

    match file.read_exact(&mut buffer) {
        Ok(_) => {
            println!("Footer: {:?}", buffer);
            buffer
        }
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    }
}

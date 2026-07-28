use std::{
    fs::File,
    io::{BufReader, Read},
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

pub fn read_header(file: File) -> [u8; 16] {
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut buffer: [u8; 16] = [0u8; 16];

    match reader.read_exact(&mut buffer) {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    }
    buffer
}

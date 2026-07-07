use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

const SIGNATURES: [(&str, &[u8]); 3] = [
    ("PNG", &[0x89, 0x50, 0x4E, 0x47]),
    ("JPEG", &[0xFF, 0xD8, 0xFF]),
    ("PDF", &[0x25, 0x50, 0x44, 0x46]),
];

fn main() -> io::Result<()> {
    let byte_array: [u8; 16] = read_header()?;

    println!("{:?}", &byte_array);

    let option: Option<&str> = detect_signature(&byte_array);

    match option {
        Some(name) => println!("This file is: {}", name),
        None => println!("File doesn't exist in record"),
    }
    Ok(())
}

fn detect_signature(byte_array: &[u8; 16]) -> Option<&str> {
    for (name, signature) in SIGNATURES {
        if byte_array.starts_with(signature) {
            return Some(name);
        }
    }
    None
}

fn read_header() -> io::Result<[u8; 16]> {
    let array: Vec<String> = env::args().collect::<Vec<String>>();

    let path: &Path = Path::new(&array[1]);

    let file: File = File::open(path)?;

    let f: BufReader<File> = BufReader::new(file);

    let mut byte_array: [u8; 16] = [0u8; 16];

    let mut index: usize = 0;

    for byte in f.bytes().take(16) {
        byte_array[index] = byte?;
        index += 1;
    }

    Ok(byte_array)
}


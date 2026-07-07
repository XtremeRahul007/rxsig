use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

fn main() -> io::Result<()> {
    let byte_array: [u8; 16]  = detect_file_type()?;

    println!("{:?}", byte_array);
    Ok(())
}

fn detect_file_type() -> io::Result<[u8; 16]> {
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


/*
const SIGNATURES: [(&str, &[u8]); 3] = [
    ("PNG", &[0x89, 0x50, 0x4E, 0x47]),
    ("JPEG", &[0xFF, 0xD8, 0xFF]),
    ("PDF", &[0x25, 0x50, 0x44, 0x46]),
];
*/
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

use crate::error::AppError;

pub fn file_path_handler(args: &[String]) -> Result<File, AppError> {
    let path_str = args.get(1).ok_or_else(|| {
        AppError::new("Error: No file path provided.\nUsage: rxsig <FILE>\nFor more information, try 'rxsig --help' or 'rxsig -h'")
    })?;

    let path = Path::new(path_str);

    let file = File::open(path).map_err(|err| {
        AppError::new(format!(
            "Error: Failed to open file.\nReason: {err}.\nUsage: rxsig <FILE>\nFor more information, try 'rxsig --help' or 'rxsig -h'"
        ))
    })?;

    Ok(file)
}

pub fn read_header(file: &mut File) -> Result<Vec<u8>, AppError> {
    let mut buffer: Vec<u8> = vec![0u8; 32768];

    file.read(&mut buffer)?;

    /*println!("Header: {:?}", buffer);*/
    Ok(buffer)
}

pub fn read_footer(file: &mut File) -> Result<Vec<u8>, AppError> {
    file.seek(SeekFrom::End(-64))?;
    let mut buffer: Vec<u8> = vec![0u8; 64];

    file.read(&mut buffer)?;

    /*println!("Footer: {:?}", buffer);*/
    Ok(buffer)
}

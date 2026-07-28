use crate::signatures::{SIGNATURES, Signature};

pub fn detect_signature(buffer: &[u8; 16]) -> Option<&Signature> {
    SIGNATURES
        .iter()
        .find(|&sig| buffer.starts_with(sig.header))
}

pub fn option_handle(buffer: &[u8; 16]) {
    match detect_signature(buffer) {
        Some(sig) => println!("File Type: {}", sig.extension),
        None => println!("File doesn't exist in record"),
    }
}

use crate::signatures::{SIGNATURES, Signature};

pub fn option_handle(header_buffer: &[u8; 576], footer_buffer: [u8; 16]) {
    match detect_signature(header_buffer, footer_buffer) {
        Some(sig) => {
            println!("File Type: {}", sig.extension);
            println!("File Class: {}", sig.class);
            println!("Description: {}", sig.description);
            println!("Offset: {}", sig.offset);
            println!("Trailer: {:?}", sig.trailer);
        }
        None => println!("File doesn't exist in record"),
    }
}

fn detect_signature(header_buffer: &[u8; 576], footer_buffer: [u8; 16]) -> Option<&Signature> {
    SIGNATURES.iter().find(|&sig| {
        header_buffer[sig.offset as usize..].starts_with(sig.header)
            && footer_buffer.ends_with(sig.trailer)
    })
}

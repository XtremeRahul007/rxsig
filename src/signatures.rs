pub struct Signature {
    /*pub description: String,*/
    pub extension: &'static str,
    /*pub class: String,*/
    pub header: &'static [u8],
    /*pub ofset: usize,
    pub trailer: Option<Vec<u8>>*/
}

pub const SIGNATURES: &[Signature] = &[
    Signature {
        extension: "JPG",
        header: &[0xFF, 0xD8, 0xFF],
    },
    Signature {
        extension: "PNG",
        header: &[0x89, 0x50, 0x4e, 0x47],
    },
    Signature {
        extension: "BMP",
        header: &[0x42, 0x4d],
    },
    Signature {
        extension: "FITS",
        header: &[0x53, 0x49, 0x4d, 0x50, 0x4c, 0x45],
    },
    Signature {
        extension: "GIF",
        header: &[0x47, 0x49, 0x46, 0x38],
    },
    Signature {
        extension: "GKS",
        header: &[0x47, 0x4b, 0x53, 0x4d],
    },
    Signature {
        extension: "IRIS RGB",
        header: &[0x01, 0xda],
    },
];

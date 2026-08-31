// NOTE:
// This is NOT a general-purpose JSON parser.
// It is a code generator designed specifically for the
// Gary Kessler file signature JSON format.
// If the input schema changes, this generator must be updated.

use std::{
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    process,
    time::Instant,
};

struct Entry {
    import_file_name: &'static str,
    import_file_path: &'static str,
    export_file_name: &'static str,
    export_file_path: &'static str,
    export_folder: &'static str,
}

const ENTRY: Entry = Entry {
    import_file_name: "./file_sigs.json",
    import_file_path: "../../data",
    export_file_name: "signatures.rs",
    export_file_path: "../.././generated-data",
    export_folder: "../.././generated-data",
};

const STRUCT: &str = "pub struct Signature {
    pub description: &'static str,
    pub header: &'static [u8],
    pub extension: &'static str,
    pub class: &'static str,
    pub offset: u16,
    pub trailer: &'static [u8],
}

pub const SIGNATURES: &[Signature] = &[";

type ParseFn = fn(&str, &str) -> String;

const PARSE_REPLACEMENTS: &[(&str, &str, ParseFn)] = &[
    (
        "        \"File description\": \"",
        "        description: ",
        parse_default,
    ),
    (
        "        \"Header (hex)\": \"",
        "        header: ",
        parse_header,
    ),
    (
        "        \"File extension\": \"",
        "        extension: ",
        parse_default,
    ),
    (
        "        \"FileClass\": \"",
        "        class: ",
        parse_default,
    ),
    (
        "        \"Header offset\": \"",
        "        offset: ",
        parse_offset,
    ),
    (
        "        \"Trailer (hex)\": \"",
        "        trailer: ",
        parse_trailer,
    ),
    ("    {", "    Signature {", parse_structure),
    ("[", STRUCT, parse_structure),
    ("]", "];", parse_structure),
];

fn main() {
    let start = Instant::now();
    run();
    let duration = start.elapsed();
    println!("Execution Time: {:?}", duration);
}

fn run() {
    create_dir();
    generate_file();
}

fn generate_file() {
    let buffer = get_buffer();

    let file = create_file();

    let mut writer = init_writer(file);

    for line in buffer.lines() {
        match line {
            Ok(line) => write_line(&mut writer, parse_lines(line)),
            Err(error) => {
                println!("{}", error);
                process::exit(1);
            }
        }
    }
    match writer.flush() {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    }
}

fn get_buffer() -> BufReader<File> {
    let file_path = Path::new(ENTRY.import_file_path).join(ENTRY.import_file_name);
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(error) => {
            println!(
                "Alert: \"{}\" doesn't exists. \nError: {}",
                file_path.display(),
                error
            );
            process::exit(1);
        }
    };

    BufReader::new(file)
}

fn init_writer(file: File) -> BufWriter<File> {
    BufWriter::new(file)
}

fn write_line(writer: &mut BufWriter<File>, string: String) {
    match writer.write_all(string.as_bytes()) {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    }
    match writer.write_all(b"\n") {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    }
}

fn create_dir() {
    match fs::create_dir_all(ENTRY.export_folder) {
        Ok(folder) => folder,
        Err(error) => {
            println!("Reason: {}", error);
            process::exit(1);
        }
    }
}

fn create_file() -> File {
    let file_path = Path::new(ENTRY.export_file_path).join(ENTRY.export_file_name);

    match File::create(&file_path) {
        Ok(file) => file,
        Err(error) => {
            println!(
                "Alert: \"{}\" doesn't exists. \nError: {}",
                file_path.display(),
                error
            );
            process::exit(1);
        }
    }
}

fn parse_lines(line: String) -> String {
    for (from, to, func) in PARSE_REPLACEMENTS {
        if line.starts_with(from) {
            match line
                .strip_prefix(from)
                .map(|s| s.strip_suffix(',').unwrap_or(s))
                .map(|s| s.strip_suffix('\"').unwrap_or(s))
            {
                Some(value) => {
                    return func(to, value);
                }
                None => process::exit(1),
            };
        }
    }
    line
}

fn parse_structure(to: &str, value: &str) -> String {
    format!("{}{}", to, value)
}

fn parse_default(to: &str, value: &str) -> String {
    format!("{}\"{}\",", to, value)
}

fn parse_trailer(to: &str, value: &str) -> String {
    format!("{}&[{}],", to, trailer(value))
}

fn parse_header(to: &str, value: &str) -> String {
    format!("{}&[{}],", to, bytes_to_rust(parse_hex_array(value)))
}

fn parse_offset(to: &str, value: &str) -> String {
    format!("{}{},", to, value)
}

fn trailer(value: &str) -> String {
    if value != "(null)" {
        return bytes_to_rust(parse_hex_array(value));
    }
    bytes_to_rust(vec![])
}

fn parse_hex_array(value: &str) -> Vec<u8> {
    value
        .split_whitespace()
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect()
}

fn bytes_to_rust(bytes: Vec<u8>) -> String {
    bytes
        .iter()
        .map(|b| format!("0x{:02X}", b))
        .collect::<Vec<String>>()
        .join(", ")
}

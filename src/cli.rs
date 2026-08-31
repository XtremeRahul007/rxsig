use std::process;

const HELP_TEXT: &str = "rxsig - Detect file types using magic numbers.

USAGE:
    rxsig <FILE>

ARGS:
    <FILE>    Path to the file to inspect.

OPTIONS:
    -h, --help    Show this help message.

EXAMPLES:
    rxsig image.png
    rxsig archive.zip
    rxsig document.pdf

SUPPORTED FILE TYPES:
    PNG
    JPEG
    PDF

DESCRIPTION:
    rxsig reads the first few bytes of a file (its magic number)
    and identifies the file type without relying on the file extension.

EXIT STATUS:
    0    Success
    1    Error
";

pub fn command_line_handler(args: &[String]) {
    validate_argument_count(args);
    handle_help_argument(args);
}

fn validate_argument_count(args: &[String]) {
    let args_length: usize = args.len();
    if args_length > 2 {
        println!(
            "Error: Too many arguments. Expected 1, received {}.\nUsage: rxsig <arg1>\nFor more information, try 'rxsig --help' or 'rxsig -h'",
            args_length - 1
        );
        process::exit(1);
    }
}

fn handle_help_argument(args: &[String]) {
    match args.get(1).map(|s: &String| s.as_str()) {
        Some("--help") | Some("-h") => {
            println!("{}", HELP_TEXT);
            process::exit(0);
        }
        None => {
            println!(
                "Error: Missing required argument.\nUsage: rxsig <arg1>\nFor more information, try 'rxsig --help' or 'rxsig -h'"
            );
            process::exit(1);
        }
        Some(_) => {}
    }
}

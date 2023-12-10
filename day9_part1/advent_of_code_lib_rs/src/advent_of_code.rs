


use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};
use super::file_parser::FileParser;

fn load_file(mut args: std::env::Args) -> Result<std::fs::File, (String, std::io::Error)> {
    let _ = args.next();
    let file = match args.next() {
        None => {
            println!("No input file specified, defaulting to input_sample.txt");

            let file = File::open("input_sample.txt");
            let mut file = match file {
                Ok(v) => v,
                Err(e) => {
                    let mut error_msg = String::from("Failed to find the default file input_sample.txt\n");
                    return Err((error_msg, e));
                }
            };

            return Ok(file);
        }

        Some(file_path) => {
            let file = File::open(file_path);
            let mut file = match file {
                Ok(v) => v,
                Err(e) => {
                    let mut error_string = String::from("Failed to open file given\n");
                    return Err((error_string, e));
                }
            };

            return Ok(file);
        }
    };
}

fn buffer_file(mut args: std::env::Args) -> Result<std::io::BufReader<File>, (String, std::io::Error)> {
    let file = load_file(args)?;
    let buf = BufReader::new(file);

    return Ok(buf);
}

pub fn scan_file(mut args: std::env::Args) -> Result<FileParser<BufReader<File>>, (String, std::io::Error)> {
    let buf = buffer_file(args)?;
    let scanner = FileParser::new_from_reader(buf);

    return Ok(scanner);
}
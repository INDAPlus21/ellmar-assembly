use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::{ ErrorKind };

const TAG_ARGUMENT_NOT_VALID: &str = "ArgumentNotValid";

/// print error message, "inspired" by Viola
fn print_error(tag: &str, err_msg: &str) {
    println!("[ERROR] {}: {}", tag, err_msg)
}

fn parse_file() -> Result<(), ErrorKind>{
    // get arguments from terminal
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_error(TAG_ARGUMENT_NOT_VALID, "A path to a file must be provided!");
        return Err(ErrorKind::InvalidData);
    }
    else {
        match fs::read_to_string(args[1].clone()) {
            // interpret if read succesfully
            Ok(_contents) => {
                println!("Running file: {:?}\n", args[1]);

                // get non-empty lines
                let lines: Vec<String> = _contents
                    .split("\n")
                    .map(|_line| _line.to_string())
                    .collect();

                // Does this work?
                return Ok(());
            }
            Err(_err) => {
                print_error(TAG_ARGUMENT_NOT_VALID, match _err.kind() {
                    ErrorKind::InvalidData => "The provided path is not a path!",
                    ErrorKind::NotFound => "File not found!",
                    _ => "Failed to read file!"
                });
                return Err(_err.kind());
            }
        }
    }
}

fn main() {
    parse_file();
}

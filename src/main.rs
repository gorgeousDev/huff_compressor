//modules
mod logic;

//imports
use std::path;
use crate::logic::{compress, print_help};

fn main() {
    run_program();
}
fn run_program() {
    let mut args = std::env::args().skip(1);

    if let Some(flag) = args.next() {
        match flag.as_str() {
            "-h" | "--help" => print_help(),
            "-f" | "--file" => {
                if let Some(file_path) = args.next() {
                    let path_buf = path::PathBuf::from(file_path);
                    match path_buf.is_file() {
                        true => compress(path_buf),
                        false => {
                            eprintln!("Error: {} is not a file.", path_buf.display());
                        }
                    }
                }
            }
            &_ => {
                eprintln!("Warning: You need a flag first to use this tool.");
                print_help();
            }
        }
    } else {
        print_help();
    }
}

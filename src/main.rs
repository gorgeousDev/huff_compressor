//modules
pub mod constants;
mod logic;
mod tests;

//imports
use crate::constants::*;
use crate::logic::{compress, print_help};
use std::path;

fn main() {
    run_program();
}

fn run_program() {
    let mut args = std::env::args().skip(1);

    // Check if the first argument is the compress flag
    if let Some(flag) = args.next() {
        if flag == "-c" || flag == "--compress" {
            // Expect the next argument to be the file path
            if let Some(file_path_str) = args.next() {
                let path_buf = std::path::PathBuf::from(file_path_str);

                // Call the compress function with the provided file path
                compress(path_buf);
            } else {
                eprintln!("{} No file path provided for compression.", *ERROR_RED_BOLD);
                print_help();
            }
        } else {
            eprintln!("{} Unknown flag '{}'", *ERROR_RED_BOLD, flag);
            print_help();
        }
    } else {
        eprintln!("{} No arguments provided.", *ERROR_RED_BOLD);
        print_help();
    }

    if let Some(flag) = args.next() {
        match flag.as_str() {
            "-h" | "--help" => print_help(),
            "-f" | "--file" => {
                if let Some(file_path) = args.next() {
                    let path_buf = path::PathBuf::from(file_path);
                    match path_buf.is_file() {
                        true => compress(path_buf),
                        false => {
                            eprintln!("{} {} is not a file.", *ERROR_RED_BOLD, path_buf.display());
                        }
                    }
                }
            }
            &_ => {
                eprintln!(
                    "{} You need a flag first to use this tool.",
                    *WARNING_YELLOW_BOLD
                );
                print_help();
            }
        }
    } else {
        print_help();
    }
}

//modules
 mod logic;
 pub mod constants;
 mod tests;

//imports
use std::path;
use crate::logic::{compress, print_help};
use crate::constants::*;

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
                            eprintln!("{} {} is not a file.", *ERROR_RED_BOLD, path_buf.display());
                        }
                    }
                }
            }
            &_ => {
                eprintln!("{} You need a flag first to use this tool.", *WARNING_YELLOW_BOLD);
                print_help();
            }
        }
    } else {
        print_help();
    }
}

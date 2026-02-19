use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub mod encoding_binary_tree;
use self::encoding_binary_tree::*;

pub fn print_help() {
    let help_message = "\
    Help Message \n\
    todo!";


    println!("{help_message}")
}




pub fn compress(path_buf: PathBuf){
    let file_text = fs::read_to_string(&path_buf).unwrap();
    let freqs = get_freqs(&file_text);
    let priority_vec = self::encoding_binary_tree::get_priority_vec(&freqs);

    println!("{:?}", priority_vec);
}


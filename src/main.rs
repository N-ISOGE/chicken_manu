//! # chicken_menu
//!
//!
//!
//!
//!

use std::{fs, io::{Read}, process};
use encoding_rs::Encoding;
// struct BeMusicScript;
// struct Mets;


fn main() {
    let bms_path = "test_resource/";
    let encoding_input = "SHIFT-JIS";
    let mut bms_file = match fs::File::open(bms_path) {
        Ok(read_file) => {
            println!("read {:?}", read_file.metadata());
            read_file
        }
        Err(error_info) => {
            println!("read {:} -> {} ", error_info.kind(), error_info);
            process::exit(-1);
        }
    };

    println!("{:?}", bms_file);
    let bms_encoding_info: &Encoding = match Encoding::for_label(encoding_input.as_bytes()) {
        None => {
            &encoding_rs::UTF_8
        }
        Some(encoding) => {
            encoding
        }
    };


    let metadata = fs::metadata(bms_path).expect("can't read file");
    let file_size: usize = metadata.len() as usize;
    let mut buffer: Vec<u8> = vec![0; file_size];

    bms_file.read_to_end(&mut buffer).expect("fail to read");

    let (cow, encoding_used, had_errors) = bms_encoding_info.decode(&buffer);

    if had_errors {
        println!("error , { }", encoding_used.name());
    } else {
        for (num, line) in cow.lines().enumerate() {
            println!("num {} -> {}", num, line);
        }
    }
}

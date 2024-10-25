use std::{
    fs,
    io::{self, Read, Write},
};

use flate2::read::ZlibDecoder;

pub fn cat_file(args: &[String]) {
    if args[0].as_str() == "-p" {
        pretty_print(args[1].as_str());
    }
}

pub fn pretty_print(hash: &str) {
    // the first two characters of the hash are the folder name, and the rest are the file name
    let folder_name = &hash[0..2];
    let file_name = &hash[2..];
    let path = format!(".vc/objects/{folder_name}/{file_name}");
    let mut object = fs::File::open(&path).expect("error opening file for pretty print");
    let mut content: Vec<u8> = vec![];
    let mut extracted_content = String::new();

    object
        .read_to_end(&mut content)
        .expect("error reading pretty print object to end");

    let mut decoder = ZlibDecoder::new(content.as_slice());

    decoder
        .read_to_string(&mut extracted_content)
        .expect("error reading pretty print to string");
    let split = extracted_content.split("\x00");
    let extracted_content = split.last().expect("error extracting pretty print");

    print!("{extracted_content}");
    io::stdout().flush().expect("error flushing pretty print");
}

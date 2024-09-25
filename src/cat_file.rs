use flate2::read::ZlibDecoder;
use std::{fs, io::Read};

pub fn pretty_print(hash: String) {
    let folder_name = &hash[0..2];
    let file_name = &hash[2..];
    let path = format!(".vc/objects/{folder_name}/{file_name}");

    let mut object = fs::File::open(&path).unwrap();
    let mut content: Vec<u8> = vec![];

    let mut extracted_content = String::new();

    object.read_to_end(&mut content).unwrap();

    let mut decoder = ZlibDecoder::new(content.as_slice());

    decoder.read_to_string(&mut extracted_content).unwrap();
    let split = extracted_content.split("\x00");
    let extracted_content = split.last().unwrap();
    print!("{extracted_content}");
}

use std::{fs, io::Read};
use flate2::read::ZlibDecoder;


pub fn pretty_print(hash:String){
  
    let folder_name=&hash[0..2];
    let file_name=&hash[2..]; 
    let path=format!(".git/objects/{folder_name}/{file_name}");

    let mut object=fs::File::open(&path).unwrap();
    let mut content:Vec<u8>=vec![];

    let mut extracted_content=String::new();

    object.read_to_end(&mut content).unwrap();

    let mut decoder = ZlibDecoder::new(content.as_slice());

   decoder.read_to_string(&mut extracted_content).unwrap();
    print!("{extracted_content}");
}
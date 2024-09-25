use std::{fs::DirBuilder, io::Write, path};

use flate2::read::ZlibDecoder;
use hex::ToHex;

use sha1::{Digest, Sha1};

pub fn hash_object(args:&[String]){
    match args[0].as_str() {
        "-w"=>{
            let filename=&args[1];
            let file=std::fs::read(filename).unwrap();
            let sha=get_sha(&file);
            create_folder(&sha);
            compress(&file);
            print_sha(&sha);
            dbg!(sha);
            let file=std::fs::read(filename).unwrap();

        },
        _=>eprint!("Unknown option")
    }
}

fn get_sha(file:&[u8])->String{
    let mut hasher=Sha1::new();
    hasher.update(file);
    hasher.finalize().encode_hex::<String>()
}

 
fn compress(file:&[u8]){
    // let mut encoded=ZlibDecoder::new();
}
fn create_folder(sha:&str){
    // find the .vc folder 
    let path=format!(".vc/objects/{}",&sha[0..2]);
    DirBuilder::new().create(path).unwrap();

}
fn print_sha(sha:&str){
    println!("{sha}");
    std::io::stdout().flush().unwrap();
}
 
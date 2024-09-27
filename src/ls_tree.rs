use std::{fs::File, io::Read, path::Path, str::from_utf8};

use crate::utils::{decompress, get_object_directory_name, get_object_file_name};

pub fn ls_tree(args:&[String]){

    match args[0].as_str(){
        "--name-only"=>{
            let hash=&args[1];
            let directory_name=get_object_directory_name(hash);
            let file_name=get_object_file_name(hash);
            let path=Path::new(".git").join("objects").join(directory_name).join(file_name);
            let mut file=File::open(path).unwrap();
            let mut content_bytes=vec![];
            file.read_to_end(&mut content_bytes).unwrap();
            let bytes=decompress(&content_bytes);
            let index=bytes.iter().position(|&byte| byte==b'\0').expect("Missing null terminator");
            let header=&bytes[0..index];
            println!("{}",std::str::from_utf8(header).unwrap());
        },
        _=>{
            eprint!("Unknown option")
        }
    }
}
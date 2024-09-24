use std::env;
use versionControl::{cat_file::pretty_print, hash_object::hash_object, init::init};
fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => init(),
        "cat-file" => {
            if args[2] == "-p" {
                let hash = args[3].clone();
                pretty_print(hash);
            }
        },
        "hash-object"=>{
            hash_object(&args[2..]);
        }
        _ => println!("unknown command:{}", args[1]),
    }
}

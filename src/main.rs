use versionControl::{
    cat_file::cat_file, clone::clone, commit_tree::commit_tree, hash_object::hash_object,
    init::init, ls_tree::ls_tree, write_tree::write_tree,
};
use hex::ToHex;
use std::{env, path::PathBuf};

#[tokio::main]
async fn main() {
    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    let rest_of_args = &args[2..];

    match args[1].as_str() {
        "init" => {
            let path = PathBuf::default();
            init(path);
        }
        "cat-file" => cat_file(rest_of_args),
        "hash-object" => {
            let write_flag = args[2].as_str() == "-w";
            let path = PathBuf::new().join(&args[3]);
            let checksum = hash_object(write_flag, path)
                .expect("error running hash object command")
                .encode_hex::<String>();
            println!("{checksum:?}");
        }
        "ls-tree" => ls_tree(rest_of_args),
        "write-tree" => {
            let checksum = write_tree().expect("error running write tree command");
            println!("{checksum}");
        }
        "commit-tree" => {
            let tree = &args[2];
            let parent = if &args[3] == "-p" {
                &args[4]
            } else {
                panic!("missing parent argument");
            };
            let message = if &args[5] == "-m" {
                &args[6]
            } else {
                panic!("missing message argument");
            };
            let hash = commit_tree(tree, parent, message).expect("error running hash command");

            println!("{hash}");
        }
        "clone" => {
            let uri = &args[2];
            let target_directory = &args[3];

            clone(uri, target_directory)
                .await
                .expect("error running clone command");
        }
        _ => println!("unknown command: {}", args[1]),
    }
}

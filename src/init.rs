use std::{fs, path::PathBuf};

pub fn init(mut path: PathBuf) {
    path.push(".vc");
    fs::create_dir_all(&path).expect("error creating .vc directory");

    let mut objects = path.clone();
    objects.push("objects");
    fs::create_dir(&objects).expect("error creating objects directory");

    let mut refs = path.clone();
    refs.push("refs");
    fs::create_dir(&refs).expect("error creating refs directory");

    let mut head = path.clone();
    head.push("HEAD");
    fs::write(&head, "ref: refs/heads/master\n").expect("error writing head info to refs");
    println!("Initialized vc directory");
}

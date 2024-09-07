use std::fs;

pub fn init(){
    if fs::metadata(".vc").is_ok(){
        println!("vc already exist");
        return 
    }
    fs::create_dir(".vc").unwrap();
    fs::create_dir(".vc/objects").unwrap();
    fs::create_dir(".vc/refs").unwrap();
    fs::write(".vc/HEAD", "ref:/heads/master\n").unwrap();
    println!("Initialized the vc directory");
}
use std::fs;
use std::env;
fn main() {
    println!("Hello, world!");
    let args:Vec<String>=env::args().collect();
    if args[1]=="init"{
        fs::create_dir(".vc").unwrap();
        fs::create_dir(".vc/object").unwrap();
        fs::create_dir(".vc/refs").unwrap();
        fs::write(".vc/HEAD", "ref:/heads/master\n").unwrap();
        println!("Initialized the vc directory");
    }else{
        println!("unkown command:{}",args[1])
    }

}

use std::fs::File;
use std::io::BufReader;
use std::env;
use tref;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if let Some(f_n) = args.get(1) { &f_n[..] } else { "file.tref" };

    if let Ok(file) = File::open(file_name) {
        let r = tref::build_tree(BufReader::new(file));
        println!("{:?}", r);
    }
    else {
        println!("Could not read file");
    }
}
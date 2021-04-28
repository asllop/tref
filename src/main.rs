use std::fs::File;
use std::io::BufReader;
use tref;

fn main() {
    if let Ok(file) = File::open("file.tref") {
        let r = tref::parse_tree(BufReader::new(file));
        println!("{:?}", r);
    }
    else {
        println!("Could not read file");
    }
}
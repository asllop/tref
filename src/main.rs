use std::fs::File;
use std::io::BufReader;
use std::env;
use tref::Forest;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if let Some(f_n) = args.get(1) { &f_n[..] } else { "file.tref" };

    if let Ok(file) = File::open(file_name) {
        match Forest::new(BufReader::new(file)) {
            Ok(f) => println!("{:#?}", f),
            Err(m) => println!("ERROR = {}", m)
        }
    }
    else {
        println!("Could not read file {}", file_name);
    }
}
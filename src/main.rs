use std::{fs::File, io::{BufReader, BufWriter}};
use tref::Model as TrefModel;

fn main() {
    if let Ok(file) = File::open("file.tref") {
        match <TrefModel>::parse(BufReader::new(file)) {
            Ok(forest) => {
                if let Some(tree) = forest.get_tree("My Original Tree!") {
                    println!("my_tree = {:#?}", tree);
                }

                let f = File::create("serialized.tref").expect("Unable to create file");
                let mut buf_writer = BufWriter::new(f);
                match <TrefModel>::serialize(&forest, &mut buf_writer) {
                    Ok(num_lines) => {
                        println!("Tree serialized correctly, num lines = {}", num_lines);
                    },
                    Err(e) => {
                        println!("Failed serializing tree: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("Failed parsing document: {}", e);
            }
        }
    }
}
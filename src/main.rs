use std::{fs::File, io::{BufReader, BufWriter}};
use tref;

fn main() {
    if let Ok(file) = File::open("file.tref") {
        let model = <tref::Model>::new();
        match model.parse(BufReader::new(file)) {
            Ok(forest) => {
                if let Some(tree) = forest.get_tree("my_tree") {
                    println!("my_tree = {:#?}", tree);
                }

                let f = File::create("serialized.tref").expect("Unable to create file");
                let mut buf_writer = BufWriter::new(f);
                match model.serialize(&forest, &mut buf_writer) {
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
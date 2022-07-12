use std::{fs::File, io::{BufReader, BufWriter}};
use tref;

fn main() {
    if let Ok(file) = File::open("file.tref") {
        match <tref::Model>::parse(BufReader::new(file)) {
            Ok(forest) => {
                for (tree_id, tree) in forest.iter() {
                    println!("{} = {:#?}", tree_id, tree);
    
                    let f = File::create("serialized.tref").expect("Unable to create file");
                    let mut buf_writer = BufWriter::new(f);
                    match <tref::Model>::serialize(&forest, &mut buf_writer) {
                        Ok(num_lines) => {
                            println!("Tree serialized correctly, num lines = {}", num_lines);
                        },
                        Err(e) => {
                            println!("Failed serializing tree: {}", e);
                        }
                    }

                    println!("--------------------------");
                }
            },
            Err(e) => {
                println!("Failed parsing document: {}", e);
            }
        }
    }
}
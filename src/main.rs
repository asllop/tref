use std::{fs::File, io::{BufReader, BufWriter}};
use tref;

fn main() {
    if let Ok(file) = File::open("file.tref") {
        let model = <tref::Model>::new();
        if let Ok(forest) = model.parse(BufReader::new(file)) {
            if let Some(tree) = forest.get_tree("my_tree") {
                println!("my_tree = {:#?}", tree);
            }

            let f = File::create("serialized.tref").expect("Unable to create file");
            let mut buf_writer = BufWriter::new(f);
            if model.serialize(&forest, &mut buf_writer) {
                println!("Tree serialized correctly");
            }
            else {
                println!("!!Failed serializing tree!!");
                return;
            }
        }
    }
}
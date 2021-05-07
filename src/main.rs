use std::fs::File;
use std::io::BufReader;
use std::env;
use tref::Forest;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if let Some(cmd_arg_1) = args.get(1) { &cmd_arg_1[..] } else { "file.tref" };

    if let Ok(file) = File::open(file_name) {
        let forest = match Forest::new(BufReader::new(file)) {
            Ok(forest) => forest,
            Err(msg) => panic!("ERROR = {}", msg)
        };

        println!("{:#?}", forest);

        if let Some(tree_model) = forest.tree(&String::from("my_tree")) {
            println!("\nTraverse my_tree in BFS:");
            for n in tree_model.bfs_iter() {
                println!("{}", n.content);
            }

            println!("\nTraverse my_tree in Inverse BFS:");
            for n in tree_model.inv_bfs_iter() {
                println!("{}", n.content);
            }
        }

        if let Some(tree_model) = forest.tree(&String::from("my_tree_2")) {
            println!("\nTraverse my_tree_2 in BFS:");
            for n in tree_model.bfs_iter() {
                println!("{}", n.content);
            }
        }
    }
    else {
        println!("Could not read file {}", file_name);
    }
}
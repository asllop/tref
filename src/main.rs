use std::fs::File;
use std::io::BufReader;
use std::env;
use tref::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if let Some(cmd_arg_1) = args.get(1) { &cmd_arg_1[..] } else { "file.tref" };

    if let Ok(file) = File::open(file_name) {
        let forest: Forest<SimpleNode> = match Forest::new(BufReader::new(file)) {
            Ok(f) => f,
            Err(m) => panic!("Could not parse TREF: {}", m)
        };

        println!("{:#?}", forest);

        if let Some(tree_model) = forest.tree(&String::from("my_tree")) {
            println!("\nTraverse my_tree:");
            for n in tree_model.iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse my_tree in BFS:");
            for n in tree_model.bfs_iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse my_tree in Inverse BFS:");
            for n in tree_model.inv_bfs_iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse my_tree in Pre-DFS:");
            for n in tree_model.pre_dfs_iter() {
                println!("{}", n.content.get_content());
            }
        }

        if let Some(tree_model) = forest.tree(&String::from("wp_tree")) {
            println!("\nTraverse wp_tree:");
            for n in tree_model.iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse wp_tree in Inverse Pre-DFS:");
            for n in tree_model.inv_pre_dfs_iter() {
                println!("{}", n.content.get_content());
            }
        }
    }
    else {
        panic!("Could not read file {}", file_name);
    }
}
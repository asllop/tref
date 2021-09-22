use std::{fs::File, io::BufReader};
use tref;
use socarel::RawNode;

fn main() {
    if let Ok(file) = File::open("file.tref") {
        let model = <tref::Model>::new();
        if let Ok(forest) = model.parse(BufReader::new(file)) {
            if let Some(tree) = forest.get_tree("my_tree") {
                println!("my_tree = {:#?}", tree);
            }
        }
    }
}

/*
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::env;
use tref::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(cmd_arg_1) = args.get(1) {
        if cmd_arg_1 == "ser" {
            serialize_tref();
        }
        else {
            parse_tref(args);
        }
    }
    else {
        parse_tref(args);
    }
}

fn parse_tref(args: Vec<String>) {
    let file_name = if let Some(cmd_arg_1) = args.get(1) { &cmd_arg_1[..] } else { "file.tref" };
    
    if let Ok(file) = File::open(file_name) {
        let forest: Forest<SimpleNode> = match Forest::build_levels(BufReader::new(file)) {
            Ok(f) => f,
            Err(m) => panic!("Could not parse TREF: {}", m)
        };

        println!("{:#?}", forest);

        if let Some(tree_model) = forest.tree("my_tree") {
            println!("\nTraverse my_tree:");
            for (n, _) in tree_model.iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse my_tree in BFS:");
            for (n, _) in tree_model.bfs_iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse my_tree in Inverse BFS:");
            for (n, _) in tree_model.inv_bfs_iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse my_tree in Inverse Level BFS:");
            for (n, _) in tree_model.inv_lev_bfs_iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse my_tree in Pre-DFS:");
            for (n, _) in tree_model.pre_dfs_iter() {
                println!("{}", n.content.get_content());
            }
        }

        if let Some(tree_model) = forest.tree("wp_tree") {
            println!("\nTraverse wp_tree:");
            for (n, _) in tree_model.iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse wp_tree in Inverse Pre-DFS:");
            for (n, _) in tree_model.inv_pre_dfs_iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse wp_tree in Post-DFS:");
            for (n, _) in tree_model.post_dfs_iter() {
                println!("{}", n.content.get_content());
            }

            println!("\nTraverse wp_tree in Inverse Post-DFS:");
            for (n, _) in tree_model.inv_post_dfs_iter() {
                println!("{}", n.content.get_content());
            }
        }
    }
    else {
        panic!("Could not read file {}", file_name);
    }
}

fn serialize_tref() {
    println!("\nGenerate trees programatically\n");

    let mut forest: Forest<SimpleNode> = Forest::empty();
    // Create new tree and root node
    forest.new_tree("my_tree");
    let _root = forest.set_root("my_tree", "root_node").unwrap();
    // Add 3 children to root
    let _node_1 = forest.link_node("my_tree", _root, "node_1").unwrap();
    let _node_2 = forest.link_node("my_tree", _root, "node_2").unwrap();
    let _node_3 = forest.link_node("my_tree", _root, "node_3").unwrap();
    // Add 1 child to node_3
    let _node_3_1 = forest.link_node("my_tree", _node_3, "node_3_1").unwrap();
    // Add 2 children to node_1
    let _node_1_1 = forest.link_node("my_tree", _node_1, "node_1_1").unwrap();
    let _node_1_2 = forest.link_node("my_tree", _node_1, "node_1_2").unwrap();

    println!("{:#?}", forest);

    if let Some(tree_model) = forest.tree("my_tree") {
        println!("\nTraverse my_tree:");
        for (n, _) in tree_model.iter() {
            println!("{} ({})", n.content.get_content(), n.level);
        }
        println!("\nTraverse my_tree in Pre-DFS:");
        for (n, _) in tree_model.pre_dfs_iter() {
            println!("{} ({})", n.content.get_content(), n.level);
        }
    }

    forest.unlink_node("my_tree", _node_1).unwrap();

    println!("\nAfter unlinking node_1 {:#?}", forest);

    if let Some(tree_model) = forest.tree("my_tree") {
        println!("\nTraverse my_tree:");
        for (n, _) in tree_model.iter() {
            println!("{} ({})", n.content.get_content(), n.level);
        }
        println!("\nTraverse my_tree in Pre-DFS:");
        for (n, _) in tree_model.pre_dfs_iter() {
            println!("{} ({})", n.content.get_content(), n.level);
        }
    }

    let f = File::create("./serialized.tref").expect("Unable to create file");
    let mut buf_writer = BufWriter::new(f);

    //let mut buf_writer = BufWriter::new(Vec::new());

    if !forest.serialize(&mut buf_writer) {
        println!("Failed serializing tree");
        return;
    }

    /*
    let bytes = buf_writer.into_inner().unwrap();
    let string = String::from_utf8(bytes).unwrap();

    println!("\nSerialized into a string =\n\n{}", string);
    */
}
*/
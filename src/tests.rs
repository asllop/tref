use std::io::BufReader;
use std::io::prelude::*;
use crate::{NodeContent, SimpleNode, Forest};

fn tref_sample() -> BufReader<impl Read> {
    let tref =
    "[test_tree]\n\
    + root_node\n\
    + + child_1\n\
    + + child_2\n\
    + + + child_2_1\n\
    + + + + child_2_1_1\n\
    + + + child_2_2\n\
    + + child_3\n";
    BufReader::new(tref.as_bytes())
}

#[test]
fn check_forest_integrity() {
    let forest: Result<Forest<SimpleNode>, String> = Forest::build(tref_sample());
    match forest {
        Ok(forest) => {
            if let Some(tree_model) = forest.tree(&String::from("test_tree")) {
                for (i,n) in tree_model.iter().enumerate() {
                    match i {
                        0 => {
                            if !n.content.get_content().eq("root_node") { panic!("Wrong root_node content!"); }
                            if let Some(_) = n.parent_position { panic!("root_node has a parent!"); }
                            if n.children.len() != 3 { panic!("root_node hasn't 3 children!"); }
                            if n.children[0] != 1 || n.children[1] != 2 || n.children[2] != 6 { panic!("root_node children are incorrect!"); }
                        },
                        1 => {
                            if !n.content.get_content().eq("child_1") { panic!("Wrong child_1 content!"); }
                            if let None = n.parent_position { panic!("child_1 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 0 {
                                    panic!("child_1 has wrong parent!");
                                }
                            }
                            if n.children.len() != 0 { panic!("child_1 hasn't 0 children!"); }
                        },
                        2 => {
                            if !n.content.get_content().eq("child_2") { panic!("Wrong child_2 content!"); }
                            if let None = n.parent_position { panic!("child_2 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 0 {
                                    panic!("child_2 has wrong parent!");
                                }
                            }
                            if n.children.len() != 2 { panic!("child_2 hasn't 2 children!"); }
                            if n.children[0] != 3 || n.children[1] != 5 { panic!("child_2 children are incorrect!"); }
                        },
                        3 => {
                            if !n.content.get_content().eq("child_2_1") { panic!("Wrong child_2_1 content!"); }
                            if let None = n.parent_position { panic!("child_2_1 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 2 {
                                    panic!("child_2_1 has wrong parent!");
                                }
                            }
                            if n.children.len() != 1 { panic!("child_2_1 hasn't 1 child!"); }
                            if n.children[0] != 4 { panic!("child_2_1 children are incorrect!"); }
                        },
                        4 => {
                            if !n.content.get_content().eq("child_2_1_1") { panic!("Wrong child_2_1_1 content!"); }
                            if let None = n.parent_position { panic!("child_2_1_1 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 3 {
                                    panic!("child_2_1_1 has wrong parent!");
                                }
                            }
                            if n.children.len() != 0 { panic!("child_2_1_1 hasn't 0 children!"); }
                        },
                        5 => {
                            if !n.content.get_content().eq("child_2_2") { panic!("Wrong child_2_2 content!"); }
                            if let None = n.parent_position { panic!("child_2_2 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 2 {
                                    panic!("child_2_2 has wrong parent!");
                                }
                            }
                            if n.children.len() != 0 { panic!("child_2_2 hasn't 0 children!"); }
                        },
                        6 => {                          
                            if !n.content.get_content().eq("child_3") { panic!("Wrong child_3 content!"); }
                            if let None = n.parent_position { panic!("child_3 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 0 {
                                    panic!("child_3 has wrong parent!");
                                }
                            }
                            if n.children.len() != 0 { panic!("child_3 hasn't 0 children!"); }
                        }
                        _ => {}
                    }
                }
            }
        },
        Err(msg) => panic!("ERROR = {}", msg)
    }
}

#[test]
fn check_bfs_iter() {
    let forest: Result<Forest<SimpleNode>, String> = Forest::build(tref_sample());
    match forest {
        Ok(forest) => {
            if let Some(tree_model) = forest.tree(&String::from("test_tree")) {
                for (i,n) in tree_model.bfs_iter().enumerate() {
                    match i {
                        0 => {
                            if !n.content.get_content().eq("root_node") { panic!("Wrong {} node position in BFS!", n.content.get_content()); }
                        },
                        1 => {
                            if !n.content.get_content().eq("child_1") { panic!("Wrong {} node position in BFS!", n.content.get_content()); }
                        },
                        2 => {
                            if !n.content.get_content().eq("child_2") { panic!("Wrong {} node position in BFS!", n.content.get_content()); }
                        },
                        3 => {
                            if !n.content.get_content().eq("child_3") { panic!("Wrong {} node position in BFS!", n.content.get_content()); }
                        },
                        4 => {
                            if !n.content.get_content().eq("child_2_1") { panic!("Wrong {} node position in BFS!", n.content.get_content()); }
                        },
                        5 => {
                            if !n.content.get_content().eq("child_2_2") { panic!("Wrong {} node position in BFS!", n.content.get_content()); }
                        },
                        6 => {
                            if !n.content.get_content().eq("child_2_1_1") { panic!("Wrong {} node position in BFS!", n.content.get_content()); }
                        }
                        _ => {}
                    }
                }
            }
        },
        Err(msg) => panic!("ERROR = {}", msg)
    }
}

//TODO: check all iterators

//TODO: check both with and without levels struct

#[test]
fn check_dialect() {
    #[derive(Debug)]
    struct WeightNode {
        content: String,
        weight: u32
    }

    impl WeightNode {
        fn get_weight(&self) -> u32 {
            self.weight
        }
    }

    impl NodeContent for WeightNode {
        fn new(content: String) -> Option<Self> {
            let vec: Vec<&str> = content.split(':').collect();
            if vec.len() == 2 {
                match vec[0].trim().parse() {
                    Ok(num) => Some(Self {
                        content: String::from(vec[1]),
                        weight: num
                    }),
                    Err(_) => None
                }
            }
            else {
                None
            }
        }

        fn get_content(&self) -> &String {
            &self.content
        }
    }

    let tref =
    "[test_tree]\n\
    + 0:root_node\n\
    + + 10:child_1\n\
    + + + 25:child_1_1\n\
    + + + + 12:child_1_1_1\n";

    let forest: Result<Forest<WeightNode>, String> = Forest::build(BufReader::new(tref.as_bytes()));
    match forest {
        Ok(forest) => {
            if let Some(tree_model) = forest.tree(&String::from("test_tree")) {
                for (i,n) in tree_model.iter().enumerate() {
                    match i {
                        0 => {
                            if !n.content.get_content().eq("root_node") { panic!("Wrong {} node content!", n.content.get_content()); }
                            if n.content.get_weight() != 0 { panic!("Wrong {} node weight!", n.content.get_weight()); }
                        },
                        1 => {
                            if !n.content.get_content().eq("child_1") { panic!("Wrong {} node content!", n.content.get_content()); }
                            if n.content.get_weight() != 10 { panic!("Wrong {} node weight!", n.content.get_weight()); }
                        },
                        2 => {
                            if !n.content.get_content().eq("child_1_1") { panic!("Wrong {} node content!", n.content.get_content()); }
                            if n.content.get_weight() != 25 { panic!("Wrong {} node weight!", n.content.get_weight()); }
                        },
                        3 => {
                            if !n.content.get_content().eq("child_1_1_1") { panic!("Wrong {} node content!", n.content.get_content()); }
                            if n.content.get_weight() != 12 { panic!("Wrong {} node weight!", n.content.get_weight()); }
                        },
                        _ => {}
                    }
                }
            }
        },
        Err(msg) => panic!("ERROR = {}", msg)
    }
}

#[test]
fn check_dialect_enum() {
    #[derive(Debug)]
    enum TypedNode {
        Text(String),
        Number(String, u32)
    }

    impl NodeContent for TypedNode {
        fn new(content: String) -> Option<Self> {
            match content.trim().parse() {
                Ok(num) => Some(Self::Number(content, num)),
                Err(_) => Some(Self::Text(content))
            }
        }

        fn get_content(&self) -> &String {
            match self {
                Self::Text(t) => t,
                Self::Number(t, _) => t
            }
        }
    }

    let tref =
    "[test_tree]\n\
    + root\n\
    + + child\n\
    + + + 2500\n\
    + + + 130\n";

    let forest: Result<Forest<TypedNode>, String> = Forest::build(BufReader::new(tref.as_bytes()));
    match forest {
        Ok(forest) => {
            if let Some(tree_model) = forest.tree(&String::from("test_tree")) {
                for (i,n) in tree_model.iter().enumerate() {
                    match i {
                        0 => {
                            if !n.content.get_content().eq("root") { panic!("Wrong {} node content!", n.content.get_content()); }
                            if let TypedNode::Number(_,_) = n.content { panic!("Wrong {} node type!", n.content.get_content()); }
                        },
                        1 => {
                            if !n.content.get_content().eq("child") { panic!("Wrong {} node content!", n.content.get_content()); }
                            if let TypedNode::Number(_,_) = n.content { panic!("Wrong {} node type!", n.content.get_content()); }
                        },
                        2 => {
                            if !n.content.get_content().eq("2500") { panic!("Wrong {} node content!", n.content.get_content()); }
                            if let TypedNode::Text(_) = n.content { panic!("Wrong {} node type!", n.content.get_content()); }
                        },
                        3 => {
                            if !n.content.get_content().eq("130") { panic!("Wrong {} node content!", n.content.get_content()); }
                            if let TypedNode::Text(_) = n.content { panic!("Wrong {} node type!", n.content.get_content()); }
                        },
                        _ => {}
                    }
                }
            }
        },
        Err(msg) => panic!("ERROR = {}", msg)
    }
}

#[test]
fn check_generate() {
    let mut forest: Forest<SimpleNode> = Forest::empty();
    let tree_id = String::from("my_tree");
    // Create new tree and root node
    forest.new_tree(&tree_id);
    let _root = forest.set_root(&tree_id, &String::from("root_node")).unwrap();
    // Add 3 children to root
    let _node_1 = forest.link_node(&tree_id, _root, &String::from("node_1")).unwrap();
    let _node_2 = forest.link_node(&tree_id, _root, &String::from("node_2")).unwrap();
    let _node_3 = forest.link_node(&tree_id, _root, &String::from("node_3")).unwrap();
    // Add 1 child to node_3
    let _node_3_1 = forest.link_node(&tree_id, _node_3, &String::from("node_3_1")).unwrap();
    // Add 2 children to node_1
    let _node_1_1 = forest.link_node(&tree_id, _node_1, &String::from("node_1_1")).unwrap();
    let _node_1_2 = forest.link_node(&tree_id, _node_1, &String::from("node_1_2")).unwrap();

    if let Some(tree_model) = forest.tree(&String::from("my_tree")) {
        for (i,n) in tree_model.pre_dfs_iter().enumerate() {
            match i {
                0 => {
                    if !n.content.get_content().eq("root_node") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                1 => {
                    if !n.content.get_content().eq("node_1") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                2 => {
                    if !n.content.get_content().eq("node_1_1") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                3 => {
                    if !n.content.get_content().eq("node_1_2") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                4 => {
                    if !n.content.get_content().eq("node_2") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                5 => {
                    if !n.content.get_content().eq("node_3") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                6 => {
                    if !n.content.get_content().eq("node_3_1") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                _ => {}
            }
        }
    }
}

#[test]
fn check_modify_tree() {
    let mut forest: Forest<SimpleNode> = Forest::build(tref_sample()).unwrap();
    // Add new nodes to an existing forest parsed from a TREF file
    let _child_4 = forest.link_node(&String::from("test_tree"), 0, &String::from("child_4")).unwrap();
    let _child_4_1 = forest.link_node(&String::from("test_tree"), _child_4, &String::from("child_4_1")).unwrap();
    let _child_4_2 = forest.link_node(&String::from("test_tree"), _child_4, &String::from("child_4_2")).unwrap();

    if let Some(tree_model) = forest.tree(&String::from("test_tree")) {
        for (i,n) in tree_model.pre_dfs_iter().enumerate() {
            match i {
                0 => {
                    if !n.content.get_content().eq("root_node") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                1 => {
                    if !n.content.get_content().eq("child_1") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                2 => {
                    if !n.content.get_content().eq("child_2") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                3 => {
                    if !n.content.get_content().eq("child_2_1") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                4 => {
                    if !n.content.get_content().eq("child_2_1_1") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                5 => {
                    if !n.content.get_content().eq("child_2_2") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                6 => {
                    if !n.content.get_content().eq("child_3") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                7 => {
                    if !n.content.get_content().eq("child_4") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                8 => {
                    if !n.content.get_content().eq("child_4_1") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                9 => {
                    if !n.content.get_content().eq("child_4_2") { panic!("Wrong {} node content!", n.content.get_content()); }
                }
                _ => {}
            }
        }
    }
}

#[test]
fn check_find_node() {
    let forest: Forest<SimpleNode> = Forest::build(tref_sample()).unwrap();
    let child_2_1_1 = forest.find_node(&String::from("test_tree"), vec!(String::from("root_node"), String::from("child_2"), String::from("child_2_1"), String::from("child_2_1_1"))).unwrap();
    let tree = forest.tree(&String::from("test_tree")).unwrap();
    if tree.tree_ref.nodes[child_2_1_1 as usize].content.get_content() != &String::from("child_2_1_1") {
        panic!("Child index incorrect");
    }

    if tree.tree_ref.nodes[child_2_1_1 as usize].level != 4 {
        panic!("Child level incorrect");
    }

    // Check incorrect path (wrong root)
    let x = forest.find_node(&String::from("test_tree"), vec!(String::from("wrong_root_node"), String::from("child_2"), String::from("child_2_1"), String::from("child_2_1_1")));
    if let Some(_) = x {
        panic!("Wrong path (root node), but returned a node");
    }

    // Check incorrect path (wrong middle node)
    let x = forest.find_node(&String::from("test_tree"), vec!(String::from("root_node"), String::from("child_2"), String::from("wrong_child_2_1"), String::from("child_2_1_1")));
    if let Some(_) = x {
        panic!("Wrong path (middle node), but returned a node");
    }

    // Check incorrect path (wrong end node)
    let x = forest.find_node(&String::from("test_tree"), vec!(String::from("root_node"), String::from("child_2"), String::from("child_2_1"), String::from("wrong_child_2_1_1")));
    if let Some(_) = x {
        panic!("Wrong path (end node), but returned a node");
    }
}
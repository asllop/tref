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
    let forest: Result<Forest<SimpleNode>, String> = Forest::new(tref_sample());
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
    let forest: Result<Forest<SimpleNode>, String> = Forest::new(tref_sample());
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
        fn new(content: String) -> Self {
            let vec: Vec<&str> = content.split(':').collect();
            if vec.len() != 2 {
                Self {
                    content: String::new(),
                    weight: 0
                }
            }
            else {
                Self {
                    content: String::from(vec[1]),
                    weight: match vec[0].trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0
                    }
                }
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
    
    let forest: Result<Forest<WeightNode>, String> = Forest::new(BufReader::new(tref.as_bytes()));
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

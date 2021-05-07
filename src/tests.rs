use std::io::BufReader;
use std::io::prelude::*;

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
    match crate::Forest::new(tref_sample()) {
        Ok(forest) => {
            if let Some(tree_model) = forest.tree(&String::from("test_tree")) {
                for (i,n) in tree_model.iter().enumerate() {
                    match i {
                        0 => {
                            if !n.content.eq("root_node") { panic!("Wrong root_node content!"); }
                            if let Some(_) = n.parent_position { panic!("root_node has a parent!"); }
                            if n.children.len() != 3 { panic!("root_node hasn't 3 children!"); }
                            if n.children[0] != 1 || n.children[1] != 2 || n.children[2] != 6 { panic!("root_node children are incorrect!"); }
                        },
                        1 => {
                            if !n.content.eq("child_1") { panic!("Wrong child_1 content!"); }
                            if let None = n.parent_position { panic!("child_1 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 0 {
                                    panic!("child_1 has wrong parent!");
                                }
                            }
                            if n.children.len() != 0 { panic!("child_1 hasn't 0 children!"); }
                        },
                        2 => {
                            if !n.content.eq("child_2") { panic!("Wrong child_2 content!"); }
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
                            if !n.content.eq("child_2_1") { panic!("Wrong child_2_1 content!"); }
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
                            if !n.content.eq("child_2_1_1") { panic!("Wrong child_2_1_1 content!"); }
                            if let None = n.parent_position { panic!("child_2_1_1 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 3 {
                                    panic!("child_2_1_1 has wrong parent!");
                                }
                            }
                            if n.children.len() != 0 { panic!("child_2_1_1 hasn't 0 children!"); }
                        },
                        5 => {
                            if !n.content.eq("child_2_2") { panic!("Wrong child_2_2 content!"); }
                            if let None = n.parent_position { panic!("child_2_2 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 2 {
                                    panic!("child_2_2 has wrong parent!");
                                }
                            }
                            if n.children.len() != 0 { panic!("child_2_2 hasn't 0 children!"); }
                        },
                        6 => {                          
                            if !n.content.eq("child_3") { panic!("Wrong child_3 content!"); }
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
    match crate::Forest::new(tref_sample()) {
        Ok(forest) => {
            if let Some(tree_model) = forest.tree(&String::from("test_tree")) {
                for (i,n) in tree_model.bfs_iter().enumerate() {
                    match i {
                        0 => {
                            if !n.content.eq("root_node") { panic!("Wrong {} node position in BFS!", n.content); }
                        },
                        1 => {
                            if !n.content.eq("child_1") { panic!("Wrong {} node position in BFS!", n.content); }
                        },
                        2 => {
                            if !n.content.eq("child_2") { panic!("Wrong {} node position in BFS!", n.content); }
                        },
                        3 => {
                            if !n.content.eq("child_3") { panic!("Wrong {} node position in BFS!", n.content); }
                        },
                        4 => {
                            if !n.content.eq("child_2_1") { panic!("Wrong {} node position in BFS!", n.content); }
                        },
                        5 => {
                            if !n.content.eq("child_2_2") { panic!("Wrong {} node position in BFS!", n.content); }
                        },
                        6 => {
                            if !n.content.eq("child_2_1_1") { panic!("Wrong {} node position in BFS!", n.content); }
                        }
                        _ => {}
                    }
                }
            }
        },
        Err(msg) => panic!("ERROR = {}", msg)
    }
}

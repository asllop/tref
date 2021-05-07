use std::io::BufReader;

#[test]
fn check_forest_generation() {
    let tref_sample =
    "[test_tree]\n\
    + root_node\n\
    + + child_1\n\
    + + child_2\n\
    + + + child_2_1\n\
    + + + + child_2_1_1\n";

    match crate::Forest::new(BufReader::new(tref_sample.as_bytes())) {
        Ok(forest) => {
            if let Some(tree_model) = forest.tree(&String::from("test_tree")) {
                for (i,n) in tree_model.bfs_iter().enumerate() {
                    match i {
                        0 => {
                            if !n.content.eq("root_node") { panic!("Wrong root node content!"); }
                            if let Some(_) = n.parent_position { panic!("Root node has a parent!"); }
                            if n.children.len() != 2 { panic!("Root node hasn't 3 children!"); }
                            if n.children[0] != 1 || n.children[1] != 2 { panic!("Root node children are incorrect!"); }
                        },
                        1 => {
                            if !n.content.eq("child_1") { panic!("Wrong node 1 content!"); }
                            if let None = n.parent_position { panic!("Node 1 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 0 {
                                    panic!("Node 1 has wrong parent!");
                                }
                            }
                            if n.children.len() != 0 { panic!("Node 1 hasn't 0 children!"); }
                        },
                        2 => {
                            if !n.content.eq("child_2") { panic!("Wrong node 2 content!"); }
                            if let None = n.parent_position { panic!("Node 2 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 0 {
                                    panic!("Node 2 has wrong parent!");
                                }
                            }
                            if n.children.len() != 1 { panic!("Node 2 hasn't 1 child!"); }
                            if n.children[0] != 3 { panic!("Node 2 children are incorrect!"); }
                        },
                        3 => {
                            if !n.content.eq("child_2_1") { panic!("Wrong node 3 content!"); }
                            if let None = n.parent_position { panic!("Node 3 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 2 {
                                    panic!("Node 3 has wrong parent!");
                                }
                            }
                            if n.children.len() != 1 { panic!("Node 3 hasn't 1 child!"); }
                            if n.children[0] != 4 { panic!("Node 3 children are incorrect!"); }
                        },
                        4 => {
                            if !n.content.eq("child_2_1_1") { panic!("Wrong node 4 content!"); }
                            if let None = n.parent_position { panic!("Node 4 has a no parent!"); }
                            if let Some(parent_n) = n.parent_position {
                                if parent_n != 3 {
                                    panic!("Node 4 has wrong parent!");
                                }
                            }
                            if n.children.len() != 0 { panic!("Node 4 hasn't 0 children!"); }
                        },
                        _ => {}
                    }
                }
            }
        },
        Err(msg) => panic!("ERROR = {}", msg)
    }
}

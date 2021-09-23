use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use socarel::{Forest, NodeContent};
use crate::Model;

//TODO:
// - check serielizing multiple invalid docs
// - check parsing a dialect

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
fn parse_valid_tref() {
    let model = <Model>::new();
    match model.parse(tref_sample()) {
        Ok(forest) => {
            if let Some(tree) = forest.get_tree("test_tree") {
                for (i, (n, _)) in tree.iterators().sequential().enumerate() {
                    match i {
                        0 => {
                            if !n.get_content_ref().get_val().eq("root_node") { panic!("Wrong root_node content") }
                            if let Some(_) = n.get_parent_position() { panic!("root_node has a parent") }
                            if n.get_num_chuildren() != 3 { panic!("root_node hasn't 3 children") }
                            if n.get_children_ref()[0] != 1 || n.get_children_ref()[1] != 2 || n.get_children_ref()[2] != 6 { panic!("root_node children are incorrect") }
                        },
                        1 => {
                            if !n.get_content_ref().get_val().eq("child_1") { panic!("Wrong child_1 content"); }
                            if let None = n.get_parent_position() { panic!("child_1 has a no parent"); }
                            if let Some(parent_n) = n.get_parent_position() {
                                if parent_n != 0 {
                                    panic!("child_1 has wrong parent");
                                }
                            }
                            if n.get_num_chuildren() != 0 { panic!("child_1 hasn't 0 children"); }
                        },
                        2 => {
                            if !n.get_content_ref().get_val().eq("child_2") { panic!("Wrong child_2 content"); }
                            if let None = n.get_parent_position() { panic!("child_2 has a no parent"); }
                            if let Some(parent_n) = n.get_parent_position() {
                                if parent_n != 0 {
                                    panic!("child_2 has wrong parent");
                                }
                            }
                            if n.get_num_chuildren() != 2 { panic!("child_2 hasn't 2 children"); }
                            if n.get_children_ref()[0] != 3 || n.get_children_ref()[1] != 5 { panic!("child_2 children are incorrect"); }
                        },
                        3 => {
                            if !n.get_content_ref().get_val().eq("child_2_1") { panic!("Wrong child_2_1 content"); }
                            if let None = n.get_parent_position() { panic!("child_2_1 has a no parent"); }
                            if let Some(parent_n) = n.get_parent_position() {
                                if parent_n != 2 {
                                    panic!("child_2_1 has wrong parent");
                                }
                            }
                            if n.get_num_chuildren() != 1 { panic!("child_2_1 hasn't 1 child"); }
                            if n.get_children_ref()[0] != 4 { panic!("child_2_1 children are incorrect"); }
                        },
                        4 => {
                            if !n.get_content_ref().get_val().eq("child_2_1_1") { panic!("Wrong child_2_1_1 content"); }
                            if let None = n.get_parent_position() { panic!("child_2_1_1 has a no parent"); }
                            if let Some(parent_n) = n.get_parent_position() {
                                if parent_n != 3 {
                                    panic!("child_2_1_1 has wrong parent");
                                }
                            }
                            if n.get_num_chuildren() != 0 { panic!("child_2_1_1 hasn't 0 children"); }
                        },
                        5 => {
                            if !n.get_content_ref().get_val().eq("child_2_2") { panic!("Wrong child_2_2 content"); }
                            if let None = n.get_parent_position() { panic!("child_2_2 has a no parent"); }
                            if let Some(parent_n) = n.get_parent_position() {
                                if parent_n != 2 {
                                    panic!("child_2_2 has wrong parent");
                                }
                            }
                            if n.get_num_chuildren() != 0 { panic!("child_2_2 hasn't 0 children"); }
                        },
                        6 => {                          
                            if !n.get_content_ref().get_val().eq("child_3") { panic!("Wrong child_3 content"); }
                            if let None = n.get_parent_position() { panic!("child_3 has a no parent"); }
                            if let Some(parent_n) = n.get_parent_position() {
                                if parent_n != 0 {
                                    panic!("child_3 has wrong parent");
                                }
                            }
                            if n.get_num_chuildren() != 0 { panic!("child_3 hasn't 0 children"); }
                        }
                        _ => {}
                    }
                }
            }
            else {
                panic!("Failed geting tree");
            }
        },
        Err(e) => {
            panic!("Failed parsing document: {}", e);
        }
    }
}

#[test]
fn parse_missing_tree_id() {
    let tref =
    "+ root_node\n\
    + + child_1\n\
    + + child_2\n";
    let tref_reader = BufReader::new(tref.as_bytes());

    let model = <Model>::new();
    if let Ok(_) = model.parse(tref_reader) {
        panic!("Parsed without tree id");
    }
}

#[test]
fn parse_invalid_statement() {
    let tref =
    "  [test_tree]\n\
    + root_node\n\
    + + child_1\n\
    + + child_2\n";
    let tref_reader = BufReader::new(tref.as_bytes());

    let model = <Model>::new();
    if let Ok(_) = model.parse(tref_reader) {
        panic!("Parsed an invalid statement");
    }
}

#[test]
fn parse_invalid_level() {
    let tref =
    "[test_tree]\n\
    + root_node\n\
    + + + child_1\n\
    + + child_2\n";
    let tref_reader = BufReader::new(tref.as_bytes());

    let model = <Model>::new();
    if let Ok(_) = model.parse(tref_reader) {
        panic!("Parsed an invalid level");
    }
}

#[test]
fn serialize_valid_tref() {
    let model = <Model>::new();
    match model.parse(tref_sample()) {
        Ok(mut forest) => {
            if let Some(tree) = forest.get_mut_tree("test_tree") {
                tree.link_node("new_node", 0).expect("Could not link new node to root");
                tree.unlink_node(3).expect("Could not unlink node 3");
                tree.unlink_node(4).expect("Could not unlink node 4");
                tree.unlink_node(5).expect("Could not unlink node 5");
                tree.unlink_node(6).expect("Could not unlink node 6");

                let mut buf_writer = BufWriter::new(Vec::new());
                match model.serialize(&forest, &mut buf_writer) {
                    Ok(_) => {
                        let bytes = buf_writer.into_inner().unwrap();
                        let buf_reader = BufReader::new(&bytes[..]);
                        match model.parse(buf_reader) {
                            Ok(forest_prima) => {
                                if let Some(tree_prima) = forest_prima.get_tree("test_tree") {
                                    for (i, (n, _)) in tree_prima.iterators().sequential().enumerate() {
                                        match i {
                                            0 => {
                                                if !n.get_content_ref().get_val().eq("root_node") { panic!("Wrong root_node content") }
                                            },
                                            1 => {
                                                if !n.get_content_ref().get_val().eq("child_1") { panic!("Wrong child_1 content") }
                                            },
                                            2 => {
                                                if !n.get_content_ref().get_val().eq("child_2") { panic!("Wrong child_2 content") }
                                            },
                                            3 => {
                                                if !n.get_content_ref().get_val().eq("new_node") { panic!("Wrong new_node content") }
                                            },
                                            _ => {
                                                panic!("Invalid number of nodes");
                                            }
                                        }
                                    }
                                }
                                else {
                                    panic!("Failed geting tree");
                                }
                            },
                            Err(e) => {
                                panic!("Failed parsing: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        panic!("Failed serializing: {}", e);
                    }
                }
            }
            else {
                panic!("Failed geting tree");
            }
        },
        Err(e) => {
            panic!("Failed parsing document: {}", e);
        }
    }
}
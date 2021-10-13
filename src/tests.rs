use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use crate::*;

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
    match <Model>::parse(tref_sample()) {
        Ok(forest) => {
            if let Some(tree) = forest.get_tree("test_tree") {
                for (i, (n, _)) in tree.iterators().sequential().enumerate() {
                    match i {
                        0 => {
                            if !n.get_content_ref().get_val().eq("root_node") { panic!("Wrong root_node content") }
                        },
                        1 => {
                            if !n.get_content_ref().get_val().eq("child_1") { panic!("Wrong child_1 content"); }
                        },
                        2 => {
                            if !n.get_content_ref().get_val().eq("child_2") { panic!("Wrong child_2 content"); }
                        },
                        3 => {
                            if !n.get_content_ref().get_val().eq("child_2_1") { panic!("Wrong child_2_1 content"); }
                        },
                        4 => {
                            if !n.get_content_ref().get_val().eq("child_2_1_1") { panic!("Wrong child_2_1_1 content"); }
                        },
                        5 => {
                            if !n.get_content_ref().get_val().eq("child_2_2") { panic!("Wrong child_2_2 content"); }
                        },
                        6 => {                          
                            if !n.get_content_ref().get_val().eq("child_3") { panic!("Wrong child_3 content"); }
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

    if let Ok(_) = <Model>::parse(tref_reader) {
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

    if let Ok(_) = <Model>::parse(tref_reader) {
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

    if let Ok(_) = <Model>::parse(tref_reader) {
        panic!("Parsed an invalid level");
    }
}

#[test]
fn serialize_valid_tref() {
    match <Model>::parse(tref_sample()) {
        Ok(mut forest) => {
            if let Some(tree) = forest.get_mut_tree("test_tree") {
                tree.link_node("new_node", 0).expect("Could not link new node to root");
                tree.unlink_node(3).expect("Could not unlink node 3");
                tree.unlink_node(4).expect("Could not unlink node 4");
                tree.unlink_node(5).expect("Could not unlink node 5");
                tree.unlink_node(6).expect("Could not unlink node 6");

                let mut buf_writer = BufWriter::new(Vec::new());
                match <Model>::serialize(&forest, &mut buf_writer) {
                    Ok(_) => {
                        let bytes = buf_writer.into_inner().unwrap();
                        let buf_reader = BufReader::new(&bytes[..]);
                        match <Model>::parse(buf_reader) {
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

#[test]
fn parse_dialect() {
    #[derive(Debug)]
    enum TypedNode {
        Text(String),
        Number(String, u32)
    }

    impl NodeContent for TypedNode {
        fn new(content: &str) -> Option<Self> {
            match content.trim().parse() {
                Ok(num) => Some(Self::Number(String::from(content), num)),
                Err(_) => Some(Self::Text(String::from(content)))
            }
        }

        fn get_val(&self) -> &str {
            match self {
                Self::Text(t) => t,
                Self::Number(t, _) => t
            }
        }

        fn gen_content(&self) -> String {
            String::from(self.get_val())
        }
    }

    let tref =
    "[test_tree]\n\
    + root\n\
    + + child\n\
    + + + 2500\n\
    + + + 130\n";

    match Model::<TypedNode>::parse(BufReader::new(tref.as_bytes())) {
        Ok(forest) => {
            if let Some(tree) = forest.get_tree("test_tree") {
                for (i, (n, _)) in tree.iterators().sequential().enumerate() {
                    match i {
                        0 => {
                            if !n.get_content_ref().get_val().eq("root") { panic!("Wrong root content") }
                            if let TypedNode::Number(_, _) = n.get_content_ref() {
                                panic!("Wrong root node type")
                            }
                        },
                        1 => {
                            if !n.get_content_ref().get_val().eq("child") { panic!("Wrong child content"); }
                            if let TypedNode::Number(_, _) = n.get_content_ref() {
                                panic!("Wrong child node type")
                            }
                        },
                        2 => {
                            if !n.get_content_ref().get_val().eq("2500") { panic!("Wrong 2500 content"); }
                            if let TypedNode::Text(_) = n.get_content_ref() {
                                panic!("Wrong 1500 node type")
                            }
                        },
                        3 => {
                            if !n.get_content_ref().get_val().eq("130") { panic!("Wrong 130 content"); }
                            if let TypedNode::Text(_) = n.get_content_ref() {
                                panic!("Wrong 130 node type")
                            }
                        },
                        _ => {
                            panic!("Invalid node index");
                        }
                    }
                }
            }
            else {
                panic!("Failed getting tree");
            }
        },
        Err(e) => {
            panic!("Failed parsing document: {}", e);
        }
    }
}
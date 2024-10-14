// src/node.rs

use std::cell::RefCell;
use std::rc::Rc;

pub type Tree = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left: Tree,
    pub right: Tree,
}

impl Node {
    pub fn new(value: i32) -> Tree {
        Some(Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        })))
    }
}
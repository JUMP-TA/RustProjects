// src/bst.rs

use crate::node::{Node, Tree};

pub struct BST {
    pub root: Tree,
}

impl BST {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        self.root = BST::insert_node(self.root.clone(), value);
    }

    fn insert_node(node: Tree, value: i32) -> Tree {
        match node {
            Some(current_node) => {
                if value < current_node.borrow().value {
                    let left_child = BST::insert_node(current_node.borrow().left.clone(), value);
                    current_node.borrow_mut().left = left_child;
                } else {
                    let right_child = BST::insert_node(current_node.borrow().right.clone(), value);
                    current_node.borrow_mut().right = right_child;
                }
                Some(current_node)
            }
            None => Node::new(value),
        }
    }

    pub fn in_order_traversal(&self, values: &mut Vec<i32>) {
        BST::in_order(&self.root, values);
    }

    fn in_order(node: &Tree, values: &mut Vec<i32>) {
        if let Some(ref current_node) = node {
            let node_borrowed = current_node.borrow();
            BST::in_order(&node_borrowed.left, values);
            values.push(node_borrowed.value);
            BST::in_order(&node_borrowed.right, values);
        }
    }

    pub fn search(&self, value: i32) -> bool {
        BST::search_node(&self.root, value)
    }

    fn search_node(node: &Tree, value: i32) -> bool {
        match node {
            Some(current_node) => {
                let node_borrowed = current_node.borrow();
                if node_borrowed.value == value {
                    true
                } else if value < node_borrowed.value {
                    BST::search_node(&node_borrowed.left, value)
                } else {
                    BST::search_node(&node_borrowed.right, value)
                }
            }
            None => false,
        }
    }
}

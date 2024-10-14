// src/main.rs

mod bst;
mod node;

use bst::BST;
use std::io;

fn main() {
    let mut bst = BST::new();

    println!("Enter numbers to insert into the BST (separated by spaces):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    for num in numbers {
        bst.insert(num);
    }

    let mut sorted_values = Vec::new();
    bst.in_order_traversal(&mut sorted_values);

    println!("Sorted values:");
    for value in &sorted_values {
        println!("{}", value);
    }

    // Search for a value
    println!("Enter a number to search in the BST:");
    let mut search_input = String::new();
    io::stdin()
        .read_line(&mut search_input)
        .expect("Failed to read line");

    if let Ok(search_value) = search_input.trim().parse::<i32>() {
        if bst.search(search_value) {
            println!("Value {} found in the BST.", search_value);
        } else {
            println!("Value {} not found in the BST.", search_value);
        }
    } else {
        println!("Invalid input.");
    }
}


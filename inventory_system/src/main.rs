// src/main.rs

mod models;

use models::{Category, Product};
use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome to the Inventory Management System!");

    let mut inventory: HashMap<u32, Product> = HashMap::new();

    loop {
        println!("\nPlease select an option:");
        println!("1) Add New Product");
        println!("2) Update Stock");
        println!("3) List All Products");
        println!("4) Exit");

        let choice = read_input();

        match choice.trim() {
            "1" => add_new_product(&mut inventory),
            "2" => update_stock(&mut inventory),
            "3" => list_products(&inventory),
            "4" => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

// Function to add a new product
fn add_new_product(inventory: &mut HashMap<u32, Product>) {
    println!("Enter product ID (number):");
    let id = loop {
        if let Ok(id) = read_input().trim().parse::<u32>() {
            if inventory.contains_key(&id) {
                println!("Product ID already exists. Please enter a unique ID.");
                continue;
            } else {
                break id;
            }
        } else {
            println!("Invalid ID. Please enter a number.");
        }
    };

    println!("Enter product name:");
    let name = read_input();

    println!("Select category:");
    println!("1) Electronics");
    println!("2) Groceries");
    println!("3) Clothing");

    let category = loop {
        match read_input().trim() {
            "1" => break Category::Electronics,
            "2" => break Category::Groceries,
            "3" => break Category::Clothing,
            _ => println!("Invalid choice. Please select 1, 2, or 3."),
        }
    };

    println!("Enter price:");
    let price = loop {
        if let Ok(price) = read_input().trim().parse::<f64>() {
            break price;
        } else {
            println!("Invalid price. Please enter a number.");
        }
    };

    println!("Enter stock quantity:");
    let stock = loop {
        if let Ok(stock) = read_input().trim().parse::<u32>() {
            break stock;
        } else {
            println!("Invalid stock quantity. Please enter a number.");
        }
    };

    let product = Product {
        id,
        name,
        category,
        price,
        stock,
    };

    inventory.insert(id, product.clone());
    println!("Product added:\n{}", product);
}

// Function to update stock
fn update_stock(inventory: &mut HashMap<u32, Product>) {
    println!("Enter product ID to update stock:");
    let id = match read_input().trim().parse::<u32>() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid ID.");
            return;
        }
    };

    if let Some(product) = inventory.get_mut(&id) {
        println!("Current stock for '{}': {}", product.name, product.stock);
        println!("Enter new stock quantity:");
        let new_stock = match read_input().trim().parse::<u32>() {
            Ok(stock) => stock,
            Err(_) => {
                println!("Invalid stock quantity.");
                return;
            }
        };
        product.stock = new_stock;
        println!("Stock updated for '{}'. New stock: {}", product.name, product.stock);
    } else {
        println!("Product ID not found.");
    }
}

// Function to list all products
fn list_products(inventory: &HashMap<u32, Product>) {
    if inventory.is_empty() {
        println!("No products in the inventory.");
        return;
    }

    println!("\nInventory List:");
    for product in inventory.values() {
        println!("{}", product);
    }
}

// Helper function to read input from the user
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}


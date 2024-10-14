// src/models.rs
use std::fmt;

// Define the Category enum
#[derive(Debug, Clone)]
pub enum Category {
    Electronics,
    Groceries,
    Clothing,
    // You can add more categories as needed
}

// Implement Display trait for Category
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Electronics => write!(f, "Electronics"),
            Category::Groceries => write!(f, "Groceries"),
            Category::Clothing => write!(f, "Clothing"),
        }
    }
}

// Define the Product struct
#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: Category,
    pub price: f64,
    pub stock: u32,
}

// Implement Display trait for Product
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}, Name: {}, Category: {}, Price: ${:.2}, Stock: {}",
            self.id, self.name, self.category, self.price, self.stock
        )
    }
}

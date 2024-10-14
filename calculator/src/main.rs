use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    // Get the first number from the user
    let num1 = loop {
        println!("Please enter the first number:");
        let input = read_input();
        match input.trim().parse::<f64>() {
            Ok(number) => break number,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        }
    };

    // Get the second number from the user
    let num2 = loop {
        println!("Please enter the second number:");
        let input = read_input();
        match input.trim().parse::<f64>() {
            Ok(number) => break number,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        }
    };

    // Ask the user to select an operation
    let operation = loop {
        println!("Select an operation:");
        println!("1) Addition (+)");
        println!("2) Subtraction (-)");
        println!("3) Multiplication (*)");
        println!("4) Division (/)");

        let input = read_input();
        match input.trim() {
            "1" | "+" => break '+',
            "2" | "-" => break '-',
            "3" | "*" => break '*',
            "4" | "/" => break '/',
            _ => {
                println!("Invalid operation. Please select 1, 2, 3, or 4.");
                continue;
            }
        }
    };

    // Perform the calculation
    let result = match operation {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            } else {
                num1 / num2
            }
        }
        _ => {
            println!("An unexpected error occurred.");
            return;
        }
    };

    // Display the result
    println!("Result: {} {} {} = {}", num1, operation, num2, result);
}

// Helper function to read input from the user
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}


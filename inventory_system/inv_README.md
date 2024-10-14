# **Inventory Management CLI Project**

## **Overview**

In this project, you will build a command-line inventory management application in Rust. The application will allow users to manage products in an inventory, including adding products, viewing the inventory, and updating product quantities. This project will help you practice working with data structures, command-line input parsing, and file I/O in Rust.

---

## **Project Objectives**

- **Implement Inventory Functions**: Create functions to add products, update product quantities, and view the inventory.
- **Command-Line Input**: Parse command-line input to handle product details and operations (e.g., add, update, view).
- **File Persistence**: Save inventory data to a file to ensure that data persists between runs of the application.
- **Error Handling**: Manage errors, such as invalid inputs or missing products.

---

## **Functional Requirements**

### **1. Inventory Operations**

The application must support the following operations:

- **Add Product**: Add a new product to the inventory with a name, price, and quantity.
  - Example: `inventory add "Laptop" 1200.00 5`
  
- **Update Product Quantity**: Update the quantity of an existing product in the inventory.
  - Example: `inventory update "Laptop" 10`
  
- **View Inventory**: View all products in the inventory, including their name, price, and quantity.
  - Example: `inventory view`

Each operation should be implemented as a separate function, and all data should be stored persistently.

---

### **2. Command-Line Input Parsing**

- The user will provide the following input:
  1. The operation (`add`, `update`, or `view`).
  2. For the `add` operation, a product name, price, and quantity.
  3. For the `update` operation, a product name and the new quantity.
  
- Use the `clap` crate to handle parsing of command-line arguments.

Example command to run the program:

```bash
cargo run -- add "Laptop" 1200.00 5
```

---

### **3. File Persistence**

- **Data Storage**: Store the inventory data in a JSON file (`inventory.json`) in the local directory. The inventory should persist between runs of the application.
  
- **Serialization**: Use the `serde` and `serde_json` crates to serialize the inventory into JSON format and save it to the file. Deserialize the JSON file back into a Rust data structure when loading the inventory.

---

### **4. Error Handling**

- **Invalid Input**: Ensure that the input is valid, such as verifying that prices are positive numbers and quantities are non-negative integers.
- **Missing Products**: Handle cases where the user tries to update or view a product that doesn’t exist in the inventory.

---

## **Project Structure**

You will break the project into the following key components:

- **CLI Argument Parsing**: Use the `clap` crate to handle user input from the command line.
- **Inventory Operations**: Implement functions to add products, update product quantities, and view the inventory.
- **File Persistence**: Ensure that inventory data is stored in a file and reloaded when the application starts.
- **Error Handling**: Handle invalid inputs and provide meaningful error messages.

---

## **Steps to Complete the Project**

1. **Set Up the Project**
   - Initialize a new Cargo project:
     ```bash
     cargo new inventory
     cd inventory
     ```

2. **Add Dependencies**
   - Add the `clap`, `serde`, and `serde_json` crates to `Cargo.toml` for parsing command-line arguments and handling JSON serialization:
     ```toml
     [dependencies]
     clap = { version = "4.1.8", features = ["derive"] }
     serde = { version = "1.0", features = ["derive"] }
     serde_json = "1.0"
     ```

3. **Parse Command-Line Input**
   - Set up basic CLI argument parsing using `clap`.

4. **Implement Inventory Functions**
   - Write functions to add products, update product quantities, and view the inventory.

5. **Add File Persistence**
   - Use `serde` and `serde_json` to serialize the inventory data into a file and reload it when the program starts.

6. **Add Error Handling**
   - Handle cases such as invalid input, missing products, or issues with reading/writing the file.

7. **Test Your Application**
   - Test the application by adding, updating, and viewing products in the inventory.

---

## **Evaluation Criteria**

Your project will be evaluated based on the following:

- **Correctness**: The application correctly performs inventory operations and handles errors.
- **Code Quality**: The code is well-structured, with clear functions for each operation and proper error handling.
- **CLI Interaction**: The command-line interface works as expected, parsing user input correctly.
- **File Persistence**: The inventory data persists between runs of the application.
- **Documentation**: The README provides clear instructions on how to run the application and includes examples.

---

## **Submission Instructions**

1. **Repository**: Upload your project to a GitHub repository or similar platform.
2. **README**: Ensure that your README file contains:
   - A brief description of the project.
   - Instructions on how to install and run the program.
   - Example usage of the inventory system.
3. **Code**: Include all source code files in your repository, ensuring that the project is well-organized and documented.

---

## **Getting Started**

To get started, follow the setup steps above to initialize the project and begin implementing the required functionality.

---

## **Conclusion**

This project will help you develop practical experience with Rust by building a functional inventory management system. You'll work with data structures, file I/O, error handling, and command-line parsing. Good luck, and enjoy building your inventory management tool!


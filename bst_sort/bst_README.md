# **Binary Search Tree CLI Project**

## **Overview**

In this project, you will build a binary search tree (BST) in Rust that allows users to perform basic operations such as inserting nodes, searching for values, and printing the tree. This project will help you understand how binary search trees work and how to implement recursive data structures in Rust.

---

## **Project Objectives**

- **Implement a Binary Search Tree**: Create a binary search tree with functionality to insert nodes, search for values, and display the tree.
- **Recursive Functions**: Use recursion to implement tree traversal and search operations.
- **Command-Line Input**: Parse command-line input to accept values to insert or search for in the tree.
- **Error Handling**: Manage errors, such as trying to search for a value in an empty tree.

---

## **Functional Requirements**

### **1. Binary Search Tree Operations**

The application must support the following operations:

- **Insert Values**: Insert a value into the BST, maintaining the correct order.
  - Example: `bst insert 15`
  
- **Search for Values**: Search for a value in the tree and return whether or not the value exists.
  - Example: `bst search 10`
  
- **Display the Tree**: Print the tree in-order (left subtree, root, right subtree) to show the current structure of the tree.
  - Example: `bst display`

Each operation should be implemented using recursive functions to traverse the tree.

---

### **2. Command-Line Input Parsing**

- The user will provide the following input:
  1. The operation (`insert`, `search`, or `display`).
  2. For the `insert` and `search` operations, a value to insert or search for.
  
- Use the `clap` crate to handle parsing of command-line arguments.

Example command to run the program:

```bash
cargo run -- insert 25
```

---

### **3. Recursive Functions**

- **Insert Values**: Write a recursive function to insert a value into the correct location in the tree.
  
- **Search for Values**: Write a recursive function to search for a value in the tree.
  
- **In-Order Traversal**: Write a recursive function to perform an in-order traversal of the tree and print the values.

---

### **4. Error Handling**

- **Invalid Input**: Ensure that the input is valid, such as verifying that the value to insert is a valid number.
- **Empty Tree**: Handle cases where the user tries to search or display an empty tree by returning an appropriate message like "The tree is empty."

---

## **Project Structure**

You will break the project into the following key components:

- **CLI Argument Parsing**: Use the `clap` crate to handle user input from the command line.
- **Binary Search Tree Implementation**: Implement functions for inserting nodes, searching for values, and printing the tree.
- **Recursive Functions**: Use recursion to traverse the tree for insertion, searching, and displaying.
- **Error Handling**: Handle invalid input and edge cases such as an empty tree.

---

## **Steps to Complete the Project**

1. **Set Up the Project**
   - Initialize a new Cargo project:
     ```bash
     cargo new bst
     cd bst
     ```

2. **Add Dependencies**
   - Add the `clap` crate to `Cargo.toml` for parsing command-line arguments:
     ```toml
     [dependencies]
     clap = { version = "4.1.8", features = ["derive"] }
     ```

3. **Parse Command-Line Input**
   - Set up basic CLI argument parsing using `clap`.

4. **Implement the Binary Search Tree**
   - Write a `BST` struct with methods for inserting values, searching for values, and performing an in-order traversal.

5. **Add Recursive Functions**
   - Implement the insertion, search, and traversal methods recursively.

6. **Add Error Handling**
   - Handle cases such as invalid input and attempting to operate on an empty tree.

7. **Test Your Application**
   - Test the application by inserting values, searching for them, and displaying the structure of the tree.

---

## **Evaluation Criteria**

Your project will be evaluated based on the following:

- **Correctness**: The application correctly implements BST operations and handles errors.
- **Code Quality**: The code is well-structured, with clear functions for each operation and proper error handling.
- **CLI Interaction**: The command-line interface works as expected, parsing user input correctly.
- **Tree Structure**: The tree maintains the correct order of nodes, and in-order traversal works as expected.
- **Documentation**: The README provides clear instructions on how to run the application and includes examples.

---

## **Submission Instructions**

1. **Repository**: Upload your project to a GitHub repository or similar platform.
2. **README**: Ensure that your README file contains:
   - A brief description of the project.
   - Instructions on how to install and run the program.
   - Example usage of the binary search tree operations.
3. **Code**: Include all source code files in your repository, ensuring that the project is well-organized and documented.

---

## **Getting Started**

To get started, follow the setup steps above to initialize the project and begin implementing the required functionality.

---

## **Conclusion**

This project will help you understand how binary search trees work and how to implement recursive algorithms in Rust. You’ll also practice parsing command-line input and managing data structures. Good luck, and enjoy building your binary search tree!
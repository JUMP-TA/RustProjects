# **Basic Calculator CLI Project**

## **Overview**

In this project, you will build a simple command-line calculator application in Rust. The calculator will support basic operations such as addition, subtraction, multiplication, and division. This project will help reinforce your understanding of Rust's basic syntax, arithmetic operations, command-line parsing, and error handling.

---

## **Project Objectives**

- **Implement Arithmetic Operations**: Create functions to perform basic arithmetic operations (addition, subtraction, multiplication, and division).
- **Command-Line Input**: Parse command-line input to accept two numbers and an operator.
- **Error Handling**: Handle invalid input and division by zero.
- **CLI Interaction**: Build a simple interface where users input commands and receive results.

---

## **Functional Requirements**

### **1. Arithmetic Operations**

The calculator must support the following operations:

- **Addition**: `+`
  - Example: `5 + 3 = 8`
- **Subtraction**: `-`
  - Example: `10 - 4 = 6`
- **Multiplication**: `*`
  - Example: `7 * 3 = 21`
- **Division**: `/`
  - Example: `8 / 2 = 4`

Each operation should be implemented as a separate function, and all functions should return a result.

---

### **2. Command-Line Input Parsing**

- The user will provide the following as input:
  1. The first number.
  2. An operator (`+`, `-`, `*`, or `/`).
  3. The second number.
  
- Use the `clap` crate to handle parsing of command-line arguments.

Example command to run the program:

```bash
cargo run -- 5 + 3
```

Expected output:

```
Result: 8
```

---

### **3. Error Handling**

- **Invalid Operator**: If the user provides an operator other than `+`, `-`, `*`, or `/`, display an error message such as "Invalid operator."
- **Division by Zero**: Handle the case where the user attempts to divide by zero, returning a message like "Error: Division by zero is not allowed."
- **Invalid Input**: Ensure the input is valid (e.g., both numbers should be numerical values).

---

## **Project Structure**

You will break the project into the following key components:

- **CLI Argument Parsing**: Use the `clap` crate to handle user input from the command line.
- **Arithmetic Operations**: Implement functions for each arithmetic operation.
- **Error Handling**: Check for invalid operators and ensure safe division operations.

---

## **Steps to Complete the Project**

1. **Set Up the Project**
   - Initialize a new Cargo project:
     ```bash
     cargo new calculator
     cd calculator
     ```

2. **Add Dependencies**
   - Add the `clap` crate to `Cargo.toml` for parsing command-line arguments:
     ```toml
     [dependencies]
     clap = { version = "4.1.8", features = ["derive"] }
     ```

3. **Parse Command-Line Input**
   - Set up basic CLI argument parsing using `clap`.

4. **Implement Arithmetic Functions**
   - Write functions for addition, subtraction, multiplication, and division.
   - Each function should take two numbers as input and return the result.

5. **Add Error Handling**
   - Handle division by zero, invalid operators, and incorrect input formats.

6. **Test Your Application**
   - Test the application by running different commands and ensuring correct outputs and error handling.

---

## **Evaluation Criteria**

Your project will be evaluated based on the following:

- **Correctness**: The application correctly performs arithmetic operations and handles errors.
- **Code Quality**: The code is well-structured, with clear functions for each operation and proper error handling.
- **CLI Interaction**: The command-line interface works as expected, parsing user input correctly.
- **Documentation**: The README provides clear instructions on how to run the application and includes examples.

---

## **Submission Instructions**

1. **Repository**: Upload your project to a GitHub repository or similar platform.
2. **README**: Ensure that your README file contains:
   - A brief description of the project.
   - Instructions on how to install and run the program.
   - Example usage of the calculator.
3. **Code**: Include all source code files in your repository, ensuring that the project is well-organized and documented.

---

## **Getting Started**

To get started, follow the setup steps above to initialize the project and begin implementing the required functionality.

---

## **Conclusion**

This project will help you develop a strong understanding of basic Rust syntax, error handling, and command-line application development. Once completed, you’ll have a working calculator that can handle various operations with robust error checking. Good luck!
# **Word Count Tool CLI Project**

## **Overview**

In this project, you will build a command-line word count tool in Rust. The tool will allow users to count the number of words, lines, and characters in a given text file. This project will help you practice file handling, text processing, and command-line argument parsing in Rust.

---

## **Project Objectives**

- **File Reading**: Implement functionality to read a text file provided by the user.
- **Word, Line, and Character Counting**: Count the number of words, lines, and characters in the file.
- **Command-Line Input**: Parse command-line input to accept the file path and display the results.
- **Error Handling**: Manage errors, such as invalid file paths or empty files.

---

## **Functional Requirements**

### **1. Word, Line, and Character Counting**

The tool must support the following operations:

- **Count Words**: Count and display the number of words in the file.
  - Example: `wordcount file.txt` should output the word count.
  
- **Count Lines**: Count and display the number of lines in the file.
  - Example: `wordcount --lines file.txt` should output the line count.
  
- **Count Characters**: Count and display the number of characters in the file.
  - Example: `wordcount --chars file.txt` should output the character count.

Each operation should be implemented as a separate function.

---

### **2. Command-Line Input Parsing**

- The user will provide the file path and optional flags (`--lines`, `--chars`) to specify what to count.
- Use the `clap` crate to handle parsing of command-line arguments.

Example command to run the program:

```bash
cargo run -- file.txt
```

Expected output:

```
Words: 234
Lines: 15
Characters: 1456
```

---

### **3. File Reading**

- **File Handling**: The program should read a text file specified by the user from the command line and process its contents.
  
- **File Validation**: Ensure that the file exists and is readable. If the file is not found, display an error message like "File not found."

---

### **4. Error Handling**

- **Invalid Input**: Ensure the user provides a valid file path. Handle errors related to missing or unreadable files.
- **Empty Files**: If the file is empty, display appropriate feedback such as "File is empty."

---

## **Project Structure**

You will break the project into the following key components:

- **CLI Argument Parsing**: Use the `clap` crate to handle user input from the command line.
- **File Reading**: Implement functionality to read the contents of a text file.
- **Word, Line, and Character Counting**: Write functions to count words, lines, and characters.
- **Error Handling**: Handle invalid input, missing files, or other errors.

---

## **Steps to Complete the Project**

1. **Set Up the Project**
   - Initialize a new Cargo project:
     ```bash
     cargo new wordcount
     cd wordcount
     ```

2. **Add Dependencies**
   - Add the `clap` crate to `Cargo.toml` for parsing command-line arguments:
     ```toml
     [dependencies]
     clap = { version = "4.1.8", features = ["derive"] }
     ```

3. **Parse Command-Line Input**
   - Set up basic CLI argument parsing using `clap` to accept file paths and flags.

4. **Implement File Reading**
   - Write functionality to read the contents of the file specified by the user.

5. **Count Words, Lines, and Characters**
   - Write functions to count the number of words, lines, and characters in the file.

6. **Add Error Handling**
   - Handle invalid file paths, unreadable files, or empty files.

7. **Test Your Application**
   - Test the application by running different commands to count words, lines, and characters in various files.

---

## **Evaluation Criteria**

Your project will be evaluated based on the following:

- **Correctness**: The application correctly counts words, lines, and characters, and handles errors.
- **Code Quality**: The code is well-structured, with clear functions for each operation and proper error handling.
- **CLI Interaction**: The command-line interface works as expected, parsing user input correctly.
- **File Handling**: The program can read files and process their contents effectively.
- **Documentation**: The README provides clear instructions on how to run the application and includes examples.

---

## **Submission Instructions**

1. **Repository**: Upload your project to a GitHub repository or similar platform.
2. **README**: Ensure that your README file contains:
   - A brief description of the project.
   - Instructions on how to install and run the program.
   - Example usage of the word count tool.
3. **Code**: Include all source code files in your repository, ensuring that the project is well-organized and documented.

---

## **Getting Started**

To get started, follow the setup steps above to initialize the project and begin implementing the required functionality.

---

## **Conclusion**

This project will help you develop practical experience with file handling, text processing, and command-line interfaces in Rust. You'll be able to create a useful tool for counting words, lines, and characters in text files. Good luck, and enjoy building your word count tool!


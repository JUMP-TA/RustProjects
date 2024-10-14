use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

/// Simple program to count words in multiple files concurrently
#[derive(Parser)]
#[command(name = "Word Count Tool")]
#[command(version = "1.0")]
#[command(about = "Counts words in files using multithreading", long_about = None)]
struct Args {
    /// List of files to process
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    if args.files.is_empty() {
        eprintln!("Please provide at least one file path.");
        return;
    }

    // Shared counter for total word count
    let total_word_count = Arc::new(Mutex::new(0));

    // Vector to hold thread handles
    let mut handles = vec![];

    for file_path in args.files {
        let total_word_count = Arc::clone(&total_word_count);

        // Spawn a new thread for each file
        let handle = thread::spawn(move || {
            match count_words_in_file(&file_path) {
                Ok(count) => {
                    println!(
                        "File: {:?} - Word Count: {}",
                        file_path.file_name().unwrap(),
                        count
                    );
                    let mut total = total_word_count.lock().unwrap();
                    *total += count;
                }
                Err(e) => {
                    eprintln!("Failed to process {:?}: {}", file_path, e);
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Get the total word count
    let total = *total_word_count.lock().unwrap();
    println!("Total Word Count: {}", total);
}

// Function to count words in a file
fn count_words_in_file(file_path: &PathBuf) -> io::Result<usize> {
    let contents = fs::read_to_string(file_path)?;
    let word_count = contents.split_whitespace().count();
    Ok(word_count)
}


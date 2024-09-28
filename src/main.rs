use std::{thread::sleep, time::Duration};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use rusty_tools::progress::track_progress;
use rusty_tools::colors::{text_colored, print_colored, Color};
use rusty_tools::diff::{DiffOp, diff_words, diff_lines, diff_files};

pub fn test_progress() {
    print!("\nTesting progress bar:\n");
    // a function to test the ProgressBar
    let vec: Vec<i32> = (0..20).collect();

    for _ in track_progress(vec.clone()) {
        sleep(Duration::from_millis(100));
    }
}

pub fn test_color() {
    print!("\nTesting colors:\n");
    print!("{}\n", text_colored("This is a green text", "green"));
    print_colored("This is a red text", "red");
    print_colored("This is a yellow text", "yellow");
    print_colored("This is a cyan text", "cyan");
    print_colored("This is the default text", Color::default());   
}

pub fn test_diff() {
    print!("\nTesting diff with words:\n");
    let _word_diff_results: Vec<DiffOp> = diff_words("word1 word2", "word1 word3");
    for item in _word_diff_results {
        print!("{}\n", item);
    }

    print!("\nTesting diff with lines\n");
    let _line_diff_results: Vec<DiffOp> = diff_lines(
        "This line is the original one\nAdded nothing to this line\nRemoving this", 
        "This line is the changed one\nAdded nothing to this line\n\nHmm"
    );
    for item in _line_diff_results {
        print!("{}\n", item);
    }
}

pub fn test_diff_files() -> io::Result<()> {
    print!("\nTesting diff between two files:\n");
    // Create two temporary test files
    let file_a_path = "test_file_a.txt";
    let file_b_path = "test_file_b.txt";

    // Write content to the first file
    let mut file_a = File::create(file_a_path)?;
    writeln!(file_a, "Hello\nWorld\nRust programming")?;

    // Write content to the second file
    let mut file_b = File::create(file_b_path)?;
    writeln!(file_b, "Hello\nUniverse\nRust programming\nBye!")?;

    // Run the diff_files function to compare the two files
    diff_files(file_a_path, file_b_path)?;

    // Cleanup: remove the temporary files after the test
    if Path::new(file_a_path).exists() {
        fs::remove_file(file_a_path)?;
    }
    if Path::new(file_b_path).exists() {
        fs::remove_file(file_b_path)?;
    }

    Ok(())
}

fn main() {
    test_progress();
    print!("\n");
    test_color();
    print!("\n");
    test_diff();
    print!("\n");
    _ = test_diff_files();
}
# rusty_tools
A crate with simple yet useful tools

The crate contains two modules - `progress`, `colors` and `diff`

## Progress
The functionality `progress` is used to track progress of an iterative process using a progress bar. It can be implemented in two ways

### 1. Example of using the `ProgressBar` struct to track progress - update progress manually
```
use std::{thread::sleep, time::Duration};
use rusty_tools::progress::ProgressBar;

fn main() {
    // define a dummy vector with elements to iterate over
    let total_len: i32 = 20;
    let dummy_vec: Vec<i32> = (0..20).collect();

    // define the progress bar object
    let mut progress = ProgressBar { total: total_len as usize, ..Default::default() };

    // iterate and update the progress
    for item in dummy_vec.iter_mut() {
        // do something with item
        // sleep for 0.5 seconds to visibly see the progress
        sleep(Duration::from_millis(500));
        // update a step of the progress
        progress.update();
    }
}
```

### 2. Example of using the `track_progress` method - without the need for updating progress manually
```
use std::{thread::sleep, time::Duration};
use rusty_tools::progress::track_progress;

fn main() {
    // define a dummy vector with elements to iterate over
    let dummy_vec: Vec<i32> = (0..20).collect();

    for item in track_progress(dummy_vec.clone()) {
        // do something with item
        // sleep for 0.5 seconds to visibly see the progress
        sleep(Duration::from_millis(100));
        // no need to manually update the progress
    }
}
```

## Colors
The functionality `colors` lets a user display colored statements on the standard output/ terminal screen

### Example of using colors
```
use std::{thread::sleep, time::Duration};
use rusty_tools::colors::{text_colored, print_colored, Color};

fn main() {
    // print normally
    print!("\nTesting colors:\n");
    // transform a string to color using `text_colored`
    print!("{}\n", text_colored("This is a green text", "green"));
    // print text in a specific color with `print_colored`
    print_colored("This is a red text", "red");
    print_colored("This is a yellow text", "yellow");
    print_colored("This is a cyan text", "cyan");
    print_colored("This is the default text", Color::default());   
}
```

## Diff
The tool `diff` helps to find difference between words, strings (lines) and files

### Example of using diff
```
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use rusty_tools::diff::{DiffOp, diff_words, diff_lines, diff_files};

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
    test_diff();
    print!("\n");
    _ = test_diff_files();
}

```

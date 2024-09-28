use std::{fs, fmt};
use super::colors::text_colored;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
pub enum DiffOp {
    Add(String),
    Remove(String),
    Change(String, String),
    Unchanged(String),
}

impl fmt::Display for DiffOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffOp::Add(line) => write!(f, "+ {}", text_colored(line, "green")),
            DiffOp::Remove(line) => write!(f, "- {}", text_colored(line, "red")),
            DiffOp::Change(old, new) => write!(f, "~ {} -> {}", text_colored(old, "red"), text_colored(new, "green")),
            DiffOp::Unchanged(line) => write!(f, "  {}", line),
        }
    }
}

// Compare two strings line-by-line and return a vector of diff operations
pub fn diff_lines(a: &str, b: &str) -> Vec<DiffOp> {
    let a_lines: Vec<&str> = a.lines().collect();
    let b_lines: Vec<&str> = b.lines().collect();

    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    // Iterate through both lists of lines
    while i < a_lines.len() || j < b_lines.len() {
        match (a_lines.get(i), b_lines.get(j)) {
            (Some(a_line), Some(b_line)) => {
                if a_line == b_line {
                    result.push(DiffOp::Unchanged(a_line.to_string()));
                    i += 1;
                    j += 1;
                } else {
                    result.push(DiffOp::Change(a_line.to_string(), b_line.to_string()));
                    i += 1;
                    j += 1;
                }
            }
            (Some(a_line), None) => {
                result.push(DiffOp::Remove(a_line.to_string()));
                i += 1;
            }
            (None, Some(b_line)) => {
                result.push(DiffOp::Add(b_line.to_string()));
                j += 1;
            }
            (None, None) => break,
        }
    }

    result
}

// Compare two strings word-by-word and return a vector of diff operations
pub fn diff_words(a: &str, b: &str) -> Vec<DiffOp> {
    let a_words: Vec<&str> = a.split_whitespace().collect();
    let b_words: Vec<&str> = b.split_whitespace().collect();

    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    // Iterate through both lists of words
    while i < a_words.len() || j < b_words.len() {
        match (a_words.get(i), b_words.get(j)) {
            (Some(a_word), Some(b_word)) => {
                if a_word == b_word {
                    result.push(DiffOp::Unchanged(a_word.to_string()));
                    i += 1;
                    j += 1;
                } else {
                    result.push(DiffOp::Change(a_word.to_string(), b_word.to_string()));
                    i += 1;
                    j += 1;
                }
            }
            (Some(a_word), None) => {
                result.push(DiffOp::Remove(a_word.to_string()));
                i += 1;
            }
            (None, Some(b_word)) => {
                result.push(DiffOp::Add(b_word.to_string()));
                j += 1;
            }
            (None, None) => break,
        }
    }

    result
}

pub fn read_file_contents(file_path: &str) -> io::Result<String> {
    // Read a file and return its content as String
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Compare the contents of two files and print the differences
pub fn diff_files(file_a: &str, file_b: &str) -> io::Result<()> {
    // Read the files into strings
    let content_a = read_file_contents(file_a)?;
    let content_b = read_file_contents(file_b)?;

    // Find the differences between the files
    let diffs = diff_lines(&content_a, &content_b);

    let to_print = format!("Diff between `{}` and `{}`\n", file_a, file_b);
    let repeat_chars = "-".repeat(to_print.len());
    // Print the differences
    print!("{}\n", repeat_chars);
    print!("{}", to_print);
    print!("{}\n", repeat_chars);
    for diff in diffs {
        print!("{}\n", diff);
    }
    print!("{}\n", repeat_chars);

    Ok(())
}
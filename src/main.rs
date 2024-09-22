use std::{thread::sleep, time::Duration};
use rusty_tools::progress::track_progress;
use rusty_tools::colors::{text_colored, print_colored, Color};

pub fn test_progress() {
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

fn main() {
    test_progress();
    print!("\n");
    test_color();
}
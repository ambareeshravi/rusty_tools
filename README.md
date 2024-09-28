# rusty_tools
A crate with simple yet useful tools

The crate contains two modules - `progress` and `colors`

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

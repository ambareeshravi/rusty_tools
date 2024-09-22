use std::io::Write;
use std::time::Instant;

pub struct ProgressBar {
    // A struct that defines a progress bar
    _time_array: Vec::<f64>,
    bar_width: usize,
    _current: usize,
    total: usize,
    prog_char: String,
    empty_char: String,
    unit: String,
    _last_time_stamp: Instant
}

impl Default for ProgressBar {
    fn default() -> ProgressBar {
        // A default initializer for the ProgressBar
        ProgressBar {
            _time_array: vec![0.0],
            bar_width: 50,
            _current: 0,
            total: 100,
            prog_char: "\u{2588}".to_string(),
            empty_char: "\u{2591}".to_string(),
            unit: "iter".to_string(),
            _last_time_stamp: Instant::now()
        }
    }
}

impl ProgressBar {
    pub fn get_avg_time_per_step(&self) -> f64 {
        // Returns the average time taken per step in seconds
        let mut sum: f64 = 0.0;
        for x in &self._time_array[1..] {
            sum = sum + x;
        }
        sum as f64 / self._time_array.len() as f64
    }

    pub fn get_current_prog(&self) -> usize {
        // Returns the current progress as a fraction of total in accordance the the bar width
        (self.bar_width as f64 * (self._current as f64 / self.total as f64)) as usize
    }

    pub fn get_current_prog_as_string(&self) -> String {
        // Returns the current progress value as a string to be displayed
        // Adds leading spaces as required
        let num_leading_zeros = self.total.to_string().len();
        let num_current_digits: usize = self._current.to_string().len();
        let mut with_leading_spaces = " ".repeat(num_leading_zeros - num_current_digits).to_owned();
        let curr: &str = &self._current.to_string();
        with_leading_spaces.push_str(&curr);
        with_leading_spaces
    }

    pub fn print_prog(&mut self) {
        // prints the current progress to the stdout in place
        let filled_part = self.prog_char.repeat(self.get_current_prog());
        let empty_part = self.empty_char.repeat(self.bar_width - self.get_current_prog());
        let num_ops_per_sec = 1.0 / self.get_avg_time_per_step();

        print!("\r{}{} [{}/{}] {:.2} {}(s)/s", filled_part, empty_part, self.get_current_prog_as_string(), self.total, num_ops_per_sec, self.unit);
        std::io::stdout().flush().unwrap();
    }

    pub fn update(&mut self) {
        // updates a step
        let mut duration_in_sec = 0.00;
        if self._current == 0 {
            self._last_time_stamp = Instant::now();
            print!("\n");
        }
        else {
            duration_in_sec = self._last_time_stamp.elapsed().as_secs_f64();    
        }
        
        self._current += 1;
        
        self._time_array.push(duration_in_sec);
        self.print_prog();
        self._last_time_stamp = Instant::now();
    }
}


pub fn track_progress<T: IntoIterator>(iterable: T) -> impl Iterator<Item = T::Item>
where
    T: IntoIterator,
    T::Item: Clone,
{
    // A wrapper function that takes in an iterable, add progress and yields its items as a generator
    let iter = iterable.into_iter();
    let total_len = iter.size_hint().0; // Get the lower bound length
    
    let mut progress = ProgressBar { total: total_len as usize, ..Default::default() };
    
    let updated_iter = iter.map(move |item| {
        progress.update();
        item // Yield the updated item
    });
    
    updated_iter
}
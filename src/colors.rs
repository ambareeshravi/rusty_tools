use std::str::FromStr;

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}

impl Color {
    fn to_ansi_code(&self) -> &str {
        match *self {
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            Color::Default => "39",  // Default terminal color
        }
    }
}

// Implement FromStr trait to convert strings to Color enum
impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Color, ()> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            _ => Ok(Color::Default), // Return Default if the input doesn't match any color
        }
    }
}

pub fn text_colored<T: Into<Color>>(text: &str, color: T) -> String {
    let color: Color = color.into();
    let color_code = color.to_ansi_code();
    format!("\x1b[{}m{}\x1b[0m", color_code, text)
}

// Generic function that accepts both Color and &str with default color Green
pub fn print_colored<T: Into<Color>>(text: &str, color: T) {
    println!("{}", text_colored(text, color));
}

// Provide an implementation for converting &str to Color
impl From<&str> for Color {
    fn from(s: &str) -> Color {
        Color::from_str(s).unwrap_or(Color::Green)  // Default to Green on invalid string
    }
}

// Optional: Provide an implementation to handle default values more easily
impl Default for Color {
    fn default() -> Self {
        Color::Default  // Default color is Green
    }
}
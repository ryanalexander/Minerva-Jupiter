//! Primary logger

use colored::Colorize;

pub fn log_info(message: &str) {
    println!("[{}] {}", "i".to_string().blue(), message);
}

pub fn log_warn(message: &str) {
    println!("[{}] {}", "!".to_string().yellow(), message);
}

pub fn log_error(message: &str) {
    println!("[{}] {}", "!".to_string().red(), message);
}

pub fn log_success(message: &str) {
    println!("[{}] {}", "✓".to_string().green(), message);
}

pub fn log_debug(message: &str) {
    println!("[{}] {}", "?".to_string().cyan(), message);
}

pub fn log_trace(message: &str) {
    println!("[{}] {}", "*".to_string().magenta(), message);
}

pub fn log_fatal(message: &str) {
    println!("[{}] {}", "x".to_string().red(), message);
}

//! Primary logger

use colored::Colorize;

#[allow(dead_code)]
pub fn log_info(message: &str) {
    println!("[{}] {}", "i".to_string().blue(), message);
}

#[allow(dead_code)]
pub fn log_warn(message: &str) {
    println!("[{}] {}", "!".to_string().yellow(), message);
}

#[allow(dead_code)]
pub fn log_error(message: &str) {
    println!("[{}] {}", "!".to_string().red(), message);
}

#[allow(dead_code)]
pub fn log_success(message: &str) {
    println!("[{}] {}", "✓".to_string().green(), message);
}

#[allow(dead_code)]
pub fn log_debug(message: &str) {
    println!("[{}] {}", "?".to_string().cyan(), message);
}

#[allow(dead_code)]
pub fn log_trace(message: &str) {
    println!("[{}] {}", "*".to_string().magenta(), message);
}

#[allow(dead_code)]
pub fn log_fatal(message: &str) {
    println!("[{}] {}", "x".to_string().red(), message);
}
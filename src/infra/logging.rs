//! Primary logger

use colored::Colorize;

pub fn log_info(message: &str) -> () {
    println!("[{}] {}", format!("i").blue(), message);
}

pub fn log_warn(message: &str) -> () {
    println!("[{}] {}", format!("!").yellow(), message);
}

pub fn log_error(message: &str) -> () {
    println!("[{}] {}", format!("!").red(), message);
}

pub fn log_success(message: &str) -> () {
    println!("[{}] {}", format!("✓").green(), message);
}

pub fn log_debug(message: &str) -> () {
    println!("[{}] {}", format!("?").cyan(), message);
}

pub fn log_trace(message: &str) -> () {
    println!("[{}] {}", format!("*").magenta(), message);
}

pub fn log_fatal(message: &str) -> () {
    println!("[{}] {}", format!("x").red(), message);
}
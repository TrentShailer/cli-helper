use core::fmt::Display;

use ts_rust_helper::style::*;

/// Print an error message with the format:
///
/// `error: {message}
pub fn print_error<S: Display>(message: S) {
    println!("{BOLD}{RED}error{RESET}{BOLD}:{RESET} {message}");
}

/// Print a warning message with the format:
///
/// `warning: {message}`
pub fn print_warning<S: Display>(message: S) {
    println!("{BOLD}{YELLOW}warning{RESET}{BOLD}:{RESET} {message}");
}

/// Print a success message with the format:
///
/// `Success: {message}`
pub fn print_success<S: Display>(message: S) {
    println!("{BOLD}{GREEN}Success{RESET}{BOLD}:{RESET} {message}");
}

/// Print a failure message with the format:
///
/// `Fail: {message}`
pub fn print_fail<S: Display>(message: S) {
    println!("{BOLD}{RED}Fail{RESET}{BOLD}:{RESET} {message}");
}

use simply_colored::{BOLD, GREEN, RED, RESET, YELLOW};

/// Print an error message.
pub fn print_error(message: &str, indent: usize) {
    let indent = " ".repeat(indent);
    println!("{indent}{BOLD}{RED}error{RESET}{BOLD}:{RESET} {message}");
}

/// Print a warning message.
pub fn print_warning(message: &str, indent: usize) {
    let indent = " ".repeat(indent);
    println!("{indent}{BOLD}{YELLOW}warning{RESET}{BOLD}:{RESET} {message}");
}

/// Print a success message.
pub fn print_success(message: &str, indent: usize) {
    let indent = " ".repeat(indent);
    println!("{indent}{BOLD}{GREEN}Success{RESET}{BOLD}:{RESET} {message}");
}

/// Print a failure message.
pub fn print_fail(message: &str, indent: usize) {
    let indent = " ".repeat(indent);
    println!("{indent}{BOLD}{RED}Fail{RESET}{BOLD}:{RESET} {message}");
}

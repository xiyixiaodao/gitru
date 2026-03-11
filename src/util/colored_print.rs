use supports_color::Stream;

pub fn print_error(msg: &str) {
    if let Some(support) = supports_color::on(Stream::Stderr) {
        if support.has_16m {
            // True color (RGB)
            eprintln!("\x1b[38;2;255;0;0m{}\x1b[0m", msg);
        } else if support.has_256 {
            // 256-color red
            eprintln!("\x1b[38;5;196m{}\x1b[0m", msg);
        } else {
            // Basic ANSI red
            eprintln!("\x1b[31m{}\x1b[0m", msg);
        }
    } else {
        eprintln!("ERROR: {}", msg);
    }
}

pub fn print_success(msg: &str) {
    if let Some(support) = supports_color::on(Stream::Stdout) {
        if support.has_16m {
            println!("\x1b[38;2;0;255;0m{}\x1b[0m", msg);
        } else if support.has_256 {
            println!("\x1b[38;5;46m{}\x1b[0m", msg);
        } else {
            println!("\x1b[32m{}\x1b[0m", msg);
        }
    } else {
        println!("SUCCESS: {}", msg);
    }
}

pub fn print_warning(msg: &str) {
    if let Some(support) = supports_color::on(Stream::Stdout) {
        if support.has_16m {
            // True color (RGB) yellow
            println!("\x1b[38;2;255;255;0m{}\x1b[0m", msg);
        } else if support.has_256 {
            // 256-color bright yellow
            println!("\x1b[38;5;226m{}\x1b[0m", msg);
        } else {
            // Basic ANSI yellow
            println!("\x1b[33m{}\x1b[0m", msg);
        }
    } else {
        println!("WARNING: {}", msg);
    }
}

pub fn print_info(msg: &str) {
    if let Some(support) = supports_color::on(Stream::Stdout) {
        if support.has_16m {
            // True color (RGB) blue
            println!("\x1b[38;2;0;0;255m{}\x1b[0m", msg);
        } else if support.has_256 {
            // 256-color bright blue
            println!("\x1b[38;5;27m{}\x1b[0m", msg);
        } else {
            // Basic ANSI blue
            println!("\x1b[34m{}\x1b[0m", msg);
        }
    } else {
        println!("INFO: {}", msg);
    }
}

#[cfg(test)]
mod tests {
    use crate::util::colored_print::{print_error, print_success};

    // In Rust's unit test environment, `cargo test` does not print output directly to the terminal by default.
    // Instead, it captures the output and only displays it when a test fails.
    // This means that even if ANSI color sequences are used, colored output will not be visible in tests.
    #[test]
    fn test_print() {
        print_error("test error");
    }

    #[test]
    fn test_print_success() {
        print_success("test success");
    }
}

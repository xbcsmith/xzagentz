//! Output formatting utilities for CLI commands
//!
//! This module provides helpers for formatting CLI output in both
//! human-readable and JSON formats.

use serde::Serialize;
use std::io;

use super::OutputFormat;

/// Format and print output based on the specified format
///
/// # Arguments
///
/// * `format` - The output format (Human or JSON)
/// * `data` - The data to output (must implement Serialize and Display)
///
/// # Errors
///
/// Returns an error if writing to stdout fails
pub fn print_output<T>(format: OutputFormat, data: &T) -> io::Result<()>
where
    T: Serialize + std::fmt::Display,
{
    match format {
        OutputFormat::Human => {
            println!("{}", data);
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(data)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            println!("{}", json);
        }
    }
    Ok(())
}

/// Print a message to stderr (for errors and warnings)
///
/// # Arguments
///
/// * `message` - The message to print
pub fn print_error(message: &str) {
    eprintln!("Error: {}", message);
}

/// Print a warning message to stderr
///
/// # Arguments
///
/// * `message` - The warning message to print
pub fn print_warning(message: &str) {
    eprintln!("Warning: {}", message);
}

/// Print a success message to stdout
///
/// # Arguments
///
/// * `message` - The success message to print
pub fn print_success(message: &str) {
    println!("Success: {}", message);
}

/// Print verbose output if verbose mode is enabled
///
/// # Arguments
///
/// * `verbose` - Whether verbose mode is enabled
/// * `message` - The message to print
pub fn print_verbose(verbose: bool, message: &str) {
    if verbose {
        eprintln!("DEBUG: {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::fmt;

    #[derive(Serialize)]
    struct TestData {
        name: String,
        value: i32,
    }

    impl fmt::Display for TestData {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}: {}", self.name, self.value)
        }
    }

    #[test]
    fn test_output_format_is_json() {
        assert!(OutputFormat::Json.is_json());
        assert!(!OutputFormat::Human.is_json());
    }

    #[test]
    fn test_output_format_is_human() {
        assert!(OutputFormat::Human.is_human());
        assert!(!OutputFormat::Json.is_human());
    }

    #[test]
    fn test_print_output_human() {
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        let result = print_output(OutputFormat::Human, &data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_output_json() {
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        let result = print_output(OutputFormat::Json, &data);
        assert!(result.is_ok());
    }
}

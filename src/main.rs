//! xzagentz - AI Agent Development Guidelines and Template Manager
//!
//! A CLI tool for managing AI agent development guidelines, project templates,
//! and implementation plans.

use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Main application entry point
///
/// # Errors
///
/// Returns an error if the application fails to initialize or execute
fn run() -> anyhow::Result<()> {
    println!("xzagentz v{}", env!("CARGO_PKG_VERSION"));
    println!("AI Agent Development Guidelines and Template Manager");

    // TODO: Initialize CLI parser and dispatch commands
    // This will be implemented in Phase 4

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_succeeds() {
        let result = run();
        assert!(result.is_ok());
    }
}

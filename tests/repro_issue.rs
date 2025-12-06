#[cfg(test)]
mod tests {
    use xzagentz::markdown::heading::normalize_headings;

    #[test]
    fn test_repro_heading_issue() {
        let content = "# Root Two\n\nMore content\n## Subsection";
        let normalized = normalize_headings(content, 2);
        println!("Content: {:?}", content);
        println!("Normalized: {:?}", normalized);
        assert!(normalized.contains("## Root Two"));
        // Use lines() to check exact matches, avoiding substring match of "# Root Two" inside "## Root Two"
        assert!(!normalized.lines().any(|line| line.trim() == "# Root Two"));
    }
}

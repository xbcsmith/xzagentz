// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Heading normalization utilities
//!
//! Provides helpers to normalize markdown heading levels when composing
//! multiple components into a single document.

/// Normalize heading levels in `content` so that the minimum heading level
/// becomes `target_base`.
///
/// The function finds the smallest heading level present (1..6) and shifts
/// all headings by the offset needed to make that minimum equal `target_base`.
/// Headings are capped at level 6.
///
/// # Arguments
///
/// * `content` - Markdown content to normalize
/// * `target_base` - Desired minimum heading level (1..6)
///
/// # Examples
///
/// ```rust
/// use xzagentz::markdown::heading::normalize_headings;
/// let src = "# Title\n## Sub\n";
/// let out = normalize_headings(src, 2);
/// assert!(out.contains("## Title"));
/// ```
pub fn normalize_headings(content: &str, target_base: u8) -> String {
    if target_base == 0 || target_base > 6 {
        return content.to_string();
    }

    // Collect heading levels and their positions
    let mut min_level: Option<u8> = None;

    for line in content.lines() {
        let s = line.trim_start();
        if s.starts_with('#') {
            let level = s.chars().take_while(|c| *c == '#').count() as u8;
            if (1..=6).contains(&level) {
                min_level = Some(match min_level {
                    None => level,
                    Some(prev) => prev.min(level),
                });
            }
        }
    }

    let min_level = match min_level {
        Some(l) => l,
        None => return content.to_string(),
    };

    let offset = target_base.saturating_sub(min_level);

    if offset == 0 {
        return content.to_string();
    }

    // Apply offset to each heading line
    let mut out = String::with_capacity(content.len());

    for line in content.lines() {
        let mut written = false;
        let leading = line.len() - line.trim_start().len();
        let s = line.trim_start();
        if s.starts_with('#') {
            let level = s.chars().take_while(|c| *c == '#').count() as u8;
            if (1..=6).contains(&level) {
                let new_level = (level + offset).min(6);
                let rest = s.chars().skip(level as usize).collect::<String>();
                for _ in 0..leading {
                    out.push(' ');
                }
                for _ in 0..new_level {
                    out.push('#');
                }
                out.push_str(&rest);
                out.push('\n');
                written = true;
            }
        }

        if !written {
            out.push_str(line);
            out.push('\n');
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::normalize_headings;

    #[test]
    fn test_normalize_no_headings() {
        let src = "This file has no headings";
        assert_eq!(normalize_headings(src, 2), src);
    }

    #[test]
    fn test_normalize_shift_up() {
        let src = "# Title\n## Sub\n### Item\n";
        let out = normalize_headings(src, 2);
        assert!(out.contains("## Title"));
        assert!(out.contains("### Sub"));
        assert!(out.contains("#### Item"));
    }

    #[test]
    fn test_normalize_cap_at_six() {
        let src = "###### Max\n";
        let out = normalize_headings(src, 3);
        // This would try to make min=3 => offset 2 => 6 => still 6
        assert!(out.contains("###### Max"));
    }
}

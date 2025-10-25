//! Progress tracking for implementation workflow
//!
//! This module provides functionality to track progress through implementation plan sections,
//! persisting state to `.implementation_progress` file.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Progress state file name
const PROGRESS_FILE: &str = ".implementation_progress";

/// Progress tracker for implementation workflow
#[derive(Debug)]
pub struct ProgressTracker {
    /// Path to state file
    state_file: PathBuf,
    /// Current progress state
    state: ProgressState,
}

/// Progress state persisted to file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressState {
    /// Current section being worked on (e.g., "1.1")
    pub current_section: String,
    /// List of completed section numbers
    pub completed_sections: Vec<String>,
    /// Total number of sections in plan
    pub total_sections: usize,
    /// Last update timestamp (RFC 3339 format)
    pub last_updated: String,
    /// Implementation plan path
    pub plan_path: String,
    /// Project name
    pub project_name: String,
}

/// Progress statistics
#[derive(Debug, Clone)]
pub struct ProgressStats {
    /// Total sections
    pub total_sections: usize,
    /// Completed sections
    pub completed: usize,
    /// Completion percentage (0-100)
    pub percentage: f64,
    /// Current section
    pub current_section: String,
    /// Remaining sections
    pub remaining: usize,
}

impl ProgressTracker {
    /// Create new progress tracker
    ///
    /// # Arguments
    ///
    /// * `project_root` - Project root directory
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::progress::ProgressTracker;
    /// use std::path::Path;
    ///
    /// let tracker = ProgressTracker::new(Path::new(".")).unwrap();
    /// ```
    pub fn new(project_root: &Path) -> Result<Self> {
        let state_file = project_root.join(PROGRESS_FILE);

        let state = if state_file.exists() {
            Self::load_state(&state_file)?
        } else {
            ProgressState::default()
        };

        Ok(Self { state_file, state })
    }

    /// Initialize progress tracking for a new plan
    ///
    /// # Arguments
    ///
    /// * `plan_path` - Path to implementation plan
    /// * `total_sections` - Total number of sections
    /// * `project_name` - Project name
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::progress::ProgressTracker;
    /// use std::path::Path;
    ///
    /// let mut tracker = ProgressTracker::new(Path::new(".")).unwrap();
    /// tracker.initialize("docs/explanations/implementation_plan.md", 42, "xzagentz").unwrap();
    /// ```
    pub fn initialize(
        &mut self,
        plan_path: &str,
        total_sections: usize,
        project_name: &str,
    ) -> Result<()> {
        self.state = ProgressState {
            current_section: "1.1".to_string(),
            completed_sections: Vec::new(),
            total_sections,
            last_updated: chrono::Utc::now().to_rfc3339(),
            plan_path: plan_path.to_string(),
            project_name: project_name.to_string(),
        };

        self.save()
    }

    /// Get current section
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::progress::ProgressTracker;
    /// use std::path::Path;
    ///
    /// let tracker = ProgressTracker::new(Path::new(".")).unwrap();
    /// let current = tracker.current();
    /// println!("Currently on section: {}", current);
    /// ```
    pub fn current(&self) -> &str {
        &self.state.current_section
    }

    /// Mark current section as complete and move to next
    ///
    /// # Arguments
    ///
    /// * `section_number` - Section number to mark complete (e.g., "1.1")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::progress::ProgressTracker;
    /// use std::path::Path;
    ///
    /// let mut tracker = ProgressTracker::new(Path::new(".")).unwrap();
    /// tracker.complete("1.1").unwrap();
    /// ```
    pub fn complete(&mut self, section_number: &str) -> Result<()> {
        if !self
            .state
            .completed_sections
            .contains(&section_number.to_string())
        {
            self.state
                .completed_sections
                .push(section_number.to_string());
        }

        self.state.last_updated = chrono::Utc::now().to_rfc3339();
        self.save()
    }

    /// Set current section
    ///
    /// # Arguments
    ///
    /// * `section_number` - Section number to set as current
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::progress::ProgressTracker;
    /// use std::path::Path;
    ///
    /// let mut tracker = ProgressTracker::new(Path::new(".")).unwrap();
    /// tracker.set_current("2.3").unwrap();
    /// ```
    pub fn set_current(&mut self, section_number: &str) -> Result<()> {
        self.state.current_section = section_number.to_string();
        self.state.last_updated = chrono::Utc::now().to_rfc3339();
        self.save()
    }

    /// Get next section number
    ///
    /// # Arguments
    ///
    /// * `all_sections` - List of all section numbers in order
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::progress::ProgressTracker;
    /// use std::path::Path;
    ///
    /// let mut tracker = ProgressTracker::new(Path::new(".")).unwrap();
    /// tracker.set_current("1.1").unwrap();
    ///
    /// let sections = vec!["1.1", "1.2", "2.1"];
    /// let next = tracker.next_section(&sections);
    /// assert_eq!(next, Some("1.2"));
    /// ```
    pub fn next_section<'a>(&self, all_sections: &'a [&str]) -> Option<&'a str> {
        let current_idx = all_sections
            .iter()
            .position(|&s| s == self.state.current_section)?;

        all_sections.get(current_idx + 1).copied()
    }

    /// Calculate completion percentage
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::prompts::progress::ProgressState;
    ///
    /// let state = ProgressState {
    ///     current_section: "1.1".to_string(),
    ///     completed_sections: vec!["1.1".to_string(), "1.2".to_string()],
    ///     total_sections: 10,
    ///     last_updated: "2024-01-01T00:00:00Z".to_string(),
    ///     plan_path: "plan.md".to_string(),
    ///     project_name: "test".to_string(),
    /// };
    ///
    /// let percentage = (state.completed_sections.len() as f64 / state.total_sections as f64) * 100.0;
    /// assert_eq!(percentage, 20.0);
    /// ```
    pub fn completion_percentage(&self) -> f64 {
        if self.state.total_sections == 0 {
            return 0.0;
        }

        (self.state.completed_sections.len() as f64 / self.state.total_sections as f64) * 100.0
    }

    /// Get progress statistics
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::progress::ProgressTracker;
    /// use std::path::Path;
    ///
    /// let tracker = ProgressTracker::new(Path::new(".")).unwrap();
    /// let stats = tracker.stats();
    /// println!("Progress: {:.1}%", stats.percentage);
    /// ```
    pub fn stats(&self) -> ProgressStats {
        let completed = self.state.completed_sections.len();
        let total = self.state.total_sections;
        let percentage = self.completion_percentage();

        ProgressStats {
            total_sections: total,
            completed,
            percentage,
            current_section: self.state.current_section.clone(),
            remaining: total.saturating_sub(completed),
        }
    }

    /// Check if section is completed
    ///
    /// # Arguments
    ///
    /// * `section_number` - Section number to check
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::progress::ProgressTracker;
    /// use std::path::Path;
    ///
    /// let tracker = ProgressTracker::new(Path::new(".")).unwrap();
    /// if tracker.is_completed("1.1") {
    ///     println!("Section 1.1 is complete");
    /// }
    /// ```
    pub fn is_completed(&self, section_number: &str) -> bool {
        self.state
            .completed_sections
            .contains(&section_number.to_string())
    }

    /// Reset progress to start
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::progress::ProgressTracker;
    /// use std::path::Path;
    ///
    /// let mut tracker = ProgressTracker::new(Path::new(".")).unwrap();
    /// tracker.reset().unwrap();
    /// ```
    pub fn reset(&mut self) -> Result<()> {
        self.state.current_section = "1.1".to_string();
        self.state.completed_sections.clear();
        self.state.last_updated = chrono::Utc::now().to_rfc3339();
        self.save()
    }

    /// Save state to file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::progress::ProgressTracker;
    /// use std::path::Path;
    ///
    /// let mut tracker = ProgressTracker::new(Path::new(".")).unwrap();
    /// tracker.save().unwrap();
    /// ```
    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(&self.state)?;

        fs::write(&self.state_file, content).map_err(|e| Error::FileIo {
            path: self.state_file.clone(),
            source: e,
        })?;

        Ok(())
    }

    /// Load state from file
    fn load_state(path: &Path) -> Result<ProgressState> {
        let content = fs::read_to_string(path).map_err(|e| Error::FileIo {
            path: path.to_path_buf(),
            source: e,
        })?;

        toml::from_str(&content).map_err(|e| Error::PlanParse {
            name: path.display().to_string(),
            reason: e.to_string(),
        })
    }

    /// Get state reference
    pub fn state(&self) -> &ProgressState {
        &self.state
    }
}

impl Default for ProgressState {
    fn default() -> Self {
        Self {
            current_section: "1.1".to_string(),
            completed_sections: Vec::new(),
            total_sections: 0,
            last_updated: chrono::Utc::now().to_rfc3339(),
            plan_path: String::new(),
            project_name: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_progress_state_default() {
        let state = ProgressState::default();
        assert_eq!(state.current_section, "1.1");
        assert_eq!(state.completed_sections.len(), 0);
        assert_eq!(state.total_sections, 0);
    }

    #[test]
    fn test_tracker_new_creates_default_state() {
        let temp_dir = tempdir().unwrap();
        let tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        assert_eq!(tracker.current(), "1.1");
    }

    #[test]
    fn test_initialize_sets_state() {
        let temp_dir = tempdir().unwrap();
        let mut tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        tracker.initialize("plan.md", 10, "test_project").unwrap();

        assert_eq!(tracker.state.total_sections, 10);
        assert_eq!(tracker.state.project_name, "test_project");
        assert_eq!(tracker.state.plan_path, "plan.md");
    }

    #[test]
    fn test_complete_marks_section() {
        let temp_dir = tempdir().unwrap();
        let mut tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        tracker.initialize("plan.md", 10, "test").unwrap();

        tracker.complete("1.1").unwrap();
        assert!(tracker.is_completed("1.1"));
        assert_eq!(tracker.state.completed_sections.len(), 1);
    }

    #[test]
    fn test_complete_prevents_duplicates() {
        let temp_dir = tempdir().unwrap();
        let mut tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        tracker.initialize("plan.md", 10, "test").unwrap();

        tracker.complete("1.1").unwrap();
        tracker.complete("1.1").unwrap();
        assert_eq!(tracker.state.completed_sections.len(), 1);
    }

    #[test]
    fn test_set_current_updates_section() {
        let temp_dir = tempdir().unwrap();
        let mut tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        tracker.set_current("2.3").unwrap();
        assert_eq!(tracker.current(), "2.3");
    }

    #[test]
    fn test_next_section_returns_next() {
        let temp_dir = tempdir().unwrap();
        let mut tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        tracker.set_current("1.1").unwrap();

        let sections = vec!["1.1", "1.2", "2.1"];
        let next = tracker.next_section(&sections);
        assert_eq!(next, Some("1.2"));
    }

    #[test]
    fn test_next_section_returns_none_at_end() {
        let temp_dir = tempdir().unwrap();
        let mut tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        tracker.set_current("2.1").unwrap();

        let sections = vec!["1.1", "1.2", "2.1"];
        let next = tracker.next_section(&sections);
        assert_eq!(next, None);
    }

    #[test]
    fn test_completion_percentage_calculation() {
        let temp_dir = tempdir().unwrap();
        let mut tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        tracker.initialize("plan.md", 10, "test").unwrap();

        tracker.complete("1.1").unwrap();
        tracker.complete("1.2").unwrap();

        let percentage = tracker.completion_percentage();
        assert_eq!(percentage, 20.0);
    }

    #[test]
    fn test_stats_returns_correct_values() {
        let temp_dir = tempdir().unwrap();
        let mut tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        tracker.initialize("plan.md", 10, "test").unwrap();
        tracker.complete("1.1").unwrap();
        tracker.complete("1.2").unwrap();

        let stats = tracker.stats();
        assert_eq!(stats.total_sections, 10);
        assert_eq!(stats.completed, 2);
        assert_eq!(stats.percentage, 20.0);
        assert_eq!(stats.remaining, 8);
    }

    #[test]
    fn test_reset_clears_progress() {
        let temp_dir = tempdir().unwrap();
        let mut tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        tracker.initialize("plan.md", 10, "test").unwrap();
        tracker.complete("1.1").unwrap();
        tracker.set_current("1.2").unwrap();

        tracker.reset().unwrap();

        assert_eq!(tracker.current(), "1.1");
        assert_eq!(tracker.state.completed_sections.len(), 0);
    }

    #[test]
    fn test_save_and_load_state() {
        let temp_dir = tempdir().unwrap();
        let mut tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        tracker.initialize("plan.md", 10, "test").unwrap();
        tracker.complete("1.1").unwrap();
        tracker.save().unwrap();

        // Load in new tracker
        let tracker2 = ProgressTracker::new(temp_dir.path()).unwrap();
        assert_eq!(tracker2.state.total_sections, 10);
        assert_eq!(tracker2.state.completed_sections.len(), 1);
        assert!(tracker2.is_completed("1.1"));
    }

    #[test]
    fn test_is_completed_returns_false_for_uncompleted() {
        let temp_dir = tempdir().unwrap();
        let tracker = ProgressTracker::new(temp_dir.path()).unwrap();
        assert!(!tracker.is_completed("1.1"));
    }
}

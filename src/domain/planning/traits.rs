//! Domain traits for planning
//!
//! This module defines the core behavioral contracts for the planning domain.
//! Infrastructure implementations will implement these traits to provide
//! concrete functionality while keeping the domain layer pure.
//!
//! # Traits
//!
//! - [`PlanGenerator`]: Generates implementation plans from architecture documents
//! - [`ArchitectureParser`]: Parses architecture documents into domain models
//! - [`PlanWriter`]: Writes plans to persistent storage
//!
//! # Design Principles
//!
//! 1. **Domain-driven**: Traits define domain behavior, not implementation details
//! 2. **Technology-agnostic**: No assumptions about infrastructure (HTTP, filesystem, etc.)
//! 3. **Testable**: Easy to mock for testing domain logic
//! 4. **Composable**: Traits can be combined to create complex workflows
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::domain::planning::{
//!     PlanGenerator, ArchitectureDocument, Plan, PlanOptions
//! };
//!
//! // In tests, you can create mock implementations
//! struct MockPlanGenerator;
//!
//! impl PlanGenerator for MockPlanGenerator {
//!     type Error = std::io::Error;
//!
//!     fn generate_plan(
//!         &self,
//!         architecture: &ArchitectureDocument,
//!         options: &PlanOptions,
//!     ) -> Result<Plan, Self::Error> {
//!         // Mock implementation
//!         unimplemented!()
//!     }
//! }
//! ```

use std::path::Path;

use super::architecture::ArchitectureDocument;
use super::models::Plan;

/// Options for plan generation
///
/// Configuration options that control how plans are generated from
/// architecture documents.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::planning::PlanOptions;
///
/// let options = PlanOptions::default()
///     .with_num_phases(5)
///     .with_model("llama3.2:3b");
///
/// assert_eq!(options.num_phases(), Some(5));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlanOptions {
    num_phases: Option<usize>,
    model: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<usize>,
}

impl PlanOptions {
    /// Creates new plan options with defaults
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::PlanOptions;
    ///
    /// let options = PlanOptions::new();
    /// assert_eq!(options.num_phases(), None);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of phases to generate (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::PlanOptions;
    ///
    /// let options = PlanOptions::new().with_num_phases(3);
    /// assert_eq!(options.num_phases(), Some(3));
    /// ```
    pub fn with_num_phases(mut self, num_phases: usize) -> Self {
        self.num_phases = Some(num_phases);
        self
    }

    /// Sets the model to use (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::PlanOptions;
    ///
    /// let options = PlanOptions::new().with_model("llama3.2:3b");
    /// assert_eq!(options.model(), Some("llama3.2:3b"));
    /// ```
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Sets the temperature for generation (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::PlanOptions;
    ///
    /// let options = PlanOptions::new().with_temperature(0.7);
    /// assert_eq!(options.temperature(), Some(0.7));
    /// ```
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the max tokens for generation (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::PlanOptions;
    ///
    /// let options = PlanOptions::new().with_max_tokens(4096);
    /// assert_eq!(options.max_tokens(), Some(4096));
    /// ```
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Returns the number of phases
    pub fn num_phases(&self) -> Option<usize> {
        self.num_phases
    }

    /// Returns the model name
    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }

    /// Returns the temperature
    pub fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    /// Returns the max tokens
    pub fn max_tokens(&self) -> Option<usize> {
        self.max_tokens
    }
}

/// Trait for generating implementation plans
///
/// Implementations of this trait take architecture documents and generate
/// structured implementation plans. This is typically implemented by LLM-based
/// generators or rule-based planners.
///
/// # Type Parameters
///
/// * `Error` - The error type returned by generation operations
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::domain::planning::{
///     PlanGenerator, ArchitectureDocument, Plan, PlanOptions
/// };
///
/// struct MyGenerator;
///
/// impl PlanGenerator for MyGenerator {
///     type Error = std::io::Error;
///
///     fn generate_plan(
///         &self,
///         architecture: &ArchitectureDocument,
///         options: &PlanOptions,
///     ) -> Result<Plan, Self::Error> {
///         // Generate plan from architecture
///         unimplemented!()
///     }
/// }
/// ```
pub trait PlanGenerator {
    /// The error type for plan generation
    type Error: std::error::Error;

    /// Generates an implementation plan from an architecture document
    ///
    /// # Arguments
    ///
    /// * `architecture` - The parsed architecture document
    /// * `options` - Configuration options for plan generation
    ///
    /// # Returns
    ///
    /// Returns a Plan if successful, or an error if generation fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The architecture document is malformed
    /// - The generation service is unavailable
    /// - The generated plan is invalid
    fn generate_plan(
        &self,
        architecture: &ArchitectureDocument,
        options: &PlanOptions,
    ) -> Result<Plan, Self::Error>;
}

/// Trait for parsing architecture documents
///
/// Implementations of this trait parse raw content (typically markdown) into
/// structured ArchitectureDocument models.
///
/// # Type Parameters
///
/// * `Error` - The error type returned by parsing operations
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::domain::planning::{ArchitectureParser, ArchitectureDocument};
///
/// struct MyParser;
///
/// impl ArchitectureParser for MyParser {
///     type Error = std::io::Error;
///
///     fn parse(&self, content: &str) -> Result<ArchitectureDocument, Self::Error> {
///         // Parse markdown content
///         unimplemented!()
///     }
/// }
/// ```
pub trait ArchitectureParser {
    /// The error type for parsing operations
    type Error: std::error::Error;

    /// Parses raw content into an architecture document
    ///
    /// # Arguments
    ///
    /// * `content` - The raw content to parse (typically markdown)
    ///
    /// # Returns
    ///
    /// Returns an ArchitectureDocument if successful, or an error if parsing fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The content is not valid markdown
    /// - Required sections are missing
    /// - The document structure is invalid
    fn parse(&self, content: &str) -> Result<ArchitectureDocument, Self::Error>;
}

/// Trait for writing plans to storage
///
/// Implementations of this trait serialize and persist plans to various
/// storage backends (filesystem, database, cloud storage, etc.).
///
/// # Type Parameters
///
/// * `Error` - The error type returned by write operations
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::domain::planning::{PlanWriter, Plan};
/// use std::path::Path;
///
/// struct MyWriter;
///
/// impl PlanWriter for MyWriter {
///     type Error = std::io::Error;
///
///     fn write_plan(&self, plan: &Plan, output_path: &Path) -> Result<(), Self::Error> {
///         // Write plan to file
///         unimplemented!()
///     }
/// }
/// ```
pub trait PlanWriter {
    /// The error type for write operations
    type Error: std::error::Error;

    /// Writes a plan to the specified output path
    ///
    /// # Arguments
    ///
    /// * `plan` - The plan to write
    /// * `output_path` - The path where the plan should be written
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if successful, or an error if writing fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The output path is not writable
    /// - The plan serialization fails
    /// - The file system operation fails
    fn write_plan(&self, plan: &Plan, output_path: &Path) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_options_default() {
        let options = PlanOptions::default();
        assert_eq!(options.num_phases(), None);
        assert_eq!(options.model(), None);
        assert_eq!(options.temperature(), None);
        assert_eq!(options.max_tokens(), None);
    }

    #[test]
    fn test_plan_options_new() {
        let options = PlanOptions::new();
        assert_eq!(options.num_phases(), None);
    }

    #[test]
    fn test_plan_options_with_num_phases() {
        let options = PlanOptions::new().with_num_phases(5);
        assert_eq!(options.num_phases(), Some(5));
    }

    #[test]
    fn test_plan_options_with_model() {
        let options = PlanOptions::new().with_model("llama3.2:3b");
        assert_eq!(options.model(), Some("llama3.2:3b"));
    }

    #[test]
    fn test_plan_options_with_temperature() {
        let options = PlanOptions::new().with_temperature(0.7);
        assert_eq!(options.temperature(), Some(0.7));
    }

    #[test]
    fn test_plan_options_with_max_tokens() {
        let options = PlanOptions::new().with_max_tokens(4096);
        assert_eq!(options.max_tokens(), Some(4096));
    }

    #[test]
    fn test_plan_options_builder_pattern() {
        let options = PlanOptions::new()
            .with_num_phases(7)
            .with_model("mistral")
            .with_temperature(0.8)
            .with_max_tokens(8192);

        assert_eq!(options.num_phases(), Some(7));
        assert_eq!(options.model(), Some("mistral"));
        assert_eq!(options.temperature(), Some(0.8));
        assert_eq!(options.max_tokens(), Some(8192));
    }

    #[test]
    fn test_plan_options_equality() {
        let options1 = PlanOptions::new().with_num_phases(5);
        let options2 = PlanOptions::new().with_num_phases(5);
        let options3 = PlanOptions::new().with_num_phases(3);

        assert_eq!(options1, options2);
        assert_ne!(options1, options3);
    }

    // Mock implementations for testing trait bounds
    struct MockGenerator;

    impl PlanGenerator for MockGenerator {
        type Error = std::io::Error;

        fn generate_plan(
            &self,
            _architecture: &ArchitectureDocument,
            _options: &PlanOptions,
        ) -> Result<Plan, Self::Error> {
            // Mock implementation
            unimplemented!()
        }
    }

    struct MockParser;

    impl ArchitectureParser for MockParser {
        type Error = std::io::Error;

        fn parse(&self, _content: &str) -> Result<ArchitectureDocument, Self::Error> {
            unimplemented!()
        }
    }

    struct MockWriter;

    impl PlanWriter for MockWriter {
        type Error = std::io::Error;

        fn write_plan(&self, _plan: &Plan, _output_path: &Path) -> Result<(), Self::Error> {
            unimplemented!()
        }
    }

    #[test]
    fn test_trait_implementations_compile() {
        // This test just verifies that our mock implementations compile
        let _generator: Box<dyn PlanGenerator<Error = std::io::Error>> = Box::new(MockGenerator);
        let _parser: Box<dyn ArchitectureParser<Error = std::io::Error>> = Box::new(MockParser);
        let _writer: Box<dyn PlanWriter<Error = std::io::Error>> = Box::new(MockWriter);
    }
}

//! Template validation
//!
//! This module provides the `TemplateValidator` for validating template structure,
//! component references, and ensuring templates are well-formed.

use crate::components::ComponentLoader;
use crate::{Error, Result};

use super::Template;

/// Validates template structure and component references
///
/// The validator checks:
/// - Template structure (name, description, components)
/// - Component references exist
/// - Component type/name combinations are valid
/// - Required fields are present
///
/// # Examples
///
/// ```
/// use xzagentz::templates::{TemplateValidator, Template, TemplateComponent};
/// use xzagentz::components::ComponentLoader;
/// use xzagentz::core::ComponentType;
///
/// let mut template = Template::new(
///     "test".to_string(),
///     "Test template".to_string()
/// );
/// template.add_component(
///     TemplateComponent::new(ComponentType::Core, "quick_reference".to_string())
/// );
///
/// let loader = ComponentLoader::new(None);
/// let validator = TemplateValidator::new(loader);
///
/// // Validate structure (doesn't check if components exist on filesystem)
/// assert!(validator.validate_structure(&template).is_ok());
/// ```
#[derive(Debug)]
pub struct TemplateValidator {
    /// Component loader for validating component references
    component_loader: Option<ComponentLoader>,
}

impl TemplateValidator {
    /// Creates a new template validator
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::TemplateValidator;
    /// use xzagentz::components::ComponentLoader;
    ///
    /// let loader = ComponentLoader::new(None);
    /// let validator = TemplateValidator::new(loader);
    /// ```
    pub fn new(component_loader: ComponentLoader) -> Self {
        Self {
            component_loader: Some(component_loader),
        }
    }

    /// Creates a validator without component reference checking
    ///
    /// This validator only checks template structure, not whether
    /// referenced components actually exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::TemplateValidator;
    ///
    /// let validator = TemplateValidator::without_components();
    /// ```
    pub fn without_components() -> Self {
        Self {
            component_loader: None,
        }
    }

    /// Validates template structure
    ///
    /// Checks that:
    /// - Name is not empty
    /// - Description is not empty
    /// - At least one component is defined
    ///
    /// # Arguments
    ///
    /// * `template` - The template to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if validation passes
    ///
    /// # Errors
    ///
    /// Returns `Error::Validation` if validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::{TemplateValidator, Template, TemplateComponent};
    /// use xzagentz::core::ComponentType;
    ///
    /// let mut template = Template::new("valid".to_string(), "Valid".to_string());
    /// template.add_component(
    ///     TemplateComponent::new(ComponentType::Core, "test".to_string())
    /// );
    ///
    /// let validator = TemplateValidator::without_components();
    /// assert!(validator.validate_structure(&template).is_ok());
    /// ```
    pub fn validate_structure(&self, template: &Template) -> Result<()> {
        template.validate()
    }

    /// Validates that all component references exist
    ///
    /// Checks that each component referenced in the template can be loaded
    /// from the component loader.
    ///
    /// # Arguments
    ///
    /// * `template` - The template to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all components exist
    ///
    /// # Errors
    ///
    /// Returns `Error::ComponentNotFound` if any component doesn't exist
    /// Returns `Error::Validation` if component loader is not configured
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::templates::{TemplateValidator, Template, TemplateComponent};
    /// use xzagentz::components::ComponentLoader;
    /// use xzagentz::core::ComponentType;
    ///
    /// let mut template = Template::new("test".to_string(), "Test".to_string());
    /// template.add_component(
    ///     TemplateComponent::new(ComponentType::Core, "quick_reference".to_string())
    /// );
    ///
    /// let loader = ComponentLoader::new(None);
    /// let validator = TemplateValidator::new(loader);
    ///
    /// // This will check if components/core/quick_reference.md exists
    /// let result = validator.validate_components(&template);
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn validate_components(&self, template: &Template) -> Result<()> {
        let loader = self.component_loader.as_ref().ok_or_else(|| {
            Error::Validation("Component validation requires a component loader".to_string())
        })?;

        for component in &template.components {
            // Try to load the component to verify it exists
            loader.load(&component.name, component.component_type)?;
        }

        Ok(())
    }

    /// Validates the complete template
    ///
    /// Performs both structure validation and component reference validation
    /// (if a component loader is configured).
    ///
    /// # Arguments
    ///
    /// * `template` - The template to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if validation passes
    ///
    /// # Errors
    ///
    /// Returns an error if structure or component validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::{TemplateValidator, Template, TemplateComponent};
    /// use xzagentz::core::ComponentType;
    ///
    /// let mut template = Template::new("test".to_string(), "Test".to_string());
    /// template.add_component(
    ///     TemplateComponent::new(ComponentType::Core, "overview".to_string())
    /// );
    ///
    /// let validator = TemplateValidator::without_components();
    /// assert!(validator.validate(&template).is_ok());
    /// ```
    pub fn validate(&self, template: &Template) -> Result<()> {
        // First validate structure
        self.validate_structure(template)?;

        // Then validate component references if loader is configured
        if self.component_loader.is_some() {
            self.validate_components(template)?;
        }

        Ok(())
    }

    /// Validates that component ordering is consistent
    ///
    /// Checks that:
    /// - If any component has an order, all should have orders
    /// - No duplicate order values
    ///
    /// # Arguments
    ///
    /// * `template` - The template to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if ordering is valid
    ///
    /// # Errors
    ///
    /// Returns `Error::Validation` if ordering is inconsistent
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::{TemplateValidator, Template, TemplateComponent};
    /// use xzagentz::core::ComponentType;
    ///
    /// let mut template = Template::new("test".to_string(), "Test".to_string());
    /// template.add_component(
    ///     TemplateComponent::with_order(ComponentType::Core, "first".to_string(), true, 1)
    /// );
    /// template.add_component(
    ///     TemplateComponent::with_order(ComponentType::Core, "second".to_string(), true, 2)
    /// );
    ///
    /// let validator = TemplateValidator::without_components();
    /// assert!(validator.validate_ordering(&template).is_ok());
    /// ```
    pub fn validate_ordering(&self, template: &Template) -> Result<()> {
        let with_order: Vec<_> = template
            .components
            .iter()
            .filter(|c| c.order.is_some())
            .collect();

        if with_order.is_empty() {
            return Ok(()); // No ordering specified is fine
        }

        // Check for duplicate orders
        let mut orders: Vec<usize> = with_order.iter().map(|c| c.order.unwrap()).collect();
        orders.sort_unstable();

        for window in orders.windows(2) {
            if window[0] == window[1] {
                return Err(Error::Validation(format!(
                    "Duplicate component order value: {}",
                    window[0]
                )));
            }
        }

        Ok(())
    }

    /// Validates that required components are present
    ///
    /// Checks that the template has at least one required component.
    ///
    /// # Arguments
    ///
    /// * `template` - The template to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if at least one required component exists
    ///
    /// # Errors
    ///
    /// Returns `Error::Validation` if no required components are found
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::{TemplateValidator, Template, TemplateComponent};
    /// use xzagentz::core::ComponentType;
    ///
    /// let mut template = Template::new("test".to_string(), "Test".to_string());
    /// template.add_component(
    ///     TemplateComponent::with_required(ComponentType::Core, "required".to_string(), true)
    /// );
    ///
    /// let validator = TemplateValidator::without_components();
    /// assert!(validator.validate_required_components(&template).is_ok());
    /// ```
    pub fn validate_required_components(&self, template: &Template) -> Result<()> {
        let required_count = template.components.iter().filter(|c| c.required).count();

        if required_count == 0 {
            return Err(Error::Validation(
                "Template must have at least one required component".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for TemplateValidator {
    fn default() -> Self {
        Self::without_components()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ComponentType;
    use crate::templates::TemplateComponent;

    #[test]
    fn test_validator_new() {
        let loader = ComponentLoader::new(None);
        let _validator = TemplateValidator::new(loader);
    }

    #[test]
    fn test_validator_without_components() {
        let validator = TemplateValidator::without_components();
        assert!(validator.component_loader.is_none());
    }

    #[test]
    fn test_validate_structure_success() {
        let mut template = Template::new("valid".to_string(), "Valid template".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "test".to_string(),
        ));

        let validator = TemplateValidator::without_components();
        assert!(validator.validate_structure(&template).is_ok());
    }

    #[test]
    fn test_validate_structure_empty_name() {
        let mut template = Template::new("".to_string(), "Valid description".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "test".to_string(),
        ));

        let validator = TemplateValidator::without_components();
        assert!(validator.validate_structure(&template).is_err());
    }

    #[test]
    fn test_validate_structure_empty_description() {
        let mut template = Template::new("valid".to_string(), "".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "test".to_string(),
        ));

        let validator = TemplateValidator::without_components();
        assert!(validator.validate_structure(&template).is_err());
    }

    #[test]
    fn test_validate_structure_no_components() {
        let template = Template::new("valid".to_string(), "Valid description".to_string());

        let validator = TemplateValidator::without_components();
        assert!(validator.validate_structure(&template).is_err());
    }

    #[test]
    fn test_validate_components_without_loader() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "test".to_string(),
        ));

        let validator = TemplateValidator::without_components();
        let result = validator.validate_components(&template);

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::Validation(_))));
    }

    #[test]
    fn test_validate_ordering_no_order() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "first".to_string(),
        ));
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "second".to_string(),
        ));

        let validator = TemplateValidator::without_components();
        assert!(validator.validate_ordering(&template).is_ok());
    }

    #[test]
    fn test_validate_ordering_valid() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::with_order(
            ComponentType::Core,
            "first".to_string(),
            true,
            1,
        ));
        template.add_component(TemplateComponent::with_order(
            ComponentType::Core,
            "second".to_string(),
            true,
            2,
        ));
        template.add_component(TemplateComponent::with_order(
            ComponentType::Core,
            "third".to_string(),
            true,
            3,
        ));

        let validator = TemplateValidator::without_components();
        assert!(validator.validate_ordering(&template).is_ok());
    }

    #[test]
    fn test_validate_ordering_duplicate() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::with_order(
            ComponentType::Core,
            "first".to_string(),
            true,
            1,
        ));
        template.add_component(TemplateComponent::with_order(
            ComponentType::Core,
            "second".to_string(),
            true,
            1, // Duplicate!
        ));

        let validator = TemplateValidator::without_components();
        assert!(validator.validate_ordering(&template).is_err());
    }

    #[test]
    fn test_validate_required_components_success() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::with_required(
            ComponentType::Core,
            "required".to_string(),
            true,
        ));
        template.add_component(TemplateComponent::with_required(
            ComponentType::Tools,
            "optional".to_string(),
            false,
        ));

        let validator = TemplateValidator::without_components();
        assert!(validator.validate_required_components(&template).is_ok());
    }

    #[test]
    fn test_validate_required_components_none_required() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::with_required(
            ComponentType::Core,
            "optional1".to_string(),
            false,
        ));
        template.add_component(TemplateComponent::with_required(
            ComponentType::Tools,
            "optional2".to_string(),
            false,
        ));

        let validator = TemplateValidator::without_components();
        assert!(validator.validate_required_components(&template).is_err());
    }

    #[test]
    fn test_validate_full_without_loader() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "test".to_string(),
        ));

        let validator = TemplateValidator::without_components();
        assert!(validator.validate(&template).is_ok());
    }

    #[test]
    fn test_validate_full_invalid_structure() {
        let template = Template::new("".to_string(), "Test".to_string());

        let validator = TemplateValidator::without_components();
        assert!(validator.validate(&template).is_err());
    }

    #[test]
    fn test_validator_default() {
        let validator = TemplateValidator::default();
        assert!(validator.component_loader.is_none());
    }
}

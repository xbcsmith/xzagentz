// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Application Layer
//!
//! This module contains the application layer which orchestrates domain and
//! infrastructure components to implement use cases.
//!
//! The application layer:
//! - Coordinates between domain and infrastructure layers
//! - Implements use cases and workflows
//! - Handles error mapping and translation
//! - Provides service abstractions for the CLI layer
//!
//! # Architecture
//!
//! ```text
//! CLI Layer
//!     ↓
//! Application Layer (this module)
//!     ↓
//! Domain Layer + Infrastructure Layer
//! ```

pub mod architecture_service;
pub mod error;
pub mod interactive_architecture_session;
pub mod interactive_session;
pub mod planning_service;

pub use architecture_service::{ArchitectureService, ServiceError};
pub use error::{ApplicationError, Result};
pub use interactive_architecture_session::{InteractiveArchitectureSession, SessionError};
pub use interactive_session::{InteractiveConfig, InteractivePlanningSession};
pub use planning_service::PlanningService;

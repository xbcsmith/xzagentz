//! Parser module for AGENTS.md and other document formats
//!
//! This module provides parsers for various document types used in xzagentz.

pub mod agents;

pub use agents::{AgentsDocument, AgentsParser, Section};

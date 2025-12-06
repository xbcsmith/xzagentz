// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Parser module for AGENTS.md and other document formats
//!
//! This module provides parsers for various document types used in xzagentz.

pub mod agents;
pub mod readme;

pub use agents::{AgentsDocument, AgentsParser, Section};

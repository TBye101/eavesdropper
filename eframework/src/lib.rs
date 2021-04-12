//! This crate provides common types for plugins to use in order to be utilized as plugins by the eavesdropper cli tool.

#[cfg(test)]
extern crate proptest;

pub mod analysis_framework;
pub mod rversion;
pub mod rversion_req;
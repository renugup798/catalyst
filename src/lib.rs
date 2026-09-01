//! Catalyst - Python-to-Rust Conversion Framework
//!
//! A comprehensive library for analyzing, converting, and optimizing
//! Python code to high-performance Rust equivalents.

pub mod analyzer;
pub mod converter;
pub mod optimizer;
pub mod generator;
pub mod types;

pub use analyzer::Analyzer;
pub use converter::Converter;
pub use optimizer::Optimizer;

#[derive(Debug, Clone)]
pub struct ConversionConfig {
    pub strict_types: bool,
    pub optimize: bool,
    pub generate_tests: bool,
    pub include_benchmarks: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            strict_types: true,
            optimize: true,
            generate_tests: true,
            include_benchmarks: true,
        }
    }
}

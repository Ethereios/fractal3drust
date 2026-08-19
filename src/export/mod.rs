//! Export module.
//!
//! Handles export of formula specifications in various formats:
//! - WGSL shader files
//! - GLSL shader files
//! - Fosfora effect packages (shader.wgsl + effect.json)
//! - Shader Studio snippets

pub mod wgsl;
pub mod glsl;
pub mod fosfora;

pub use wgsl::*;
pub use glsl::*;
pub use fosfora::*;

use crate::db::FormulaEntry;
use crate::formula_dsl::prime::PrimeSchedule;

/// Export a formula to the specified format.
pub fn export_formula(entry: &FormulaEntry, format: ExportFormat, prime_schedule: Option<&PrimeSchedule>, output: Option<&str>) -> Result<String, String> {
    let content = match format {
        ExportFormat::Wgsl => wgsl::export_wgsl(entry, prime_schedule),
        ExportFormat::Glsl => glsl::export_glsl_with_docs(entry),
        ExportFormat::Fosfora => fosfora::export_fosfora_json(entry),
    };
    
    if let Some(path) = output {
        std::fs::write(path, &content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
    }
    
    Ok(content)
}

#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum ExportFormat {
    Wgsl,
    Glsl,
    Fosfora,
}

impl std::str::FromStr for ExportFormat {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wgsl" => Ok(ExportFormat::Wgsl),
            "glsl" => Ok(ExportFormat::Glsl),
            "fosfora" => Ok(ExportFormat::Fosfora),
            _ => Err(format!("Unknown export format: {}", s)),
        }
    }
}

impl From<&str> for ExportFormat {
    fn from(s: &str) -> Self {
        s.parse().unwrap_or(ExportFormat::Wgsl)
    }
}

impl From<String> for ExportFormat {
    fn from(s: String) -> Self {
        s.as_str().parse().unwrap_or(ExportFormat::Wgsl)
    }
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportFormat::Wgsl => write!(f, "wgsl"),
            ExportFormat::Glsl => write!(f, "glsl"),
            ExportFormat::Fosfora => write!(f, "fosfora"),
        }
    }
}
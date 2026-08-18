//! Fosfora Export Module
//!
//! Exports fractal formulas as Fosfora effect packages.
//! Fosfora is a shader-based fractal generator that runs on various platforms.

use crate::db::{FormulaEntry, PresetEntry};
use serde::{Serialize};

/// The shader effect file format for Fosfora.
#[derive(Serialize, Debug)]
struct FosforaEffect {
    /// Effect name
    pub name: String,
    /// Short description
    pub description: String,
    /// The WGSL/GLSL shader code
    pub shader: String,
    /// Uniform parameters
    pub parameters: Vec<FosforaField>,
    /// Known-good presets
    pub presets: Vec<FosforaPreset>,
    /// Author/origin
    pub author: String,
}

#[derive(Serialize, Debug)]
struct FosforaField {
    pub name: String,
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub default: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub group: Option<String>,
}

#[derive(Serialize, Debug)]
struct FosforaPreset {
    pub name: String,
    #[serde(rename = "values")]
    pub parameter_values: std::collections::HashMap<String, f32>,
    pub tags: Vec<String>,
}

/// Export a formula as a Fosfora JSON effect file.
pub fn export_fosfora_json(entry: &FormulaEntry) -> String {
    let parameters: Vec<FosforaField> = entry.parameter_fields
        .iter()
        .map(|f| FosforaField {
            name: f.name.clone(),
            label: f.label.clone(),
            field_type: match f.field_type {
                crate::db::ParamFieldType::Float => "float".to_string(),
                crate::db::ParamFieldType::Int => "int".to_string(),
                crate::db::ParamFieldType::Bool => "bool".to_string(),
            },
            default: f.default as f32,
            min_value: f.min as f32,
            max_value: f.max as f32,
            group: f.group.clone(),
        })
        .collect();
    
    let presets: Vec<FosforaPreset> = entry.known_good_presets
        .iter()
        .map(|p| FosforaPreset {
            name: p.name.clone(),
            parameter_values: p.parameters.iter()
                .map(|(k, v)| (k.clone(), *v as f32))
                .collect(),
            tags: p.tags.clone(),
        })
        .collect();
    
    let effect = FosforaEffect {
        name: entry.name.clone(),
        description: entry.notes.clone(),
        shader: entry.wgsl_shader.clone(),
        parameters,
        presets,
        author: entry.origin.clone(),
    };
    
    serde_json::to_string_pretty(&effect).unwrap_or_else(|_| "{}".to_string())
}

/// Export a formula as a complete Fosfora package directory.
pub fn export_fosfora_package(_entry: &FormulaEntry, _output_dir: &str) -> Result<(), String> {
    // In a real implementation, would:
    // 1. Create output directory
    // 2. Write effect.json
    // 3. Copy any required assets
    // 4. Generate README.txt
    
    Ok(())
}
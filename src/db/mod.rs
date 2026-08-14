//! Fractal formula database schema.
//!
//! This module defines the schema for storing and querying fractal formulas,
//! following the structure proposed in `docs/architecture.md`:
//!
//! Table: fractal_formula
//!   id, name, family, dimension, category, iteration_core,
//!   parameter_fields, de_supported, de_expression, wgsl_core_fn,
//!   known_good_presets, origin, notes
//!
//! Table: fractal_hybrid
//!   id, name, formula_ids, mix_rule, iteration_counts, origin, notes

pub mod catalogue;
pub mod hybrid;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Category of a fractal formula, following MB3D/Mandelbulber conventions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormulaCategory {
    /// Analytic / escape-time formula (e.g. Mandelbrot, Julia).
    #[serde(rename = "escape_time")]
    EscapeTime,
    /// Distance estimator formula (raymarchable, e.g. Mandelbulb, Mandelbox).
    #[serde(rename = "distance_estimator")]
    DistanceEstimator,
    /// Iterated function system (affine/contracting maps).
    #[serde(rename = "ifs")]
    IFS,
    /// Coordinate transform / modifier (folds, rotations, warps).
    #[serde(rename = "transform")]
    Transform,
    /// Hybrid composition of multiple formulas.
    #[serde(rename = "hybrid")]
    Hybrid,
}

/// A single parameter field for a formula.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParameterField {
    pub name: String,
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: ParamFieldType,
    pub default: f64,
    pub min: f64,
    pub max: f64,
    /// UI group label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Vector dimension (2, 3, or 4) — produces `vecN<f32>` in WGSL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_dim: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParamFieldType {
    Float,
    Int,
    Bool,
}

impl ParameterField {
    pub fn float(name: &str, label: &str, default: f64, min: f64, max: f64) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            field_type: ParamFieldType::Float,
            default,
            min,
            max,
            group: None,
            vector_dim: None,
        }
    }

    pub fn int(name: &str, label: &str, default: f64, min: f64, max: f64) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            field_type: ParamFieldType::Int,
            default,
            min,
            max,
            group: None,
            vector_dim: None,
        }
    }

    pub fn bool(name: &str, label: &str, default: f64) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            field_type: ParamFieldType::Bool,
            default,
            min: 0.0,
            max: 1.0,
            group: None,
            vector_dim: None,
        }
    }

    /// Create a vec3<f32> parameter (e.g. a Julia constant).
    pub fn vec3(name: &str, label: &str, default: [f64; 3], min: f64, max: f64) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            field_type: ParamFieldType::Float,
            default: default[0],
            min,
            max,
            group: None,
            vector_dim: Some(3),
        }
    }

    pub fn with_group(self, group: &str) -> Self {
        let mut s = self;
        s.group = Some(group.to_string());
        s
    }

    /// Generate WGSL uniform field declaration line.
    pub fn as_wgsl_field(&self) -> String {
        let wgsl_type = match (self.field_type, self.vector_dim) {
            (_, Some(2)) => "vec2<f32>",
            (_, Some(3)) => "vec3<f32>",
            (_, Some(4)) => "vec4<f32>",
            (ParamFieldType::Float, None) => "f32",
            (ParamFieldType::Int, None) => "i32",
            (ParamFieldType::Bool, None) => "bool",
            _ => "f32",
        };
        format!("    {}: {};", self.name, wgsl_type)
    }
}

/// A known-good parameter preset for a formula.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PresetEntry {
    pub name: String,
    pub parameters: HashMap<String, f64>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Formula DSL: a sequence of operations describing the iteration core.
///
/// Each step is an operation with a name and JSON parameters.  This provides
/// a machine-readable representation that can be analyzed, composed, and
/// converted to WGSL.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormulaDSL {
    pub steps: Vec<DSLStep>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DSLStep {
    pub op: String,
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

impl FormulaDSL {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn with_step(mut self, op: &str, params: HashMap<String, serde_json::Value>) -> Self {
        self.steps.push(DSLStep {
            op: op.to_string(),
            params,
        });
        self
    }

    pub fn add(&mut self, op: &str, params: HashMap<String, serde_json::Value>) {
        self.steps.push(DSLStep {
            op: op.to_string(),
            params,
        });
    }
}

impl Default for FormulaDSL {
    fn default() -> Self {
        Self::new()
    }
}

/// A complete fractal formula entry in the database.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormulaEntry {
    /// Unique identifier (e.g. "mandelbulb_n8").
    pub id: String,
    /// Display name.
    pub name: String,
    /// Formula family (e.g. "mandelbulb", "mandelbox").
    pub family: String,
    /// Number of dimensions.
    pub dimension: usize,
    /// Formula category.
    #[serde(rename = "category")]
    pub category: FormulaCategory,
    /// DSL/AST representation of the iteration core.
    pub iteration_core: FormulaDSL,
    /// Tunable parameters with ranges for UI.
    pub parameter_fields: Vec<ParameterField>,
    /// Whether a distance estimator is available.
    pub de_supported: bool,
    /// Human-readable DE expression (for documentation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub de_expression: Option<String>,
    /// Complete WGSL implementation (core functions + full shader).
    pub wgsl_core_fn: String,
    /// Complete WGSL shader source.
    pub wgsl_shader: String,
    /// Known-good parameter presets.
    #[serde(default)]
    pub known_good_presets: Vec<PresetEntry>,
    /// Source of this formula ("mandelbulb3d", "mandelbulber", "shadertoy", "research", "user").
    pub origin: String,
    /// Notes: provenance, references, caveats.
    pub notes: String,
}

impl FormulaEntry {
    pub fn new(id: &str, name: &str, family: &str, dimension: usize, category: FormulaCategory) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            family: family.to_string(),
            dimension,
            category,
            iteration_core: FormulaDSL::new(),
            parameter_fields: Vec::new(),
            de_supported: false,
            de_expression: None,
            wgsl_core_fn: String::new(),
            wgsl_shader: String::new(),
            known_good_presets: Vec::new(),
            origin: "research".to_string(),
            notes: String::new(),
        }
    }

    pub fn with_dsl(mut self, dsl: FormulaDSL) -> Self {
        self.iteration_core = dsl;
        self
    }

    pub fn with_params(mut self, params: Vec<ParameterField>) -> Self {
        self.parameter_fields = params;
        self
    }

    pub fn with_wgsl(mut self, core: &str, full: &str) -> Self {
        self.wgsl_core_fn = core.to_string();
        self.wgsl_shader = full.to_string();
        self
    }

    pub fn with_de(mut self, expr: &str) -> Self {
        self.de_supported = true;
        self.de_expression = Some(expr.to_string());
        self
    }

    pub fn with_presets(mut self, presets: Vec<PresetEntry>) -> Self {
        self.known_good_presets = presets;
        self
    }

    pub fn with_origin(mut self, origin: &str) -> Self {
        self.origin = origin.to_string();
        self
    }

    pub fn with_notes(mut self, notes: &str) -> Self {
        self.notes = notes.to_string();
        self
    }
}

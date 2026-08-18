//! Validation metrics for fractal formula correctness.
//!
//! Provides validation checks for fractal formulas to ensure they
//! are mathematically sound before export.

use crate::db::FormulaEntry;
use crate::constraint_integration::{PythagoreanManifold, snap};
use num_complex::Complex64;

/// Validation result for a formula.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub passed: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub metrics: Metrics,
}

/// Metrics computed for a fractal formula.
#[derive(Debug, Clone)]
pub struct Metrics {
    /// Number of iteration steps in the formula
    pub iteration_count: usize,
    /// Number of parameters
    pub parameter_count: usize,
    pub complexity_score: f64,
    pub constraint_error: f64,
    pub stability_metric: f64,
    pub escape_velocity: f64,
    pub periodicity_index: f64,
    pub symmetry_preservation: f64,
    pub dynamic_range: f64,
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics {
            iteration_count: 0,
            parameter_count: 0,
            complexity_score: 0.0,
            constraint_error: 0.0,
            stability_metric: 1.0,
            escape_velocity: 0.0,
            periodicity_index: 0.0,
            symmetry_preservation: 0.0,
            dynamic_range: 0.0,
        }
    }
}

impl ValidationResult {
    pub fn new() -> Self {
        ValidationResult {
            passed: true,
            warnings: Vec::new(),
            errors: Vec::new(),
            metrics: Metrics::default(),
        }
    }

    pub fn warning(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }

    pub fn error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
        self.passed = false;
    }

    pub fn compute_complexity(&mut self, formula: &FormulaEntry) -> f64 {
        let base_complexity = formula.iteration_core.steps.len() as f64;
        let param_complexity = formula.parameter_fields.len() as f64;
        let dsl_complexity = formula
            .iteration_core
            .steps
            .iter()
            .map(|s| s.params.len() as f64)
            .sum::<f64>();
        
        let total = (base_complexity * 0.5 + param_complexity * 0.3 + dsl_complexity * 0.2) / 3.0;
        self.metrics.complexity_score = total;
        total
    }

    pub fn validate_parameters(&mut self, formula: &FormulaEntry) {
        for param in &formula.parameter_fields {
            if param.max <= param.min {
                self.error(format!(
                    "Parameter {} has invalid range: min={} >= max={}",
                    param.name, param.min, param.max
                ));
            }

            if param.default < param.min || param.default > param.max {
                self.error(format!(
                    "Parameter {} default {} is outside allowed range [{}, {}]",
                    param.name, param.default, param.min, param.max
                ));
            }

            if param.name.is_empty() {
                self.error("Parameter has empty name");
            }

            if param.vector_dim.is_some() && param.vector_dim.unwrap() < 2 {
                self.warning(format!(
                    "Parameter {} has low vector dimension {}",
                    param.name, param.vector_dim.unwrap()
                ));
            }
        }
    }

    pub fn validate_dsl(&mut self, formula: &FormulaEntry) {
        if formula.iteration_core.steps.is_empty() {
            self.error("Formula has no DSL steps");
            return;
        }

        let valid_ops = [
            "spherical_power", "box_fold", "sphere_fold", "scale_offset",
            "complex_power", "triplex_mul", "quaternion_square",
            "add_c", "de_accumulate", "modulus", "abs_fold",
            "component_sort", "ifs_select", "waveguide_scatter",
            "prime_modulate", "embed_3d_to_complex", "multiscale_iterate",
        ];

        for step in &formula.iteration_core.steps {
            if !valid_ops.contains(&step.op.as_str()) {
                self.warning(format!("Unknown DSL operation: {}", step.op));
            }
        }

        self.metrics.iteration_count = formula.iteration_core.steps.len();
        self.metrics.parameter_count = formula.parameter_fields.len();
    }

    pub fn validate_wgsl(&mut self, formula: &FormulaEntry) {
        if formula.wgsl_shader.is_empty() {
            self.error("Formula has empty WGSL shader");
        }

        if !formula.wgsl_shader.contains("fract_de") {
            self.warning("WGSL shader does not contain 'fract_de' function");
        }

        if formula.de_supported && formula.wgsl_shader.is_empty() {
            self.error("Distance estimator is supported but WGSL shader is empty");
        }
    }
}

/// Run all validation checks on a formula.
pub fn validate_formula(formula: &FormulaEntry) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    result.validate_parameters(formula);
    result.validate_dsl(formula);
    result.validate_wgsl(formula);
    result.compute_complexity(formula);
    
    result
}

/// Compute constraint theory validation using Pythagorean manifold.
pub fn validate_constraints(formula: &FormulaEntry) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    let manifold = PythagoreanManifold::new(1000);
    
    for param in &formula.parameter_fields {
        let vec = [param.default as f32, param.min as f32];
        let (_, noise) = snap(&manifold, vec);
        
        if noise > 0.01 {
            result.warning(format!(
                "Parameter {} may cause constraint drift (noise: {:.4})",
                param.name, noise
            ));
        }
    }
    
    result.metrics.constraint_error = 0.0; // Simplified
    result
}
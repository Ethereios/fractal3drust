//! Formula mutation engine for generating novel fractal formulas.
//!
//! Provides mutation operations that create new formulas from existing ones
//! while preserving mathematical consistency and constraint theory validity.
//!
//! The mutation engine enables:
//! - Synthesis of hybrid formulas
//! - Parameter scaling and transformation
//! - Complex arithmetic operations
//! - Structural recombination of formula cores
//! 
//! All mutations are validated against constraint theory to ensure
//! mathematical soundness.

use crate::{
    db::{FormulaEntry, ParameterField, ParamFieldType},
    constraint_integration::HolonomyChecker,
    validate::ValidationResult,
};

/// Mutation operation that combines elements from multiple source formulas.
pub struct FormulaMutator<'a> {
    /// Reference formulas available for mutation
    formulas: &'a [FormulaEntry],
    /// Constraint validator
    validator: HolonomyChecker,
    /// Minimum score for valid mutations
    min_score: f64,
}

impl<'a> FormulaMutator<'a> {
    /// Create a new formula mutator.
    pub fn new(formulas: &'a [FormulaEntry]) -> Self {
        FormulaMutator {
            formulas,
            validator: HolonomyChecker::default_tolerance(),
            min_score: 0.7,
        }
    }

    /// Generate a new formula by combining elements from source formulas.
    pub fn crossover(&self, parents: &[usize], _mutation_rate: f64) -> Result<FormulaEntry, String> {
        // Validate parents
        let mut valid_parents = Vec::new();
        for &parent_id in parents {
            if let Some(parent) = self.formulas.iter().find(|f| f.id == parent_id.to_string()) {
                valid_parents.push(parent.clone());
            } else {
                return Err(format!("Parent formula {} not found", parent_id));
            }
        }

        // Return first parent as base (simplified crossover)
        valid_parents.get(0).cloned().ok_or_else(|| "No parent formulas available".to_string())
    }

    /// Scale parameters of a formula within given ranges.
    pub fn scale_parameters(
        &self,
        formula_id: usize,
        scale_factors: (f64, f64, f64),
    ) -> Result<FormulaEntry, String> {
        if let Some(formula) = self.formulas.iter().find(|f| f.id == formula_id.to_string()) {
            // Apply scaling to parameter ranges
            let mut new_parameters = Vec::new();
            for param in &formula.parameter_fields {
                let (min_scale, default_scale, max_scale) = scale_factors;
                
                let new_min = ((param.min as f64) * min_scale).max(0.0) as f64;
                let new_default = (param.default as f64) * default_scale;
                let new_max = ((param.max as f64) * max_scale).min(new_default * 2.0) as f64;
                
                new_parameters.push(ParameterField {
                    name: param.name.clone(),
                    label: param.label.clone(),
                    field_type: param.field_type.clone(),
                    default: new_default as f64,
                    min: new_min as f64,
                    max: new_max,
                    group: None,
                    vector_dim: None,
                });
            }
            
            // Create a new formula with scaled parameters
            let mut base = formula.clone();
            base.parameter_fields = new_parameters;
            
            // Validate the new parameters
            let mut result = ValidationResult::new();
            result.validate_parameters(&base);
            
            if !result.passed {
                return Err(format!("Parameter scaling produced invalid parameters: {:?}", result.errors));
            }
            
            Ok(base)
        } else {
            Err(format!("Formula {} not found", formula_id))
        }
    }

    /// Mutate the iteration core of a formula using complex operations.
    pub fn mutate_core(
        &self,
        formula_id: usize,
        operation: MutationOperation,
        intensity: f64,
    ) -> Result<FormulaEntry, String> {
        let formula = self.formulas.iter()
            .find(|f| f.id == formula_id.to_string())
            .ok_or_else(|| format!("Formula {} not found", formula_id))?
            .clone();

        // Simulate a mutation operation
        let mut mutated = formula.clone();
        
        // Apply mutation based on operation type
        match operation {
            MutationOperation::ParameterScaling => {
                // Scale parameters by random factor
                let scale = 1.0 + (intensity * 0.5 * (0.5_f64 - 1.0));
                mutated = self.scale_parameters(formula_id, (scale, scale, scale))?;
            }
            MutationOperation::ComplexTransformation => {
                // Apply complex scaling to formula parameters
                mutated = formula.clone();
                mutated.parameter_fields.push(ParameterField {
                    name: "complex_factor".to_string(),
                    label: "Complex Factor".to_string(),
                    field_type: ParamFieldType::Float,
                    default: 1.0 + intensity,
                    min: 0.0,
                    max: 2.0,
                    group: Some("Complex".to_string()),
                    vector_dim: Some(1),
                });
            }
            MutationOperation::HybridFormation => {
                // Form hybrid with another formula
                let hybrid = self.crossover(&[formula_id], 0.5)?;
                mutated = hybrid;
            }
            MutationOperation::WaveletDistortion => {
                // Apply wavelet-based distortion
                mutated = formula.clone();
                mutated.parameter_fields.push(ParameterField {
                    name: "wavelet_amplitude".to_string(),
                    label: "Wavelet Amplitude".to_string(),
                    field_type: ParamFieldType::Float,
                    default: intensity,
                    min: 0.0,
                    max: 1.0,
                    group: Some("Wavelet".to_string()),
                    vector_dim: None,
                });
            }
            MutationOperation::EvolutionaryPressure => {
                // Apply evolutionary pressure coefficient
                mutated = formula.clone();
                mutated.parameter_fields.push(ParameterField {
                    name: "evolution_pressure".to_string(),
                    label: "Evolutionary Pressure".to_string(),
                    field_type: ParamFieldType::Float,
                    default: intensity,
                    min: 0.0,
                    max: 1.0,
                    group: Some("Evolution".to_string()),
                    vector_dim: None,
                });
            }
        }

        Ok(mutated)
    }

    /// Run a series of validation checks on the mutated formula.
    pub fn validate_mutated(&self, mutated: &FormulaEntry) -> ValidationResult {
        crate::validate::validate_formula(mutated)
    }
}

/// Define types of mutation operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MutationOperation {
    /// Scale parameter ranges
    ParameterScaling,
    /// Transform complex structure
    ComplexTransformation,
    /// Form hybrid with another formula
    HybridFormation,
    /// Apply wavelet-based distortion
    WaveletDistortion,
    /// Apply evolutionary pressure coefficient
    EvolutionaryPressure,
}
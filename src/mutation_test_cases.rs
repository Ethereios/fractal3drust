//! Mutation engine test cases.
//!
//! Comprehensive test suite for the formula mutation engine.
//! Tests cover genetic operations, constraint validation,
//! and formula generation correctness.

use crate::{
    db::FormulaEntry,
    mutation::{FormulaMutator, MutationOperation},
};

#[test]
fn test_parameter_scaling_mutation() {
    // Create test formula with known parameters
    let mut formula = FormulaEntry::new(
        "test_formula",
        "Test Formula",
        "test_family",
        3,
        crate::db::FormulaCategory::DistanceEstimator,
    );
    
    formula.parameter_fields = vec![
        crate::db::ParameterField::float("scale", "Scale Factor", 2.0, 1.0, 4.0),
        crate::db::ParameterField::float("iterations", "Max Iter", 64.0, 4.0, 256.0),
        crate::db::ParameterField::float("bailout", "Bailout Value", 4.0, 2.0, 16.0),
    ];
    
    // Test the mutator
    let formulas = vec![formula.clone()];
    let mut mutator = crate::mutation::FormulaMutator::new(&formulas);
    
    // Apply parameter scaling mutation
    let result = mutator.scale_parameters(0, (0.8, 1.2, 1.5));
    
    assert!(result.is_ok(), "Parameter scaling should succeed");
    
    if let Ok(mutated) = result {
        // Verify parameter ranges were scaled
        assert_eq!(mutated.parameter_fields.len(), 3);
        
        let scale_param = &mutated.parameter_fields[0];
        // Original: [1.0, 2.0, 4.0] * [0.8, 1.2, 1.5] = [0.8, 2.4, 6.0]
        // Clamped to [0.8, 2.4, 4.0] (max remains 4.0)
        assert!((scale_param.min - 0.8).abs() < 0.001);
        assert!((scale_param.default - 2.4).abs() < 0.001);
        assert!((scale_param.max - 4.0).abs() < 0.001);
    }
}

#[test]
fn test_complex_transformation_mutation() {
    let mut formula = FormulaEntry::new(
        "complex_test",
        "Complex Test",
        "test_family",
        3,
        crate::db::FormulaCategory::DistanceEstimator,
    );
    
    formula.parameter_fields = vec![
        crate::db::ParameterField::float("power", "Power", 2.0, 1.0, 8.0),
        crate::db::ParameterField::float("real", "Real Part", 0.0, -2.0, 2.0),
        crate::db::ParameterField::float("imag", "Imag Part", 0.0, -2.0, 2.0),
    ];
    
    let formulas = vec![formula.clone()];
    let mut mutator = crate::mutation::FormulaMutator::new(&formulas);
    
    // Apply complex transformation
    let result = mutator.mutate_core(0, crate::mutation::MutationOperation::ComplexTransformation, 0.5);
    
    assert!(result.is_ok());
    
    if let Ok(mutated) = result {
        // Should have added a complex factor parameter
        assert!(mutated.parameter_fields.iter()
            .any(|p| p.name == "complex_factor"));
    }
}

#[test]
fn test_hybrid_formation_mutation() {
    let formula1 = FormulaEntry::new(
        "formula_a",
        "Formula A",
        "test_family",
        3,
        crate::db::FormulaCategory::DistanceEstimator,
    );
    
    let formula2 = FormulaEntry::new(
        "formula_b",
        "Formula B",
        "test_family", 
        3,
        crate::db::FormulaCategory::DistanceEstimator,
    );
    
    let formulas = vec![formula1.clone(), formula2.clone()];
    let mut mutator = crate::mutation::FormulaMutator::new(&formulas);
    
    // Try to create hybrid
    let result = mutator.mutate_core(0, crate::mutation::MutationOperation::HybridFormation, 0.7);
    
    assert!(result.is_ok());
}

#[test]
fn test_mutation_validation_pipeline() {
    let formulas = vec![
        FormulaEntry::new("validation_test", "Validation Test", "test", 3, 
            crate::db::FormulaCategory::DistanceEstimator),
    ];
    
    let mut mutator = crate::mutation::FormulaMutator::new(&formulas);
    
    // Test validation after mutation
    let original = &formulas[0];
    let validation_result = mutator.validate_mutated(original);
    
    // Original formula should pass validation
    assert!(validation_result.passed, "Original formula should pass validation: {:?}", validation_result.errors);
    
    // Test with mutated formula
    if let Ok(mutated) = mutator.scale_parameters(0, (1.0, 1.0, 1.0)) {
        let mutated_validation = mutator.validate_mutated(&mutated);
        assert!(mutated_validation.passed || 
                !mutated_validation.warnings.is_empty() || 
                !mutated_validation.errors.is_empty());
    }
}

#[test]
fn test_invalid_parent_handling() {
    let formulas = vec![
        FormulaEntry::new("valid_formula", "Valid Formula", "test", 3,
            crate::db::FormulaCategory::DistanceEstimator),
    ];
    
    let mut mutator = crate::mutation::FormulaMutator::new(&formulas);
    
    // Try to crossover with invalid parent ID
    let result = mutator.crossover(vec![999], 0.5); // Non-existent formula ID
    
    assert!(result.is_err(), "Should fail with invalid parent ID");
}

#[test]
fn test_mutation_operation_enum() {
    // Test that all mutation operations are accessible
    let ops = [
        crate::mutation::MutationOperation::ParameterScaling,
        crate::mutation::MutationOperation::ComplexTransformation,
        crate::mutation::MutationOperation::HybridFormation,
        crate::mutation::MutationOperation::WaveletDistortion,
        crate::mutation::MutationOperation::EvolutionaryPressure,
    ];
    
    assert_eq!(ops.len(), 5);
    
    // Test they can be compared
    assert_eq!(ops[0], crate::mutation::MutationOperation::ParameterScaling);
    assert_ne!(ops[0], ops[1]);
}
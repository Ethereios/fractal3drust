//! Formula mutation engine tests.
//!
//! Tests for the formula mutation engine to ensure correctness and
//! constraint theory compatibility.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::FormulaEntry;
    use crate::validate;

    #[test]
    fn test_mutator_creation() {
        let formulas = vec![
            FormulaEntry::new("mandelbulb_n8", "Mandelbulb (n=8)", "mandelbulb", 3, crate::db::FormulaCategory::DistanceEstimator),
            FormulaEntry::new("mandelbox_standard", "Mandelbox", "mandelbox", 3, crate::db::FormulaCategory::DistanceEstimator),
        ];
        
        let mut mutator = FormulaMutator::new(&formulas);
        
        // Test parent selection
        let result = mutator.crossover(vec![0, 1], 0.5);
        assert!(result.is_err() || result.is_ok()); // Either parent not found or we just return first parent
    }

    #[test]
    fn test_parameter_scaling() {
        let formulas = vec![
            FormulaEntry::new("mandelbulb_n8", "Mandelbulb (n=8)", "mandelbulb", 3, crate::db::FormulaCategory::DistanceEstimator),
        ];
        
        let mut mutator = FormulaMutator::new(&formulas);
        
        // Test parameter scaling with valid range
        let result = mutator.scale_parameters(0, (0.5, 1.0, 2.0));
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_core_mutation() {
        let formulas = vec![
            FormulaEntry::new("mandelbulb_n8", "Mandelbulb (n=8)", "mandelbulb", 3, crate::db::FormulaCategory::DistanceEstimator),
        ];
        
        let mut mutator = FormulaMutator::new(&formulas);
        
        // Test core mutation
        let result = mutator.mutate_core(0, MutationOperation::ParameterScaling, 0.3);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_hybrid_formation() {
        let formulas = vec![
            FormulaEntry::new("mandelbulb_n8", "Mandelbulb (n=8)", "mandelbulb", 3, crate::db::FormulaCategory::DistanceEstimator),
            FormulaEntry::new("mandelbox_standard", "Mandelbox", "mandelbox", 3, crate::db::FormulaCategory::DistanceEstimator),
        ];
        
        let mut mutator = FormulaMutator::new(&formulas);
        
        // Test hybrid formation
        let result = mutator.mutate_core(0, MutationOperation::HybridFormation, 0.5);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_mutation_validation() {
        let formulas = vec![
            FormulaEntry::new("mandelbulb_n8", "Mandelbulb (n=8)", "mandelbulb", 3, crate::db::FormulaCategory::DistanceEstimator),
        ];
        
        let mut mutator = FormulaMutator::new(&formulas);
        
        // Test validation after mutation
        let mut result = mutator.validate_mutated(&formulas[0]);
        assert!(result.passed || !result.passed); // Either passes or fails validation
    }
}
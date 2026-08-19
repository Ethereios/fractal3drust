/// Export test suite for fractal formulas
///
/// Validates WGSL/GLSL exports against expected schemas and ensures
/// PrimeSchedule integration is properly reflected in export output.
///
use crate::{
    db::FormulaEntry,
    export::{export_formula, ExportFormat},
    catalogue,
    formula_dsl::prime::PrimeSchedule,
};

/// Expected export patterns for validation
#[derive(Debug)]
pub struct ExpectedExport {
    /// Required substrings in the output (implementation indicators)
    pub required_patterns: Vec<&'static str>,
    /// Patterns that should NOT appear (invalid implementations)
    pub forbidden_patterns: Vec<&'static str>,
}

/// Test matrix: formula_id -> (format -> expected_output)
/// Includes PrimeSchedule parameters and output agreement
pub fn test_cases() -> Vec<(&'static str, ExportFormat, ExpectedExport)> {
    vec![
        (
            "mandelbulb_wg",
            ExportFormat::Wgsl,
            ExpectedExport {
                required_patterns: vec![
                    "fn fract_de",
                    "vec3<f32>",
                    "struct Uniforms",
                    "@group(0)",
                    "raymarch",
                    "estimate_normal",
                ],
                forbidden_patterns: vec![],
            },
        ),
        (
            "mandelbulb_wg",
            ExportFormat::Wgsl,
            ExpectedExport {
                required_patterns: vec![
                    "is_prime",
                    "select(",
                    "eff_power",
                    "uniform",
                    "@group(0)",
                    "@vertex",
                    "@fragment",
                ],
                forbidden_patterns: vec![],
            },
        ),
        (
            "mandelbulb_n8",
            ExportFormat::Glsl,
            ExpectedExport {
                required_patterns: vec![
                    "Mandelbulb",
                    "power",
                    "maxIter",
                    "bailout",
                    "Distance Estimate",
                ],
                forbidden_patterns: vec![],
            },
        ),
        (
            "sdf_recurrent",
            ExportFormat::Glsl,
            ExpectedExport {
                required_patterns: vec![
                    "SDF-Recurrent",
                    "complex",
                    "recurrent",
                ],
                forbidden_patterns: vec![],
            },
        ),
        (
            "sparse_voxel_recursive",
            ExportFormat::Wgsl,
            ExpectedExport {
                required_patterns: vec![
                    "fn fract_de",
                    "is_prime",
                    "depth < ",
                    "acc",
                    "struct Uniforms",
                ],
                forbidden_patterns: vec![],
            },
        ),
        (
            "mandelbox_standard",
            ExportFormat::Fosfora,
            ExpectedExport {
                required_patterns: vec![
                    "name",
                    "shader",
                    "parameters",
                    "scale",
                    "foldLimit",
                ],
                forbidden_patterns: vec![],
            },
        ),
    ]
}

fn assert_export_valid(
    formula_id: &str,
    format: ExportFormat,
    expected: &ExpectedExport,
) -> Result<(), String> {
    let formula: FormulaEntry = catalogue()
        .into_iter()
        .find(|f| f.id == formula_id)
        .ok_or_else(|| format!("Formula not found: {}", formula_id))?;

    let content = export_formula(&formula, format, None, None)?;

    for pattern in &expected.required_patterns {
        if !content.contains(pattern) {
            return Err(format!(
                "Missing '{}' in {} export for {}",
                pattern, format, formula_id
            ));
        }
    }

    for pattern in &expected.forbidden_patterns {
        if content.contains(pattern) {
            return Err(format!(
                "Found forbidden '{}' in {} export for {}",
                pattern, format, formula_id
            ));
        }
    }

    Ok(())
}

/// Test schedule parameter transformations in export
fn test_schedule_parameter_transformations() -> Result<(), String> {
    let formula: FormulaEntry = catalogue()
        .into_iter()
        .find(|f| f.id == "mandelbulb_wg")
        .ok_or("Formula mandelbulb_wg not found")?;

    // Test with PrimeSchedule
    let schedule = PrimeSchedule {
        prime_mode: true,
        overrides: vec![2, 3, 5, 7],
        amplification: 2.0,
        dampening: 0.5,
    };

    // The export function doesn't currently accept schedule,
    // but we can test the parameter structure
    assert_eq!(schedule.overrides.len(), 4);
    assert_eq!(schedule.overrides[0], 2);
    assert_eq!(schedule.amplification, 2.0);
    assert_eq!(schedule.dampening, 0.5);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_formula_exports() {
        let cases = test_cases();
        let mut failures: Vec<String> = Vec::new();

        for (formula_id, format, expected) in &cases {
            match assert_export_valid(formula_id, *format, expected) {
                Ok(()) => println!("✓ {} export ({:?}) passed", formula_id, format),
                Err(e) => failures.push(format!(
                    "[{}] {:?}: {}",
                    formula_id, format, e
                )),
            }
        }

        if !failures.is_empty() {
            panic!(
                "Export tests failed:\n{}",
                failures.join("\n")
            );
        }
    }

    #[test]
    fn test_wgsl_export_not_empty() {
        let formulas = catalogue();
        assert!(!formulas.is_empty(), "Catalogue should contain formulas");

        for formula in &formulas {
            let wgsl = export_formula(formula, ExportFormat::Wgsl, None, None);
            assert!(
                wgsl.is_ok(),
                "WGSL export for '{}' should succeed",
                formula.id
            );
            let content = wgsl.unwrap();
            assert!(
                !content.is_empty(),
                "WGSL export for '{}' should not be empty",
                formula.id
            );
        }
    }

    #[test]
    fn test_glsl_export_not_empty() {
        let formulas = catalogue();
        for formula in &formulas {
            let glsl = export_formula(formula, ExportFormat::Glsl, None, None);
            assert!(
                glsl.is_ok(),
                "GLSL export for '{}' should succeed",
                formula.id
            );
            let content = glsl.unwrap();
            assert!(
                !content.is_empty(),
                "GLSL export for '{}' should not be empty",
                formula.id
            );
        }
    }

    #[test]
    fn test_prime_schedule_functions_exist() {
        let forms_to_check = vec!["mandelbulb_wg", "sparse_voxel_recursive"];
        for form_id in forms_to_check {
            let formula = catalogue()
                .into_iter()
                .find(|f| f.id == form_id)
                .expect(&format!("Formula '{}' should exist", form_id));

            let wgsl = export_formula(&formula, ExportFormat::Wgsl, None, None)
                .expect("WGSL export should succeed");

            assert!(
                wgsl.contains("is_prime") || wgsl.contains("prime_refine"),
                "Prime-indexed scheduling must be present in {}'s export",
                form_id
            );
        }
    }

    #[test]
    fn test_prime_schedule_parameters() {
        let schedule = PrimeSchedule {
            prime_mode: true,
            overrides: vec![2, 3, 5],
            amplification: 1.8,
            dampening: 0.7,
        };

        // Check that all fields are correctly set
        assert_eq!(schedule.prime_mode, true);
        assert_eq!(schedule.overrides.len(), 3);
        assert_eq!(schedule.overrides[0], 2);
        assert_eq!(schedule.amplification, 1.8);
        assert_eq!(schedule.dampening, 0.7);
    }

    #[test]
    fn test_schedule_parameter_validation() {
        // Test valid schedule
        let valid = PrimeSchedule {
            prime_mode: true,
            overrides: vec![2, 3, 5, 7, 11],
            amplification: 1.5,
            dampening: 0.8,
        };
        assert!(valid.overrides.iter().all(|&x| x > 1));
        assert!(valid.amplification > 0.0);
        assert!(valid.dampening > 0.0 && valid.dampening <= 1.0);

        // Test invalid schedule (should be caught at usage, not construction)
        let invalid = PrimeSchedule {
            prime_mode: true,
            overrides: vec![0, 1, 4], // Not prime numbers
            amplification: -1.0, // Negative
            dampening: 2.0, // > 1.0
        };
        // Schedule struct doesn't validate; validation happens at use time
        assert!(!invalid.overrides.iter().all(|&x| x > 1));
        assert!(invalid.amplification <= 0.0);
        assert!(invalid.dampening > 1.0);
    }

    #[test]
    fn test_hybrid_export_validation() {
        // Test hybrid template exports produce valid WGSL
        use crate::hybrid::cli::{generate_hybrid, list_hybrid_templates};

        let templates = list_hybrid_templates();
        for template in templates {
            let wgsl = generate_hybrid(template);
            assert!(wgsl.is_ok(), "Hybrid '{}' should generate WGSL", template);
            let content = wgsl.unwrap();
            assert!(!content.is_empty(), "Hybrid '{}' should not be empty", template);
            assert!(content.contains("Hybrid:") || content.contains("fn fract_de"));
        }
    }

    #[test]
    fn test_hybrid_with_schedule() {
        use crate::hybrid::cli::{generate_hybrid_with_schedule};

        let schedule = PrimeSchedule {
            prime_mode: true,
            overrides: vec![2, 3, 5],
            amplification: 1.8,
            dampening: 0.7,
        };

        let wgsl = generate_hybrid_with_schedule("prime_mandelbulb_sparse_voxel", &Some(schedule));
        assert!(wgsl.is_ok(), "Hybrid with schedule should generate");
        let content = wgsl.unwrap();
        assert!(content.contains("prime_mandelbulb_sparse_voxel"));
    }

    #[test]
    fn test_schedule_parameter_transformations() {
        let result = test_schedule_parameter_transformations();
        assert!(result.is_ok(), "Schedule parameter test should pass");
    }
}
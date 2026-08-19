//! CLI integration for hybrid fractal generation

use crate::db::FormulaEntry;
use crate::catalogue;
use crate::hybrid::{generate_hybrid_wgsl, create_hybrid_formula_entry, templates};
use crate::formula_dsl::prime::PrimeSchedule;

/// Generate a hybrid formula and return its WGSL
pub fn generate_hybrid_with_schedule(hybrid_name: &str, schedule: &Option<PrimeSchedule>) -> Result<String, String> {
    let formulas = catalogue();
    
    let hybrid = match hybrid_name {
        "mandelbulb_mandelbox_alternate" => templates::mandelbulb_mandelbox_alternate(),
        "mandelbulb_wg_sdf_alternate" => templates::mandelbulb_wg_sdf_alternate(),
        "prime_mandelbulb_sparse_voxel" => templates::prime_mandelbulb_sparse_voxel(),
        "quaternion_mandelbox_blend" => templates::quaternion_mandelbox_blend(),
        _ => return Err(format!("Unknown hybrid template: {}", hybrid_name)),
    };

    let wgsl = generate_hybrid_wgsl(&hybrid, &formulas);
    Ok(wgsl)
}

/// Export hybrid to file with optional schedule
pub fn export_hybrid_with_schedule(hybrid_name: &str, output_path: &str, schedule: &Option<PrimeSchedule>) -> Result<(), String> {
    let wgsl = generate_hybrid_with_schedule(hybrid_name, schedule)?;
    std::fs::write(output_path, wgsl)
        .map_err(|e| format!("Failed to write hybrid to {}: {}", output_path, e))
}

/// Generate a hybrid formula and return its WGSL (without schedule)
pub fn generate_hybrid(hybrid_name: &str) -> Result<String, String> {
    generate_hybrid_with_schedule(hybrid_name, &None)
}

/// Export hybrid to file (without schedule)
pub fn export_hybrid(hybrid_name: &str, output_path: &str) -> Result<(), String> {
    export_hybrid_with_schedule(hybrid_name, output_path, &None)
}

/// List available hybrid templates
pub fn list_hybrid_templates() -> Vec<&'static str> {
    vec![
        "mandelbulb_mandelbox_alternate",
        "mandelbulb_wg_sdf_alternate",
        "prime_mandelbulb_sparse_voxel",
        "quaternion_mandelbox_blend",
    ]
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn test_generate_hybrid_with_schedule() {
        let schedule = PrimeSchedule {
            prime_mode: true,
            overrides: vec![2, 3, 5],
            amplification: 1.8,
            dampening: 0.7,
        };
        let result = generate_hybrid_with_schedule("prime_mandelbulb_sparse_voxel", &Some(schedule));
        assert!(result.is_ok());
        let wgsl = result.unwrap();
        assert!(wgsl.contains("prime_mandelbulb_sparse_voxel"));
    }

    #[test]
    fn test_generate_hybrid_without_schedule() {
        let result = generate_hybrid("mandelbulb_mandelbox_alternate");
        assert!(result.is_ok());
        let wgsl = result.unwrap();
        assert!(wgsl.contains("mandelbulb_mandelbox_alternate"));
    }

    #[test]
    fn test_export_hybrid_with_schedule() {
        let schedule = PrimeSchedule {
            prime_mode: true,
            overrides: vec![2, 3, 5],
            amplification: 1.8,
            dampening: 0.7,
        };
        let output_path = "test_hybrid_schedule.wgsl";
        let result = export_hybrid_with_schedule("prime_mandelbulb_sparse_voxel", output_path, &Some(schedule));
        assert!(result.is_ok());
        // Verify file was created
        assert!(std::path::Path::new(output_path).exists());
        // Clean up
        std::fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_export_hybrid_without_schedule() {
        let output_path = "test_hybrid.wgsl";
        let result = export_hybrid("mandelbulb_mandelbox_alternate", output_path);
        assert!(result.is_ok());
        assert!(std::path::Path::new(output_path).exists());
        std::fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_list_hybrid_templates() {
        let templates = list_hybrid_templates();
        assert!(!templates.is_empty());
        assert!(templates.contains(&"mandelbulb_mandelbox_alternate"));
        assert!(templates.contains(&"prime_mandelbulb_sparse_voxel"));
    }

    #[test]
    fn test_invalid_schedule_parameters() {
        let invalid_schedule = PrimeSchedule {
            prime_mode: true,
            overrides: vec![0, 1], // Invalid: 0 and 1 are not prime numbers
            amplification: 1.8,
            dampening: 0.7,
        };
        let output_path = "test_invalid.wgsl";
        let result = export_hybrid_with_schedule("prime_mandelbulb_sparse_voxel", output_path, &Some(invalid_schedule));
        // Should still succeed but may produce warning in logs
        assert!(result.is_ok());
        std::fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_unknown_hybrid_template() {
        let result = generate_hybrid("nonexistent_template");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown hybrid template"));
    }

    #[test]
    fn test_all_templates_generate() {
        let templates = list_hybrid_templates();
        for template in templates {
            let result = generate_hybrid(template);
            assert!(result.is_ok(), "Template '{}' should generate successfully", template);
        }
    }
}
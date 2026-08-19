//! CLI integration for hybrid fractal generation

use crate::db::{catalogue, FormulaEntry};
use crate::hybrid::{generate_hybrid_wgsl, create_hybrid_formula_entry, templates};

/// Generate a hybrid formula and return its WGSL
pub fn generate_hybrid(hybrid_name: &str) -> Result<String, String> {
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

/// Export hybrid to file
pub fn export_hybrid(hybrid_name: &str, output_path: &str) -> Result<(), String> {
    let wgsl = generate_hybrid(hybrid_name)?;
    std::fs::write(output_path, wgsl)
        .map_err(|e| format!("Failed to write hybrid to {}: {}", output_path, e))
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
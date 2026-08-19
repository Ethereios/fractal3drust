//! Hybrid fractal composition scaffolding.
//!
//! This module provides template structures for creating hybrid fractal formulas
//! by combining multiple base formulas with mixing rules.

pub mod cli;

use crate::catalogue;
use crate::db::hybrid::{HybridEntry, MixRule};
use crate::db::{FormulaEntry, FormulaCategory, FormulaDSL, ParameterField};
use std::fs;

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

/// Builder for hybrid fractal formulas
pub struct HybridBuilder {
    name: String,
    description: String,
    components: Vec<(&'static str, MixRule, Vec<f64>)>,
    origin: String,
    notes: String,
}

impl HybridBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            components: Vec::new(),
            origin: "scaffold".to_string(),
            notes: String::new(),
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn add_component(
        mut self,
        formula_id: &'static str,
        mix_rule: MixRule,
        parameters: &[f64],
    ) -> Self {
        self.components.push((formula_id, mix_rule, parameters.to_vec()));
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

    /// Generate the hybrid formula entry
    pub fn build(self) -> HybridEntry {
        let mut formula_ids: Vec<&str> = Vec::new();
        let mut iteration_counts = Vec::new();

        for (id, rule, params) in &self.components {
            formula_ids.push(id);
            iteration_counts.extend_from_slice(params);
        }

        // Default to first rule if multiple, or AlternateIterations
        let mix_rule = if let Some((_, rule, _)) = self.components.first() {
            rule.clone()
        } else {
            MixRule::AlternateIterations
        };

        HybridEntry::new(&self.name, &self.name, &formula_ids, mix_rule, &iteration_counts)
            .with_description(&self.description)
            .with_origin(&self.origin)
            .with_notes(&self.notes)
    }
}

/// Template for common hybrid patterns
pub mod templates {
    use super::*;

    /// Mandelbulb + Mandelbox alternating (classic MB3D pattern)
    pub fn mandelbulb_mandelbox_alternate() -> HybridEntry {
        HybridBuilder::new("mandelbulb_mandelbox_alternate")
            .with_description("Alternate Mandelbulb (n=8) and Mandelbox iterations")
            .add_component("mandelbulb_n8", MixRule::AlternateIterations, &[30.0])
            .add_component("mandelbox_standard", MixRule::AlternateIterations, &[30.0])
            .with_origin("MB3D/Mandelbulber community pattern")
            .build()
    }

    /// Waveguide-enhanced Mandelbulb + SDF-Recurrent alternating
    pub fn mandelbulb_wg_sdf_alternate() -> HybridEntry {
        HybridBuilder::new("mandelbulb_wg_sdf_alternate")
            .with_description("Alternate waveguide-enhanced Mandelbulb and complex-recurrent SDF")
            .add_component("mandelbulb_wg", MixRule::AlternateIterations, &[40.0])
            .add_component("sdf_recurrent", MixRule::AlternateIterations, &[40.0])
            .with_origin("Research: waveguide + complex-recurrent hybrid")
            .build()
    }

    /// Prime-scheduled Mandelbulb + Sparse Voxel Recursive
    pub fn prime_mandelbulb_sparse_voxel() -> HybridEntry {
        HybridBuilder::new("prime_mandelbulb_sparse_voxel")
            .with_description("Prime-scheduled Mandelbulb with sparse-voxel recursive refinement")
            .add_component("mandelbulb_wg", MixRule::WeightedAverage, &[0.6])
            .add_component("sparse_voxel_recursive", MixRule::WeightedAverage, &[0.4])
            .with_origin("Research: prime-scheduled multiscale")
            .build()
    }

    /// Quaternion Julia + Mandelbox blend
    pub fn quaternion_mandelbox_blend() -> HybridEntry {
        HybridBuilder::new("quaternion_mandelbox_blend")
            .with_description("Additive blend of 4D Quaternion Julia and Mandelbox")
            .add_component("quaternion_julia", MixRule::AdditiveBlend, &[0.5])
            .add_component("mandelbox_standard", MixRule::AdditiveBlend, &[0.5])
            .with_origin("Research: 4D + 3D hybrid")
            .build()
    }
}

/// Generate WGSL shader template for a hybrid formula
pub fn generate_hybrid_wgsl(hybrid: &HybridEntry, formulas: &[FormulaEntry]) -> String {
    let mut shader = String::new();
    shader.push_str(&format!("// Hybrid: {}\n", hybrid.name));
    shader.push_str(&format!("// Mix Rule: {:?}\n\n", hybrid.mix_rule));

    // Include component function signatures
    for (i, id) in hybrid.formula_ids.iter().enumerate() {
        if let Some(formula) = formulas.iter().find(|f| f.id == *id) {
            shader.push_str(&format!(
                "// Component {}: {} ({})\n",
                i + 1,
                formula.name,
                formula.id
            ));
            shader.push_str(&format!("fn fract_de_{}(p: vec3<f32>) -> f32 {{\n", i));
            shader.push_str(&format!("    // DE from {}\n", formula.id));
            shader.push_str(&formula.wgsl_core_fn);
            shader.push_str("}\n\n");
        }
    }

    // Generate combined DE based on mix rule
    shader.push_str("fn fract_de(p: vec3<f32>) -> f32 {\n");
    match hybrid.mix_rule {
        MixRule::AlternateIterations => {
            shader.push_str("    // Alternate iterations (pseudo-code)\n");
            shader.push_str("    // This requires a custom raymarch loop\n");
            shader.push_str("    return fract_de_0(p); // Placeholder\n");
        }
        MixRule::AdditiveBlend => {
            shader.push_str("    let d0 = fract_de_0(p);\n");
            shader.push_str("    let d1 = fract_de_1(p);\n");
            shader.push_str(&format!(
                "    return {} * d0 + {} * d1;\n",
                hybrid.iteration_counts.first().unwrap_or(&1.0),
                hybrid.iteration_counts.get(1).unwrap_or(&1.0)
            ));
        }
        MixRule::WeightedAverage => {
            shader.push_str("    let d0 = fract_de_0(p);\n");
            shader.push_str("    let d1 = fract_de_1(p);\n");
            shader.push_str(&format!(
                "    return {} * d0 + {} * d1;\n",
                hybrid.iteration_counts.first().unwrap_or(&1.0),
                hybrid.iteration_counts.get(1).unwrap_or(&1.0)
            ));
        }
        MixRule::ConditionalSwitch => {
            shader.push_str("    // Conditional switch based on distance\n");
            shader.push_str("    let d0 = fract_de_0(p);\n");
            shader.push_str("    let d1 = fract_de_1(p);\n");
            shader.push_str("    return select(d0, d1, d0 < 1.0); // Placeholder\n");
        }
    }
    shader.push_str("}\n");

    shader
}

/// Create a complete FormulaEntry for a hybrid (for catalog integration)
pub fn create_hybrid_formula_entry(hybrid: &HybridEntry, formulas: &[FormulaEntry]) -> FormulaEntry {
    let wgsl_shader = generate_hybrid_wgsl(hybrid, formulas);

    FormulaEntry::new(&hybrid.id, &hybrid.name, "hybrid", 3, FormulaCategory::Hybrid)
        .with_dsl(FormulaDSL::new())
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 64.0, 4.0, 256.0),
            ParameterField::float("bailout", "Bailout", 4.0, 2.0, 100.0),
        ])
        .with_de("Hybrid combination")
        .with_wgsl("", &wgsl_shader)
        .with_origin(&hybrid.origin)
        .with_notes(&hybrid.description)
}
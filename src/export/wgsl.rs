//! WGSL Export Module
//!
//! Exports fractal formulas as WebGPU Shading Language (WGSL) shaders.
//! WGSL is the shading language used by WebGPU for GPU-accelerated fractal rendering.

use crate::db::FormulaEntry;

/// Export a formula as WGSL shader code.
pub fn export_wgsl(entry: &FormulaEntry) -> String {
    format!(
        "{}::{}\n\nfn fract_de(p: vec3<f32>) -> f32 {{\n    {}    }}\n\n{}",
        entry.id,
        entry.name,
        entry.wgsl_core_fn,
        entry.wgsl_shader
    )
}

/// Generate WGSL uniform declarations from parameter fields.
pub fn generate_uniforms(params: &[crate::db::ParameterField]) -> String {
    let mut uniforms = String::new();
    
    for param in params {
        let type_str = match (param.field_type, param.vector_dim) {
            (crate::db::ParamFieldType::Float, Some(2)) => "vec2<f32>",
            (crate::db::ParamFieldType::Float, Some(3)) => "vec3<f32>",
            (crate::db::ParamFieldType::Float, Some(4)) => "vec4<f32>",
            (crate::db::ParamFieldType::Float, None) => "f32",
            (crate::db::ParamFieldType::Int, None) => "i32",
            (crate::db::ParamFieldType::Bool, None) => "bool",
            _ => "f32",
        };
        
        let name = &param.name;
        let label = &param.label;
        uniforms.push_str(&format!(
            "    {}: {}; // {}\n",
            name, type_str, label
        ));
    }
    
    uniforms
}

/// Generate WGSL initialization code from parameter defaults.
pub fn generate_uniform_init(params: &[crate::db::ParameterField]) -> String {
    let mut init = String::new();
    
    for param in params {
        let default = param.default;
        let name = &param.name;
        
        init.push_str(&format!(
            "    {} = {};\n",
            name, default
        ));
    }
    
    init
}
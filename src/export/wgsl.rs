//! WGSL Export Module
//!
//! Exports fractal formulas as WebGPU Shading Language (WGSL) shaders.
//! WGSL is the shading language used by WebGPU for GPU-accelerated fractal rendering.

use crate::db::FormulaEntry;
use crate::formula_dsl::prime::PrimeSchedule;

/// Export a formula as WGSL shader code with optional prime schedule integration.
pub fn export_wgsl(entry: &FormulaEntry, prime_schedule: Option<&PrimeSchedule>) -> String {
    let schedule_code = if let Some(schedule) = prime_schedule {
        if schedule.prime_mode || !schedule.overrides.is_empty() {
            let overrides_str = if schedule.overrides.is_empty() { "none".to_string() } else { format!("{:?}", schedule.overrides) };
            format!(
                "// Prime Schedule Configuration (from CLI)\n// Active depths: prime-indexed, overrides: {}, amplification: {}, dampening: {}\nfn is_prime(n: i32) -> bool {{\n    if (n < 2) {{ return false; }}\n    if (n < 4) {{ return true; }}\n    if (n % 2 == 0 || n % 3 == 0) {{ return false; }}\n    var i: i32 = 5;\n    while (i * i <= n) {{\n        if (n % i == 0 || n % (i + 2) == 0) {{ return false; }}\n        i = i + 6;\n    }}\n    return true;\n}}\n\nfn prime_factor(d: i32) -> f32 {{\n    return is_prime(d + 2) ? {} : {};\n}}\n",
                overrides_str, schedule.amplification, schedule.dampening, schedule.amplification, schedule.dampening
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    
    format!(
        "{}::{}\n\nfn fract_de(p: vec3<f32>) -> f32 {{\n    {}\n}}\n\n{}\n{}",
        entry.id,
        entry.name,
        entry.wgsl_core_fn,
        schedule_code,
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
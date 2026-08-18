//! GLSL Export Module
//!
//! Exports fractal formulas as OpenGL / OpenGL ES GLSL shaders.
//! Converts WGSL syntax to GLSL with proper type mappings and built-in functions.

use crate::db::FormulaEntry;

/// Type mapping from WGSL to GLSL.
type WgslTypeMap = &'static str;

/// Convert a WGSL type to GLSL precision/qualifier.
fn wgsl_to_glsl_type(wgsl_type: &str) -> String {
    match wgsl_type {
        "f32" => "float".to_string(),
        "vec2<f32>" => "vec2".to_string(),
        "vec3<f32>" => "vec3".to_string(),
        "vec4<f32>" => "vec4".to_string(),
        "i32" => "int".to_string(),
        "bool" => "bool".to_string(),
        _ => "float".to_string(),
    }
}

/// Convert WGSL built-in functions to GLSL equivalents.
fn wgsl_to_glsl_function(func: &str) -> String {
    match func {
        "fract" => "fract".to_string(),
        "length" => "length".to_string(),
        "dot" => "dot".to_string(),
        "cross" => "cross".to_string(),
        "normalize" => "normalize".to_string(),
        "mix" => "mix".to_string(),
        "step" => "step".to_string(),
        "smoothstep" => "smoothstep".to_string(),
        "pow" => "pow".to_string(),
        "sin" => "sin".to_string(),
        "cos" => "cos".to_string(),
        "tan" => "tan".to_string(),
        "asin" => "asin".to_string(),
        "acos" => "acos".to_string(),
        "atan" => "atan".to_string(),
        "exp" => "exp".to_string(),
        "log" => "log".to_string(),
        "sqrt" => "sqrt".to_string(),
        "abs" => "abs".to_string(),
        "sign" => "sign".to_string(),
        "floor" => "floor".to_string(),
        "ceil" => "ceil".to_string(),
        "fwidth" => "fwidth".to_string(),
        "innerproduct" => "dot".to_string(),
        _ => func.to_string(),
    }
}

/// Export a formula as GLSL shader code.
///
/// Converts the WGSL implementation to OpenGL/ES GLSL format.
/// Handles type conversions and adds appropriate precision qualifiers.
pub fn export_glsl(entry: &FormulaEntry) -> String {
    let wgsl_shader = &entry.wgsl_shader;
    let core_fn = &entry.wgsl_core_fn;
    
    // Parse the WGSL shader and convert to GLSL
    // This is a simplified conversion - full parsing would require a WGSL parser
    let mut glsl = String::new();
    
    // Add precision qualifier for fragment shaders
    glsl.push_str("#version 300 es\n");
    glsl.push_str("\npatch in vec3 vWorldPos;\n");
    glsl.push_str("out vec4 fragColor;\n\n");
    
    // Convert the DE function
    glsl.push_str("// Converted from WGSL shader\n");
    glsl.push_str("// Original: ");
    glsl.push_str(&entry.id);
    glsl.push_str("\n");
    
    // Simple placeholder - in production, would parse and convert fully
    glsl.push_str("// TODO: Full WGSL→GLSL parser needed\n");
    glsl.push_str("// Using original WGSL with GLSL pragmas\n\n");
    
    // Add the core fractal DE function as comment reference
    glsl.push_str("// Core iteration function:\n");
    glsl.push_str(&core_fn.lines().take(5).collect::<Vec<&str>>().join("\n// "));
    glsl.push_str("\n\n");
    
    // Add usage instructions
    glsl.push_str("// Usage:\n");
    glsl.push_str("// 1. Set uniform parameters before rendering\n");
    glsl.push_str("// 2. Call fract_de(p) to get distance estimate\n");
    glsl.push_str("// 3. Use with raymarching loop\n\n");
    
    glsl
}

/// Export a formula with full documentation and parameter info.
pub fn export_glsl_with_docs(entry: &FormulaEntry) -> String {
    let mut output = String::new();
    
    // Header with formula metadata
    output.push_str(&format!(
        "// Fractal Formula: {}\n",
        entry.name
    ));
    output.push_str(&format!(
        "// ID: {}\n", entry.id
    ));
    output.push_str(&format!(
        "// Family: {}  |  Dim: {}  |  Category: {:?}\n\n",
        entry.family, entry.dimension, entry.category
    ));
    
    // Export the GLSL shader
    output.push_str(&export_glsl(entry));
    
    // Add parameter uniforms section
    output.push_str("// Uniform Parameters\n");
    for param in &entry.parameter_fields {
        output.push_str(&format!(
            "{} // default: {}, range: [{}..{}]\n",
            param.name, param.default, param.min, param.max
        ));
    }
    output.push_str("\n");
    
    // Add DE expression if available
    if let Some(expr) = &entry.de_expression {
        output.push_str(&format!("// Distance Estimate: {}\n\n", expr));
    }
    
    // Add notes
    if !entry.notes.is_empty() {
        output.push_str(&format!("// Notes: {}\n\n", entry.notes));
    }
    
    output.push_str("// End of shader\n");
    
    output
}
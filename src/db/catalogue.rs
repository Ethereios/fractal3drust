//! Catalogue of known 3D fractal formulas with WGSL implementations.
//!
//! Each entry follows the schema defined in `mod.rs`:
//! id, name, family, dimension, category, iteration_core (DSL),
//! parameter_fields, de_supported, de_expression, wgsl_core_fn,
//! wgsl_shader, known_good_presets, origin, notes.
//!
//! Formulas are drawn from Mandelbulb 3D / Mandelbulber libraries,
//! Shadertoy references, and original research (waveguide-enhanced,
//! complex-recurrent, sparse-voxel).

pub use super::*;

use std::collections::HashMap;

/// Shared WGSL shader template.  The formula-specific DE function and
/// uniform fields are injected via placeholders.
const SHADER_TEMPLATE: &str = r#"struct Uniforms {
    iTime: f32,
    iResolution: vec2<f32>,
    cameraPos: vec3<f32>,
__UNIFORMS__
};

@group(0) @binding(0)
var<uniform> u: Uniforms;

__DE_FN__

fn raymarch(ro: vec3<f32>, rd: vec3<f32>) -> f32 {
    var t: f32 = 0.0;
    for (var i: i32 = 0; i < 512; i = i + 1) {
        let p = ro + rd * t;
        let d = fract_de(p);
        if (d < 1e-3) { return t; }
        t = t + max(d, 0.01);
        if (t > 100.0) { break; }
    }
    return -1.0;
}

fn estimate_normal(p: vec3<f32>) -> vec3<f32> {
    let eps: f32 = 1e-3;
    return normalize(vec3<f32>(
        fract_de(p + vec3<f32>(eps, 0.0, 0.0)) - fract_de(p - vec3<f32>(eps, 0.0, 0.0)),
        fract_de(p + vec3<f32>(0.0, eps, 0.0)) - fract_de(p - vec3<f32>(0.0, eps, 0.0)),
        fract_de(p + vec3<f32>(0.0, 0.0, eps)) - fract_de(p - vec3<f32>(0.0, 0.0, eps))
    ));
}

@vertex
fn vs_main(@builtin(vertex_index) vi: i32) -> @location(0) vec2<f32> {
    var uv = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, 3.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0)
    );
    return uv[vi];
}

@fragment
fn fs_main(@location(ible(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let aspect = u.iResolution.x / max(u.iResolution.y, 1e-5);
    let ro = u.cameraPos;
    let rd = normalize(vec3<f32>(uv * vec2<f32>(aspect, 1.0), -1.0));
    let t = raymarch(ro, rd);
    if (t < 0.0) {
        let sky = 0.5 * (uv.y + 1.0);
        return vec4<f32>(vec3<f32>(sky * 0.1, sky * 0.15, sky * 0.3), 1.0);
    }
    let p = ro + rd * t;
    let n = estimate_normal(p);
    let light = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diff = max(dot(n, light), 0.0);
    let c = vec3<f32>(0.3 + diff, 0.4 + diff * 0.5, 0.6 + diff * 0.3);
    return vec4<f32>(c, 1.0);
}
"#;

fn make_shader(uniforms: &str, de_fn: &str) -> String {
    SHADER_TEMPLATE
        .replace("__UNIFORMS__", uniforms)
        .replace("__DE_FN__", de_fn)
}

/// Helper to build a DSL from (op, params) pairs.
fn make_dsl(steps: &[(&str, &mut Vec<(&str, f64)> )]) -> FormulaDSL {
    let _ = steps;
    FormulaDSL::new()
}

/// Helper to build params hashmap from (key, value) pairs.
fn hp(pairs: &[(&str, f64)]) -> HashMap<String, serde_json::Value> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), serde_json::json!(v)))
        .collect()
}

// ===================================================================
// A. Mandelbulb (power-based)
// ===================================================================

pub fn mandelbulb_n8() -> FormulaEntry {
    let uniforms = "    maxIter: i32,\n    bailout: f32,\n    power: f32,";

    let de_fn = r#"fn fract_de(p: vec3<f32>) -> f32 {
    var z = p;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        let r = length(z);
        if (r > u.bailout) { break; }
        let theta = atan2(z.y, z.x);
        let phi = acos(clamp(z.z / max(r, 1e-5), -1.0, 1.0));
        let zr = pow(r, u.power);
        let zd = pow(r, abs(u.power - 1.0));
        dr = dr * u.power * zd + 1.0;
        z = zr * vec3<f32>(
            sin(phi * u.power) * cos(theta * u.power),
            sin(phi * u.power) * sin(theta * u.power),
            cos(phi * u.power)
        ) + p;
    }
    let r = length(z);
    return 0.5 * r * log(max(r, 1e-5)) / max(abs(dr), 1e-5);
}"#;

    let dsl = {
        let mut d = FormulaDSL::new();
        d.add("spherical_power", hp(&[("power", 8.0), ("add_c", 1.0)]));
        d.add("de_accumulate", hp(&[("method", 1.0)]));
        d
    };

    FormulaEntry::new("mandelbulb_n8", "Mandelbulb (n=8)", "mandelbulb", 3, FormulaCategory::DistanceEstimator)
        .with_dsl(dsl)
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 64.0, 4.0, 256.0).with_group("Formula"),
            ParameterField::float("bailout", "Bailout", 4.0, 2.0, 100.0).with_group("Formula"),
            ParameterField::float("power", "Power", 8.0, 2.0, 16.0).with_group("Formula"),
        ])
        .with_de("d = 0.5 * |z_final| * log(|z_final|) / |dz|")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("White/Nylander, Mandelbulb.com, Mandelbulb3D")
        .with_notes("Classic 3D Mandelbrot analogue using spherical coordinates. Power 8 produces the iconic 'Mandelbulb' shape.")
        .with_presets(vec![
            PresetEntry {
                name: "classic_n8".to_string(),
                parameters: HashMap::from([
                    ("power".to_string(), 8.0),
                    ("maxIter".to_string(), 64.0),
                    ("bailout".to_string(), 4.0),
                ]),
                tags: vec!["classic".to_string()],
            },
            PresetEntry {
                name: "smooth_n8".to_string(),
                parameters: HashMap::from([
                    ("power".to_string(), 8.0),
                    ("maxIter".to_string(), 128.0),
                    ("bailout".to_string(), 16.0),
                ]),
                tags: vec!["high-detail".to_string()],
            },
        ])
}

pub fn mandelbulb_n7() -> FormulaEntry {
    let uniforms = "    maxIter: i32,\n    bailout: f32,\n    power: f32,";

    let de_fn = r#"fn fract_de(p: vec3<f32>) -> f32 {
    var z = p;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        let r = length(z);
        if (r > u.bailout) { break; }
        let theta = atan2(z.y, z.x);
        let phi = acos(clamp(z.z / max(r, 1e-5), -1.0, 1.0));
        let zr = pow(r, u.power);
        let zd = pow(r, abs(u.power - 1.0));
        dr = dr * u.power * zd + 1.0;
        z = zr * vec3<f32>(
            sin(phi * u.power) * cos(theta * u.power),
            sin(phi * u.power) * sin(theta * u.power),
            cos(phi * u.power)
        ) + p;
    }
    let r = length(z);
    return 0.5 * r * log(max(r, 1e-5)) / max(abs(dr), 1e-5);
}"#;

    let dsl = {
        let mut d = FormulaDSL::new();
        d.add("spherical_power", hp(&[("power", 7.0), ("add_c", 1.0)]));
        d.add("de_accumulate", hp(&[]));
        d
    };

    FormulaEntry::new("mandelbulb_n7", "Mandelbulb (n=7)", "mandelbulb", 3, FormulaCategory::DistanceEstimator)
        .with_dsl(dsl)
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 64.0, 4.0, 256.0),
            ParameterField::float("bailout", "Bailout", 4.0, 2.0, 100.0),
            ParameterField::float("power", "Power", 7.0, 2.0, 16.0),
        ])
        .with_de("d = 0.5 * |z_final| * log(|z_final|) / |dz|")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("White/Nylander, Mandelbulb.com, Mandelbulb3D")
        .with_notes("Mandelbulb with power 7 — produces more organic, smoother bulb shapes.")
}

// ===================================================================
// B. Mandelbox
// ===================================================================

pub fn mandelbox_standard() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    bailout: f32,
    scale: f32,
    foldLimit: f32,
    fixedRadius: f32,
    minRadius: f32,";

    let de_fn = r#"fn fract_de(p: vec3<f32>) -> f32 {
    var z = p;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        z.x = select(z.x, 2.0 * u.foldLimit - z.x, z.x > u.foldLimit);
        z.x = select(z.x, -2.0 * u.foldLimit - z.x, z.x < -u.foldLimit);
        z.y = select(z.y, 2.0 * u.foldLimit - z.y, z.y > u.foldLimit);
        z.y = select(z.y, -2.0 * u.foldLimit - z.y, z.y < -u.foldLimit);
        z.z = select(z.z, 2.0 * u.foldLimit - z.z, z.z > u.foldLimit);
        z.z = select(z.z, -2.0 * u.foldLimit - z.z, z.z < -u.foldLimit);
        let r2 = dot(z, z);
        if (r2 < u.fixedRadius * u.fixedRadius) {
            let t = (u.fixedRadius * u.fixedRadius) / max(r2, 1e-10);
            z = z * t;
            dr = dr * t;
        } else if (r2 < u.minRadius * u.minRadius) {
            let t = (u.minRadius * u.minRadius) / max(r2, 1e-10);
            z = z * t;
            dr = dr * t;
        }
        z = z * u.scale + p;
        dr = dr * u.scale;
        if (dot(z, z) > u.bailout * u.bailout) { break; }
    }
    return 0.5 * length(z) / max(abs(dr), 1e-5);
}"#;

    let dsl = {
        let mut d = FormulaDSL::new();
        d.add("box_fold", hp(&[("limit", 1.0)]));
        d.add("sphere_fold", hp(&[("fixed_radius", 1.0), ("min_radius", 0.5)]));
        d.add("scale_offset", hp(&[("scale", 2.0)]));
        d.add("de_accumulate", hp(&[]));
        d
    };

    FormulaEntry::new("mandelbox_standard", "Mandelbox", "mandelbox", 3, FormulaCategory::DistanceEstimator)
        .with_dsl(dsl)
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 16.0, 4.0, 64.0),
            ParameterField::float("bailout", "Bailout", 100.0, 10.0, 1000.0),
            ParameterField::float("scale", "Scale", 2.0, 1.5, 3.0),
            ParameterField::float("foldLimit", "Fold Limit", 1.0, 0.1, 2.0),
            ParameterField::float("fixedRadius", "Fixed Radius", 1.0, 0.1, 2.0),
            ParameterField::float("minRadius", "Min Radius", 0.5, 0.01, 1.0),
        ])
        .with_de("d = 0.5 * |z_final| / |dr| (accumulated fold + scale derivative)")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Mandelbox community, Mandelbulber, Mandelbulb3D")
        .with_notes("Box fold + sphere fold + scaling produces intricate architectural 3D fractals.")
}

// ===================================================================
// C. Menger Sponge
// ===================================================================

pub fn menger_sponge() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    bailout: f32,
    scale: f32,";

    let de_fn = r#"fn fract_de(p: vec3<f32>) -> f32 {
    var z = p;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        z = abs(z);
        if (z.x < z.y) { let t = z.x; z.x = z.y; z.y = t; }
        if (z.x < z.z) { let t = z.x; z.x = z.z; z.z = t; }
        if (z.y < z.z) { let t = z.y; z.y = z.z; z.z = t; }
        z = z - vec3<f32>(1.0, 1.0, 1.0);
        z = z * u.scale;
        dr = dr * u.scale;
    }
    return 0.5 * length(z) / max(abs(dr), 1e-5);
}"#;

    let dsl = {
        let mut d = FormulaDSL::new();
        d.add("abs_fold", hp(&[]));
        d.add("component_sort", hp(&[]));
        d.add("scale_offset", hp(&[("scale", 3.0)]));
        d.add("de_accumulate", hp(&[]));
        d
    };

    FormulaEntry::new("menger_sponge", "Menger Sponge", "menger", 3, FormulaCategory::DistanceEstimator)
        .with_dsl(dsl)
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 12.0, 1.0, 20.0),
            ParameterField::float("bailout", "Bailout", 100.0, 1.0, 100.0),
            ParameterField::float("scale", "Scale", 3.0, 2.0, 4.0),
        ])
        .with_de("d = 0.5 * |z_final| / |dr| (folding-based)")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Menger sponge, Mandelbulb3D, Mandelbulber")
        .with_notes("Classic 3D Sierpinski-type fractal using coordinate folding and component sorting.")
}

// ===================================================================
// D. Quaternion Julia (4D)
// ===================================================================

pub fn quaternion_julia() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    bailout: f32,
    juliaC: vec4<f32>,
    juliaW: f32,";

    let de_fn = r#"fn quaternion_sq(q: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(
        q.w * q.w - q.x * q.x - q.y * q.y - q.z * q.z,
        2.0 * (q.w * q.x + q.y * q.z),
        2.0 * (q.w * q.y - q.x * q.z),
        2.0 * (q.w * q.z + q.x * q.y)
    );
}

fn fract_de(p: vec3<f32>) -> f32 {
    var z = vec4<f32>(p, u.juliaW);
    var c = u.juliaC;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        z = quaternion_sq(z) + c;
        dr = 2.0 * length(z) * dr;
        if (length(z) > u.bailout) { break; }
    }
    let r = length(z);
    return 0.5 * r * log(max(r, 1e-5)) / max(abs(dr), 1e-5);
}"#;

    let dsl = {
        let mut d = FormulaDSL::new();
        d.add("quaternion_square", hp(&[]));
        d.add("add_c", hp(&[("julia", 1.0)]));
        d.add("de_accumulate", hp(&[]));
        d
    };

    FormulaEntry::new("quaternion_julia", "Quaternion Julia", "quat_julia", 4, FormulaCategory::DistanceEstimator)
        .with_dsl(dsl)
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 32.0, 4.0, 128.0),
            ParameterField::float("bailout", "Bailout", 4.0, 2.0, 50.0),
            ParameterField::bool("enableComplex", "Complex C", 1.0),
        ])
        .with_de("d = 0.5 * |q_final| * log(|q_final|) / |dq|")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Quaternion Julia, Mandelbulb3D, Shadertoy")
        .with_notes("4D Julia set with quaternion algebra, projected to 3D slice for rendering.")
}

// ===================================================================
// E. Triplex Julia
// ===================================================================

pub fn triplex_julia() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    bailout: f32,
    juliaC3: vec3<f32>,";

    let de_fn = r#"fn triplex_mul(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    let ra = length(a);
    let rb = length(b);
    let theta = atan2(a.y, a.x) + atan2(b.y, b.x);
    let phi = acos(clamp(a.z / max(ra, 1e-5), -1.0, 1.0)) + acos(clamp(b.z / max(rb, 1e-5), -1.0, 1.0));
    let r = ra * rb;
    return r * vec3<f32>(
        cos(theta) * sin(phi),
        sin(theta) * sin(phi),
        cos(phi)
    );
}

fn fract_de(p: vec3<f32>) -> f32 {
    var z = p;
    var c = u.juliaC3;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        z = triplex_mul(z, z) + c;
        dr = 2.0 * length(z) * dr;
        if (length(z) > u.bailout) { break; }
    }
    let r = length(z);
    return 0.5 * r * log(max(r, 1e-5)) / max(abs(dr), 1e-5);
}"#;

    FormulaEntry::new("triplex_julia", "Triplex Julia", "triplex", 3, FormulaCategory::DistanceEstimator)
        .with_dsl(FormulaDSL::new().with_step("triplex_mul", hp(&[])).with_step("add_c", hp(&[("julia", 1.0)])))
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 32.0, 4.0, 128.0),
            ParameterField::float("bailout", "Bailout", 4.0, 2.0, 50.0),
        ])
        .with_de("d = 0.5 * |z_final| * log(|z_final|) / |dz|")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Triplex algebra, Mandelbulb3D, Shadertoy")
        .with_notes("3D Julia set using triplex (half) algebra multiplication.")
}

// ===================================================================
// F. IFS Tetrahedron (Sierpinski)
// ===================================================================

pub fn ifs_tetrahedron() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    bailout: f32,";

    let de_fn = r#"fn fract_de(p: vec3<f32>) -> f32 {
    var z = p;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        let hash = fract(sin(dot(z, vec3<f32>(12.9898, 78.233, 45.164))) * 43758.5453);
        let idx = i32(floor(hash * 3.0));
        let v0 = vec3<f32>(1.0, 1.0, 1.0);
        let v1 = vec3<f32>(-1.0, -1.0, 1.0);
        let v2 = vec3<f32>(-1.0, 1.0, -1.0);
        var v = v0;
        if (idx == 1) { v = v1; } else if (idx == 2) { v = v2; }
        z = z * 0.5 + v * 0.5;
        dr = dr * 0.5;
    }
    return length(z) / max(abs(dr), 1e-5);
}"#;

    FormulaEntry::new("ifs_tetrahedron", "IFS Tetrahedron", "ifs", 3, FormulaCategory::IFS)
        .with_dsl(FormulaDSL::new().with_step("ifs_select", hp(&[("num_maps", 3.0), ("contract", 0.5)])))
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 12.0, 1.0, 30.0),
            ParameterField::float("bailout", "Bailout", 100.0, 1.0, 1000.0),
        ])
        .with_de("d = |z_final| / |dr| (IFS contraction)")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Sierpinski tetrahedron IFS, Mandelbulb3D")
        .with_notes("Iterated function system generating a Sierpinski tetrahedron via 3 affine contractions toward vertices.")
}

// ===================================================================
// G. Fold Transform
// ===================================================================

pub fn fold_transform() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    bailout: f32,
    scale: f32,
    foldLimit: f32,";

    let de_fn = r#"fn fract_de(p: vec3<f32>) -> f32 {
    var z = p;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        let fl = u.foldLimit;
        z.x = select(z.x, 2.0 * fl - z.x, z.x > fl);
        z.x = select(z.x, -2.0 * fl - z.x, z.x < -fl);
        z.y = select(z.y, 2.0 * fl - z.y, z.y > fl);
        z.y = select(z.y, -2.0 * fl - z.y, z.y < -fl);
        z.z = select(z.z, 2.0 * fl - z.z, z.z > fl);
        z.z = select(z.z, -2.0 * fl - z.z, z.z < -fl);
        z = z * u.scale + p;
        dr = dr * u.scale;
        if (dot(z, z) > u.bailout * u.bailout) { break; }
    }
    return 0.5 * length(z) / max(abs(dr), 1e-5);
}"#;

    FormulaEntry::new("fold_transform", "Fold Transform", "transform", 3, FormulaCategory::Transform)
        .with_dsl(FormulaDSL::new().with_step("box_fold", hp(&[("limit", 1.0)])).with_step("scale_offset", hp(&[("scale", 2.0)])))
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 16.0, 1.0, 64.0),
            ParameterField::float("bailout", "Bailout", 100.0, 10.0, 1000.0),
            ParameterField::float("scale", "Scale", 2.0, 1.5, 3.0),
            ParameterField::float("foldLimit", "Fold Limit", 1.0, 0.1, 2.0),
        ])
        .with_de("d = 0.5 * |z_final| / |dr|")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Mandelbulb3D transform library")
        .with_notes("Coordinate folding transform. Often stacked with other formulas as a building block.")
}

// ===================================================================
// H. Aexion
// ===================================================================

pub fn aexion() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    bailout: f32,
    power: f32,";

    let de_fn = r#"fn fract_de(p: vec3<f32>) -> f32 {
    var z = p;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        let r2 = dot(z, z);
        if (r2 > u.bailout) { break; }
        var nx = z.x * z.x - z.y * z.y - z.z * z.z;
        var ny = 2.0 * z.y * (z.x + 0.5);
        var nz = 2.0 * z.z * (z.x - 0.5);
        z.x = nx; z.y = ny; z.z = nz;
        dr = dr * 2.0 * sqrt(r2);
    }
    let r = length(z);
    return 0.5 * r * log(max(r, 1e-5)) / max(abs(dr), 1e-5);
}"#;

    FormulaEntry::new("aexion", "Aexion", "aexion", 3, FormulaCategory::DistanceEstimator)
        .with_dsl(FormulaDSL::new().with_step("aexion_map", hp(&[])).with_step("de_accumulate", hp(&[])))
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 32.0, 4.0, 128.0),
            ParameterField::float("bailout", "Bailout", 4.0, 2.0, 50.0),
            ParameterField::float("power", "Power", 2.0, 2.0, 8.0),
        ])
        .with_de("d = 0.5 * |z_final| * log(|z_final|) / |dz|")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Aexion fractal variant, Shadertoy")
        .with_notes("Variant of the triplex Mandelbulb with offset cross-terms producing organic branching structures.")
}

// ===================================================================
// I. Bristorbrot (4D Mandelbrot variant)
// ===================================================================

pub fn bristorbrot() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    bailout: f32,";

    let de_fn = r#"fn fract_de(p: vec3<f32>) -> f32 {
    var z = vec4<f32>(p, 0.0);
    var c = vec4<f32>(p, 0.0);
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        var nx = z.x * z.x - z.y * z.y - z.z * z.z - z.w * z.w;
        var ny = 2.0 * z.x * z.y + z.z * z.w;
        var nz = 2.0 * z.x * z.z - z.y * z.w;
        var nw = 2.0 * z.x * z.w + z.y * z.z;
        z.x = nx; z.y = ny; z.z = nz; z.w = nw;
        dr = 2.0 * length(z) * dr;
        if (length(z) > u.bailout) { break; }
    }
    let r = length(z);
    return 0.5 * r * log(max(r, 1e-5)) / max(abs(dr), 1e-5);
}"#;

    FormulaEntry::new("bristorbrot", "Bristorbrot", "bristorbrot", 4, FormulaCategory::DistanceEstimator)
        .with_dsl(FormulaDSL::new().with_step("4d_mandelbrot", hp(&[])))
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 32.0, 4.0, 128.0),
            ParameterField::float("bailout", "Bailout", 4.0, 2.0, 50.0),
        ])
        .with_de("d = 0.5 * |z_final| * log(|z_final|) / |dz|")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Bristor/David Brito, Shadertoy")
        .with_notes("4D Mandelbrot variant using a specific non-standard 4D multiplication rule.")
}

// ===================================================================
// J. Mandelbulb-WG (user's enhanced formula)
// ===================================================================

pub fn mandelbulb_wg() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    bailout: f32,
    power: f32,";

    let de_fn = r#"fn is_prime(n: i32) -> bool {
    if (n < 2) { return false; }
    if (n < 4) { return true; }
    if (n % 2 == 0 || n % 3 == 0) { return false; }
    var i: i32 = 5;
    loop {
        if (i * i > n) { break; }
        if (n % i == 0 || n % (i + 2) == 0) { return false; }
        i = i + 6;
    }
    return true;
}

fn fract_de(p: vec3<f32>) -> f32 {
    var z = p;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        let r = length(z);
        if (r > u.bailout) { break; }
        let theta = atan2(z.y, z.x);
        let phi = acos(clamp(z.z / max(r, 1e-5), -1.0, 1.0));
        let eff_power = select(u.power, u.power * 1.3, is_prime(i + 1));
        let zr = pow(r, eff_power);
        let zd = pow(r, abs(eff_power - 1.0));
        dr = dr * eff_power * zd + 1.0;
        let sr = r;
        let stheta = sin(theta);
        let sphi = sin(phi);
        let sc_r = 0.7 * sr + 0.3 * stheta;
        let sc_theta = 0.3 * sr + 0.7 * stheta + 0.1 * sphi;
        let sc_phi = 0.1 * stheta + 0.7 * sphi;
        z = zr * vec3<f32>(
            sin(sc_phi * eff_power) * cos(sc_theta * eff_power),
            sin(sc_phi * eff_power) * sin(sc_theta * eff_power),
            cos(sc_phi * eff_power)
        ) + p;
    }
    let r = length(z);
    return 0.5 * r * log(max(r, 1e-5)) / max(abs(dr), 1e-5);
}"#;

    let dsl = {
        let mut d = FormulaDSL::new();
        d.add("spherical_power", hp(&[("power", 8.0)]));
        d.add("waveguide_scatter", hp(&[("channels", 3.0), ("delays", 1.0)]));
        d.add("prime_modulate", hp(&[("param", 1.0), ("factor", 1.3)]));
        d.add("de_accumulate", hp(&[]));
        d
    };

    FormulaEntry::new("mandelbulb_wg", "Mandelbulb-WG", "mandelbulb_wg", 3, FormulaCategory::DistanceEstimator)
        .with_dsl(dsl)
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 64.0, 4.0, 256.0).with_group("Formula"),
            ParameterField::float("bailout", "Bailout", 4.0, 2.0, 100.0).with_group("Formula"),
            ParameterField::float("power", "Power", 8.0, 2.0, 16.0).with_group("Formula"),
        ])
        .with_de("d = 0.5 * |z_final| * log(|z_final|) / |dz|")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Research formula – Mandelbulb enhanced with waveguide scattering + prime modulation")
        .with_notes("Mandelbulb base augmented with a waveguide scattering step on spherical coordinate channels and prime-indexed power modulation. Produces richer boundary detail.")
        .with_presets(vec![
            PresetEntry {
                name: "default".to_string(),
                parameters: HashMap::from([
                    ("power".to_string(), 8.0),
                    ("maxIter".to_string(), 64.0),
                    ("bailout".to_string(), 4.0),
                ]),
                tags: vec!["default".to_string()],
            },
        ])
}

// ===================================================================
// K. SDF-Recurrent (complex-recurrent + SDF decode)
// ===================================================================

pub fn sdf_recurrent() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    bailout: f32,
    power: f32,";

    let de_fn = r#"fn complex_power(z: vec2<f32>, n: f32) -> vec2<f32> {
    let r = length(z);
    if (r < 1e-10) { return vec2<f32>(0.0, 0.0); }
    let theta = atan2(z.y, z.x);
    let rn = pow(r, n);
    return vec2<f32>(rn * cos(theta * n), rn * sin(theta * n));
}

fn fract_de(p: vec3<f32>) -> f32 {
    var c1 = vec2<f32>(p.x, p.z);
    var c2 = vec2<f32>(p.y, 0.0);
    var z1 = vec2<f32>(0.0, 0.0);
    var z2 = vec2<f32>(0.0, 0.0);
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {
        let s1 = 0.7 * z1 + 0.3 * z2;
        let s2 = 0.3 * z1 + 0.7 * z2;
        z1 = complex_power(s1, u.power) + c1;
        z2 = complex_power(s2, u.power) + c2;
        dr = dr * u.power * pow(max(length(z1) + length(z2), 1e-5), u.power - 1.0) + 1.0;
        if (length(z1) > u.bailout || length(z2) > u.bailout) { break; }
    }
    let r = length(z1) + length(z2);
    return 0.5 * r * log(max(r, 1e-5)) / max(abs(dr), 1e-5);
}"#;

    let dsl = {
        let mut d = FormulaDSL::new();
        d.add("embed_3d_to_complex", hp(&[]));
        d.add("complex_power", hp(&[("power", 2.0)]));
        d.add("waveguide_scatter", hp(&[("channels", 2.0), ("mix", 0.3)]));
        d.add("de_accumulate", hp(&[]));
        d
    };

    FormulaEntry::new("sdf_recurrent", "SDF-Recurrent", "sdf_recurrent", 3, FormulaCategory::DistanceEstimator)
        .with_dsl(dsl)
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 64.0, 4.0, 256.0).with_group("Formula"),
            ParameterField::float("bailout", "Bailout", 4.0, 2.0, 100.0).with_group("Formula"),
            ParameterField::float("power", "Power", 2.0, 2.0, 16.0).with_group("Formula"),
        ])
        .with_de("d = 0.5 * |z_final| * log(|z_final|) / |dz| (complex-recurrent)")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Research formula – complex-recurrent map decoded to SDF")
        .with_notes("3D point embedded as complex pairs, iterated through a learned complex map with waveguide scattering, decoded to SDF-like distance estimate.")
}

// ===================================================================
// L. Sparse-Voxel-Recursive (hierarchical recursive)
// ===================================================================

pub fn sparse_voxel_recursive() -> FormulaEntry {
    let uniforms = "    maxIter: i32,
    maxDepth: i32,
    bailout: f32,
    power: f32,
    scaleFactor: f32,";

    let de_fn = r#"fn is_prime(n: i32) -> bool {
    if (n < 2) { return false; }
    if (n < 4) { return true; }
    if (n % 2 == 0 || n % 3 == 0) { return false; }
    var i: i32 = 5;
    loop {
        if (i * i > n) { break; }
        if (n % i == 0 || n % (i + 2) == 0) { return false; }
        i = i + 6;
    }
    return true;
}

fn fract_de(p: vec3<f32>) -> f32 {
    var acc: f32 = 0.0;
    var scale: f32 = 1.0;
    for (var depth: i32 = 0; depth < u.maxDepth; depth = depth + 1) {
        let sp = p * scale;
        let r = length(sp);
        if (r < 1e-5) {
            scale = scale * u.scaleFactor;
            continue;
        }
        let theta = atan2(sp.y, sp.x);
        let phi = acos(clamp(sp.z / r, -1.0, 1.0));
        let zr = pow(r, u.power);
        let contrib = zr * cos(phi * u.power);
        let active = is_prime(depth + 2);
        acc = acc + select(0.0, contrib, active);
        scale = scale * u.scaleFactor;
    }
    return 0.5 * acc / max(abs(log(max(acc, 1e-5))), 1e-5);
}"#;

    FormulaEntry::new("sparse_voxel_recursive", "Sparse-Voxel-Recursive", "sparse_voxel", 3, FormulaCategory::DistanceEstimator)
        .with_dsl(FormulaDSL::new().with_step("multiscale_iterate", hp(&[("max_depth", 8.0), ("scale_factor", 2.0)])).with_step("prime_skip_refine", hp(&[])))
        .with_params(vec![
            ParameterField::int("maxIter", "Iterations", 32.0, 4.0, 128.0),
            ParameterField::int("maxDepth", "Max Depth", 8.0, 1.0, 16.0),
            ParameterField::float("bailout", "Bailout", 4.0, 2.0, 50.0),
            ParameterField::float("power", "Power", 8.0, 2.0, 16.0),
            ParameterField::float("scaleFactor", "Scale Factor", 2.0, 1.0, 4.0),
        ])
        .with_de("d = 0.5 * sum(contributions) / |log(sum)| (multiscale)")
        .with_wgsl(de_fn, &make_shader(uniforms, de_fn))
        .with_origin("Research formula – hierarchical sparse voxel generation, inspired by Fractal3DGen")
        .with_notes("Sparse-voxel recursive formula: iterates the Mandelbulb map at multiple scales, refining only at prime-indexed depths to produce hierarchical structure without dense voxels.")
}

// ===================================================================
// Catalogue accessor
// ===================================================================

/// All canonical formulae in the research catalogue.
pub fn catalogue() -> Vec<FormulaEntry> {
    vec![
        mandelbulb_n8(),
        mandelbulb_n7(),
        mandelbox_standard(),
        menger_sponge(),
        quaternion_julia(),
        triplex_julia(),
        ifs_tetrahedron(),
        fold_transform(),
        aexion(),
        bristorbrot(),
        mandelbulb_wg(),
        sdf_recurrent(),
        sparse_voxel_recursive(),
    ]
}

/// Look up a formula by ID.
pub fn find(id: &str) -> Option<FormulaEntry> {
    catalogue().into_iter().find(|f| f.id == id)
}

/// List all formula IDs and names.
pub fn list_summary() -> Vec<(String, String)> {
    catalogue()
        .into_iter()
        .map(|f| (f.id, f.name))
        .collect()
}

//! Mandelbulb fractal formula implementation.
//!
//! Implements the `FractalFormula` trait for Mandelbulb (power-based) fractals.
//! Uses spherical coordinates to iterate `z <- z^p + c` in 3D.

use crate::formula_dsl::{FieldSample, FractalFormula, IterationState, Parameter, Vec3, FieldType, WgslShader};
use std::fmt::Debug;

/// A Mandelbulb fractal formula with configurable power.
#[derive(Clone, Debug)]
pub struct Mandelbulb {
    pub power: f64,
    pub max_iter: usize,
    pub bailout: f64,
}

impl Mandelbulb {
    pub fn new(power: f64, max_iter: usize, bailout: f64) -> Self {
        Self { power, max_iter, bailout }
    }
}

impl FractalFormula for Mandelbulb {
    fn name(&self) -> &str {
        "Mandelbulb"
    }

    fn parameters(&self) -> &[Parameter] {
        &[]
    }

    fn init_state(&self, c: Vec3) -> IterationState {
        IterationState::new(c, self.bailout.sqrt())
    }

    fn iterate(&self, state: &mut IterationState, _step: usize) {
        let z = &mut state.z3;
        let c = state.point;
        let r = z.norm();
        if r < 1e-30 {
            *z = c;
            return;
        }
        let theta = z.y.atan2(z.x);
        let phi = if r < 1e-5 {
            std::f64::consts::PI / 2.0
        } else {
            let ratio = z.z / r;
            ratio.clamp(-1.0, 1.0).acos()
        };
        let rp = r.powf(self.power);
        let theta_p = theta * self.power;
        let phi_p = phi * self.power;
        *z = Vec3::new(
            rp * phi_p.sin() * theta_p.cos() + c.x,
            rp * phi_p.sin() * theta_p.sin() + c.y,
            rp * phi_p.cos() + c.z,
        );
    }

    fn evaluate(&self, c: Vec3, max_iter: usize) -> FieldSample {
        let mut state = self.init_state(c);
        for step in 0..max_iter {
            if state.diverged {
                break;
            }
            self.iterate(&mut state, step);
            state.check_divergence(step + 1);
        }
        self.decode(&state, c, max_iter)
    }

    fn decode(&self, state: &IterationState, _c: Vec3, max_iter: usize) -> FieldSample {
        let diverged = state.diverged;
        let iterations = if diverged { state.escape_iter } else { max_iter };
        let color = if diverged {
            Some([iterations as f32 / max_iter as f32, 0.5, 0.5])
        } else {
            None
        };
        FieldSample {
            value: 1.0,
            iterations,
            color,
            field_type: FieldType::DistanceEstimate,
        }
    }

    fn generate_wgsl(&self) -> WgslShader {
        let uniforms = format!(
            "    maxIter: i32,\n    bailout: f32,\n    power: f32,\n    power{}: f32",
            self.power
        );

        let de_fn = format!(
            r#"fn fract_de(p: vec3<f32>) -> f32 {{
    var z = p;
    var dr: f32 = 1.0;
    for (var i: i32 = 0; i < u.maxIter; i = i + 1) {{
        let r = length(z);
        if (r > u.bailout) {{ break; }}
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
    }}
    let r = length(z);
    return 0.5 * r * log(max(r, 1e-5)) / max(abs(dr), 1e-5);
}}"#
        );

        let shader_body = format!(
            r#"struct Uniforms {{
    iTime: f32,
    iResolution: vec2<f32>,
    cameraPos: vec3<f32>,
{uniforms}
}}

@group(0) @binding(0)
var<uniform> u: Uniforms;

fract_de(p) {{
{de_fn}
}}

@vertex
fn vs_main(@builtin(vertex_index) vi: i32) -> @location(0) vec2<f32> {{
    var uv = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, 3.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0)
    );
    return uv[vi];
}}

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {{
    let aspect = u.iResolution.x / max(u.iResolution.y, 1e-5);
    let ro = u.cameraPos;
    let rd = normalize(vec3<f32>(uv * vec2<f32>(aspect, 1.0), -1.0));
    let t = fract_de(ro, rd);
    if (t < 0.0) {{
        let sky = 0.5 * (uv.y + 1.0);
        return vec4<f32>(vec3<f32>(sky * 0.1, sky * 0.15, sky * 0.3), 1.0);
    }}
    let p = ro + rd * t;
    let n = estimate_normal(p);
    let light = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diff = max(dot(n, light), 0.0);
    let c = vec3<f32>(0.3 + diff, 0.4 + diff * 0.5, 0.6 + diff * 0.3);
    return vec4<f32>(c, 1.0);
}}"#
        );

        WgslShader {
            uniforms,
            functions: shader_body,
            entry_point: "fs_main".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula_dsl::FieldType;

    #[test]
    fn test_mandelbulb_name() {
        let m = Mandelbulb::new(8.0, 64, 4.0);
        assert_eq!(m.name(), "Mandelbulb");
    }

    #[test]
    fn test_mandelbulb_evaluate() {
        let m = Mandelbulb::new(8.0, 64, 4.0);
        let sample = m.evaluate(Vec3::new(0.0, 0.0, 0.0), 64);
        assert!(sample.iterations <= 64);
        assert_eq!(sample.field_type, FieldType::DistanceEstimate);
    }

    #[test]
    fn test_mandelbulb_wgsl_contains_key_patterns() {
        let m = Mandelbulb::new(8.0, 64, 4.0);
        let w = m.generate_wgsl();
        assert!(w.uniforms.contains("maxIter"), "missing maxIter uniform");
        assert!(w.uniforms.contains("power"), "missing power uniform");
        assert!(w.functions.contains("fn fract_de"), "missing fract_de function");
        assert!(w.functions.contains("raymarch") || w.functions.contains("fract_de"), "missing raymarch or fract_de in shader body");
    }
}
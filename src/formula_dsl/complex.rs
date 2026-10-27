//! Complex-valued operators for fractal formulas.
//!
//! These operators work in C^n and are the natural core for learned or
//! hand-designed nonlinear maps.  The classical Mandelbrot/Julia iteration
//! `z_{n+1} = z^2 + c` is a special case of `ComplexPower(2)` followed by
//! `ComplexAdd(c)`.

use super::{Cx, Vec3};
use crate::formula_dsl::{FractalFormula, IterationState, FieldSample, FieldType, WgslShader};
use num_complex::Complex64;
use std::fmt::Display;

/// Activation function for learned complex maps.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Activation {
    Tanh,
    Relu,
    Identity,
}

impl Activation {
    pub fn apply(self, z: Cx) -> Cx {
        match self {
            Activation::Tanh => Cx::new(z.re.tanh(), z.im.tanh()),
            Activation::Relu => Cx::new(z.re.max(0.0), z.im.max(0.0)),
            Activation::Identity => z,
        }
    }

    pub fn as_wgsl(self) -> &'static str {
        match self {
            Activation::Tanh => "fn act_tanh(z: vec2<f32>) -> vec2<f32> { return vec2<f32>(tanh(z.x), tanh(z.y)); }",
            Activation::Relu => "fn act_relu(z: vec2<f32>) -> vec2<f32> { return vec2<f32>(max(z.x, 0.0), max(z.y, 0.0)); }",
            Activation::Identity => "fn act_identity(z: vec2<f32>) -> vec2<f32> { return z; }",
        }
    }
}

/// Learned complex-valued linear map: `out = activation(W * in + b)`.
///
/// Weights are complex and initialized via Xavier-like scheme:
/// `scale = 1 / sqrt(in_dim * out_dim)` for real weights, complex weights use
/// magnitude `sqrt(2 * scale^2)` for unit-norm complex entries.
#[derive(Clone, Debug)]
pub struct LearnedComplexMap {
    pub in_dim: usize,
    pub out_dim: usize,
    pub weights: Vec<Vec<Cx>>,
    pub biases: Vec<Cx>,
    pub activation: Activation,
}

impl LearnedComplexMap {
    pub fn new(in_dim: usize, out_dim: usize) -> Self {
        let scale = (1.0 / ((in_dim + out_dim) as f64).sqrt()).max(0.01);
        // Deterministic pseudo-random complex weights via formula
        let weights = (0..out_dim)
            .map(|i| {
                (0..in_dim)
                    .map(|j| {
                        let mag = scale;
                        let angle = std::f64::consts::PI * (i * 7 + j * 13) as f64;
                        Cx::new((mag * angle.cos()), (mag * angle.sin()))
                    })
                    .collect()
            })
            .collect();
        let biases = (0..out_dim).map(|_| Cx::new(0.0, 0.0)).collect();
        Self {
            in_dim,
            out_dim,
            weights,
            biases,
            activation: Activation::Tanh,
        }
    }

    pub fn forward(&self, inputs: &[Cx]) -> Vec<Cx> {
        let mut out = vec![Cx::new(0.0, 0.0); self.out_dim];
        for i in 0..self.out_dim {
            let mut acc = Cx::new(0.0, 0.0);
            for j in 0..self.in_dim {
                acc += self.weights[i][j] * inputs[j];
            }
            acc = acc + self.biases[i];
            out[i] = self.activation.apply(acc);
        }
        out
    }
}

impl FractalFormula for LearnedComplexMap {
    fn name(&self) -> &str {
        "LearnedComplexMap"
    }

    fn parameters(&self) -> &[super::Parameter] {
        &[]
    }

    fn init_state(&self, c: Vec3) -> IterationState {
        IterationState::new(c, 16.0)
    }

    fn iterate(&self, state: &mut IterationState, step: usize) {
        let inputs = vec![
            Cx::new(state.z_complex[0].re, state.z_complex[0].im),
            Cx::new(state.z_complex[1].re, state.z_complex[1].im),
        ];
        let out = self.forward(&inputs);
        state.z_complex[0] = out[0];
        state.z_complex[1] = out[1];
        state.check_divergence(step + 1);
    }

    fn evaluate(&self, c: Vec3, max_iter: usize) -> FieldSample {
        let mut state = self.init_state(c);
        for step in 0..max_iter {
            if state.diverged {
                break;
            }
            self.iterate(&mut state, step);
        }
        self.decode(&state, c, max_iter)
    }

    fn decode(&self, state: &IterationState, _c: Vec3, max_iter: usize) -> FieldSample {
        let diverged = state.diverged;
        let iterations = if diverged { state.escape_iter } else { max_iter };
        FieldSample {
            value: if diverged { 0.0 } else { 1.0 },
            iterations,
            color: None,
            field_type: FieldType::DistanceEstimate,
        }
    }

    fn generate_wgsl(&self) -> WgslShader {
        WgslShader {
            uniforms: format!("    inDim: i32,\n    outDim: i32,\n    activation: i32,"),
            functions: format!(
                r#"fn learned_complex_map(in_dim: i32, out_dim: i32, inputs: vec2<f32>) -> vec2<f32> {{
    var result: vec2<f32> = vec2<f32>(0.0, 0.0);
    var i: i32;
    for (i = 0; i < out_dim; i++) {{
        var j: i32;
        for (j = 0; j < in_dim; j++) {{
            // matrix multiplication + bias + activation
        }}
    }}
    return result;
}}"#
            ),
            entry_point: "fs_main".to_string(),
        }
    }
}

/// Compute z^n for a complex number using polar form.
/// `z^n = |z|^n * exp(i * n * arg(z))`.
pub fn complex_power(z: Cx, n: f64) -> Cx {
    let r = z.norm();
    if r < 1e-30 {
        return Cx::new(0.0, 0.0);
    }
    let theta = z.arg();
    let new_r = r.powf(n);
    let new_theta = theta * n;
    Cx::from_polar(new_r, new_theta)
}

/// Complex power applied iteratively (Mandelbrot-style).
pub fn complex_power_iter(z: &mut Cx, n: f64, c: Cx) {
    *z = complex_power(*z, n) + c;
}

/// Modulus-based nonlinearity: `z -> z / |z| * f(|z|)`.
/// Common choices: `f(r) = r` (identity), `f(r) = r^n`, `f(r) = tanh(r)`.
pub fn modulus_nonlinear(z: Cx, f: ModulusFunc) -> Cx {
    let r = z.norm();
    if r < 1e-30 {
        return Cx::new(0.0, 0.0);
    }
    let new_r = f.apply(r);
    Cx::from_polar(new_r, z.arg())
}

/// Function applied to the modulus in `modulus_nonlinear`.
#[derive(Clone, Copy, Debug)]
pub enum ModulusFunc {
    /// f(r) = r (pass-through, preserves magnitude).
    Identity,
    /// f(r) = r^n.
    Power(f64),
    /// f(r) = tanh(r).
    Tanh,
    /// f(r) = log(1 + r).
    Log,
}

impl ModulusFunc {
    pub fn apply(self, r: f64) -> f64 {
        match self {
            ModulusFunc::Identity => r,
            ModulusFunc::Power(n) => r.powf(n),
            ModulusFunc::Tanh => r.tanh(),
            ModulusFunc::Log => (1.0 + r).ln(),
        }
    }

    pub fn as_wgsl(self) -> &'static str {
        match self {
            ModulusFunc::Identity => "fn modulus_fn(r: f32) -> f32 { return r; }",
            ModulusFunc::Tanh => "fn modulus_fn(r: f32) -> f32 { return tanh(r); }",
            ModulusFunc::Log => "fn modulus_fn(r: f32) -> f32 { return log(1.0 + r); }",
            ModulusFunc::Power(_) => "fn modulus_fn(r: f32) -> f32 { return r; }",
        }
    }
}

/// Rotate the phase of z by angle `theta`.
pub fn phase_rotate(z: Cx, theta: f64) -> Cx {
    let rot = Complex64::from_polar(1.0, theta);
    z * rot
}

/// Linear combination of complex vectors: `sum(weights[i] * inputs[i])`.
pub fn complex_sum(inputs: &[Cx], weights: &[f64]) -> Cx {
    inputs
        .iter()
        .zip(weights.iter())
        .map(|(z, w)| *z * *w)
        .fold(Cx::new(0.0, 0.0), |a, b| a + b)
}

/// Embed a 3D point into a complex pair (for complex-recurrent formulas).
pub fn embed_3d_to_complex(p: Vec3) -> [Cx; 2] {
    [
        Cx::new(p.x, p.z),
        Cx::new(p.y, 0.0),
    ]
}

/// Generate WGSL for a complex power operation.
pub fn complex_power_wgsl(
    var: &str,
    n: f64,
    c_var: &str,
    result_var: &str,
) -> String {
    format!(
        r#"{{
    let {var}_r = length({var});
    let {var}_theta = atan2({var}.y, {var}.x);
    let {result_var} = vec2<f32>(
        pow({var}_r, {n}) * cos({n} * {var}_theta) - {c_var}.x,
        pow({var}_r, {n}) * sin({n} * {var}_theta) - {c_var}.y
    );
}}"#,
        var = var,
        n = n,
        c_var = c_var,
        result_var = result_var
    )
}

/// Complex modulus nonlinearity in WGSL.
pub fn modulus_nonlinear_wgsl(input: &str, func: ModulusFunc, output: &str) -> String {
    let fn_name = func.as_wgsl();
    // Extract function name from the WGSL fn string
    let fn_name_str = fn_name
        .strip_prefix("fn ")
        .and_then(|s| s.split('(').next())
        .unwrap_or("modulus_fn");
    format!(
        "let {output} = {input}_r > 1e-6 ? vec2<f32>({fn_name_str}({input}_r) * cos({input}_theta), {fn_name_str}({input}_r) * sin({input}_theta)) : vec2<f32>(0.0, 0.0);"
    )
}

//! Formula DSL: composable fractal formula operators.
//!
//! The DSL is built around the `FractalFormula` trait.  Each operator
//! implements `init`, `iterate`, and `decode`, and can generate WGSL shader
//! code for GPU preview.
//!
//! Three orthogonal concerns drive formula design:
//!
//! - **Complex maps** (`complex` submodule) — the nonlinear update in C^n.
//! - **Waveguide transport** (`waveguide` submodule) — delay, scattering,
//!   and reflection between iterations.
//! - **Prime scheduling** (`prime` submodule) — recursion-depth control and
//!   branch selection via prime-indexed rules.

pub mod complex;
pub mod mandelbulb;
pub mod prime;
pub mod waveguide;

use crate::error::FractalResult;
use nalgebra::Vector3;
use num_complex::Complex64;
use std::collections::VecDeque;
use std::fmt::Debug;

/// A 3D point or vector.
pub type Vec3 = Vector3<f64>;

/// A single complex channel value.
pub type Cx = Complex64;

/// Output of decoding iteration state into a field value.
#[derive(Clone, Debug)]
pub struct FieldSample {
    /// Scalar field value (density, distance estimate, escape potential).
    pub value: f64,
    /// Number of iterations before convergence/bailout.
    pub iterations: usize,
    /// Optional RGB color for direct rendering.
    pub color: Option<[f32; 3]>,
    /// What the value represents.
    pub field_type: FieldType,
}

/// How `FieldSample::value` should be interpreted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldType {
    /// Escape-time count (normalized or raw).
    EscapeTime,
    /// Distance estimate for raymarching.
    DistanceEstimate,
    /// Volumetric density (>= 0).
    Density,
    /// Occupancy (0 = outside, 1 = inside).
    Occupancy,
}

/// Mutable state carried through one formula's iteration.
#[derive(Clone, Debug)]
pub struct IterationState {
    /// The sample point c (constant across iterations).
    pub point: Vec3,

    /// 3D vector state — used by Mandelbulb-style and SDF formulas.
    pub z3: Vec3,

    /// Complex channel states — used by complex-recurrent and INR formulas.
    pub z_complex: Vec<Cx>,

    /// Waveguide delay/scatter channels.
    pub wg: Vec<WaveguideChannel>,

    /// Whether the orbit has diverged beyond the bailout radius.
    pub diverged: bool,

    /// Iteration at which divergence occurred (0 = not diverged).
    pub escape_iter: usize,

    /// Squared bailout radius.
    pub escape_radius_sq: f64,

    /// Distance estimate accumulator (for DE formulas).
    pub distance_estimate: f64,
}

impl IterationState {
    pub fn new(point: Vec3, escape_radius: f64) -> Self {
        Self {
            point,
            z3: Vec3::zeros(),
            z_complex: Vec::new(),
            wg: Vec::new(),
            diverged: false,
            escape_iter: 0,
            escape_radius_sq: escape_radius * escape_radius,
            distance_estimate: 0.0,
        }
    }

    /// Allocate `n` complex channels initialised to zero.
    pub fn with_complex_channels(point: Vec3, n: usize, escape_radius: f64) -> Self {
        let mut state = Self::new(point, escape_radius);
        state.z_complex = vec![Cx::new(0.0, 0.0); n];
        state
    }

    /// Check divergence of the 3D vector state.
    pub fn check_divergence(&mut self, max_iter: usize) -> bool {
        if self.z3.norm_squared() > self.escape_radius_sq {
            self.diverged = true;
            if self.escape_iter == 0 {
                self.escape_iter = max_iter;
            }
            true
        } else {
            false
        }
    }

    /// Check divergence of complex state channel `idx`.
    pub fn check_complex_divergence(&mut self, idx: usize, max_iter: usize) -> bool {
        if self.z_complex[idx].norm_sqr() > self.escape_radius_sq {
            self.diverged = true;
            if self.escape_iter == 0 {
                self.escape_iter = max_iter;
            }
            true
        } else {
            false
        }
    }
}

/// A single waveguide delay channel.
#[derive(Clone, Debug)]
pub struct WaveguideChannel {
    /// FIFO delay line of complex samples.
    pub buffer: VecDeque<Cx>,
    /// Maximum delay length (in samples).
    pub max_delay: usize,
    /// Reflection coefficient (0 = no reflection, 1 = full reflection).
    pub reflection: f64,
    /// Gain/multiiply factor applied on each pass.
    pub gain: f64,
}

impl WaveguideChannel {
    pub fn new(max_delay: usize, reflection: f64, gain: f64) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_delay + 1),
            max_delay,
            reflection,
            gain,
        }
    }

    /// Write a new sample (oldest sample drops out).
    pub fn write(&mut self, sample: Cx) {
        self.buffer.push_back(sample);
        while self.buffer.len() > self.max_delay + 1 {
            self.buffer.pop_front();
        }
    }

    /// Read the delayed sample (front of the buffer).
    pub fn read(&self) -> Cx {
        self.buffer.front().copied().unwrap_or(Cx::new(0.0, 0.0))
    }

    /// Scatter `incoming` through the junction:
    /// `out = reflection * delayed + (1 - reflection) * incoming`
    pub fn scatter(&mut self, incoming: Cx) -> Cx {
        let delayed = self.read();
        let output = delayed * self.reflection + incoming * (1.0 - self.reflection);
        self.write(incoming);
        output * self.gain
    }
}

/// An NxN scattering matrix for mixing N waveguide channels.
#[derive(Clone, Debug)]
pub struct ScatteringMatrix {
    /// Row-major matrix[m][n] describes how channel n feeds into output m.
    pub matrix: Vec<Vec<f64>>,
    pub size: usize,
}

impl ScatteringMatrix {
    pub fn new(size: usize) -> Self {
        let mut matrix = vec![vec![0.0; size]; size];
        // Default: identity-like (each channel mostly passes through).
        for i in 0..size {
            matrix[i][i] = 0.7;
            for j in 0..size {
                if i != j {
                    matrix[i][j] = 0.3 / (size as f64 - 1.0).max(1.0);
                }
            }
        }
        Self { matrix, size }
    }

    /// Mix an array of channel values through the scattering matrix.
    pub fn scatter(&self, inputs: &[Cx]) -> Vec<Cx> {
        (0..self.size)
            .map(|i| {
                let mut acc = Cx::new(0.0, 0.0);
                for (j, input) in inputs.iter().enumerate() {
                    if j < self.size {
                        acc += *input * self.matrix[i][j];
                    }
                }
                acc
            })
            .collect()
    }

    /// Generate the WGSL representation of the matrix application.
    pub fn as_wgsl(&self, var_prefix: &str) -> String {
        let mut lines = Vec::new();
        for i in 0..self.size {
            let mut terms: Vec<String> = Vec::new();
            for j in 0..self.size {
                let coeff = self.matrix[i][j];
                if coeff.abs() > 1e-9 {
                    terms.push(format!(
                        "{} * {}.{}{}",
                        coeff,
                        var_prefix,
                        j,
                        if self.size > 1 { "[i]" } else { "" }
                    ));
                }
            }
            if terms.is_empty() {
                lines.push(format!("    let {}{}_out = 0.0;", var_prefix, i));
            } else {
                lines.push(format!("    let {}{}_out = {};", var_prefix, i, terms.join(" + ")));
            }
        }
        lines.join("\n")
    }
}

/// Parameter descriptor for UI generation and JSON export.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    pub name: String,
    pub label: String,
    #[serde(rename = "type")]
    pub param_type: ParamType,
    pub default: f64,
    pub min: f64,
    pub max: f64,
    /// Group for UI organization (e.g. "Waveguide", "Prime", "Complex").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParamType {
    Float,
    Int,
    Bool,
}

impl Parameter {
    pub fn float(name: &str, label: &str, default: f64, min: f64, max: f64) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            param_type: ParamType::Float,
            default,
            min,
            max,
            group: None,
        }
    }

    pub fn int(name: &str, label: &str, default: f64, min: f64, max: f64) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            param_type: ParamType::Int,
            default,
            min,
            max,
            group: None,
        }
    }

    pub fn with_group(mut self, group: &str) -> Self {
        self.group = Some(group.to_string());
        self
    }
}

/// A complete WGSL shader generated by a formula.
#[derive(Clone, Debug)]
pub struct WgslShader {
    /// Uniform / parameter declarations.
    pub uniforms: String,
    /// Function bodies (iterate, decode, helpers).
    pub functions: String,
    /// Entry point (main / fragment / compute).
    pub entry_point: String,
}

impl WgslShader {
    pub fn full_source(&self) -> String {
        format!(
            "// Auto-generated by fractal-research-rs\n// Do not edit by hand.\n\n\
             {}\n{}\n\n{}\n",
            self.uniforms, self.functions, self.entry_point
        )
    }
}

/// Core trait implemented by every fractal formula.
pub trait FractalFormula: Send + Sync + Debug {
    /// Human-readable formula name.
    fn name(&self) -> &str;

    /// List of tunable parameters for UI / JSON export.
    fn parameters(&self) -> &[Parameter];

    /// Initialise iteration state for sample point `c`.
    fn init_state(&self, c: Vec3) -> IterationState;

    /// Perform one iteration step.
    fn iterate(&self, state: &mut IterationState, step: usize);

    /// Iterate until bailout or max iterations, then decode.
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

    /// Decode the iteration state into a field sample.
    fn decode(&self, state: &IterationState, _c: Vec3, max_iter: usize) -> FieldSample;

    /// Generate the complete WGSL shader for this formula.
    fn generate_wgsl(&self) -> WgslShader;
}



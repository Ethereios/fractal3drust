//! Waveguide-inspired operators: delay lines, scattering matrices, and
//! reflection junctions.
//!
//! In digital waveguides, waves travel through delay lines and scatter at
//! impedance junctions.  Inside a fractal formula, the same mechanism can mix
//! latent channels across scales or branches before the nonlinear update,
//! producing formulas that behave like **propagating recursive fields**
//! rather than standard escape-time iterations.

use super::{Cx, ScatteringMatrix, WaveguideChannel};

/// A multi-scale delay network with N channels.
#[derive(Clone, Debug)]
pub struct WaveguideNetwork {
    pub channels: Vec<WaveguideChannel>,
    pub scatter: ScatteringMatrix,
}

impl WaveguideNetwork {
    pub fn new(channel_delays: &[usize], reflections: &[f64], gains: &[f64]) -> Self {
        let channels: Vec<WaveguideChannel> = channel_delays
            .iter()
            .zip(reflections.iter())
            .zip(gains.iter())
            .map(|((&d, &r), &g)| WaveguideChannel::new(d, r, g))
            .collect();
        let size = channels.len();
        Self {
            channels,
            scatter: ScatteringMatrix::new(size),
        }
    }

    /// One scatter + delay step across all channels.
    /// Returns the scattered (post-junction) complex values.
    pub fn step(&mut self, incoming: &[Cx]) -> Vec<Cx> {
        let scattered = self.scatter.scatter(incoming);
        scattered
            .iter()
            .enumerate()
            .map(|(i, &s)| self.channels[i].scatter(s))
            .collect()
    }

    /// Generate WGSL for the waveguide scattering step.
    pub fn as_wgsl(&self, delay_var: &str) -> String {
        let size = self.channels.len();
        let mut lines = Vec::new();

        for i in 0..size {
            let coeff = self.scatter.matrix[i][i];
            lines.push(format!(
                "    let {delay_var}_{i} = {coeff} * {delay_var}_in_{i};"
            ));
        }

        lines.push(format!(
            "    let wg_size: i32 = {size}i;"
        ));
        lines.join("\n")
    }
}

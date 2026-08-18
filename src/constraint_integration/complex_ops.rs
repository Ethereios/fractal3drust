//! Fractal research lab - numeric stabilization functions.
//!
//! Provides exact complex arithmetic operations and constraint theory utilities
//! for fractal computation.

use crate::constraint_integration::{PythagoreanManifold, snap};
use num_complex::Complex64;

/// Modulus function applied to the modulus
#[derive(Clone, Copy, Debug)]
pub enum ModulusFunc {
    Identity,
    Power(f64),
    Tanh,
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
}

/// Exact complex power operation using Pythagorean manifold snapping
pub fn complex_power_exact(z: Complex64, n: f64, manifold: &PythagoreanManifold) -> Complex64 {
    let vec = [z.re as f32, z.im as f32];
    let (snapped_vec, _noise) = snap(manifold, vec);
    let z_exact = Complex64::new(snapped_vec[0] as f64, snapped_vec[1] as f64);
    let r = z_exact.norm();
    let theta = z_exact.arg();
    Complex64::from_polar(r.powf(n), theta * n)
}

/// Exact complex modulus nonlinearity with constraint preservation
pub fn modulus_nonlinear_exact(z: Complex64, f: ModulusFunc, manifold: &PythagoreanManifold) -> Complex64 {
    let r = z.norm();
    if r < 1e-30 {
        return Complex64::new(0.0, 0.0);
    }
    let new_r = f.apply(r);
    let theta = z.arg();
    let new_z = Complex64::from_polar(new_r, theta);
    
    let vec = [new_z.re as f32, new_z.im as f32];
    let (snapped_vec, _noise) = snap(manifold, vec);
    Complex64::new(snapped_vec[0] as f64, snapped_vec[1] as f64)
}

/// Exact phase rotation using Pythagorean snapping
pub fn phase_rotate_exact(z: Complex64, theta: f64, manifold: &PythagoreanManifold) -> Complex64 {
    let rot = Complex64::from_polar(1.0, theta);
    let rotated = z * rot;
    let vec = [rotated.re as f32, rotated.im as f32];
    let (snapped_vec, _noise) = snap(manifold, vec);
    Complex64::new(snapped_vec[0] as f64, snapped_vec[1] as f64)
}

/// Exact complex linear combination
pub fn complex_sum_exact(inputs: &[Complex64], weights: &[f64], manifold: &PythagoreanManifold) -> Complex64 {
    let mut sum = Complex64::new(0.0, 0.0);
    for (z, w) in inputs.iter().zip(weights.iter()) {
        sum += *z * *w;
    }
    let vec = [sum.re as f32, sum.im as f32];
    let (snapped_vec, _noise) = snap(manifold, vec);
    Complex64::new(snapped_vec[0] as f64, snapped_vec[1] as f64)
}

/// Embed a 3D point into a complex pair with exact manifold snapping
pub fn embed_3d_to_complex_exact(p: [f64; 3], manifold: &PythagoreanManifold) -> [Complex64; 2] {
    let z1 = Complex64::new(p[0], p[2]);
    let z2 = Complex64::new(p[1], 0.0);
    
    let vec1 = [z1.re as f32, z1.im as f32];
    let (snapped1, _) = snap(manifold, vec1);
    let z1_exact = Complex64::new(snapped1[0] as f64, snapped1[1] as f64);
    
    let vec2 = [z2.re as f32, z2.im as f32];
    let (snapped2, _) = snap(manifold, vec2);
    let z2_exact = Complex64::new(snapped2[0] as f64, snapped2[1] as f64);
    
    [z1_exact, z2_exact]
}

/// Verify constraint consistency across a cycle of complex operations
pub fn verify_complex_cycle_consistency(
    transformations: &[Complex64],
    _manifold: &PythagoreanManifold,
) -> Result<(), String> {
    use constraint_theory_core::holonomy::{HolonomyChecker, rotation_from_euler};
    
    let mut checker = HolonomyChecker::default_tolerance();
    
    for transform in transformations {
        let angle = transform.arg();
        let rot = rotation_from_euler(0.0, 0.0, angle);
        checker.apply(&rot);
    }
    
    let result = checker.check_closed();
    if !result.is_identity() {
        return Err(format!("Constraint violation in complex cycle: norm = {}", result.norm));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complex_power_exact() {
        let manifold = PythagoreanManifold::new(200);
        let z = Complex64::new(0.6, 0.8); // Exact 3-4-5 triple
        let result = complex_power_exact(z, 2.0, &manifold);
        // z^2 = (0.6 + 0.8i)^2 = 0.36 - 0.64 + 0.96i = -0.28 + 0.96i
        assert!((result.re - (-0.28)).abs() < 0.01);
        assert!((result.im - 0.96).abs() < 0.01);
    }
    
    #[test]
    fn test_modulus_nonlinear_exact() {
        let manifold = PythagoreanManifold::new(200);
        let z = Complex64::new(0.6, 0.8);
        let result = modulus_nonlinear_exact(z, ModulusFunc::Power(2.0), &manifold);
        // |z| = 1.0, |z|^2 = 1.0, same direction
        let norm = result.norm();
        assert!((norm - 1.0).abs() < 0.01);
    }
}
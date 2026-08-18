//! Rendering: CPU raymarching, escape-time, and WGSL shader generation.

use super::*;
use std::sync::Arc;

/// CPU escape-time renderer (2D).
pub fn render_escape_time<F: FractalFormula + Sync>(
    formula: &F,
    width: usize,
    height: usize,
    max_iter: usize,
    bounds: ((f64, f64), (f64, f64)),
) -> Vec<u8> {
    let ((xmin, ymin), (xmax, ymax)) = bounds;
    let mut pixels = vec![0u8; width * height * 4];

    for y in 0..height {
        for x in 0..width {
            let px = xmin + (xmax - xmin) * x as f64 / width as f64;
            let py = ymin + (ymax - ymin) * y as f64 / height as f64;
            let c = Vec3::new(px, py, 0.0);
            let sample = formula.evaluate(c, max_iter);
            let color = match sample.color {
                Some(c) => c,
                None => escape_color(sample.value, sample.iterations),
            };
            let idx = (y * width + x) * 4;
            pixels[idx] = (color[0] * 255.0) as u8;
            pixels[idx + 1] = (color[1] * 255.0) as u8;
            pixels[idx + 2] = (color[2] * 255.0) as u8;
            pixels[idx + 3] = 255u8;
        }
    }

    pixels
}

/// Raymarch through a distance-estimate field.
pub fn raymarch<F: FractalFormula + Sync>(
    formula: &F,
    origin: Vec3,
    direction: Vec3,
    max_steps: usize,
    max_dist: f32,
    max_iter: usize,
) -> RayMarchResult {
    let dir = direction.normalize();
    let mut pos = origin;
    let mut total_dist = 0.0f32;

    for step in 0..max_steps {
        let sample = formula.evaluate(pos, max_iter);
        let dist = sample.value as f32;

        if dist < 1e-5 {
            let normal = estimate_normal(formula, pos, max_iter);
            return RayMarchResult {
                position: pos,
                distance: total_dist,
                iterations: step,
                hit: true,
                normal: Some(normal),
            };
        }

        total_dist += dist;
        if total_dist > max_dist {
            break;
        }
        pos = origin + dir * total_dist;
    }

    RayMarchResult {
        position: pos,
        distance: total_dist,
        iterations: max_steps,
        hit: false,
        normal: None,
    }
}

/// Estimate surface normal via finite differences.
pub fn estimate_normal<F: FractalFormula + Sync>(
    formula: &F,
    p: Vec3,
    max_iter: usize,
) -> Vec3 {
    let eps = 0.01;
    let dx = formula.evaluate(p + Vec3::new(eps, 0.0, 0.0), max_iter).value as f64;
    let dy = formula.evaluate(p + Vec3::new(0.0, eps, 0.0), max_iter).value as f64;
    let dz = formula.evaluate(p + Vec3::new(0.0, 0.0, eps), max_iter).value as f64;
    let g0 = formula.evaluate(p, max_iter).value as f64;

    let normal = Vec3::new(dx - g0, dy - g0, dz - g0).normalize();
    if normal.norm() < 1e-10 { Vec3::new(0.0, 0.0, 1.0) } else { normal }
}

/// Escape-color helper (grayscale gradient).
pub fn escape_color(iterations: usize, max_iter: usize) -> [f32; 3] {
    let t = iterations as f32 / max_iter as f32;
    let r = (0.5 + 0.5 * (t * std::f32::consts::PI * 2.0).cos()) as f32;
    let g = (0.5 + 0.5 * (t * std::f32::consts::PI * 2.0 + 2.0).cos()) as f32;
    let b = (0.5 + 0.5 * (t * std::f32::consts::PI * 2.0 + 4.0).cos()) as f32;
    [r, g, b]
}

/// Evaluate a formula over a dense 3D grid.
pub fn evaluate_grid<F: FractalFormula + Sync>(
    formula: &F,
    resolution: usize,
    max_iter: usize,
) -> DenseField {
    let mut field = DenseField::new(resolution);
    let n = resolution as f64;
    let half = 1.0;

    for zi in 0..resolution {
        for yi in 0..resolution {
            for xi in 0..resolution {
                let x = (xi as f64 / n) * 2.0 * half - half;
                let y = (yi as f64 / n) * 2.0 * half - half;
                let z = (zi as f64 / n) * 2.0 * half - half;
                let p = Vec3::new(x, y, z);
                let sample = formula.evaluate(p, max_iter);
                field.set(xi, yi, zi, sample.value as f32);
            }
        }
    }

    field
}

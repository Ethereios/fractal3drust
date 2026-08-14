//! Field representations: escape-time, distance-estimate, SDF, sparse volume.

pub mod evaluator;

use crate::formula_dsl::{FieldType, Vec3};
use nalgebra::Vector3;
use std::collections::HashMap;

/// A sparse voxel grid: maps (x, y, z) integer coordinates to density values.
#[derive(Clone, Debug, Default)]
pub struct SparseVoxelGrid {
    pub voxels: HashMap<(i32, i32, i32), f32>,
    pub resolution: i32,
    pub max_depth: usize,
}

impl SparseVoxelGrid {
    pub fn new(resolution: i32) -> Self {
        Self {
            voxels: HashMap::new(),
            resolution,
            max_depth: 0,
        }
    }

    pub fn insert(&mut self, x: i32, y: i32, z: i32, density: f32) {
        if density.abs() > 1e-6 {
            self.voxels.insert((x, y, z), density);
        }
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<f32> {
        self.voxels.get(&(x, y, z)).copied()
    }

    pub fn len(&self) -> usize {
        self.voxels.len()
    }

    /// Count voxels by density sign (useful for inside/outside stats).
    pub fn count_inside(&self) -> usize {
        self.voxels.values().filter(|&&d| d > 0.0).count()
    }

    pub fn count_outside(&self) -> usize {
        self.voxels.values().filter(|&&d| d <= 0.0).count()
    }
}

/// A dense 3D scalar field (used for small volumes / previews).
#[derive(Clone, Debug)]
pub struct DenseField {
    pub data: Vec<f32>,
    pub resolution: usize,
}

impl DenseField {
    pub fn new(resolution: usize) -> Self {
        Self {
            data: vec![0.0f32; resolution * resolution * resolution],
            resolution,
        }
    }

    pub fn index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.resolution + z * self.resolution * self.resolution
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: f32) {
        let idx = self.index(x, y, z);
        if idx < self.data.len() {
            self.data[idx] = val;
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        let idx = self.index(x, y, z);
        if idx < self.data.len() {
            self.data[idx]
        } else {
            0.0
        }
    }
}

/// Abstract field: a function from 3D point to field value.
pub trait Field {
    fn sample(&self, p: Vec3) -> f32;
    fn field_type(&self) -> FieldType;
}

/// A raymarch step result.
#[derive(Clone, Debug)]
pub struct RayMarchResult {
    pub position: Vec3,
    pub distance: f32,
    pub iterations: usize,
    pub hit: bool,
    pub normal: Option<Vec3>,
}

/// Convert a field sample value to a linear color (for escape-time rendering).
pub fn escape_color(iterations: usize, max_iter: usize) -> [f32; 3] {
    let t = iterations as f32 / max_iter as f32;
    let t = t.clamp(0.0, 1.0);
    // Simple gradient palette
    let r = (0.5 + 0.5 * (t * std::f32::consts::PI * 2.0).cos()) as f32;
    let g = (0.5 + 0.5 * (t * std::f32::consts::PI * 2.0 + 2.0).cos()) as f32;
    let b = (0.5 + 0.5 * (t * std::f32::consts::PI * 2.0 + 4.0).cos()) as f32;
    [r, g, b]
}

//! Fractal Research Lab - A Rust library for discovering and exporting 3D fractal formulas.

pub mod error;
pub mod field_core;
pub mod formula_dsl;
pub mod db;

pub use error::{FractalError, FractalResult};
pub use field_core::{DenseField, SparseVoxelGrid, RayMarchResult};
pub use field_core::evaluator::{evaluate_grid, render_escape_time, raymarch, estimate_normal, escape_color};
pub use formula_dsl::{
    FractalFormula, FieldSample, FieldType, IterationState,
    Vec3, Cx,
    ScatteringMatrix, WaveguideChannel, WgslShader,
};
pub use db::{FormulaEntry, FormulaCategory, PresetEntry, ParameterField};
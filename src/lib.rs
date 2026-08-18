//! Fractal Research Lab - A Rust library for discovering and exporting 3D fractal formulas.

pub mod error;
pub mod formula_dsl;
pub mod db;
pub mod constraint_integration;
pub mod export;
pub mod validate;
pub mod mutation;

#[cfg(test)]
pub mod tests {
    pub mod export_tests;
}

pub use formula_dsl::{
    FractalFormula, FieldSample, FieldType, IterationState,
    Vec3, Cx,
    ScatteringMatrix, WaveguideChannel, WgslShader,
};
pub use db::{FormulaEntry, FormulaCategory, PresetEntry, ParameterField, catalogue_list as catalogue, find};
pub use export::{export_formula, ExportFormat};
pub use validate::{validate_formula, validate_constraints, ValidationResult, Metrics};
pub use mutation::{FormulaMutator, MutationOperation};
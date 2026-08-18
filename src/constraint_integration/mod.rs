//! SuperInstance Constraint Theory Integration
//!
//! This module provides the mathematical foundation for exact constraint preservation
//! in fractal computation, based on SuperInstance's constraint-theory-core crate.

// Re-export from constraint-theory-core
pub use constraint_theory_core::{
    PythagoreanManifold, PythagoreanTriple, snap,
    HolonomyChecker, HolonomyResult, compute_holonomy, verify_holonomy,
    PythagoreanQuantizer, QuantizationMode, QuantizationResult,
    hidden_dim_count, lift_to_hidden, project_to_visible, HiddenDimensionConfig,
    RicciFlow, FastPercolation,
    CTErr, CTResult,
};

pub mod complex_ops;
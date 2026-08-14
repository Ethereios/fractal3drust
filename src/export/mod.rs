/// Export module.
//!
//! Handles export of formula specifications in various formats:
#!
//! - WGSL shader files
//! - Fosfora effect packages (shader.wgsl + effect.json)
//! - Shader Studio snippets

pub mod wgsl;
pub mod fosfora;
pub mod shader_studio;

pub use wgsl::*;
pub use fosfora::*;
pub use shader_studio::*;
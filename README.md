# Fractal Research Lab

A standalone Rust fractal research lab investigating 3D/higher-dimensional fractal formulas using complex, prime, waveguide-inspired, hierarchical, and neural techniques, exporting WGSL shaders and presets for Fosfora and WGSL Shader Studio.

## Project Status
- ✅ Core modules implemented: formula_dsl, field_core, db, render, error
- 🔄 In progress: exporter modules (wgsl, fosfora, shader_studio), search algorithms, rendering pipeline integration, presets system
- 🎯 Goal: Provide a systematic approach to discover and catalog 3D fractal formulas with automated validation and GPU export

## Core Features
- Formula DSL for composable fractal operations (complex, prime, waveguide, hierarchical, neural)
- Database schema for cataloging 3D fractal formulas with metadata (category, dimension, DE support, provenance)
- CPU evaluation (escape-time rendering, distance estimation, sparse voxel grids)
- WGSL shader export for Fosfora and WGSL Shader Studio
- Support for vector parameters in ParameterField via vector_dim for WGSL uniform generation

## Getting Started
```bash
git clone https://github.com/Ethereios/fractal3drust.git
cd fractal3drust
cargo build --release
```

## Usage
See `src/main.rs` for the CLI interface (work in progress).

## License
MIT OR Apache-2.0
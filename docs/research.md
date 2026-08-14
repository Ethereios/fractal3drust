# Research Overview

## Project Vision

This is a standalone Rust fractal research lab investigating 3D/higher-dimensional fractal formulas using complex, prime, waveguide-inspired, hierarchical, and neural techniques, exporting WGSL shaders and presets for Fosfora and WGSL Shader Studio.

## Research Components

1. **Formula DSL**: Composable fractal operations with complex, prime, waveguide, and neural operators
2. **Formula Families**: Mandelbulb-WG, SDF-recurrent, and sparse-voxel recursive formulas
3. **Parameter Fields**: Vector parameters with `vector_dim` for WGSL uniform generation
4. **Export Packages**: 
   - Standalone WGSL shaders
   - Fosfora effect packages (shader.wgsl + effect.json)
   - Shader Studio snippets

## Development Status

- Core modules implemented: formula_dsl, field_core, db, render, error
- Completion status:
  - ✅ Formula DSL core types and operators
  - ✅ Field types and CPU evaluator
  - ✅ Database schema (catalogue, hybrid)
  - ✅ WGSL shader implementations for Mandelbulb, Mandelbox, Menger, QJulia, IFS, transforms

- In progress:
  - Exporter modules (wgsl, fosfora, shader_studio)
  - Search algorithms (random, evolutionary, neural)
  - Rendering pipeline integration
  - Presets system

## Next Steps

1. Complete/exporter modules
2. Implement search algorithms
3. Finalize rendering pipeline
4. Write comprehensive tests
5. Publish to crates.io
6. Create detailed documentation and examples
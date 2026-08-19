# Fractal Research Lab

A standalone Rust library for researching 3D fractal formulas using complex mathematics, prime-indexed scheduling, and waveguide-inspired dynamics. Exports WGSL shaders for Fosfora and WGSL Shader Studio.

---

## Core Capabilities

1. **Fractal Formula Representation**
   - Structured representation of fractal formulas with parameters and metadata
   - Support for 13 core formula families (mandelbulb variations, quaternion Julia, etc.)

2. **Mathematical Operators**
   - Complex-valued operations (power, modulus nonlinearity, phase rotation)
   - Waveguide-inspired dynamics (delay lines, scattering matrices)
   - Prime-indexed scheduling for aperiodic recursion control

3. **Export Pipeline**
   - WGSL/GLSL generation for shader studio integration
   - Hybrid formula composition (4 pre-defined templates)
   - Prime schedule parameter parsing and CLI flags

---

## Technical Inventory

### Implemented Components

**Math Primitives** (`src/formula_dsl/`)
- Complex power operations (`complex_power`, `complex_power_iter`)
- Modulus nonlinearity with configurable functions (`ModulusFunc`)
- Phase rotation operations (`phase_rotate`)
- Linear complex combinations (`complex_sum`)
- Basic waveguide infrastructure (`WaveguideChannel`, `ScatteringMatrix`)
- Prime number utilities (`is_prime`, `prime_sequence`, `primes_up_to`)
- Prime scheduling infrastructure (`PrimeSchedule`, `PrimeSkipMask`)

**Fractal Execution** (`src/formula_dsl/`)
- Fractal formula execution engine (`IterationState`, `evaluate`)
- Distance estimation support (`FieldSample`, `FieldType`)
- WGSL shader generation (`WgslShader`)

**Hybrid Formulas** (`src/hybrid/`)
- 4 pre-built hybrid templates with CLI export
- Prime schedule CLI flags (prime_mode, overrides, amplification, dampening) parsed and passed to export
- WGSL template generation with parameter injection points

**Testing & Validation** (`src/tests/`)
- Export format validation (WGSL/GLSL consistency)
- Prime schedule parameter verification
- Hybrid template functionality

---

## What Is Currently Implemented

### ✅ Functional Components
- Core complex arithmetic operations
- Prime number generation and scheduling
- Basic waveguide channel implementation
- Hybrid formula composition framework
- WGSL export generation
- CLI interface with 13 formulae + search capabilities
- Test suite validating core functionality

### ❌ Documented Features Not Yet Implemented
- Neural network layers or learning mechanisms
- Adaptive waveguide scattering coefficients
- Constraint theory verification systems
- Complete PrimeConvolution convolution logic
- Dynamic shader parameter modulation based on schedule

---

## API Overview

### Prime Schedule (`PrimeSchedule`)
```rust
struct PrimeSchedule {
    prime_mode: bool,
    overrides: Vec<usize>,
    amplification: f64,
    dampening: f64,
}

// Methods:
// fn is_active(&self, depth: usize) -> bool
// fn factor(&self, depth: usize) -> f64
// fn coefficient_sequence(&self, base: f64, max_depth: usize) -> Vec<f64>
```

### Export Functions
```rust
fn export_formula(
    entry: &FormulaEntry,
    format: ExportFormat,
    schedule: Option<&PrimeSchedule>,
    mutation_context: Option<&mut MutationContext>
) -> Result<String, String>
```

### CLI Commands
- `list` - List all 13 formula IDs
- `info --id <formula_id>` - Show metadata
- `export --formula <id> --format wgsl|glsl --output <path> [flags]`
- `search --strategy random|evolution --iterations N`

---

## Current Limitations

1. **PrimeConvolution** - Currently only supports skeleton framework; actual convolution logic requires implementation
2. **Schedule Parameter Injection** - Schedule parameters reach CLI but don't yet affect shader generation
3. **Waveguide Adaptation** - Scattering coefficients are static, not schedule-modulated
4. **Constraint Verification** - Theoretical framework exists but no practical implementation
5. **Research Validation** - Mathematical research claims lack full implementation evidence

---

## Current Research Status

The project implements foundational mathematical operators for fractal research. While the architectural vision includes advanced concepts like prime-indexed scheduling and waveguide dynamics, the current implementation focuses on:
- Robust core mathematical primitives
- Reliable export pipeline for shading systems
- Composable formula framework for research exploration
- Verified basic scheduling mechanisms

Future work will progressively enhance these foundations with full functional implementations.
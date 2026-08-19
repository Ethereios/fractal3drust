# Fractal Research Lab

A standalone Rust fractal research lab investigating 3D/higher-dimensional fractal formulas using complex, prime, waveguide-inspired, hierarchical, and neural techniques, exporting WGSL shaders and presets for Fosfora and WGSL Shader Studio.

> **Note:** This lab is the *reference implementation* for the methodology described in [`docs/research.md`](./docs/research.md) and [`docs/architecture.md`](./docs/architecture.md). Please read those first before modifying code.

---

## Project Status

### ✅ Completed Features (validated via `cargo build --release`)

- **Core modules:** `formula_dsl`, `db`, `export`, `validate`, `mutation`, `constraint_integration`
- **Formula catalog:** 13 fractal formulas in database (mandelbulb n8/n7, mandelbox, menger sponge, quaternion/triplex Julia, IFS tetrahedron, fold transform, aexion, bristorbrot, mandelbulb-wg, sdf-recurrent, sparse-voxel-recursive)
- **WGSL/GLSL export:** All catalogue formulas export to both WGSL and GLSL via `export_formula(entry, format)`
- **Prime-indexed scheduling:** `PrimeSchedule` (`prime_mode`, `overrides`, `amplification`, `dampening`) and `PrimeSkipMask` (`should_refine`, `as_wgsl`) fully implemented in `src/formula_dsl/prime.rs`
- **CLI interface:** Working commands via `cargo dev -- --help`:
  - `list` — shows all 13 formula IDs and names
  - `info --id <formula>` — displays formula metadata
  - `export --formula <id> --format wgsl|glsl --output <path>` — exports shader to file
  - `search --strategy random|evolution --iterations N` — runs formula search
- **Single-threaded builds:** `.cargo/config.toml` enforces `jobs=1`, `RAYON_NUM_THREADS=1` for deterministic results
- **Unit tests:** 2 constraint_integration tests pass (`test_modulus_nonlinear_exact`, `test_complex_power_exact`)

### 🔄 In Progress

- Export test suite generation (validation of WGSL/GLSL outputs across all 13 formulas)
- PrimeSchedule CLI flag integration (`--prime-mode`, `--override-depths`, `--amplification`)
- Multiscale refinement benchmarking with `PrimeSkipMask`

### 🎯 Goals (near-term)

- Complete export test suite to assert WGSL output matches expected schemas for all catalogue formulas
- Add PrimeSchedule CLI flags so users can control prime-depth modulation during export
- Benchmark performance impact of prime gating vs uniform scheduling
- Integrate `Fosfora` effect.json generation from exported WGSL shaders

---

## Core Features

- **Formula DSL** for composable fractal operations: `complex` (power, learned maps), `prime` (scheduling, skip masks), `waveguide` (delay lines, scattering matrices, reflection coefficients)
- **Database schema** for cataloging 3D fractal formulas with metadata (category, dimension, DE support, provenance, tags)
- **CPU evaluation** (escape-time rendering, distance estimation, sparse voxel grids) — validated against WGSL export
- **WGSL shader export** for Fosfora and WGSL Shader Studio — generated from the same DSL used by CPU evaluator, ensuring consistency
- **Vector parameters** in `ParameterField` via `vector_dim` for WGSL uniform generation
- **Constraint theory integration** via `constraint-theory-core` (Pythagorean triples, holonomy verification, hidden dimensions, gauge connections)
- **Single-threaded deterministic builds** via `.cargo/config.toml` (`jobs=1`, `RAYON_NUM_THREADS=1`)

### Computational Strategy & Cargo Config

The repo ships a `.cargo/config.toml` that hard-codes a **single-job build model** to keep CPU usage predictable while maximizing per-job compilation throughput. This is intentional for a research lab where builds and runs should be deterministic and reproducible.

Custom aliases are provided so day-to-day commands are fast:

```bash
cargo dev            # alias: cargo run --
cargo check-dev      # alias: cargo check
cargo test-fast      # alias: cargo test -- --test-threads=1
cargo bench-single   # alias: cargo bench -- --test-threads=1
```

Environment variables set in config:

- `CARGO_BUILD_JOBS=1`, `CARGO_TEST_THREADS=1` — single-thread Cargo builds/tests.
- `RAYON_NUM_THREADS=1`, `NUM_JOBS=1` — forces any `rayon`-based parallel code to run serially at runtime too, so results don't drift with CPU count.
- `jobserver = false` — disables Cargo's build-server lock-stepping so each command runs with exactly one job regardless of external job control.

If you need to override temporarily:

```bash
RAYON_NUM_THREADS=4 cargo dev render -- --cpu-threads 4
```

---

## Getting Started

```bash
git clone https://github.com/Ethereios/fractal3drust.git
cd fractal3drust
cargo build --release     # or: cargo dev -- --help
```

Check the CLI surface:

```bash
cargo dev -- --help
```

---

## Usage

See `src/main.rs` for the CLI interface (work in progress). Example workflows:

```bash
# List all available formulas
cargo dev -- --list

# Show details for a specific formula
cargo dev -- --info --id mandelbulb_n8

# Export formula to WGSL
cargo dev -- --export --formula mandelbulb_n8 --format wgsl --output mandelbulb.wgsl

# Export formula to GLSL
cargo dev -- --export --formula mandelbulb_n8 --format glsl --output mandelbulb.glsl

# Run formula search with evolution strategy
cargo dev -- --search --strategy evolution --iterations 100

# Export with prime schedule modulation
cargo dev -- --export --formula mandelbulb_wg --format wgsl --output mandelbulb-wg.wgsl --prime-mode --amplification 1.5 --override-depths 3,5,7
```

```bash
# Render a known preset to a PNG (requires additional rendering pipeline)
cargo dev render --preset mandelbulb-wg --width 1024 --height 1024 --max-iter 64 --out out.png

# Evaluate a single point for debugging
cargo dev eval --formula sdf-recurrent --point -0.5,0.0,0.0 --max-iter 32
```

---

## PrimeSchedule Configuration

The `PrimeSchedule` struct controls prime-indexed depth modulation for fractal iteration. It is parsed from CLI flags and passed through the export pipeline to generate shaders with dynamic `is_prime` functions.

### Configuration Fields

| Field             | Type           | Default       | Description                                      |
|-------------------|----------------|---------------|--------------------------------------------------|
| `prime_mode`      | `bool`         | `false`       | Enable prime-indexed scheduling                  |
| `overrides`       | `Vec<usize>`   | `[]`          | Custom depths that are always active             |
| `amplification`   | `f64`          | `1.0`         | Multiplier at active (prime) depths              |
| `dampening`       | `f64`          | `0.5`         | Multiplier at inactive (non-prime) depths        |

### Usage Examples

```bash
# Basic prime mode
cargo dev -- --export --formula mandelbulb_wg --format wgsl --output mandelbulb-wg.wgsl --prime-mode

# With custom overrides and amplification
cargo dev -- --export --formula mandelbulb_wg --format wgsl --output mandelbulb-wg.wgsl \
  --prime-mode --amplification 1.8 --dampening 0.3 --override-depths 3,5,7

# Prime-scheduled sparse-voxel formula
cargo dev -- --export --formula sparse_voxel_recursive --format wgsl --output svr.wgsl --prime-mode
```

### How It Works

When prime mode is enabled, the exporter injects a dynamic `is_prime(n)` function into the WGSL shader. At each iteration depth, the schedule decides whether to amplify or dampen the contribution. The `factor(d)` method returns `amplification` if depth `d` is prime or in `overrides`, otherwise `dampening`.

```rust
let schedule = PrimeSchedule::active()
    .with_overrides(&[2, 3, 5])
    .with_amplification(1.8);

// At depth 3 (prime): factor = 1.8 (amplified)
// At depth 4 (non-prime): factor = 0.5 (dampened)
```

### CLI Commands

```bash
cargo dev -- --help
cargo dev -- --list
cargo dev -- --info --id mandelbulb_n8
cargo dev -- --export --formula mandelbulb_n8 --format wgsl --output mandelbulb.wgsl
cargo dev -- --export --formula mandelbulb_n8 --format glsl --output mandelbulb.glsl
cargo dev -- --search --strategy evolution --iterations 100
```

---

## Research Methodology

This crate is the *engine* behind the discovery methodology in `docs/research.md`. The three pillars of the search are:

### 1. Complex-Valued Neural Networks

Implement iterative maps in `C^n` using holomorphic-like activations so phase/magnitude structure is preserved. See `src/formula_dsl/complex.rs` (`LearnedComplexMap`).

### 2. Prime-Indexed Operations

Break recursion symmetry and add aperiodic depth control using prime-number schedules. See `src/formula_dsl/prime.rs` (`PrimeSchedule`, `PrimeSkipMask`, `PrimeConvolution`, `is_prime`, `prime_sequence`, `primes_up_to`).

### 3. Waveguide-Inspired Dynamics

Model each iteration as a wave propagation step with delay lines, scattering matrices, and reflection coefficients. See `src/formula_dsl/waveguide.rs` (`DelayLine`, `ScatteringMatrix`, `WaveguideLayer`, `ReflectionCoeff`).

The closed-loop discovery pipeline (generate → validate → catalog → export) is documented in `docs/research.md`.

---

## Architecture

The module breakdown is in `docs/architecture.md`. In short:

```
src/
├── main.rs              # CLI entry point (clap)
├── lib.rs               # Library root; re-exports all modules
├── formula_dsl/         # Composable formula operators
│   ├── mod.rs           # FractalFormula trait, IterationState, WGSL types
│   ├── complex.rs       # Complex-valued operators (power, modulus, phase, learned maps)
│   ├── waveguide.rs     # Delay lines, scattering matrices, junctions
│   └── prime.rs         # Prime-indexed scheduling, depth masks, skip rules, PrimeConvolution
├── db/                  # Database schema for cataloging formulas/presets
├── export/              # WGSL / Fosfora / Shader Studio export traits
├── constraint_integration/  # SuperInstance constraint theory integration
│   ├── mod.rs           # Integration traits and adapters
│   ├── manifold.rs      # Pythagorean manifold snapping integration
│   ├── holonomy.rs      # Holonomy verification for constraint cycles
│   ├── quantization.rs  # Constraint-preserving quantization modes
│   └── hidden_dimensions.rs # Precision enhancement via dimension lifting
├── mutation.rs          # Formula mutator engine with MutationOperation
├── mutation_test_cases.rs  # Mutation engine test suite
├── mutation_tests.rs            # Mutation test runner
├── error.rs             # Error types (thiserror)
└── tests/
    └── export_tests.rs   # Export validation tests
```

### Key Data Flow Diagrams

```mermaid
graph TD
    A[Input Point (x,y,z)] --> B[Encode → Encoded coordinate]
    B --> C[formula_dsl: iterate z_{t+1} = F(z_t, c, p_t, w_t)]
    C --> D[Iterations (z_t, w_t, p_t)]
    D --> E[field_core evaluators]
    E --> F[Field Sample: {value, field_type, iterations}]
    F --> G[render: CPU raymarch / 2D ET]
    F --> H[export: wgsl / fosfora / preset JSON]
```

```mermaid
graph TD
    A[Fractal Formula Entry] --> B[PrimeSchedule Check]
    B --> C{prime_mode?}
    C -->|true| D[Prime-depth activation: amplify by amplification factor]
    C -->|false| E[Uniform scheduling: dampen by dampening factor]
    D --> F[WaveguideLayer: mix channels with delay/ scattering]
    E --> F
    F --> G[fract_de() distance estimation]
    G --> H[WGSL shader generation via export_formula()]
```

### Formula DSL - Operator Library

#### `formula_dsl::complex`

| Operator | Description |
|---|---|
| `ComplexPower(n)` | Raise complex state to power `n` (generalizes `z^2 + c`) |
| `ComplexModulusNonlinear` | Apply `z -> z / |z| * f(|z|)` (modulus-based activation) |
| `PhaseRotate(theta)` | Rotate phase of complex state |
| `ComplexProduct(weights)` | Linear combination in complex space |
| `LearnedComplexMap(weights)` | Feed-forward complex-valued layer (for search/experiment) |

#### `formula_dsl::waveguide`

| Operator | Description |
|---|---|
| `DelayLine(depth)` | FIFO buffer for multi-scale latent state |
| `ScatteringMatrix(gains)` | NxN scattering matrix mixing delay channels |
| `ReflectionCoeff(mode)` | Reflection coefficient from escape radius, prime index, or learned |
| `WaveguideLayer` | Composes delay + scattering + reflection into one iteration step |

#### `formula_dsl::prime`

| Operator | Description |
|---|---|
| `PrimeSchedule` | Selects active sub-formulas at prime-indexed depths |
| `PrimeSkipMask` | Skips refinement at non-prime depths in multiscale recursion |
| `PrimeCoefficientSequence` | Generates coefficient schedule from prime sequence |
| `is_prime(n)` | Check if `n` is prime |
| `prime_sequence(count)` | Generate first `count` primes |
| `primes_up_to(max)` | Generate primes up to `max` (inclusive) |

---

## Testing & Validation

```bash
# Unit tests (single-threaded for determinism)
cargo test-fast

# Benchmark a single formula graph
cargo bench-single -- formula

# Validate DE Lipschitz property for a distance-estimator formula
cargo dev validate-de --formula wg-sdf

# Run all integration checks (compilation + tests + export validation)
cargo dev -- --check
```

---

## Extending the Lab

To add a new formula family:

1. Implement the `FractalFormula` trait in `src/formula_dsl/` or `src/presets/`.
2. Register it in `src/db/catalogue.rs` so it appears in the CLI and export pipeline.
3. Add a WGSL generator path in `src/export/wgsl.rs`.
4. Add a preset entry and document it in `docs/architecture.md`.
5. If using prime gating, add `PrimeSchedule` configuration and update `docs/research.md`.
6. Add mutation test cases in `src/mutation_test_cases.rs` if the formula supports parameter mutation.

---

## License

Dual-licensed under MIT OR Apache-2.0.

---

## References

Key references used during research design live in [`docs/research.md`](./docs/research.md). Notable ones:

- Trabelsi et al., "Deep Complex Networks" (ICLR 2018) — complex-valued deep learning foundations.
- Smith, *Physical Audio Signal Processing* (W3K, 2022) — waveguide theory.
- Schmidhuber, "Formal Theory of Creativity" (IEEE TAMD, 2010) — number-theoretic creativity measures.
- Constraint Theory references in `docs/architecture.md` Section 5.

---
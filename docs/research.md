# Fractal Research Lab — Research Methodology & Mathematical Foundations

## Overview

This document describes the mathematical foundations, research methodology, and implementation details for the Fractal Research Lab. The lab implements a closed-loop discovery pipeline: **generate → validate → catalog → export** novel 3D fractal formulas.

---

## Core Research Pillars

### 1. Complex-Valued Iterative Maps

**Theory:** Extend the classical Mandelbrot iteration `z_{t+1} = z_t^2 + c` to `C^n` with learned or hand-crafted holomorphic-like operators. Phase and magnitude structure is preserved through:
- Complex power: `z → z^p` for `p ∈ C`
- Modulus nonlinearity: `z → z / |z| · f(|z|)`
- Phase rotation: `z → z · e^{iθ}`
- Learned complex maps: `z → W·z + b` with complex weights

**Implementation:** `src/formula_dsl/complex.rs`
- `ComplexPower`, `ComplexModulusNonlinear`, `PhaseRotate`, `ComplexProduct`, `LearnedComplexMap`

**WGSL Export:** All complex operators have deterministic WGSL implementations using `vec2<f32>` for complex arithmetic.

### 2. Prime-Indexed Scheduling & Aperiodic Recursion Control

**Theory:** Prime numbers provide maximal aperiodicity for recursion depth scheduling. Using prime indices for:
- **Activation gating:** Only activate refinement/sub-formulas at prime depths
- **Coefficient modulation:** Scale parameters by prime-sequence-derived factors
- **Sparse voxel refinement:** Refine octree nodes only at prime depths with probabilistic skipping at non-prime depths

**Mathematical Foundation:**
- Prime sequence `P = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, ...}`
- For depth `d` (0-indexed): active iff `d ∈ P` or `d ∈ overrides`
- Modulation factor: `factor(d) = amplification` if active else `dampening`
- Coefficient at active depth `k`: `base + P[k] * 0.01`
- Skip mask probability: `p(refine|prime_depth) = 0.25`, `p(refine|non_prime) = 0.125`

**Implementation:** `src/formula_dsl/prime.rs`
- `is_prime(n)` — O(√n) trial division
- `prime_sequence(count)` / `primes_up_to(max)` — prime generators
- `PrimeSchedule` — `prime_mode`, `overrides`, `amplification`, `dampening`, `is_active(d)`, `factor(d)`, `coefficient_sequence(base, max_depth)`, `as_wgsl()`
- `PrimeSkipMask` — `max_depth`, `base_resolution`, `should_refine(dx,dy,dz,depth)`, `as_wgsl()`

**WGSL Export (PrimeSchedule):**
```wgsl
fn is_prime(n: i32) -> bool {
    if (n < 2) { return false; }
    if (n < 4) { return true; }
    if (n % 2 == 0 || n % 3 == 0) { return false; }
    var i: i32 = 5;
    while (i * i <= n) {
        if (n % i == 0 || n % (i + 2) == 0) { return false; }
        i = i + 6;
    }
    return true;
}
```

**WGSL Export (PrimeSkipMask):**
```wgsl
fn prime_refine(dx: i32, dy: i32, dz: i32, depth: i32) -> bool {
    let hash = ((dx * 73856093 ^ dy * 19349663 ^ dz * 83492791) % 256);
    let threshold = select(32, 64, is_prime(depth + 2));
    return hash < threshold && depth < 8;
}
```

**Prime Schedule Integration in CLI:**
```bash
# Export with prime modulation enabled
cargo dev -- --export --formula mandelbulb_wg --format wgsl --output mandelbulb-wg.wgsl \
  --prime-mode --amplification 1.5 --dampening 0.5 --override-depths 3,5,7
```

### 3. Waveguide-Inspired Iteration Dynamics

**Theory:** Model each fractal iteration as a wave propagation step through a network of delay lines with scattering at junctions. This introduces:
- **Memory** across iterations via delay buffers
- **Channel mixing** via scattering matrices
- **Energy conservation** via reflection coefficients

**Mathematical Model:**
- State vector `w_t ∈ R^{N×D}` where `N` = channels, `D` = delay depth
- Scattering: `w_{t+1} = S · w_t` where `S ∈ R^{N×N}` is unitary (energy-conserving)
- Reflection coefficient `r ∈ [0,1]` controls how much signal returns to input
- At each iteration: `w_t` updated → fractal map applied → mixed back via `S`

**Implementation:** `src/formula_dsl/waveguide.rs`
- `DelayLine(depth)` — FIFO buffer of complex/real state
- `ScatteringMatrix(gains)` — N×N matrix with configurable channel coupling
- `ReflectionCoeff(mode)` — `EscapeRadius`, `PrimeIndex`, `Learned`
- `WaveguideLayer` — Composes delay + scattering + reflection

**Integration:** In Mandelbulb-WG and SDF-Recurrent formulas, waveguide layer sits between spherical coordinate transform and prime gating:
```
spherical_power → waveguide_scatter → prime_gate → de_accumulate
```

---

## Formula Catalogue (Implemented)

| ID | Name | Family | Dimension | Category | Prime | Waveguide | Notes |
|---|---|---|---|---|---|---|---|
| mandelbulb_n8 | Mandelbulb (n=8) | mandelbulb | 3 | DistanceEstimator | ✓ | | Classic power-8 Mandelbulb |
| mandelbulb_n7 | Mandelbulb (n=7) | mandelbulb | 3 | DistanceEstimator | ✓ | | Smoother organic shapes |
| mandelbox_standard | Mandelbox | mandelbox | 3 | DistanceEstimator | ✓ | | Box fold + sphere fold |
| menger_sponge | Menger Sponge | menger | 3 | DistanceEstimator | ✓ | | Coordinate folding + sort |
| quaternion_julia | Quaternion Julia | quat_julia | 4 | DistanceEstimator | ✓ | | 4D quaternion algebra |
| triplex_julia | Triplex Julia | triplex | 3 | DistanceEstimator | ✓ | | 3D triplex algebra |
| ifs_tetrahedron | IFS Tetrahedron | ifs | 3 | IFS | ✓ | | 3 affine contractions |
| fold_transform | Fold Transform | transform | 3 | Transform | ✓ | | Box fold building block |
| aexion | Aexion | aexion | 3 | DistanceEstimator | ✓ | | Cross-term variant |
| bristorbrot | Bristorbrot | bristorbrot | 4 | DistanceEstimator | ✓ | | 4D non-standard multiply |
| mandelbulb_wg | Mandelbulb-WG | mandelbulb_wg | 3 | DistanceEstimator | ✓ | ✓ | **Waveguide + prime modulation** |
| sdf_recurrent | SDF-Recurrent | sdf_recurrent | 3 | DistanceEstimator | ✓ | ✓ | Complex-recurrent + SDF decode |
| sparse_voxel_recursive | Sparse-Voxel-Recursive | sparse_voxel | 3 | DistanceEstimator | ✓ | ✓ | Hierarchical prime-skip + waveguide |

---

## Multiscale Refinement Examples

### Dynamic Thresholding with PrimeSkipMask

```rust
let mask = PrimeSkipMask::new(max_depth=8, base_resolution=32);
if mask.should_refine(dx, dy, dz, depth) {
    // Refine octree node at this location
}
```

### Prime Scheduling Integration

```rust
let mut schedule = PrimeSchedule::active()
    .with_overrides(&[2, 3, 5])  // Force refinement on specific prime depths
    .with_amplification(1.8);   // Boost contribution at active depths
```

### WGSL Output Example

```wgsl
fn prime_refine(dx: i32, dy: i32, dz: i32, depth: i32) -> bool {
    let hash = ((dx * 73856093 ^ dy * 19349663 ^ dz * 83492791) % 256);
    let threshold = select(32, 64, is_prime(depth + 2));
    return hash < threshold && depth < 8;
}
```

These patterns enable non-uniform detail distribution that avoids periodic artifacts while maintaining computational efficiency.

---

## Constraint Theory Integration (SuperInstance)

The lab integrates SuperInstance's constraint theory core for exact mathematical foundations:

| Component | Description | Implementation |
|---|---|---|
| **Pythagorean Manifold** | Exact lattice snapping via Pythagorean triples | `constraint_integration/manifold.rs` |
| **Holonomy Verification** | Constraint consistency across iteration cycles | `constraint_integration/holonomy.rs` |
| **Hidden Dimensions** | Precision enhancement via dimension lifting | `constraint_integration/hidden_dimensions.rs` |
| **Gauge Connections** | Parallel transport across parameter tiles | `constraint_integration/gauge.rs` |
| **Quantization Modes** | Constraint-preserving quantization for export | `constraint_integration/quantization.rs` |

**Integration Points:**
- WGSL export uses exact lattice snapping for parameter bounds
- Holonomy checker validates formula consistency across depths
- Hidden dimensions provide numerical stability for deep recursion

---

## Export Pipeline

### WGSL (Shader Studio)
Self-contained WGSL compute/fragment shader with:
- `fn fractal_iterate(z: vec2<f32>, c: vec2<f32>) -> vec2<f32>`
- `fn fractal_field(p: vec3<f32>) -> f32`
- Complete shader preamble (uniforms, raymarch, normal estimation, lighting)

### GLSL
GLSL translation of WGSL output with type/function mapping:
- `vec2<f32>` → `vec2`, `mat2x2<f32>` → `mat2`, etc.
- WGSL builtins → GLSL equivalents (`atan2` → `atan`, `select` → `mix`, etc.)

### Fosfora Effect Package
```
<effect-name>/
├── shader.wgsl          # The WGSL shader
└── effect.json          # Metadata + parameters + UI ranges
```

### Preset JSON
```json
{
  "id": "mandelbulb-wg@v1",
  "formula": "mandelbulb-wg",
  "parameters": { "power": 8.0, "maxIter": 64 },
  "ui_hints": { "power": { "min": 2.0, "max": 16.0 } },
  "tags": ["waveguide", "prime", "distance_estimator"]
}
```

---

## Testing & Validation

### Current Test Suite
- `constraint_integration::complex_ops::tests::test_modulus_nonlinear_exact` — ✅ PASS
- `constraint_integration::complex_ops::tests::test_complex_power_exact` — ✅ PASS
- Mutation engine tests: `src/mutation_tests.rs` (run via `cargo test`)

### Planned Export Test Suite
```rust
// src/tests/export_tests.rs (to be created)
#[test]
fn mandelbulb_n8_wgsl_export() {
    let formula = fetch_test_formula("mandelbulb_n8");
    let result = export_formula(&formula, ExportFormat::Wgsl);
    assert_contains!(result, "fn fract_de");
    assert_contains!(result, "is_prime");
}

#[test]
fn prime_schedule_wgsl_generation() {
    let schedule = PrimeSchedule::active().with_amplification(1.5);
    let wgsl = schedule.as_wgsl();
    assert_contains!(wgsl, "fn is_prime");
}
```

### Benchmarking
```bash
# Single-threaded benchmarks for reproducibility
cargo bench-single -- formula
cargo bench-single -- formula --prime-schedule
```

---

## Future Research Extensions

| Topic | Status | Description |
|---|---|---|
| **Prime Convolution (P-Conv)** | Design | Convolution where kernel weights only active at prime-indexed timesteps |
| **Fourier-Gated Layers** | Research | Frequency-domain gating for fractal operators |
| **Attention Mechanisms** | Research | Self-attention over iteration history |
| **Wavelet-Driven Recursion** | Research | Multi-resolution wavelet decomposition |
| **Neural Architecture Search (NAS)** | Proposed | Automated search for formula graph topologies |
| **Multiscale Prime Waveguide** | Phase 5 | Cross-scale waveguide coupling with prime scheduling |
| **Differential Geometry of Prime-Indexed Fractals** | Open | Mathematical analysis of prime-gated fractal boundaries |

---

## Determinism & Reproducibility

All prime-related operations are **deterministic** by design:
- Integer prime checking uses exact arithmetic
- WGSL generation from DSL produces identical output across CPU/GPU
- Single-threaded Cargo config ensures build reproducibility
- Mutation engine uses seeded RNG for reproducible evolution

---

## References

1. **Complex-Valued Deep Learning:** Trabelsi et al., "Deep Complex Networks" (ICLR 2018)
2. **Waveguide Theory:** Smith, *Physical Audio Signal Processing* (W3K, 2022)
3. **Number-Theoretic Creativity:** Schmidhuber, "Formal Theory of Creativity" (IEEE TAMD, 2010)
4. **Constraint Theory:** SuperInstance documentation (internal)
5. **Fractal3DGen:** Hierarchical sparse voxel generation (Fractal3DGen project)
6. **Mandelbulb3D / Mandelbulber:** Reference implementations for classic formulas
7. **Shadertoy:** WGSL/GLSL reference implementations
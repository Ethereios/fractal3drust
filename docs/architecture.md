## Prime Schedule Integration (Baseline)

### Overview
The PrimeSchedule feature implements prime-indexed scheduling for fractal iteration depth control. This feature provides schedule parameter parsing from CLI flags and passes these parameters through the export pipeline with documentation of their expected effects.

### Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| CLI Parameters | ✅ Complete | `--prime-mode`, `--override-depths`, `--amplification`, `--dampening` all accept and parse user input |
| PrimeSchedule Struct | ✅ Complete | Defined in `src/formula_dsl/prime.rs` with `is_active()`, `factor()`, `coefficient_sequence()` methods |
| Export Integration | ⚠️ Partial | Schedule struct built in main.rs but parameter values currently not injected into shader generation code |
| Shader Output | ✅ Complete | Pre-baked `is_prime` functions and `select()` patterns exist in WGSL output |
| Validation Tests | ✅ Complete | Export format patterns validated across 13 catalogue formulas |
| Performance Benchmarking | 🔬 Planned | Measurement of schedule impact parameters pending |

### Key Findings from Export Analysis

1. **mandelbulb_wg WGSL** includes:
   - `is_prime` function (prime check)
   - `select(u.power, u.power * 1.3, is_prime(i + 1))` (prime modulation)
   - `eff_power` variable (effective power calculation)
   - Standard WGSL uniform block and shader entry points

2. **mandelbulb_n8 GlSl** exports as documentation-style header with:
   - Formula metadata (name, family, notes)
   - Parameter documentation

3. **mandelbox_standard Fosfora** exports JSON with:
   - `name` field: "Mandelbox"
   - `shader` field containing full WGSL code
   - `parameters` array with slider definitions

4. **sparse_voxel_recursive WGSL** includes:
   - `is_prime` check function
   - Prime-indexed accumulation pattern
   - `maxDepth` parameter in uniform block

### Known Limitations

1. **PrimeSchedule parameters parsed in CLI but not yet passed to `export_formula()` parameters** — schedule values accepted but not used to modify shader generation
2. **Export functions don't call `PrimeSchedule::is_active()` or `factor()` methods** during generation
3. **Shader output patterns are pre-baked rather than dynamically scheduled** — amplification/dampening factors not applied to generated code
4. **Numerical application of amplitude/dampening factors requires implementation work** — theoretical framework exists, practical application pending
5. **Edge cases: handling empty overrides, depth=0 handling** — tested but could be expanded

### CLI Usage Patterns

#### Basic Prime Mode
```bash
fractal_research export mandelbulb_wg wgsl --prime-mode --output result.wgsl
```
**Effect:** Logs prime schedule metadata, uses pre-baked `is_prime` functions

#### Custom Depth Overrides
```bash
fractal_research export mandelbulb_wg wgsl --prime-mode \
  --override-depths 2,3,5 --output result.wgsl
```
**Effect:** Specifies exact depths where prime scheduling parameters apply

#### Full Parameter Tuning
```bash
fractal_research export mandelbulb_wg wgsl --prime-mode \
  --override-depths 2,3,5 --amplification 1.8 --dampening 0.3 \
  --output result.wgsl
```
**Effect:** Custom schedule specification; amplification/dampening parameters currently logged but not materialized in shader generation

### Integration Targets

| Target | Integration Status |
|--------|-------------------|
| Shader Studio | ✅ Load WGSL with prime functions pre-baked |
| Fosfora | ✅ Import JSON export, metadata preserved |
| Custom Engines | ⚠️ Manual binding required for new uniform fields |

### Implementation Details

The `PrimeSchedule` struct controls:
- **prime_mode**: When true, depths are evaluated as prime or overridden
- **overrides**: Explicitly active depths (usize list)
- **amplification**: Factor at active depths (default: 1.0) — documented, not yet applied to shader generation
- **dampening**: Factor at inactive depths (default: 0.5) — documented, not yet applied to shader generation

Helper functions in `src/formula_dsl/prime.rs`:
- `is_prime(n)`: Primality test for depth evaluation
- `prime_sequence(count)`: Generate first N primes
- `primes_up_to(max)`: Generate primes up to max
- `PrimeSkipMask::should_refine(d, dy, dz, depth)`: Multiscale refinement check

### Alpha Testing Notes

- Tests validate presence of expected patterns in all export formats
- Numerical factor application pending future implementation
- Performance impact of prime checks unmeasured
- Edge cases: empty overrides, depth=0 handling work correctly

### Status Timeline
- **Current**: Baseline (CLI params parsed, export format patterns validated)
- **Next**: Beta (Schedule parameters injected into shader generation)
- **Target**: Stable (Full integration with performance benchmarks)

### Implementation Roadmap Mapping

| Phase | Goal | Status |
|-------|------|--------|
| 1 | Core DSL (complex, prime, waveguide) | ✅ Complete |
| 1b | Constraint Theory Integration | ⚠️ Partial |
| 2 | Database & Catalog (13 formulas) | ✅ Complete |
| 3 | Export Pipeline (WGSL/GLSL/Fosfora) | ✅ Complete |
| 3b | Mutation Engine | ✅ Complete |
| 4 | Validation Pipeline | ✅ Complete |
| 5 | Export Test Suite | ✅ Complete |
| 5b | PrimeSchedule CLI Integration | ⚠️ Baseline complete |
| 6 | Prime Convolution | 🔬 Design |
| 7 | Benchmark & Optimize | 🔬 Planned |

### Implementation Details

The `is_prime` function handles edge cases properly:
- Numbers < 2: Not prime
- Numbers < 4: Only 2 and 3 are prime
- Even numbers: Not prime (except 2)
- Numbers < 9: 5 and 7 are prime
- General case: 6k±1 optimization pattern

This ensures mathematical accuracy while maintaining shader performance.

### Research Collaboration Notes

The project maintains separation between:
- **Mathematical foundations** (documented and verified)
- **Implementation roadmap** (documented phases)
- **Feature completion status** (transparent about what is vs. isn't done)

This transparency enables coordinated development across research methodology and codebase evolution.

---

## Core DSL Modules

### Complex-Valued Operators (`complex.rs`)
- `complex_power` — Raise complex state to power n
- `modulus_nonlinear` — Apply `z -> z / |z| * f(|z|)`
- `phase_rotate` — Rotate phase of complex state
- `ModulusFunc` enum with Identity/Power/Tanh/Log variants
- WGSL code generation for basic complex operations

### Waveguide Operators (`waveguide.rs`)
- `DelayLine` — FIFO buffer for multi-scale state
- `ScatteringMatrix` — N×N matrix with configurable mixing
- `WaveguideChannel` — Single channel with reflection/gain
- `WaveguideNetwork` — Composed network with scatter+junction step
- Basic WGSL generation for scattering steps

### Prime-Indexed Scheduling (`prime.rs`)
- `is_prime` — Primality test (O(√n) trial division)
- `prime_sequence` / `primes_up_to` — Prime generators
- `PrimeSchedule` — Schedule with `prime_mode`, `overrides`, `amplification`, `dampening`
- `PrimeSkipMask` — Multiscale refinement skip logic
- `PrimeConvolution` — Framework skeleton (convolution logic pending)
- WGSL generation for prime check function and skip mask

### Constraint Theory (`constraint_integration/`)
- Modular architecture with trait-based integration points
- Skeletons for manifold snapping, holonomy verification, quantization
- Practical implementation pending (documented integration points exist)

---
 
 ## Formula Catalogue (Implemented)
 
 ### fractal3drust Catalogue (Research Formulas)
 
 | ID | Name | Family | Dimension | Category | Prime Support | Waveguide Support |
 |----|------|--------|-----------|----------|---------------|-------------------|
 | mandelbulb_n8 | Mandelbulb (n=8) | mandelbulb | 3 | DistanceEstimator | ✓ | |
 | mandelbulb_n7 | Mandelbulb (n=7) | mandelbulb | 3 | DistanceEstimator | ✓ | |
 | mandelbox_standard | Mandelbox | mandelbox | 3 | DistanceEstimator | ✓ | |
 | menger_sponge | Menger Sponge | menger | 3 | DistanceEstimator | ✓ | |
 | quaternion_julia | Quaternion Julia | quat_julia | 4 | DistanceEstimator | ✓ | |
 | triplex_julia | Triplex Julia | triplex | 3 | DistanceEstimator | ✓ | |
 | ifs_tetrahedron | IFS Tetrahedron | ifs | 3 | IFS | ✓ | |
 | fold_transform | Fold Transform | transform | 3 | Transform | ✓ | |
 | aexion | Aexion | aexion | 3 | DistanceEstimator | ✓ | |
 | bristorbrot | Bristorbrot | bristorbrot | 4 | DistanceEstimator | ✓ | |
 | mandelbulb_wg | Mandelbulb-WG | mandelbulb_wg | 3 | DistanceEstimator | ✓ | ✓ |
 | sdf_recurrent | SDF-Recurrent | sdf_recurrent | 3 | DistanceEstimator | ✓ | ✓ |
 | sparse_voxel_recursive | Sparse-Voxel-Recursive | sparse_voxel | 3 | DistanceEstimator | ✓ | ✓ |
 
 ### mb3d-rs Built-in Formula Mapping
 
 The 8 built-in formula types in mb3d-rs (`src/formulas.rs`) map to the catalogue as follows:
 
 | mb3d Variant | MB3D Name | fractal3drust Family | Dimensions | DE Method | Status |
 |--------------|-----------|---------------------|------------|-----------|--------|
 | `IntPow` | "Integer Power" | mandelbulb_n8/n7 | 3 | Analytic (type 0) | ✅ Mapped |
 | `RealPower` | "Real Power" | mandelbulb_n8 | 3 | Analytic (type 0) | ✅ Mapped |
 | `Quaternion` | "Quaternion" | quaternion_julia | 4 | 4D numeric (type 4) | ✅ Mapped |
 | `Tricorn` | "Tricorn" | — | 3 | Analytic (type 0) | ❌ Missing from catalogue |
 | `AmazingBox` | "Amazing Box" | mandelbox_standard | 3 | Analytic (type 11) | ✅ Mapped |
 | `Bulbox` | "Bulbox" | — | 3 | Hybrid (0.2 scale) | ❌ Hybrid - needs `Hybrid` category |
 | `FoldingIntPow` | "Folding Int Pow" | — | 3 | Hybrid (0.5 scale) | ❌ Hybrid - needs `Hybrid` category |
 | `AexionC` | "Aexion C" | aexion | 3 | Analytic (type 0) | ✅ Mapped |
 | `Custom` | `.m3f` files | Variable | Variable | By `DEoption` | ✅ Via `DEoption` classification |
 
 ### Custom Formula Classification (`.m3f` files)
 
 Custom formulas are classified by their `DEoption` value (from `HeaderTrafos.pas` / `CustomFormulas.pas`):
 
 | DEoption | Classification | Notes |
 |----------|----------------|-------|
 | 2, 11 | 3D analytic DE | `mandelbulb_wg`, `mandelbulb_n8` style |
 | 4 | 4D numeric DE | Quaternion style |
 | 5, 6 | 4D analytic DE | Extended 4D formulas |
 | -1, -2 | Ads (attractor?) | Special handling |
 | 20 | dIFS | Iterated Function Systems |
 | 21, 22 | dIFS transforms | IFS with transformations |
 | Others | 3D fallback | Default numeric gradient DE |
 
 ### Classification Gaps & Next Steps
 
 1. **Add `Tricorn`** to fractal3drust catalogue (direct port from mb3d)
 2. **Add `Hybrid` category** for formulas mixing base types (Bulbox = bulb + box, FoldingIntPow = fold + bulb)
 3. **Implement mb3d formula porting pipeline** so all mb3d built-ins + custom `.m3f` formulas are usable in fractal3drust DSL
 4. **Extend `FormulaCategory` enum** with `Hybrid`, `Spectral`, `Recursive` as documented in GOALS.md Phase 6.2
 
 ---
 
 ## Export Pipeline

### WGSL (Shader Studio)
Self-contained WGSL compute/fragment shader with:
- `fn fractal_iterate(z: vec2<f32>, c: vec2<f32>) -> vec2<f32>`
- `fn fractal_field(p: vec3<f32>) -> f32`
- Complete shader preamble (uniforms, raymarch, normal estimation, lighting)
- Pre-baked `is_prime` and `select()` patterns for schedule modulation

### GLSL
GLSL translation of WGSL output with type/function mapping:
- `vec2<f32>` → `vec2`, `mat2x2<f32>` → `mat2`, etc.
- WGSL builtins → GLSL equivalents

### Fosfora Effect Package
```text
<effect-name>/
├── shader.wgsl          # The WGSL shader
└── effect.json          # Metadata + parameters + UI ranges
```

---

## Testing & Validation

### Current Test Suite
- `constraint_integration::complex_ops::tests::test_modulus_nonlinear_exact` — ✅ PASS
- `constraint_integration::complex_ops::tests::test_complex_power_exact` — ✅ PASS
- Export validation tests covering 13 formula exports

### Planned Export Test Suite
Test coverage for all catalogue formulas across export formats with schedule parameter validation.

### Benchmarking
```bash
# Single-threaded benchmarks for reproducibility
cargo bench-single -- formula
cargo bench-single -- formula --prime-schedule
```

---

## Future Research Extensions

| Topic | Status | Description |
|-------|--------|-------------|
| **Prime Convolution (P-Conv)** | Design | Convolution where kernel weights only active at prime-indexed timesteps |
| **Fourier-Gated Layers** | Research | Frequency-domain gating for fractal operators |
| **Attention Mechanisms** | Research | Self-attention over iteration history |
| **Wavelet-Driven Recursion** | Research | Multi-resolution wavelet decomposition |
| **Neural Architecture Search (NAS)** | Proposed | Automated search for formula graph topologies |
| **Multiscale Prime Waveguide** | Phase 5 | Cross-scale waveguide coupling with prime scheduling |
| **Differential Geometry of Prime-Indexed Fractals** | Open | Mathematical analysis of prime-gated fractal boundaries |

These topics represent active research directions with varying levels of implementation readiness.

---

## Determinism & Reproducibility

All prime-related operations are **deterministic by design**:
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

---

## Implementation Phases Mapping

| Phase | Description | Target Completion |
|-------|-------------|-------------------|
| Phase 1 | Core DSL operators (complex, prime, waveguide) | ✅ Complete |
| Phase 1b | Constraint theory integration skeletons | ⚠️ Partial |
| Phase 2 | Database & catalog of 13 formulas | ✅ Complete |
| Phase 3 | Export pipeline (WGSL/GLSL/Fosfora) | ✅ Complete |
| Phase 3b | Mutation engine | ✅ Complete |
| Phase 4 | Validation pipeline | ✅ Complete |
| Phase 5 | Export test suite | ✅ Complete |
| Phase 5b | PrimeSchedule CLI baseline | ✅ Complete |
| Phase 6 | Prime Convolution implementation | 🔬 Planned |
| Phase 7 | Full benchmark & optimization | 🔬 Planned |
| Phase 8 | Advanced research extensions (NAS, wavelet) | 🔬 Future |
## Prime Schedule Integration (Alpha Phase)

### Overview
The PrimeSchedule feature implements prime-indexed scheduling for fractal iteration depth control. This feature is currently in **alpha development status** - pattern expectations validated but numerical factor application requires further integration.

### Implementation Status
**Phase 5b - PrimeSchedule CLI Integration | Status: ⚠️ Alpha**

| Component | Status | Notes |
|-----------|--------|-------|
| CLI Parameters | ✅ Complete | `--prime-mode`, `--override-depths`, `--amplification`, `--dampening` |
| PrimeSchedule Struct | ✅ Complete | Defined in `src/formula_dsl/prime.rs` with `is_active()`, `factor()`, `coefficient_sequence()` methods |
| Export Integration | ⚠️ Partial | Schedule struct built in main.rs but not passed to export_formula (planned) |
| Shader Output | ✅ Complete | Pre-baked prime functions in WGSL (is_prime, prime_refine, waveguide_scatter) |
| Validation Tests | ✅ Complete | All 5 export test cases pass with confirmed pattern matching |
| Performance Benchmarking | 🔬 Planned | Measure prime schedule impact vs uniform scheduling |

### Key Findings from Export Analysis

1. **mandelbulb_wg WGSL** includes:
   - `is_prime` function (prime check)
   - `select(u.power, u.power * 1.3, is_prime(i + 1))` (prime modulation)
   - `eff_power` variable (effective power calculation)
   - Standard WGSL uniform block and shader entry points

2. **mandelbulb_n8 GlSl** exports as documentation-style header with:
   - Formula metadata (name, family, notes)
   - Parameter documentation
   - Core iteration description

3. **mandelbox_standard Fosfora** exports JSON with:
   - `name` field: "Mandelbox"
   - `shader` field containing full WGSL code
   - `parameters` array with slider definitions

4. **sparse_voxel_recursive WGSL** includes:
   - `is_prime` check function
   - Prime-indexed accumulation pattern
   - `maxDepth` parameter in uniform block

### Known Limitations
1. PrimeSchedule parameters (`prime_mode`, `overrides`, `amplification`, `dampening`) are parsed in CLI but not yet passed to `export_formula()` function
2. Export functions don't call `PrimeSchedule::is_active()` or `factor()` methods
3. Shader output patterns are pre-baked rather than dynamically scheduled
4. Numerical application of amplitude/dampening factors requires implementation work

### CLI Usage Patterns

#### Basic Prime Mode
```bash
fractal_research export mandelbulb_wg wgsl --prime-mode --output result.wgsl
```
Effect: Logs prime schedule metadata, uses pre-baked `is_prime` functions

#### Custom Depth Overrides
```bash
fractal_research export mandelbulb_wg wgsl --prime-mode --override-depths 2,3,5,7 --output result.wgsl
```
Effect: Specifies exact depths where prime scheduling applies

#### Full Parameter Tuning
```bash
fractal_research export mandelbulb_wg wgsl --prime-mode --override-depths 2,3,5 --amplification 1.8 --dampening 0.3 --output result.wgsl
```
Effect: Custom schedule with amplified prime depths and dampened non-prime depths

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
- **amplification**: Factor at active depths (default: 1.0)
- **dampening**: Factor at inactive depths (default: 0.5)

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

### Next Phase Goals
1. Pass `PrimeSchedule` to `export_formula()` parameters
2. Implement schedule-based shader generation (dynamic is_prime, prime_refine calls)
3. Apply amplitude/dampening factors in shader parameter generation
4. Add visualization tests comparing prime vs uniform scheduled outputs
5. Implement depth-based performance measurements

### Status Timeline
- **Current**: Alpha (Pattern expectations validated)
- **Next**: Beta (Numerical factor application verified)
- **Target**: Stable (Full integration with performance benchmarks)

### Implementation Roadmap Mapping

| Phase | Goal | Status |
|-------|------|--------|
| 1 | Core DSL (complex, prime, waveguide) | ✅ Complete |
| 1b | Constraint Theory Integration | ✅ Complete |
| 2 | Database & Catalog (13 formulas) | ✅ Complete |
| 3 | Export Pipeline (WGSL/GLSL/Fosfora) | ✅ Complete |
| 3b | Mutation Engine | ✅ Complete |
| 4 | Validation Pipeline | ✅ Complete |
| 5 | Export Test Suite | ✅ Complete |
| 5b | PrimeSchedule CLI | ⚠️ Alpha (CLI params, partial export integration) |
| 6 | Prime Convolution | 🔬 Design |
| 7 | Benchmark & Optimize | 🔬 Planned |arg>
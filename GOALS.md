# Goals List: Fractal Research Project

Based on full code review and methodology understanding, here are the concrete implementation goals:

## ✅ Phase 0: Baseline Validation (Completed)
- [x] `cargo check --release` passes for all existing formulas
- [x] CLI commands work: `list`, `info`, `render`, `export`
- [x] Core DSL modules functional (`complex.rs`, `prime.rs`, `waveguide.rs`)
- [x] Catalogue of 12 formulas available (including research formulas)
- [x] WGSL generation and export working

## 🟡 Phase 1: Learning & Prime Systems Implementation
### 1.1 LearnedComplexMap (complex-valued neural network)
- [ ] Add `LearnedComplexMap` struct to `src/formula_dsl/complex.rs`
- [ ] Implement Xavier-like weight initialization
- [ ] Add forward pass with activation (tanh/ReLU/identity)
- [ ] Implement `FractalFormula` trait for it
- [ ] Add WGSL generation support in `src/render/wgsl.rs`
- [ ] Add serialization/deserialization (serde)

### 1.2 Prime Gate System
- [ ] Add `PrimeGate` struct to `src/formula_dsl/prime.rs`
- [ ] Implement prime checking (sieve + trial division fallback)
- [ ] Add `eval(step, max_depth)` method
- [ ] Create wrapper that makes sub-formula identity on non-prime steps
- [ ] Add to DSL builder API

### 1.3 Prime Convolution (NEW)
- [ ] Add `PrimeConvolution` struct to `src/formula_dsl/prime.rs`
- [ ] Implement kernel with prime-indexed weight updates
- [ ] Add forward pass applying only prime-ranked kernel weights
- [ ] Implement `FractalFormula` trait
- [ ] Add WGSL generation
- [ ] Add to DSL builder API

### 1.4 Prime Coefficient Sequence
- [ ] Add `PrimeCoefficientSequence` struct
- [ ] Implement sequence generation from prime numbers
- [ ] Add as multiplier for other operators (power, gain, etc.)

## 🟢 Phase 2: Integration & DSL Enhancement
### 2.1 DSL Builder API Enhancement
- [ ] Add methods to `FormulaBuilder`:
  - `fn learned_map(&mut self, in_dim: usize, out_dim: usize)`
  - `fn prime_gate(&mut self, enabled: bool, max_depth: usize)`
  - `fn prime_conv(&mut self, in_ch: usize, out_ch: usize, kernel: usize)`
  - `fn prime_coeff_seq(&mut self, base: f64, max_depth: usize)`
- [ ] Update `FormulaGraph` to store new operator types
- [ ] Ensure operator composition order is preserved

### 2.2 Formula Trait Implementation
- [ ] Ensure all new operators implement:
  - `fn init_state(&self, c: Vec3) -> IterationState`
  - `fn iterate(&self, state: &mut IterationState, step: usize)`
  - `fn decode(&self, state: &IterationState, _c: Vec3, max_iter: usize) -> FieldSample`
  - `fn generate_wgsl(&self) -> WgslShader`
  - `fn parameters(&self) -> &[Parameter]`

### 2.3 WGSL Generation
- [ ] Add WGSL templates for:
  - `LearnedComplexMap` (matrix multiply + activation)
  - `PrimeConvolution` (conditional prime-indexed multiply)
  - `PrimeGate` (conditional execution based on step)
  - `PrimeCoefficientSequence` (dynamic coefficient lookup)

## 🟡 Phase 3: Research Formula Creation
### 3.1 New Formula Families
- [ ] Create `PrimeConvMandelbulb` in `src/presets/formulas.rs`
  - Structure: PrimeGate → PrimeConv → ComplexPower → WaveguideLayer
- [ ] Create `LearnedComplexRecurrent` 
  - Structure: LearnedComplexMap → WaveguideLayer (recurrent complex map)
- [ ] Create `PrimeWaveguideHybrid`
  - Structure: PrimeGate → WaveguideLayer with prime-modulated reflection

### 3.2 Catalogue Integration
- [ ] Add new formulas to `catalogue()` function in `src/db/catalogue.rs`
- [ ] Set appropriate `category` (mostly `DistanceEstimator`)
- [ ] Define `parameter_fields` for tunable parameters
- [ ] Add `known_good_presets` with sensible defaults
- [ ] Set `origin` to "Research formula – prime-convolution enhanced"

## 🟢 Phase 4: Search & Validation Pipeline
### 4.1 Fitness Function Enhancement
- [ ] Update `src/search/evolution.rs` to reward:
  - Novelty (perceptual hash distance from known formulas)
  - Visual complexity (edge entropy, fractal dimension estimate)
  - DE stability (Lipschitz constant near 1 for DE formulas)
  - Convergence speed (iterations to escape)
  - Parameter efficiency (penalize excessive trainable params)

### 4.2 Validation Enhancements
- [ ] Add DE verification in `src/field_core/evaluator.rs`
- [ ] Add parameter sensitivity analysis
- [ ] Add visual inspection hooks (save preview images during search)

## 🟡 Phase 5: Export & Documentation
### 5.1 Export Enhancements
- [ ] Update `src/export/fosfora.rs` to handle new parameters
- [ ] Add prime-specific JSON options to effect.json
- [ ] Ensure WGSL snippets work in Shader Studio
- [ ] Add preset JSON export for new formulas

### 5.2 Documentation Completion
- [ ] Update `README.md` with new formula usage examples
- [ ] Add examples section showing:
  - How to run evolutionary search: `cargo run --search evolution`
  - How to render a discovered formula: `cargo run --render --id <formula_id>`
  - How to export to Fosfora: `cargo run --export --format fosfora --id <id>`

### 5.3 Testing
- [ ] Add unit tests for new operators in `tests/`
- [ ] Add integration test for full pipeline: build → validate → export → render
- [ ] Add property tests for prime-related functions

## 🔴 Phase 6: Advanced Research (Future)
### 6.1 Future Research Methods (Documented Only)
- [ ] Fourier-Gated Layers (docs/research.md)
- [ ] Attention Mechanisms (docs/research.md)
- [ ] Wavelet-Driven Recursion (docs/research.md)
- [ ] Neural Architecture Search for Primitives (docs/research.md)

### 6.2 Extended Classification
- [ ] Consider adding `Hybrid` category for formulas mixing multiple base types
- [ ] Consider adding `Spectral` category for frequency-domain formulas
- [ ] Consider adding `Recursive` category for true recursive voxel formulas

---

## Immediate Next Steps (This Week)

1. **Implement LearnedComplexMap** in `src/formula_dsl/complex.rs`
2. **Implement PrimeGate** in `src/formula_dsl/prime.rs`
3. **Implement PrimeConvolution** in `src/formula_dsl/prime.rs`
4. **Update FormulaBuilder** in `src/formula_dsl/mod.rs` with new methods
5. **Add WGSL generation** for new operators in `src/render/wgsl.rs`
6. **Create first research formula** using these components
7. **Test with cargo check --release** and basic CLI commands
8. **Update todo list** to reflect completion

## Success Criteria

When complete, the system should:
- Compile with `cargo check --release` (single-threaded as per .cargo/config)
- Allow creation of formulas like: `PrimeGate(true, 30).wrap(LearnedComplexMap::new(2,2)).wrap(PrimeConvolution::new(3,3,3))`
- Generate valid WGSL that compiles in shader compilers
- Export usable effect presets for Fosfora/Shader Studio
- Support evolutionary search discovering novel prime-convolution based fractals
- Maintain backward compatibility with all existing 12 formulas

This goals list provides a clear, actionable roadmap to implement the prime convolution neural network system and fully realize the research vision documented in the updated docs.
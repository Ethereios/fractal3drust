# Architecture

Implementation plan for `fractal-research-rs`: a research-first Rust lab that
investigates 3D/higher-dimensional fractal formulas and exports tested WGSL
shaders and presets for Fosfora and WGSL Shader Studio.

---

## Design principles

1. **Research-first.** The tool exists to discover, evaluate, and iterate on
   novel fractal formulas.  Export is a downstream concern.
2. **Two tiers: complex + prime + waveguide.** Complex iteration defines the
   nonlinear map; prime logic controls recursion depth and branch selection;
   waveguide logic provides structured delay/scattering between iterations.
3. **CPU reference + GPU export.** All formulas are validated against a CPU
   reference evaluator.  WGSL is generated from the same formula DSL so that
   the GPU preview matches the validated result.
4. **Export as primary integration path.** Fosfora accepts a WGSL shader +
   JSON metadata for new effects, so export is the bridge from research to
   production runtime.

---

## Module breakdown

```
src/
├── main.rs              # CLI entry point (clap)
├── lib.rs               # Library root; re-exports all modules
├── formula_dsl/         # Composable formula operators
│   ├── mod.rs           # FractalFormula trait, FormulaGraph, operators
│   ├── complex.rs       # Complex-valued operators (power, modulus, phase)
│   ├── waveguide.rs     # Delay lines, scattering matrices, junctions
│   └── prime.rs         # Prime-indexed scheduling, depth masks, skip rules
├── field_core/          # Field representations
│   ├── mod.rs           # Field trait, evaluation results
│   └── evaluator.rs     # CPU escape-time + raymarch evaluator
├── search/              # Formula search strategies
│   ├── mod.rs           # SearchStrategy trait
│   └── evolution.rs     # Evolutionary / random parameter search
├── render/              # Rendering
│   ├── mod.rs           # Renderer trait
│   ├── cpu.rs           # CPU raymarcher / escape-time renderer
│   └── wgsl.rs          # WGSL shader body generator
├── export/              # Export formats
│   ├── mod.rs           # Exporter trait
│   ├── wgsl.rs          # Standalone WGSL shader export
│   ├── fosfora.rs       # Fosfora effect package (WGSL + effect.json)
│   └── shader_studio.rs # Shader Studio snippet export
├── presets/             # Built-in formula families
│   ├── mod.rs           # Preset registry, JSON loading
│   └── formulas.rs      # Mandelbulb-WG, SDF-recurrent, sparse-voxel
└── error.rs             # Error types (thiserror)
```

---

## Data flow

```
  Point (x, y, z) ──encode──> Encoded coordinate
                                  │
                                  v
                    +-----------+-----------+
                    |  formula_dsl          |
                    |  z_{t+1} = F(z_t, c,  |
                    |           p_t, w_t)  |
                    +-----------+-----------+
                                  │
                    iterates (z_t, w_t, p_t)
                                  │
                    +-------------+-------------+
                    | field_core evaluators:    |
                    |  - Escape-time field      |
                    |  - Distance-estimate      |
                    |  - SDF proxy decode       |
                    |  - Sparse voxel           |
                    +---------------------------+
                                  │
                                  v
                         Field sample value
                                  │
                    +-------------+-------------+
                    | render:                   |
                    |  CPU: raymarch / 2D ET    |
                    |  GPU: same DSL -> WGSL     |
                    +---------------------------+
                                  │
                    +-------------+-------------+
                    | export:                   |
                    |  wgsl      (Shader Studio) |
                    |  fosfora   (Fosfora)       |
                    |  preset    (JSON)          |
                    +---------------------------+
```

### Formula DSL data flow

A formula is a **composition graph** of operators.  Each operator implements
the `FractalFormula` trait:

```rust
trait FractalFormula {
    /// Initialize state for a given sample point c.
    fn init(&self, c: &Point3) -> State;

    /// One iteration step: update state in place.
    fn iterate(&self, state: &mut State, c: &Point3, step: usize);

    /// Decode the final state into a field value (density / escape / DE).
    fn decode(&self, state: &State, c: &Point3) -> FieldSample;
}
```

Operators can wrap other operators (decorator / pipeline pattern), enabling
composition like:

```
WaveguideLayer(
    ScatteringMatrix,
    ComplexNonlinear(F(z, c, p))
)
```

---

## Formula DSL

### Core types

| Type | Description |
|---|---|
| `Point3` | 3D sample point `(f64, f64, f64)` |
| `Complex` | `num_complex::Complex64` |
| `Quaternion` | 4D hypercomplex (for 3D rotation-fold) |
| `State` | Mutable iteration state: complex latents, waveguide delay buffers, prime depth, escape status |
| `FieldSample` | Output: `{{ value: f64, field_type: FieldType, iterations: usize }}` |
| `FieldType` | `EscapeTime`, `DistanceEstimate`, `Density`, `Occupancy` |

### Operator library

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

---

## Export format

### WGSL (Shader Studio)

The export module generates a self-contained WGSL compute or fragment shader.
Key entry points:

- `fn fractal_iterate(z: vec2<f32>, c: vec2<f32>) -> vec2<f32>`
- `fn fractal_field(p: vec3<f32>) -> f32`
- `fn main(...)` with standard shader preamble

The WGSL body is **generated from the same formula DAG** used by the CPU
evaluator, ensuring consistency.

### Fosfora effect package

A Fosfora effect requires:

```
<effect-name>/
├── shader.wgsl          # The WGSL shader
└── effect.json           # Metadata + parameters + UI ranges
```

`effect.json` schema:

```json
{
  "name": "mandelbulb-wg",
  "label": "Mandelbulb Waveguide",
  "description": "...",
  "shader": "shader.wgsl",
  "entryPoint": "fs_main",
  "parameters": [
    {
      "name": "maxIter",
      "label": "Iterations",
      "type": "int",
      "default": 32,
      "min": 4,
      "max": 256
    },
    {
      "name": "power",
      "label": "Power",
      "type": "float",
      "default": 8.0,
      "min": 2.0,
      "max": 16.0
    }
  ]
}
```

### Shader Studio snippet

Plain WGSL with `preset` comments and `#define` injection points for quick
pasting into any WGSL editor.

### Preset JSON

Standalone parameter snapshots with metadata:

```json
{
  "id": "mandelbulb-wg@v1",
  "formula": "mandelbulb-wg",
  "parameters": { "power": 8.0, "maxIter": 64, ... },
  "ui_hints": { "power": { "min": 2.0, "max": 16.0 } }
}
```

---

## Integration targets

| Target | Integration path | Status |
|---|---|---|
| Fosfora | Full WGSL + `effect.json` export | Primary |
| WGSL Shader Studio (nanovis) | Plain WGSL snippet | Supported |
| ibaryshnikov/shader-editor | Plain WGSL snippet | Supported |
| Bevy/WGPU | Native wgpu pipeline (future) | Planned |

---

## Formula families to implement

### 1. Mandelbulb-WG (Mandelbulb-like + waveguide)

Base: classic Mandelbulb iteration in spherical coordinates with power `p`.
Enhancement: insert a **waveguide scattering step** between iterations that
mixes the radial, azimuthal, and elevation channels at different delays.
Prime scheduling: refine at prime-numbered depths.

### 2. SDF-Recurrent (complex-recurrent map + SDF decode)

Core: iterate a **complex recurrent map** `z_{t+1} = F_complex(z_t, c)` where
`F_complex` is composed of complex operators.  After `T` steps, decode the
final complex state into an **SDF-like** scalar using a modulus/lattice
projection.  This is the 3D analogue of neural fractal generation.

### 3. Sparse-Voxel-Recursive (hierarchical recursive)

Generate a sparse voxel field recursively: at each depth, apply a
**prime-indexed skip mask** to decide which octants refine, and a
**waveguide scattering step** to propagate latent state across scales.  The
output is a sparse `HashMap` of voxel -> density, suitable for mesh extraction
or export as a sparse SDF grid (following Fractal3DGen's approach).

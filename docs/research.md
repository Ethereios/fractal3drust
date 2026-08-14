# Research Overview: 3D Fractal Formula Discovery and Export System

## 1. Project Vision and Research Goals

This Rust-based fractal research laboratory investigates 3D/higher-dimensional fractal formulas using complex, prime, waveguide-inspired, hierarchical, and neural techniques. The system exports tested WGSL shaders and presets for Fosfora and WGSL Shader Studio, enabling integration with modern GPU-accelerated visualization pipelines.

**Core Research Objectives:**
- **Formula Discovery**: Systematically discover and catalog 3D fractal formulas beyond existing community collections
- **Structured Database**: Build a scientific database of 3D fractal formulas with proper metadata, DE support, and provenance tracking
- **GPU Export**: Generate verified WGSL shaders for real-time preview and Fosfora effect integration
- **Research Methodology**: Provide a repeatable process for fractal formula discovery, moving beyond manual experimentation

## 2. Analysis of Existing 3D Fractal Formula Databases

### Mandelbulb3D and Mandelbulber

Mandelbulb3D and Mandelbulber have something close to a database of 3D fractal formulas, but it's **not a single "scientific DB"** with a clearly documented automated discovery process. It's a **curated and community-extended collection** of hand-designed formulas, plus hybrids and transforms, accumulated over years by the original authors and many contributors.

**Mandelbulb3D Structure:**
- **Dozens of core formulas**: Mandelbulb variants, Mandelbox, Menger sponge, Quaternion Julias, IFS shapes, etc.
- **Transform/modifier formulas**: folds, rotations, twists, warps, tilings, etc.
- **Hybrid engine**: Can stack and blend up to 6 formulas in one scene with separate iteration counts and mixing rules.
- **Categorization**: Formulas categorized by type: "3D", "3Da", "4D", "4Da", and transform/IFS types.
- **Loading**: Loadable as M3F formula files, with additional packs released by power users.

### How Existing Formulas Were Created

**Original Mandelbulb Discovery:**
The original Mandelbulb was discovered by Daniel White and Paul Nylander via hand analysis and experimentation in spherical coordinates. They searched for a 3D analogue of `z -> z^n + c` that preserves a useful notion of "power" in ℝ³.

**Other Formulas in Mandelbulb3D:**
- Classical 3D extensions (Mandelbox, Menger, Quaternion Julia, Hypercomplex) known from math/graphics communities
- Custom constructs and transforms coded by Jesse and other contributors, based on their own experiments with algebraic forms, IFS theory, and distance estimation

**Creation Process:**
- **Manual discovery and experimentation**: "Stay in discovery mode, don't try to be deterministic."
- **Hybridization**: "Stack and blend dozens of formulas; alternate iterations to get alien landscapes."
- **Community contributions**: Power users explicitly hand-port new formulas to MB3D.

### The Database Aspect

What Mandelbulb3D/Mandelbulber actually has:
1. **Formula registry**: Each formula is a named code module with a known category and UI parameters
2. **Parameter database**: M3P files containing user-saved parameter sets and scenes combining formulas, transform sequences, and camera/lighting settings
3. **Community repository**: DeviantArt, forums, tutorials, and packs with additional formulas and transforms

However, there is **no published algorithm** documenting how these formulas were automatically discovered - they are the result of human mathematical insight + manual experimentation + incremental community contributions.

## 3. Research Methodology: Building a Scientific 3D Fractal Database

To match or exceed MB3D/Mandelbulber with a research-grade database, the following methodology is required:

### Step 1: Define a Formula DSL (Domain-Specific Language)
Instead of keeping formulas as arbitrary WGSL snippets, define:
- **Base maps**: Complex/hypercomplex/WG fields
- **Transforms**: Fold, rotate, scale, warp
- **Compositions**: Escape-time, DE, IFS
- **Hybrid mixing rules**: Alternating iterations, additive blends, masks

Each formula becomes an AST node (or graph) in that DSL, enabling:
- Storage in a DB
- Systematic analysis
- Automatic generation

### Step 2: Build a Structured Formula Registry

For each formula, store:
| Field | Type | Description |
|-------|------|-------------|
| `id` | String | Unique ID (e.g., "mandelbulb_n8") |
| `name` | String | Display name |
| `family` | String | Mandelbulb, Mandelbox, DE, etc. |
| `dimension` | Int | 3, 4, etc. |
| `category` | String | Escape-time, distance_estimator, IFS, transform, hybrid |
| `iteration_core` | DSL/AST or structured JSON | Describing the map F |
| `parameter_fields` | JSON Array | Names, types, ranges |
| `de_supported` | Bool | Has a distance estimator? |
| `wgsl_core_fn` | Text | WGSL function implementing core |
| `known_good_presets` | JSON Array | Valid parameter sets + tags |
| `origin` | String | "mandelbulb3d", "mandelbulber", "shadertoy", "research", "user" |
| `notes` | Text | Provenance, references, caveats |

### Step 3: Implement Discovery as Search on the DSL

Use program search methods:
- **Local search** in parameter space
- **Evolutionary search** over formula graphs
- **Symbolic regression-like operators** on the DSL
- **Neural proposal models** (suggest DSL graphs given target descriptors)

Use constraints:
- Enforce DE stability
- Enforce boundedness criteria
- Enforce desired symmetry or dimensionality

Each accepted candidate is added to the registry with metadata about how it was found.

### Step 4: Human-in-the-Loop Discovery

- Log every explored formula/parameter combination
- Store user ratings or tags ("usable in VJ", "good for motion", "too slow to render")
- Stream this into the DB
- This way, human exploration is part of the search process

## 4. Literature References

Key references informing this research:

1. **Fractal3DGen: Coarse-to-Fine 3D Generation** (openreview.net) - Hierarchical coarse-to-fine 3D generation for self-similar structure without dense voxels
2. **Where Do We Stand with Implicit Neural Representations?** (arxiv.org/abs/2411.03688) - Survey of continuous field representations, strengths, and current limits
3. **Awesome Implicit Representations** (github.com/vsitzmann/awesome-implicit-representations) - Curated gateway into INR literature
4. **Generating Fractals Using Complex Valued Neural Networks** (amirabbasasadi.github.io) - Neural fractal generation using complex neural networks
5. **A review of differentiable digital signal processing for music and audio** (comma.eecs.qmul.ac) - Structured differentiable systems, relevant for waveguide-inspired recurrence
6. **Four Decades of Digital Waveguides** (alphaxiv.org/abs/2604.12878) - Background for wave propagation, delay/scattering networks, and structured dynamics
7. **FractalNet** (github.com/gustavla/fractalnet) - Recursive fractal network generation ideas for operator-graph design
8. **Neural Networks with Fractal Architecture** (mdpi.com/2504-3110/10/7/452) - Newer perspective on recursive neural design ideas

## 5. Current Development Status

### Core Modules Implemented
- ✅ Formula DSL core types and operators (complex, prime, waveguide)
- ✅ Field types and CPU evaluator (DenseField, SparseVoxelGrid, escape-time rendering)
- ✅ Database schema (catalogue, hybrid composition)
- ✅ WGSL shader implementations for Mandelbulb, Mandelbox, Menger, QJulia, IFS, transforms

### In Progress
- Exporter modules (WGSL standalone export, Fosfora effect package, Shader Studio snippet)
- Search algorithms (random search, evolutionary baseline, neural proposal models)
- Rendering pipeline integration (GPU WGSL preview path)
- Presets system (JSON loading/saving, parameter management)

## 6. Next Steps

1. **Complete exporter modules** for WGSL, Fosfora, and Shader Studio formats
2. **Implement search algorithms** for systematic formula discovery
3. **Finalize rendering pipeline** with GPU acceleration via WGSL
4. **Write comprehensive tests** for all formula families and export paths
5. **Publish to crates.io** as `fractal-research-rs`
6. **Create detailed documentation and examples** for researchers

## 7. Integration Targets

| Target | Integration Path | Status |
|--------|-----------------|--------|
| Fosfora | Full WGSL + `effect.json` export | Primary |
| WGSL Shader Studio (nanovis) | Plain WGSL snippet | Supported |
| ibaryshnikov/shader-editor | Plain WGSL snippet | Supported |
| Bevy/WGPU | Native wgpu pipeline (future) | Planned |

## 8. Formula Design Methodology

A practical family of 3D fractal formulas iterates a latent state `z_t` plus a propagated state `w_t`, then decodes to a signed distance field, voxel field, or density field. 

**Core Template:**
```
z_{t+1} = F_θ(z_t, c, p_t, w_t)
```
Where:
- `z_t`: Complex or hypercomplex state
- `c`: 3D point parameter
- `p_t`: Prime-derived control code
- `w_t`: Waveguide transport state

**Three promising components:**
1. **Complex NN**: Defines the nonlinear map `F` iterated in ℂⁿ or quaternion-like space
2. **Prime Logic**: Controls recursion depth, branch selection, skip connections, coefficient schedules
3. **Waveguide Logic**: Introduces propagation, delay, reflection, and scattering between state channels

**Output Modes:**
1. **Escape-time field**: For each 3D point, iterate until divergence and store escape count
2. **Signed distance proxy**: Decode final latent state into SDF-like scalar, extract mesh
3. **Sparse hierarchical volume**: Generate voxels recursively (Fractal3DGen approach)

## 8. Repository Structure

```
/docs/
  research.md        # This document - research methodology and analysis
  architecture.md    # Implementation plan: modules, data flow, export format
/README.md          # Project overview and quickstart
/src/
  formula_dsl/      # Composable formula operators
  field_core/       # Field representations and evaluators
  search/           # Formula search strategies
  render/           # CPU + GPU rendering
  export/           # WGSL, Fosfora, Shader Studio exporters
  presets/          # Built-in formula families and parameter sets
  lib.rs            # Library root; re-exports all modules
  main.rs           # CLI entry point (clap)
Cargo.toml          # Rust project configuration
```

## Summary

This project fills the gap between Mandelbulb3D's informal formula registry and a genuine research-grade fractal database. By providing:
1. A **formal DSL + registry** to store formulas plus metadata
2. A **search/discovery layer** that systematically explores the space and logs results
3. **WGSL export** to Fosfora/Shader Studio for immediate integration

The tool enables genuine research into 3D fractal formula discovery rather than just manual experimentation within existing VJ runtimes.

---
*Document generated as part of fractal research lab implementation. Last updated: 2026-08-14*
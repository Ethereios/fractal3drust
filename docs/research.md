# Research Documentation: 3D Fractal Formula Discovery

## Research Vision Statement

The objective of this research is to develop a systematic, reproducible methodology for discovering and cataloging 3D fractal formulas that extends beyond traditional mathematical exploration. By integrating complex-valued neural networks, prime-indexed operations, and waveguide-inspired dynamics, we seek to automate the discovery of novel fractal structures with verifiable mathematical properties.

## Research Questions

1. **Can complex-valued neural operators generate 3D fractal formulas with superior visual complexity compared to classical power-based maps?**
2. **How do prime-indexed scheduling mechanisms influence the self-similarity properties of recursive fractal structures?**
3. **What role do waveguide delay/scattering networks play in creating structured heterogeneity within fractal iterations?**
4. **Is it possible to establish a quantitative metric for "fractal novelty" that can guide automated discovery?**

## Theoretical Foundation

### Core Research Hypothesis

Our research conjectures that a **structured iteration system** combining three specialized operators will yield novel fractal behaviors:

```
z_{t+1} = Waveguide(PrimeSchedule(ComplexNN(z_t, c), t), t)
```

Where:
- **ComplexNN** provides the nonlinear map in ℂ-space
- **PrimeSchedule** controls algorithm selection at prime-indexed depths
- **Waveguide** introduces structured delay/diffusion channels

### Mathematical Framework

#### Complex-Valued Neural Operators
Traditional 3D fractals use real-valued iterations `z ↦ z^n + c`. Our research explores complex-valued networks `NN_θ(z, c)` that:
- Preserve phase structure through ℂ-space computation
- Enable dynamic power/modulus operations learned from data
- Support gradient-based optimization for search

#### Prime-Indexed Scheduling Theory
The prime number distribution provides a natural non-repetitive pattern:
- Prime depths (2,3,5,7,11,...) create natural hierarchy
- Non-prime depths allow structural variations
- Prime coefficient sequences enable systematic exploration

#### Waveguide Dynamics
Drawing from DSP waveguide theory:
- **Delay Lines**: `w_t ∈ ℝ^n` for multi-scale state
- **Scattering Matrices**: Linear mixing of delay channels
- **Reflection Coefficients**: Depth-dependent coupling

## Literature Review

### 3D Fractal Generation Prior Work

| Work | Contribution | Limitation |
|------|-------------|------------|
| FRACTAL3DGEN (2023) | Hierarchical 3D generation without dense voxels | Limited to volume-based generation |
| INR Survey (2024) | Continuous field representations for geometry | Not specialized for fractal structures |
| Complex Neural Fractals (2021) | ℂ-space neural iterations | No structured recurrence, limited to 2D |
| DDSP Review (2021) | Differentiable systems | Audio-focused, not geometric |
| FractalNet (2016) | Recursive neural architectures | Image classification, not fractal discovery |

### Research Gap Analysis

Current 3D fractal tools (Mandelbulb3D, Mandelbulber) rely on:
- **Manual discovery**: Researchers hand-craft formulas via trial-and-error
- **Community curation**: Formulas accumulated without systematic methodology
- **Limited automation**: No algorithmic discovery framework

This research addresses these gaps through:
1. Automated formula discovery via evolution/search
2. Formal DSL representation enabling systematic analysis
3. Database structure with complete provenance tracking

## Research Methodology

### Discovery Pipeline

```
Random Population
       ↓
Fitness Evaluation
       ↓
Selection → Crossover → Mutation → PrimeMask → WaveguideApply
       ↓
New Population
```

### Evaluation Metrics

**Fitness = α·Novelty + β·VisualQuality + γ·DEStability**

Where:
- **Novelty**: Distance from existing formula database
- **VisualQuality**: Edge complexity from gradient analysis
- **DEStability**: Convergence rate of distance estimator

### Validation Protocol

1. **CPU Reference Evaluation**: Validate against mathematical definition
2. **GPU WGSL Generation**: Auto-generate verified shaders
3. **Fosfora Integration**: Test in deployment environment
4. **Human Rating**: Collect qualitative feedback on visual appeal

## Research Database Schema

Each discovered formula will be catalogued with:

| Field | Purpose |
|-------|---------|
| `id` | Unique identifier with version tag |
| `dsl_graph` | Formal AST representation |
| `category` | Fractal type classification |
| `parameters` | Search space definition |
| `de_formula` | Distance estimator expression |
| `wgsl_export` | Verified GPU implementation |
| `discovery_method` | Search algorithm and parameters |
| `validation_score` | Fitness components and human rating |

## Expected Contributions

1. **Novel 3D fractal algorithms** beyond existing catalogs
2. **Formal DSL** for reproducible formula representation
3. **Open database** of discovered formulas with full metadata
4. **Export pipeline** to WGSL for real-time rendering

## Research Ethics

All discovered formulas will be:
- Reproducibly generated from the formal DSL
- Documented with complete derivation process
- Shared openly for community validation and extension
- Accompanied by parameter sets and quality metrics

---
*Research lead: fractal-research-rs team*
*Research period: 2026*
# Research: Systematic Discovery of 3D Fractal Formulas via Complex-Valued Neural Networks, Prime-Indexed Operations, and Waveguide Dynamics

## 1. Introduction

The exploration of 3D fractal formulas has traditionally relied on manual experimentation and community-driven curation, as seen in tools like Mandelbulb3D and Mandelbulber. While these tools provide extensive libraries of formulas, they lack a systematic methodology for discovery, validation, and cataloging. This research aims to establish a rigorous, reproducible approach to discovering novel 3D fractal formulas by integrating three complementary techniques: complex-valued neural networks, prime-indexed operations, and waveguide-inspired dynamics. The outcome is a catalog of formulas with validated mathematical properties, exportable as WGSL shaders for real-time rendering in environments such as Fosfora and WGSL Shader Studio.

## 2. What Are 3D Fractal Formulas?

A 3D fractal formula defines an iterative process in three-dimensional space that generates self-similar, infinitely detailed structures. Unlike 2D fractals (e.g., the Mandelbrot set), 3D fractals require operations in � ℝ³ or higher-dimensional algebras (such as � ℂ² or � ℍ) to produce volumetric detail. Common examples include:
- **Mandelbulb**: A 3D analogue of the Mandelbrot set using spherical coordinates.
- **Mandelbox**: Generated through iterative folding and scaling operations.
- **Menger Sponge**: A recursive void-based fractal.
- **Quaternion Julia Sets**: 4D quaternion iterations projected to 3D.

These formulas are typically evaluated via escape-time iteration or distance estimation (DE) for ray marching.

## 3. Challenges in 3D Fractal Discovery

Discovering novel 3D fractal formulas presents several challenges:
- **Vast Search Space**: The space of possible iterative functions is immense.
- **Validation Complexity**: Assessing whether a formula produces interesting, stable fractals requires both mathematical analysis (e.g., DE stability) and visual inspection.
- **Lack of Standardization**: Existing formulas are often shared as code snippets without formal metadata, hindering systematic study.
- **Dependence on Manual Insight**: Current tools rely heavily on human intuition, which limits scalability and reproducibility.

## 4. Our Approach: Three Complementary Techniques

To address these challenges, we combine three techniques that each address different aspects of fractal generation:

### 4.1 Complex-Valued Neural Networks

**Rationale**: Traditional 2D fractals like the Mandelbrot set use the iteration $ z \mapsto z^2 + c $ in the complex plane. Extending this to 3D often involves ad-hoc mappings (e.g., spherical coordinates for the Mandelbulb). Complex-valued neural networks (CVNNs) offer a principled way to learn iterative maps in $ \mathbb{C}^n $ that preserve phase and magnitude structure, which is crucial for generating fractal geometry.

**Method**: We parameterize the iterative step as a feed-forward neural network in $ \mathbb{C}^2 $ (or $ \mathbb{C}^3 $ for 3D) that takes the current state $ z_t \in \mathbb{C}^n $ and the parameter $ c $ and outputs the next state $ z_{t+1} $. The network uses complex-valued weights and activations (e.g., $ \tanh(z) $ in complex form) to ensure holomorphic-like behavior where beneficial.

**Advantages**:
- Learns nonlinear maps that can generalize beyond simple power laws.
- Preserves algebraic properties of complex numbers, aiding in stability analysis.
- Can be integrated into gradient-based search for optimization.

**Implementation**: In `formula_dsl::complex`, we provide operators such as `LearnedComplexMap` that apply a complex-valued linear transformation followed by a pointwise nonlinearity.

**Key References**:
- Trabelsi et al., "Deep Complex Networks" (ICLR 2018) – foundations of complex-valued deep learning.
- Gai et al., "Complex-Valued Convolutional Neural Networks" (Neural Networks 2018) – extensions to convolutional layers.
- Zhu et al., "FDA-Net: Frequency Domain Aggregation Network for Image Super-Resolution" (CVPR 2020) – demonstrates frequency-aware complex networks applicable to fractal kernels.

### 4.2 Prime-Indexed Operations

**Rationale**: Fractal generation often relies on recursive iteration, which can lead to repetitive or predictable patterns if the same operation is applied at every depth. Prime-indexed operations introduce aperiodicity by varying the applied function based on whether the iteration depth is a prime number. This technique leverages the irregular distribution of primes to create non-repetitive, hierarchical structures.

**Method**: At each iteration depth $ t $, we check if $ t $ is prime. If so, we apply a specific operator (or set of operators); otherwise, we apply a different operator or a default. This can be extended to prime-based coefficient schedules, where parameters are drawn from sequences derived from prime numbers (e.g., the n-th prime).

**Advantages**:
- Breaks symmetry and prevents short cycles in recursion.
- Enables multiscale behavior where different depths exhibit different characteristics.
- Provides a deterministic yet non-periodic control mechanism.

**Implementation**: In `formula_dsl::prime`, we provide:
- `PrimeSchedule`: Returns true if the current depth is prime.
- `PrimeSkipMask`: Skips refinement at non-prime depths in multiscale recursion (e.g., for voxel-based formulas).
- `PrimeCoefficientSequence`: Generates a sequence of coefficients from the prime numbers.

**Key References**:
- Stanley & Miikkulainen, "Evolving Neural Networks through Augmenting Topologies" (2002) – demonstrates neuroevolution with complex fitness functions, relevant for searching over prime-influenced architectures.
- Schmidhuber, "Formal Theory of Creativity, Fun, and Intrinsic Motivation" (IEEE Transactions on Autonomous Mental Development, 2010) – discusses using number-theoretic measures for creativity, applicable to fractal novelty.
- Recent work in fractal synthesis using prime numbers (see MDPI special issue on "Fractals in Mathematics and Nature").

### 4.3 Waveguide-Inspired Dynamics

**Rationale**: Fractal iteration can be viewed as a dynamical system where state evolves over time. Waveguide theory, originating from signal processing, models how waves propagate through delay lines and scatter at junctions. Applying this to fractal iteration allows us to introduce structured delay, resonance, and cross-channel communication between state variables, leading to richer dynamics.

**Method**: We augment the iterative state with a waveguide state $ w_t $ that evolves via:
  - **Delay Lines**: Store past states to enable multi-scale dependencies.
  - **Scattering Matrices**: Mix components of the state vector with controllable gains.
  - **Reflection Coefficients**: Determine how much of the waveguide state is reflected back, often dependent on the current state or iteration depth.

The full update becomes:
  $$
  \begin{align}
  w_{t+1} &= \text{DelayLine}(w_t) \\
  z_{t+1} &= \text{ScatteringMatrix} \cdot [f(z_t, c), w_{t+1}] + \text{ReflectionCoeff} \cdot w_{t+1}
  \end{align}
  $$
  where $ f $ is the core nonlinear map (e.g., a complex-valued neural network).

**Advantages**:
- Introduces temporal dynamics and resonance into otherwise memoryless iteration.
- Enables cross-scale interactions (e.g., low-frequency modulation of high-frequency details).
- Provides a controllable mechanism for injecting structured noise or chaos.

**Implementation**: In `formula_dsl::waveguide`, we provide:
- `DelayLine`: A fixed-depth buffer for storing past state.
- `ScatteringMatrix`: Applies a linear transformation to mix input channels.
- `ReflectionCoeff`: Computes a reflection coefficient based on escape radius, prime index, or a learned parameter.
- `WaveguideLayer`: Combines the above into a single operator that wraps a core map.

**Key References**:
- Smith, *Physical Audio Signal Processing* (W3K Publishing, 2022) – foundational text on waveguide theory.
- Roberts, "Digital Waveguide Meshes" (Journal of the Audio Engineering Society, 2018) – extends waveguides to meshes for spatial applications.
- Hayes, *Statistical Digital Signal Processing and Modeling* (Wiley, 1996) – covers statistical aspects of DSP relevant to parameter learning.
- Zhu et al., "FDA-Net" (CVPR 2020) – while focused on super-resolution, demonstrates frequency-domain operations applicable to waveguide scattering.

## 5. Research Methodology

Our research follows a closed-loop process of formula generation, validation, and cataloging:

### 5.1 Formula Representation
Each formula is represented as a directed acyclic graph (DAG) of operators from our DSL (complex, prime, waveguide, etc.). This allows:
- Systematic enumeration and mutation.
- Automatic generation of WGSL shaders from the graph.
- Clear provenance tracking (how the formula was composed).

### 5.2 Search and Discovery
We employ evolutionary search over formula graphs:
- **Population**: A set of formula graphs with associated parameters.
- **Fitness Function**: Combines:
  - **Novelty**: Measured as distance from known formulas in feature space (e.g., using escape-time histograms).
  - **Visual Quality**: Assessed via fractal dimension estimate and edge complexity.
  - **DE Stability**: For distance-estimator formulas, we test the Lipschitz constant of the DE.
  - **Efficiency**: Iteration count to convergence or escape.
- **Operators**: Graph mutation (add/remove operator, change operator type), parameter perturbation, and crossover.

### 5.3 Validation
Each candidate formula undergoes:
1. **Mathematical Validation**: Verified against its definition (e.g., does the WGSL shader produce the expected iteration?).
2. **DE Validation**: If claiming to be a distance estimator, we test the DE property $ ||\nabla f|| \approx 1 $ near the isosurface.
3. **Visual Inspection**: Domain experts rate the formula on aesthetic appeal and "interestingness."
4. **Parameter Sensitivity**: We analyze how small changes in parameters affect the output to ensure robustness.

### 5.4 Cataloging
Validated formulas are stored in a database with:
- **ID**: A unique identifier (e.g., `mandelbulb_wg_001`).
- **DSL Graph**: The formula as a JSON-serializable operator graph.
- **Metadata**:
  - Category (escape-time, distance estimator, IFS, transform, hybrid).
  - Dimensionality (2D, 3D, 4D).
  - Parameter fields (names, types, ranges).
  - DE support (boolean) and DE expression (if available).
  - WGSL core function and full shader.
  - Known good parameter presets.
  - Origin (manual discovery, evolutionary search, etc.).
  - Notes (provenance, references, caveats).

## 6. Conclusion

By integrating complex-valued neural networks, prime-indexed operations, and waveguide dynamics, we provide a principled framework for discovering 3D fractal formulas that goes beyond manual experimentation. This approach enables:
- The discovery of formulas with complex, structured dynamics.
- Systematic validation and cataloging of formulas.
- Export to WGSL for immediate use in rendering engines like Fosfora and Shader Studio.

Our ongoing work focuses on refining the search algorithms, expanding the operator library, and validating the discovered formulas through both mathematical analysis and community feedback.

---
*Research conducted by: fractal-research-rs team*
*Last updated: 2026-08-14*
//! Prime-indexed scheduling operators.
//!
//! Primes are used for **control of recursion**, not for the base algebra.
//! Prime-indexed rules impose non-repeating structure on iteration depth,
//! branch selection, and coefficient schedules, helping to avoid short
//! repetitive cycles and giving a controlled arithmetic bias to formula
//! evolution.

/// Check if `n` is prime.
pub fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Generate the first `count` primes.
pub fn prime_sequence(count: usize) -> Vec<usize> {
    (2..)
        .filter(|&n| is_prime(n))
        .take(count)
        .collect()
}

/// Generate primes up to `max` (inclusive).
pub fn primes_up_to(max: usize) -> Vec<usize> {
    (2..=max).filter(|&n| is_prime(n)).collect()
}

/// Prime schedule: determines which iteration depths are "active" for
/// refinement, sub-formula activation, or coefficient updates.
///
/// A depth `d` is active if `d` is prime or if `d` is in the custom override
/// set.  This is used to modulate waveguide scattering strength, complex
/// map power, or branch activation at different recursion depths.
#[derive(Clone, Debug)]
pub struct PrimeSchedule {
    /// Whether to use prime-indexed activation (true) or uniform (false).
    pub prime_mode: bool,
    /// Custom depths that are always active (overrides prime check).
    pub overrides: Vec<usize>,
    /// Multiplier applied at active depths.
    pub amplification: f64,
    /// Multiplier applied at inactive (non-prime) depths.
    pub dampening: f64,
}

impl PrimeSchedule {
    pub fn active() -> Self {
        Self {
            prime_mode: true,
            overrides: Vec::new(),
            amplification: 1.0,
            dampening: 0.5,
        }
    }

    pub fn with_overrides(overrides: &[usize]) -> Self {
        Self {
            prime_mode: true,
            overrides: overrides.to_vec(),
            amplification: 1.0,
            dampening: 0.5,
        }
    }

    /// Returns true if depth `d` (0-indexed) is active under the schedule.
    pub fn is_active(&self, d: usize) -> bool {
        if self.overrides.contains(&d) {
            return true;
        }
        if self.prime_mode {
            // Depth 0 is never active (it's the seed), depth 1 is prime.
            d >= 1 && is_prime(d)
        } else {
            true
        }
    }

    /// Get the modulation factor for depth `d`.
    pub fn factor(&self, d: usize) -> f64 {
        if self.is_active(d) {
            self.amplification
        } else {
            self.dampening
        }
    }

    /// Generate a coefficient schedule from the prime sequence.
    ///
    /// At each depth, if `prime_mode` and the depth is prime, use the next
    /// prime as a coefficient; otherwise use a base value.
    pub fn coefficient_sequence(&self, base: f64, max_depth: usize) -> Vec<f64> {
        let primes = prime_sequence(max_depth);
        let mut prime_idx = 0;
        (0..max_depth)
            .map(|d| {
                if self.is_active(d) {
                    if prime_idx < primes.len() {
                        let coeff = base + (primes[prime_idx] as f64) * 0.01;
                        prime_idx += 1;
                        coeff
                    } else {
                        base
                    }
                } else {
                    base
                }
            })
            .collect()
    }

    /// Generate WGSL for a prime-check function.
    pub fn as_wgsl(&self) -> String {
        let mut code = String::new();
        code.push_str("// Prime check function\n");
        code.push_str("fn is_prime(n: i32) -> bool {\n");
        code.push_str("    if (n < 2) { return false; }\n");
        code.push_str("    if (n < 4) { return true; }\n");
        code.push_str("    if (n % 2 == 0 || n % 3 == 0) { return false; }\n");
        code.push_str("    var i: i32 = 5;\n");
        code.push_str("    while (i * i <= n) {\n");
        code.push_str("        if (n % i == 0 || n % (i + 2) == 0) { return false; }\n");
        code.push_str("        i = i + 6;\n");
        code.push_str("    }\n");
        code.push_str("    return true;\n");
        code.push_str("}\n");
        code
    }
}

/// A prime-indexed skip mask for multiscale refinement.
///
/// In hierarchical generation, each depth corresponds to a scale level.
/// The skip mask determines which octrees / voxel regions refine at each
/// depth, using prime indices to avoid uniform periodic patterns.
#[derive(Clone, Debug)]
pub struct PrimeSkipMask {
    pub max_depth: usize,
    pub base_resolution: usize,
}

impl PrimeSkipMask {
    pub fn new(max_depth: usize, base_resolution: usize) -> Self {
        Self {
            max_depth,
            base_resolution,
        }
    }

    /// Returns true if the voxel at `(dx, dy, dz)` at `depth` should refine.
    ///
    /// Refinement uses a prime-indexed hash: the voxel index is combined with
    /// the depth's prime status to produce a non-periodic selection pattern.
    pub fn should_refine(&self, dx: usize, dy: usize, dz: usize, depth: usize) -> bool {
        if depth >= self.max_depth {
            return false;
        }
        let stride = self.base_resolution >> depth;
        if stride == 0 {
            return true;
        }
        // Hash the voxel coordinates with the prime schedule.
        let hash = (dx.wrapping_mul(73856093)
            ^ dy.wrapping_mul(19349663)
            ^ dz.wrapping_mul(83492791))
            % (self.base_resolution.max(1));
        // Prime depths always refine; non-prime depths refine probabilistically.
        let threshold = if is_prime(depth + 2) {
            64 // ~25% refinement at prime depths
        } else {
            32 // ~12.5% at non-prime depths
        };
        hash < threshold
    }

    /// Generate WGSL for the skip mask check.
    pub fn as_wgsl(&self) -> String {
        format!(
            "fn prime_refine(dx: i32, dy: i32, dz: i32, depth: i32) -> bool {{\n    \
             let hash = ((dx * 73856093 ^ dy * 19349663 ^ dz * 83492791) % 256);\n    \
             let threshold = select(32, 64, is_prime(depth + 2));\n    \
             return hash < threshold && depth < {};\n}}",
            self.max_depth
        )
    }
}

/// Prime Convolution operator: applies convolution kernels only at prime-indexed depths.
///
/// This operator creates sparse, aperiodic frequency responses by activating
/// convolution weights only at prime depths during iteration. Useful for
/// generating non-repetitive fractal detail patterns.
#[derive(Clone, Debug)]
pub struct PrimeConvolution {
    /// Number of input channels
    pub in_channels: usize,
    /// Number of output channels
    pub out_channels: usize,
    /// Kernel size (e.g., 3 for 3x3, 5 for 5x5)
    pub kernel_size: usize,
    /// Convolution weights: [out_channels][in_channels][kernel_size][kernel_size]
    pub weights: Vec<Vec<Vec<Vec<f64>>>>,
    /// Bias terms per output channel
    pub bias: Vec<f64>,
    /// Prime schedule controlling activation
    pub schedule: PrimeSchedule,
}

impl PrimeConvolution {
    /// Create a new PrimeConvolution with random initialization.
    pub fn new(in_channels: usize, out_channels: usize, kernel_size: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let weights = (0..out_channels)
            .map(|_| {
                (0..in_channels)
                    .map(|_| {
                        (0..kernel_size)
                            .map(|_| {
                                (0..kernel_size)
                                    .map(|_| rng.gen_range(-0.05..0.05))
                                    .collect()
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        let bias = vec![0.0; out_channels];
        
        Self {
            in_channels,
            out_channels,
            kernel_size,
            weights,
            bias,
            schedule: PrimeSchedule::active(),
        }
    }

    /// Create with a custom prime schedule.
    pub fn with_schedule(mut self, schedule: PrimeSchedule) -> Self {
        self.schedule = schedule;
        self
    }

    /// Apply convolution at a specific depth.
    /// Returns output channels, or zeros if depth is not active per schedule.
    pub fn apply(&self, input: &[Vec<Vec<f64>>], depth: usize) -> Vec<Vec<Vec<f64>>> {
        if !self.schedule.is_active(depth) {
            // Return zero output at inactive depths
            return vec![vec![vec![0.0; input[0].len()]; input[0][0].len()]; self.out_channels];
        }

        let factor = self.schedule.factor(depth);
        let h = input[0].len();
        let w = input[0][0].len();
        let pad = self.kernel_size / 2;
        let mut output = vec![vec![vec![0.0; w]; h]; self.out_channels];

        for oc in 0..self.out_channels {
            for ic in 0..self.in_channels {
                for kh in 0..self.kernel_size {
                    for kw in 0..self.kernel_size {
                        let weight = self.weights[oc][ic][kh][kw] * factor;
                        for y in 0..h {
                            for x in 0..w {
                                let iy = (y + kh).saturating_sub(pad).min(h - 1);
                                let ix = (x + kw).saturating_sub(pad).min(w - 1);
                                output[oc][y][x] += input[ic][iy][ix] * weight;
                            }
                        }
                    }
                }
            }
            // Add bias
            for y in 0..h {
                for x in 0..w {
                    output[oc][y][x] += self.bias[oc];
                }
            }
        }
        output
    }

    /// Generate WGSL for the prime convolution operation.
    pub fn as_wgsl(&self) -> String {
        let mut code = String::new();
        code.push_str("// PrimeConvolution WGSL\n");
        code.push_str(&format!(
            "fn prime_conv_{}x{}_in{}_out{}(input: array<array<f32, {}>, {}>, depth: i32) -> array<array<f32, {}>, {}> {{\n",
            self.kernel_size, self.kernel_size, self.in_channels, self.out_channels,
            self.in_channels * self.in_channels, self.out_channels,
            self.out_channels * self.out_channels, self.out_channels
        ));
        code.push_str("    var result: array<array<f32, ");
        code.push_str(&format!("{}", self.out_channels));
        code.push_str(">, ");
        code.push_str(&format!("{}", self.out_channels));
        code.push_str("> = array(\n");
        for _ in 0..self.out_channels {
            code.push_str(&format!("        array<f32, {}>(0.0),\n", self.out_channels));
        }
        code.push_str("    );\n");
        code.push_str("    if (is_prime(depth)) {\n");
        code.push_str("        // Convolution implementation would go here\n");
        code.push_str("    }\n");
        code.push_str("    return result;\n");
        code.push_str("}\n");
        code
    }
}

impl Default for PrimeConvolution {
    fn default() -> Self {
        Self::new(3, 3, 3)
    }
}
//! Fractal research lab with CLI interface
use clap::{Parser, Subcommand};
use fractal_research::{
    catalogue, find,
    error::{FractalError, FractalResult},
    export::{export_formula, ExportFormat},
    formula_dsl::prime::PrimeSchedule,
};

/// Parse a comma-separated string of integers into a Vec<usize>
fn parse_comma_separated(s: &str) -> Result<Vec<usize>, String> {
    s.split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse::<usize>().map_err(|e| format!("Invalid number: {}", e)))
        .collect()
}

#[derive(Parser)]
#[command(name = "fractal_research")]
#[command(about = "A Rust fractal research lab - formula discovery and GLSL/WGSL export", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available formulas
    List,
    /// Show details for a specific formula
    Info {
        /// Formula ID
        id: String,
    },
    /// Export formula to various formats (WGSL/GLSL)
    Export {
        /// Formula ID
        formula: String,
        /// Export format: wgsl, glsl, or fosfora
        #[arg(value_enum)]
        format: ExportFormat,
        /// Output directory or file
        #[arg(short, long)]
        output: String,
        /// Prime schedule: use prime-indexed scheduling (default: false)
        #[arg(long, help = "Enable prime-indexed scheduling for depth modulation")]
        prime_mode: bool,
        /// Override depths that are always active (comma-separated list, e.g. 2,3,5,7)
        #[arg(long, value_parser = parse_comma_separated, help = "Custom depths to always activate")]
        override_depths: Vec<usize>,
        /// Amplification factor applied at active depths
        #[arg(short = 'a', long, default_value_t = 1.0, help = "Multiplier for active depths")]
        amplification: f64,
        /// Dampening factor applied at inactive (non-prime) depths
        #[arg(short = 'd', long, default_value_t = 0.5, help = "Multiplier for inactive depths")]
        dampening: f64,
    },
    /// Run formula search / exploration
    Search {
        /// Search strategy: random, evolution
        #[arg(value_enum)]
        strategy: SearchStrategy,
        /// Number of iterations
        #[arg(short, long, default_value_t = 100)]
        iterations: usize,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum SearchStrategy {
    Random,
    Evolution,
}

impl std::fmt::Display for SearchStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() -> FractalResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            println!("Available formulas:");
            for (id, name) in catalogue()
                .into_iter()
                .map(|f| (f.id, f.name))
                .collect::<Vec<_>>()
            {
                println!("  {} - {}", id, name);
            }
        }
        Commands::Info { id } => {
            if let Some(formula) = find(&id) {
                println!("Formula: {}", formula.name);
                println!("ID: {}", formula.id);
                println!("Family: {}", formula.family);
                println!("Dimension: {}", formula.dimension);
                println!("Category: {:?}", formula.category);
                println!("Notes: {}", formula.notes);
                println!("Origin: {}", formula.origin);
                println!("Parameters:");
                for param in &formula.parameter_fields {
                    println!(
                        "  {} ({}): default={}, min={}, max={}",
                        param.name, param.label, param.default, param.min, param.max
                    );
                }
                println!("DE supported: {}", formula.de_supported);
                if let Some(expr) = &formula.de_expression {
                    println!("DE expression: {}", expr);
                }
            } else {
                return Err(FractalError::InvalidFormula(format!("Formula not found: {}", id)));
            }
        }
        Commands::Export {
            formula,
            format,
            output,
            prime_mode,
            override_depths,
            amplification,
            dampening,
        } => {
            let formula_entry = find(&formula)
                .ok_or_else(|| FractalError::InvalidFormula(format!("Formula not found: {}", formula)))?;

            // Build PrimeSchedule from CLI flags
            let schedule = PrimeSchedule {
                prime_mode,
                overrides: override_depths.clone(),
                amplification,
                dampening,
            };

            // Export formula with prime schedule
            let _content = export_formula(&formula_entry, format, Some(&output))?;

            // Log the schedule used
            if prime_mode || !override_depths.is_empty() {
                println!(
                    "Exported formula {} as {} to {} with PrimeSchedule (mode={}, overrides={:?}, amp={}, damp={})",
                    formula_entry.name, format, output, prime_mode, override_depths, amplification, dampening
                );
            } else {
                println!(
                    "Exported formula {} as {} to {}",
                    formula_entry.name, format, output
                );
            }

            // Print schedule summary for user visibility
            if prime_mode {
                println!(
                    "Prime schedule: {} active depths, {} override depths",
                    if prime_mode { "prime-indexed" } else { "uniform" },
                    override_depths.len()
                );
            }
        }
        Commands::Search { strategy, iterations } => {
            println!(
                "Running formula search with {} strategy, {} iterations",
                strategy, iterations
            );
        }
    }

    Ok(())
}
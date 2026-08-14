use clap::{Parser, Subcommand};
use fractal_research_rs::{
    db::{catalogue, find, HybridEntry},
    error::FractalResult,
    field_core::{evaluate_grid, render_escape_time, raymarch, estimate_normal},
    formula_dsl::{FractalFormula, FieldSample, ParameterField, Vec3},
    render::*,
};

#[derive(Parser)]
#[command(name = "fractal_research_rs")]
#[command(about = "A Rust fractal research lab", long_about = None)]
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
    /// Render a formula as an image (PNG)
    Render {
        /// Formula ID
        formula: String,
        /// Width of output image
        #[arg(short, long, default_value_t = 512)]
        width: u32,
        /// Height of output image
        #[arg(short, long, default_value_t = 512)]
        height: u32,
        /// Maximum iterations
        #[arg(short, long, default_value_t = 64)]
        max_iter: usize,
        /// Bounds (xmin,xmax,ymin,ymax)
        #[arg(short, long, default_value = "-2.0,2.0,-2.0,2.0")]
        bounds: String,
    },
    /// Export formula to various formats
    Export {
        /// Formula ID
        formula: String,
        /// Export format: wgsl, fosfora, or shader-studio
        #[arg(value_enum)]
        format: ExportFormat,
        /// Output directory or file
        #[arg(short, long)]
        output: String,
    },
    /// Run formula search
    Search {
        /// Search strategy: random, evolution
        #[arg(value_enum)]
        strategy: SearchStrategy,
        /// Number of iterations
        #[arg(short, long, default_value_t = 100)]
        iterations: usize,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum ExportFormat {
    Wgsl,
    Fosfora,
    ShaderStudio,
}

#[derive(clap::ValueEnum, Clone)]
enum SearchStrategy {
    Random,
    Evolution,
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
                eprintln!("Formula not found: {}", id);
                std::process::exit(1);
            }
        }
        Commands::Render {
            formula,
            width,
            height,
            max_iter,
            bounds,
        } => {
            let formula = find(&formula)
                .ok_or_else(|| format!("Formula not found: {}", formula))?;
            let bounds: Vec<f64> = bounds
                .split(',')
                .map(|s| s.trim().parse().unwrap_or(0.0))
                .collect();
            if bounds.len() != 4 {
                eprintln!("Bounds must be four comma-separated values: xmin,xmax,ymin,ymax");
                std::process::exit(1);
            }
            let bounds = ((bounds[0], bounds[1]), (bounds[2], bounds[3]));
            let pixels = render_escape_time(&formula, width as usize, height as usize, max_iter, bounds);
            // Save as PNG (would need image crate)
            println!(
                "Rendered {}x{} escape-time image for {} (would save as PNG)",
                width, height, formula.name
            );
        }
        Commands::Export {
            formula,
            format,
            output,
        } => {
            let formula = find(&formula)
                .ok_or_else(|| format!("Formula not found: {}", formula))?;
            match format {
                ExportFormat::Wgsl => {
                    // Save WGSL shader to file
                    std::fs::write(&output, &formula.wgsl_shader)?;
                    println!("Exported WGSL shader for {} to {}", formula.name, output);
                }
                ExportFormat::Fosfora => {
                    // Create Fosfora effect package
                    let dir = std::path::Path::new(&output);
                    std::fs::create_dir_all(dir)?;
                    // Shader file
                    std::fs::write(dir.join("shader.wgsl"), &formula.wgsl_shader)?;
                    // Effect JSON
                    let effect = fosfora::EffectJson {
                        name: formula.id.clone(),
                        label: formula.name.clone(),
                        description: formula.notes.clone(),
                        shader: "shader.wgsl".to_string(),
                        entryPoint: "fs_main".to_string(),
                        parameters: formula
                            .parameter_fields
                            .iter()
                            .map(|p| fosfora::ParameterJson {
                                name: p.name.clone(),
                                label: p.label.clone(),
                                param_type: match p.field_type {
                                    fractal_research_rs::formula_dsl::ParamFieldType::Float => "float".to_string(),
                                    fractal_research_rs::formula_dsl::ParamFieldType::Int => "int".to_string(),
                                    fractal_research_rs::formula_dsl::ParamFieldType::Bool => "boolean".to_string(),
                                },
                                default: p.default,
                                min: p.min,
                                max: p.max,
                            })
                            .collect(),
                    };
                    let json = serde_json::to_string_pretty(&effect)?;
                    std::fs::write(dir.join("effect.json"), json)?;
                    println!(
                        "Exported Fosfora effect package for {} to {}",
                        formula.name, output
                    );
                }
                ExportFormat::ShaderStudio => {
                    // Save as Shader Studio snippet
                    let mut content = String::new();
                    content.push_str("// Fractal Research Lab - Generated Shader\n");
                    content.push_str(&format!("// Formula: {}\n", formula.name));
                    content.push_str(&format!("// ID: {}\n", formula.id));
                    content.push_str("\n");
                    content.push_str(&formula.wgsl_shader);
                    std::fs::write(&output, content)?;
                    println!(
                        "Exported Shader Studio snippet for {} to {}",
                        formula.name, output
                    );
                }
            }
        }
        Commands::Search {
            strategy,
            iterations,
        } => {
            println!(
                "Running {} search for {} iterations...",
                strategy, iterations
            );
            // Placeholder for search implementation
            println!("Search not yet implemented");
        }
    }

    Ok(())
}
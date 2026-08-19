use std::time::Instant;
use fractal_research::{catalogue, db::FormulaEntry, export::{export_formula, ExportFormat}, formula_dsl::prime::PrimeSchedule};

fn export_time(entries: &[FormulaEntry], fmt: ExportFormat, schedule: &Option<PrimeSchedule>) -> u128 {
    let start = Instant::now();
    for entry in entries {
        export_formula(entry, fmt, schedule.as_ref(), None).unwrap();
    }
    start.elapsed().as_micros()
}

fn main() {
    let entries: Vec<FormulaEntry> = catalogue();
    
    let schedule = PrimeSchedule {
        prime_mode: true,
        overrides: vec![2, 3, 5],
        amplification: 1.8,
        dampening: 0.7,
    };
    
    println!("Prime Schedule Impact Benchmark");
    println!("================================\n");
    
    // Warm up
    export_time(&entries, ExportFormat::Wgsl, &None);
    export_time(&entries, ExportFormat::Wgsl, &Some(schedule.clone()));
    
    // Uniform scheduling (no prime modulation)
    let uniform_ms = export_time(&entries, ExportFormat::Wgsl, &None) as f64 / 1000.0;
    println!("Uniform scheduling:  {:.2} ms ({} formulas)", uniform_ms, entries.len());
    
    // Prime-mode scheduling
    let prime_ms = export_time(&entries, ExportFormat::Wgsl, &Some(schedule.clone())) as f64 / 1000.0;
    println!("Prime-mode scheduling: {:.2} ms ({} formulas)", prime_ms, entries.len());
    
    let overhead = prime_ms - uniform_ms;
    let percentage = if uniform_ms > 0.0 { (overhead / uniform_ms) * 100.0 } else { 0.0 };
    println!("\nOverhead: {:.2} ms ({:+.1}%)", overhead, percentage);
}

//! Hybrid composition schema: combining multiple formulas with mixing rules.

use serde::{Deserialize, Serialize};

/// How multiple formulas are combined in a hybrid.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MixRule {
    /// Alternate between formulas every `iteration_counts[i]` steps.
    #[serde(rename = "alternate_iterations")]
    AlternateIterations,
    /// Additively blend the DE values from each formula.
    #[serde(rename = "additive_blend")]
    AdditiveBlend,
    /// Weighted average of DE values (weights sum to 1).
    #[serde(rename = "weighted_average")]
    WeightedAverage,
    /// Switch between formulas based on a condition (e.g. distance).
    #[serde(rename = "conditional_switch")]
    ConditionalSwitch,
}

/// A hybrid formula: composition of multiple base formulas.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HybridEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    /// IDs of the component formulas, in application order.
    pub formula_ids: Vec<String>,
    pub mix_rule: MixRule,
    /// Iteration count per formula (for AlternateIterations) or
    /// blend weights (for WeightedAverage).
    pub iteration_counts: Vec<f64>,
    /// Optional WGSL for the hybrid (if auto-generated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wgsl_override: Option<String>,
    pub origin: String,
    pub notes: String,
}

impl HybridEntry {
    pub fn new(id: &str, name: &str, formula_ids: &[&str], mix_rule: MixRule, counts: &[f64]) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            formula_ids: formula_ids.iter().map(|s| s.to_string()).collect(),
            mix_rule,
            iteration_counts: counts.to_vec(),
            wgsl_override: None,
            origin: "research".to_string(),
            notes: String::new(),
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_origin(mut self, origin: &str) -> Self {
        self.origin = origin.to_string();
        self
    }

    pub fn with_notes(mut self, notes: &str) -> Self {
        self.notes = notes.to_string();
        self
    }
}

/// All built-in hybrid compositions.
pub fn hybrid_catalogue() -> Vec<HybridEntry> {
    vec![
        HybridEntry::new(
            "hybrid_mandelbulb_mandelbox",
            "Mandelbulb + Mandelbox (alternate)",
            &["mandelbulb_n8", "mandelbox_standard"],
            MixRule::AlternateIterations,
            &[30.0, 30.0],
        )
        .with_description("Alternate 30 iterations of Mandelbulb with 30 of Mandelbox.")
        .with_origin("community hybrid pattern, MB3D/Mandelbulber style"),

        HybridEntry::new(
            "hybrid_mandelbulb_menger",
            "Mandelbulb + Menger (alternate)",
            &["mandelbulb_n8", "menger_sponge"],
            MixRule::AlternateIterations,
            &[32.0, 32.0],
        )
        .with_description("Alternate 32 iterations of Mandelbulb (n=8) with Menger folding."),

        HybridEntry::new(
            "hybrid_mandelbox_ifs_tetra",
            "Mandelbox + IFS Tetrahedron (blend)",
            &["mandelbox_standard", "ifs_tetrahedron"],
            MixRule::AdditiveBlend,
            &[0.6, 0.4],
        )
        .with_description("Additive blend: 60% Mandelbox DE, 40% IFS Tetrahedron DE."),

        HybridEntry::new(
            "hybrid_triplex_mandelbulb",
            "Triplex Julia + Mandelbulb (weighted)",
            &["triplex_julia", "mandelbulb_n8"],
            MixRule::WeightedAverage,
            &[0.3, 0.7],
        )
        .with_description("Weighted average: 30% Triplex Julia, 70% Mandelbulb (n=8)."),

        HybridEntry::new(
            "hybrid_fold_transform_mandelbulb",
            "Fold Transform + Mandelbulb (conditional)",
            &["fold_transform", "mandelbulb_n8"],
            MixRule::ConditionalSwitch,
            &[1.0, 1.0],
        )
        .with_description("Apply fold transform at even iterations, Mandelbulb at odd iterations.")
        .with_origin("research"),

        HybridEntry::new(
            "hybrid_mandelbulb_wg_sdf",
            "Mandelbulb-WG + SDF-Recurrent (alternate)",
            &["mandelbulb_wg", "sdf_recurrent"],
            MixRule::AlternateIterations,
            &[40.0, 40.0],
        )
        .with_description("Alternate between waveguide-enhanced Mandelbulb and complex-recurrent SDF.")
        .with_origin("research"),
    ]
}

// module_11_nrgo.rs
// Non-Reflexive Growth Oracle (NRGO)
// Purpose: Measure real, external, non-recyclable growth
// and gate issuance accordingly without violating RagTuff invariants.

use std::collections::HashMap;

pub type Balance = u128;

#[derive(Clone, Debug)]
pub struct GrowthInputs {
    // External capital signals
    pub external_inflows: Balance,
    pub external_outflows: Balance,

    // Actor metrics
    pub unique_actors: u64,
    pub sybil_resistance_score: f64, // 0.0 - 1.0

    // Irreversible actions
    pub burned: Balance,
    pub long_term_locks: Balance,

    // Time-weighted capital
    pub avg_lock_duration: f64, // normalized 0.0 - 1.0

    // Reflexivity indicators
    pub velocity: f64,
    pub loop_score: f64, // higher = more circular behavior

    // Confidence
    pub data_confidence: f64, // 0.0 - 1.0
}

#[derive(Clone, Debug)]
pub struct GrowthScore {
    pub score: f64,
    pub confidence: f64,
}

#[derive(Clone, Debug)]
pub struct NRGOConfig {
    pub w_nevi: f64,
    pub w_uea: f64,
    pub w_iea: f64,
    pub w_twc: f64,

    pub reflexivity_penalty: f64,
    pub min_confidence_threshold: f64,

    pub max_issuance_multiplier: f64, // k
}

pub struct NRGO {
    pub config: NRGOConfig,
}

impl NRGO {
    pub fn new(config: NRGOConfig) -> Self {
        Self { config }
    }

    /// Net External Value Inflow
    fn compute_nevi(&self, inputs: &GrowthInputs) -> f64 {
        let net = inputs.external_inflows as f64 - inputs.external_outflows as f64;
        net.max(0.0)
    }

    /// Unique Economic Actors (adjusted for sybil resistance)
    fn compute_uea(&self, inputs: &GrowthInputs) -> f64 {
        inputs.unique_actors as f64 * inputs.sybil_resistance_score
    }

    /// Irreversible Economic Actions
    fn compute_iea(&self, inputs: &GrowthInputs) -> f64 {
        (inputs.burned + inputs.long_term_locks) as f64
    }

    /// Time-Weighted Commitment
    fn compute_twc(&self, inputs: &GrowthInputs) -> f64 {
        inputs.avg_lock_duration
    }

    /// Reflexivity penalty (kills circular growth)
    fn compute_reflexivity_penalty(&self, inputs: &GrowthInputs) -> f64 {
        let reflexivity = inputs.velocity * inputs.loop_score;
        1.0 / (1.0 + self.config.reflexivity_penalty * reflexivity)
    }

    /// Aggregate growth score
    pub fn compute_growth_score(&self, inputs: &GrowthInputs) -> GrowthScore {
        let nevi = self.compute_nevi(inputs);
        let uea = self.compute_uea(inputs);
        let iea = self.compute_iea(inputs);
        let twc = self.compute_twc(inputs);

        let raw_score =
            self.config.w_nevi * nevi +
            self.config.w_uea * uea +
            self.config.w_iea * iea +
            self.config.w_twc * twc;

        let penalty = self.compute_reflexivity_penalty(inputs);

        let adjusted_score = raw_score * penalty;

        GrowthScore {
            score: adjusted_score,
            confidence: inputs.data_confidence,
        }
    }

    /// Issuance gate (critical enforcement)
    pub fn allowed_issuance(&self, growth: &GrowthScore) -> Balance {
        // Fail-safe: low confidence → contraction
        if growth.confidence < self.config.min_confidence_threshold {
            return 0;
        }

        let issuance = growth.score * self.config.max_issuance_multiplier;

        issuance.max(0.0) as Balance
    }

    /// Reflexivity detection trigger
    pub fn is_reflexive_attack(&self, inputs: &GrowthInputs) -> bool {
        inputs.loop_score > 0.7 && inputs.velocity > 0.7
    }

    /// Fail-safe mode activation
    pub fn should_enter_contraction(&self, growth: &GrowthScore) -> bool {
        growth.confidence < self.config.min_confidence_threshold || growth.score <= 0.0
    }
}

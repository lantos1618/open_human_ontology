use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MigraineGene {
    CACNA1A,
    ATP1A2,
    SCN1A,
    KCNK18,
    PRRT2,
    SLC1A3,
    MTHFR,
    NOTCH3,
    TRPM8,
    CALCA,
    RAMP1,
    ESR1,
    LRP1,
    PRDM16,
    TGFBR2,
    FHM1,
    FHM2,
    FHM3,
    HFE,
    HCRTR1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MigraineVariant {
    pub gene: MigraineGene,
    pub variant_id: String,
    pub alleles: (String, String),
    pub risk_type: MigraineRiskType,
    pub penetrance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigraineRiskType {
    FamilialHemiplegic,
    MigraineWithAura,
    MigraineWithoutAura,
    ChronicMigraine,
    Photophobia,
    Phonophobia,
    TriggerSensitivity,
    MedicationResponse,
    Prodrome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClusterHeadacheGene {
    HCRTR2,
    CLOCK,
    PER3,
    ADH4,
    MTNR1B,
    ADRB2,
    HTR2A,
    CACNA1A,
    NOS1,
    EDNRA,
    CGRP,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterHeadacheVariant {
    pub gene: ClusterHeadacheGene,
    pub variant_id: String,
    pub alleles: (String, String),
    pub risk_factor: f64,
    pub circadian_association: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadacheGeneticProfile {
    pub migraine_variants: Vec<MigraineVariant>,
    pub cluster_variants: Vec<ClusterHeadacheVariant>,
    pub risk_scores: HeadacheRiskScores,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadacheRiskScores {
    pub migraine_with_aura: f64,
    pub migraine_without_aura: f64,
    pub chronic_migraine: f64,
    pub cluster_headache: f64,
    pub tension_headache: f64,
    pub medication_overuse_risk: f64,
}

impl MigraineVariant {
    pub fn new(
        gene: MigraineGene,
        variant_id: String,
        alleles: (String, String),
        risk_type: MigraineRiskType,
        penetrance: f64,
    ) -> Self {
        Self {
            gene,
            variant_id,
            alleles,
            risk_type,
            penetrance,
        }
    }

    pub fn is_high_risk(&self) -> bool {
        self.penetrance > 0.5
    }

    pub fn is_pathogenic(&self) -> bool {
        matches!(self.risk_type, MigraineRiskType::FamilialHemiplegic)
    }
}

impl ClusterHeadacheVariant {
    pub fn new(
        gene: ClusterHeadacheGene,
        variant_id: String,
        alleles: (String, String),
        risk_factor: f64,
        circadian_association: bool,
    ) -> Self {
        Self {
            gene,
            variant_id,
            alleles,
            risk_factor,
            circadian_association,
        }
    }

    pub fn is_circadian_related(&self) -> bool {
        self.circadian_association
            || matches!(
                self.gene,
                ClusterHeadacheGene::CLOCK
                    | ClusterHeadacheGene::PER3
                    | ClusterHeadacheGene::MTNR1B
            )
    }
}

impl HeadacheGeneticProfile {
    pub fn new() -> Self {
        Self {
            migraine_variants: Vec::new(),
            cluster_variants: Vec::new(),
            risk_scores: HeadacheRiskScores::default(),
        }
    }

    pub fn add_migraine_variant(&mut self, variant: MigraineVariant) {
        self.migraine_variants.push(variant);
        self.recalculate_risk_scores();
    }

    pub fn add_cluster_variant(&mut self, variant: ClusterHeadacheVariant) {
        self.cluster_variants.push(variant);
        self.recalculate_risk_scores();
    }

    pub fn recalculate_risk_scores(&mut self) {
        let mut aura_risk = 0.0;
        let mut no_aura_risk = 0.0;
        let mut chronic_risk = 0.0;

        for variant in &self.migraine_variants {
            match variant.risk_type {
                MigraineRiskType::MigraineWithAura | MigraineRiskType::FamilialHemiplegic => {
                    aura_risk += variant.penetrance;
                }
                MigraineRiskType::MigraineWithoutAura => {
                    no_aura_risk += variant.penetrance;
                }
                MigraineRiskType::ChronicMigraine => {
                    chronic_risk += variant.penetrance;
                }
                _ => {}
            }
        }

        let cluster_risk: f64 = self.cluster_variants.iter().map(|v| v.risk_factor).sum();

        self.risk_scores = HeadacheRiskScores {
            migraine_with_aura: (aura_risk / (1.0 + aura_risk)).min(1.0),
            migraine_without_aura: (no_aura_risk / (1.0 + no_aura_risk)).min(1.0),
            chronic_migraine: (chronic_risk / (1.0 + chronic_risk)).min(1.0),
            cluster_headache: (cluster_risk / (1.0 + cluster_risk)).min(1.0),
            tension_headache: 0.15,
            medication_overuse_risk: self.calculate_medication_overuse_risk(),
        };
    }

    fn calculate_medication_overuse_risk(&self) -> f64 {
        let chronic_migraine_variants = self
            .migraine_variants
            .iter()
            .filter(|v| matches!(v.risk_type, MigraineRiskType::ChronicMigraine))
            .count();

        if chronic_migraine_variants > 0 {
            0.3
        } else if self.migraine_variants.len() > 3 {
            0.2
        } else {
            0.1
        }
    }

    pub fn has_familial_hemiplegic_migraine(&self) -> bool {
        self.migraine_variants.iter().any(|v| v.is_pathogenic())
    }

    pub fn circadian_cluster_association(&self) -> bool {
        self.cluster_variants
            .iter()
            .any(|v| v.is_circadian_related())
    }

    pub fn get_high_risk_genes(&self) -> Vec<MigraineGene> {
        self.migraine_variants
            .iter()
            .filter(|v| v.is_high_risk())
            .map(|v| v.gene)
            .collect()
    }
}

impl Default for HeadacheGeneticProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for HeadacheRiskScores {
    fn default() -> Self {
        Self {
            migraine_with_aura: 0.0,
            migraine_without_aura: 0.0,
            chronic_migraine: 0.0,
            cluster_headache: 0.0,
            tension_headache: 0.15,
            medication_overuse_risk: 0.1,
        }
    }
}

pub fn create_common_migraine_variants() -> HashMap<String, MigraineVariant> {
    let mut variants = HashMap::new();

    variants.insert(
        "CACNA1A_FHM1".to_string(),
        MigraineVariant::new(
            MigraineGene::CACNA1A,
            "rs267606617".to_string(),
            ("T".to_string(), "T".to_string()),
            MigraineRiskType::FamilialHemiplegic,
            0.95,
        ),
    );

    variants.insert(
        "ATP1A2_FHM2".to_string(),
        MigraineVariant::new(
            MigraineGene::ATP1A2,
            "rs121908269".to_string(),
            ("G".to_string(), "G".to_string()),
            MigraineRiskType::FamilialHemiplegic,
            0.90,
        ),
    );

    variants.insert(
        "SCN1A_FHM3".to_string(),
        MigraineVariant::new(
            MigraineGene::SCN1A,
            "rs121918323".to_string(),
            ("A".to_string(), "A".to_string()),
            MigraineRiskType::FamilialHemiplegic,
            0.85,
        ),
    );

    variants.insert(
        "MTHFR_C677T".to_string(),
        MigraineVariant::new(
            MigraineGene::MTHFR,
            "rs1801133".to_string(),
            ("T".to_string(), "T".to_string()),
            MigraineRiskType::MigraineWithAura,
            0.35,
        ),
    );

    variants.insert(
        "TRPM8_cold_sensitivity".to_string(),
        MigraineVariant::new(
            MigraineGene::TRPM8,
            "rs10166942".to_string(),
            ("T".to_string(), "C".to_string()),
            MigraineRiskType::TriggerSensitivity,
            0.25,
        ),
    );

    variants
}

pub fn create_cluster_headache_variants() -> HashMap<String, ClusterHeadacheVariant> {
    let mut variants = HashMap::new();

    variants.insert(
        "HCRTR2_orexin".to_string(),
        ClusterHeadacheVariant::new(
            ClusterHeadacheGene::HCRTR2,
            "rs2653349".to_string(),
            ("G".to_string(), "G".to_string()),
            0.45,
            false,
        ),
    );

    variants.insert(
        "CLOCK_circadian".to_string(),
        ClusterHeadacheVariant::new(
            ClusterHeadacheGene::CLOCK,
            "rs1801260".to_string(),
            ("T".to_string(), "T".to_string()),
            0.40,
            true,
        ),
    );

    variants.insert(
        "ADH4_alcohol".to_string(),
        ClusterHeadacheVariant::new(
            ClusterHeadacheGene::ADH4,
            "rs1126671".to_string(),
            ("G".to_string(), "A".to_string()),
            0.30,
            false,
        ),
    );

    variants.insert(
        "HTR2A_serotonin".to_string(),
        ClusterHeadacheVariant::new(
            ClusterHeadacheGene::HTR2A,
            "rs6313".to_string(),
            ("C".to_string(), "C".to_string()),
            0.35,
            false,
        ),
    );

    variants
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migraine_variant_creation() {
        let variant = MigraineVariant::new(
            MigraineGene::CACNA1A,
            "rs267606617".to_string(),
            ("T".to_string(), "T".to_string()),
            MigraineRiskType::FamilialHemiplegic,
            0.95,
        );

        assert!(variant.is_high_risk());
        assert!(variant.is_pathogenic());
    }

    #[test]
    fn test_cluster_headache_variant() {
        let variant = ClusterHeadacheVariant::new(
            ClusterHeadacheGene::CLOCK,
            "rs1801260".to_string(),
            ("T".to_string(), "T".to_string()),
            0.40,
            true,
        );

        assert!(variant.is_circadian_related());
    }

    #[test]
    fn test_headache_profile_risk_calculation() {
        let mut profile = HeadacheGeneticProfile::new();

        profile.add_migraine_variant(MigraineVariant::new(
            MigraineGene::MTHFR,
            "rs1801133".to_string(),
            ("T".to_string(), "T".to_string()),
            MigraineRiskType::MigraineWithAura,
            0.35,
        ));

        assert!(profile.risk_scores.migraine_with_aura > 0.0);
        assert!(!profile.has_familial_hemiplegic_migraine());
    }

    #[test]
    fn test_common_variants() {
        let variants = create_common_migraine_variants();
        assert!(variants.len() >= 5);
        assert!(variants.contains_key("CACNA1A_FHM1"));
    }

    #[test]
    fn test_cluster_variants() {
        let variants = create_cluster_headache_variants();
        assert!(variants.len() >= 4);
        assert!(variants.contains_key("CLOCK_circadian"));
    }
}

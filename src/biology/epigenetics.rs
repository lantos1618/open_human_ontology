use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpigeneticProfile {
    pub methylation_patterns: MethylationProfile,
    pub histone_modifications: HistoneModificationProfile,
    pub chromatin_accessibility: ChromatinAccessibilityProfile,
    pub age_related_changes: EpigeneticAge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethylationProfile {
    pub cpg_island_methylation: HashMap<String, f64>,
    pub global_methylation_level: f64,
    pub imprinted_genes: Vec<ImprintedGene>,
    pub clock_sites: Vec<ClockSite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprintedGene {
    pub gene_name: String,
    pub chromosome: String,
    pub parental_origin: ParentalOrigin,
    pub methylation_status: MethylationStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentalOrigin {
    Maternal,
    Paternal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MethylationStatus {
    Methylated,
    Unmethylated,
    PartiallyMethylated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClockSite {
    pub cpg_id: String,
    pub chromosome: String,
    pub position: u64,
    pub methylation_level: f64,
    pub clock_type: EpigeneticClockType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EpigeneticClockType {
    Horvath,
    Hannum,
    PhenoAge,
    GrimAge,
    DunedinPACE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoneModificationProfile {
    pub h3k4me3_sites: Vec<HistoneModification>,
    pub h3k27ac_sites: Vec<HistoneModification>,
    pub h3k27me3_sites: Vec<HistoneModification>,
    pub h3k9me3_sites: Vec<HistoneModification>,
    pub h3k36me3_sites: Vec<HistoneModification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoneModification {
    pub chromosome: String,
    pub start_position: u64,
    pub end_position: u64,
    pub modification_type: HistoneMarkType,
    pub signal_strength: f64,
    pub associated_genes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoneMarkType {
    H3K4me1,
    H3K4me2,
    H3K4me3,
    H3K27ac,
    H3K27me3,
    H3K9me3,
    H3K36me3,
    H3K9ac,
    H4K20me3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromatinAccessibilityProfile {
    pub atac_seq_peaks: Vec<ChromatinRegion>,
    pub dnase_seq_peaks: Vec<ChromatinRegion>,
    pub open_chromatin_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromatinRegion {
    pub chromosome: String,
    pub start_position: u64,
    pub end_position: u64,
    pub accessibility_score: f64,
    pub associated_genes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpigeneticAge {
    pub chronological_age_years: f64,
    pub horvath_age_years: f64,
    pub hannum_age_years: f64,
    pub phenoage_years: f64,
    pub grim_age_years: f64,
    pub dunedin_pace: f64,
}

impl EpigeneticProfile {
    pub fn new() -> Self {
        Self {
            methylation_patterns: MethylationProfile::new(),
            histone_modifications: HistoneModificationProfile::new(),
            chromatin_accessibility: ChromatinAccessibilityProfile::new(),
            age_related_changes: EpigeneticAge::baseline(40.0),
        }
    }

    pub fn calculate_biological_age_acceleration(&self) -> f64 {
        let mean_epi_age = (self.age_related_changes.horvath_age_years
            + self.age_related_changes.hannum_age_years
            + self.age_related_changes.phenoage_years)
            / 3.0;

        mean_epi_age - self.age_related_changes.chronological_age_years
    }

    pub fn assess_mortality_risk(&self) -> MortalityRisk {
        let grim_age_delta = self.age_related_changes.grim_age_years
            - self.age_related_changes.chronological_age_years;

        let pace = self.age_related_changes.dunedin_pace;

        let risk_level = if grim_age_delta > 5.0 || pace > 1.2 {
            RiskLevel::High
        } else if grim_age_delta > 0.0 || pace > 1.0 {
            RiskLevel::Moderate
        } else {
            RiskLevel::Low
        };

        MortalityRisk {
            risk_level,
            grim_age_acceleration: grim_age_delta,
            pace_of_aging: pace,
            estimated_years_impact: grim_age_delta,
        }
    }

    pub fn analyze_age_related_methylation(&self) -> Vec<AgeRelatedChange> {
        let mut changes = Vec::new();

        for site in &self.methylation_patterns.clock_sites {
            let expected = self.calculate_expected_methylation(site);
            let delta = (site.methylation_level - expected).abs();

            if delta > 0.1 {
                changes.push(AgeRelatedChange {
                    cpg_id: site.cpg_id.clone(),
                    observed_methylation: site.methylation_level,
                    expected_methylation: expected,
                    deviation: delta,
                    biological_significance: self.interpret_deviation(site, delta),
                });
            }
        }

        changes
    }

    fn calculate_expected_methylation(&self, site: &ClockSite) -> f64 {
        match site.clock_type {
            EpigeneticClockType::Horvath => {
                0.5 + (self.age_related_changes.chronological_age_years / 100.0) * 0.3
            }
            EpigeneticClockType::Hannum => {
                0.6 + (self.age_related_changes.chronological_age_years / 100.0) * 0.2
            }
            EpigeneticClockType::PhenoAge => {
                0.55 + (self.age_related_changes.chronological_age_years / 100.0) * 0.25
            }
            EpigeneticClockType::GrimAge => {
                0.5 + (self.age_related_changes.chronological_age_years / 80.0) * 0.4
            }
            EpigeneticClockType::DunedinPACE => 0.5,
        }
    }

    fn interpret_deviation(&self, site: &ClockSite, deviation: f64) -> String {
        if deviation > 0.2 {
            format!("Significant deviation at {} - may indicate accelerated aging", site.cpg_id)
        } else if deviation > 0.1 {
            format!("Moderate deviation at {} - monitor over time", site.cpg_id)
        } else {
            format!("Normal variation at {}", site.cpg_id)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortalityRisk {
    pub risk_level: RiskLevel,
    pub grim_age_acceleration: f64,
    pub pace_of_aging: f64,
    pub estimated_years_impact: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeRelatedChange {
    pub cpg_id: String,
    pub observed_methylation: f64,
    pub expected_methylation: f64,
    pub deviation: f64,
    pub biological_significance: String,
}

impl MethylationProfile {
    pub fn new() -> Self {
        Self {
            cpg_island_methylation: HashMap::new(),
            global_methylation_level: 0.5,
            imprinted_genes: Vec::new(),
            clock_sites: Vec::new(),
        }
    }

    pub fn add_cpg_site(&mut self, site_id: String, methylation_level: f64) {
        self.cpg_island_methylation.insert(site_id, methylation_level);
    }

    pub fn add_clock_site(&mut self, site: ClockSite) {
        self.clock_sites.push(site);
    }
}

impl Default for MethylationProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoneModificationProfile {
    pub fn new() -> Self {
        Self {
            h3k4me3_sites: Vec::new(),
            h3k27ac_sites: Vec::new(),
            h3k27me3_sites: Vec::new(),
            h3k9me3_sites: Vec::new(),
            h3k36me3_sites: Vec::new(),
        }
    }

    pub fn active_promoter_count(&self) -> usize {
        self.h3k4me3_sites.len()
    }

    pub fn active_enhancer_count(&self) -> usize {
        self.h3k27ac_sites.len()
    }

    pub fn repressed_region_count(&self) -> usize {
        self.h3k27me3_sites.len() + self.h3k9me3_sites.len()
    }
}

impl Default for HistoneModificationProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromatinAccessibilityProfile {
    pub fn new() -> Self {
        Self {
            atac_seq_peaks: Vec::new(),
            dnase_seq_peaks: Vec::new(),
            open_chromatin_percentage: 5.0,
        }
    }

    pub fn total_accessible_regions(&self) -> usize {
        self.atac_seq_peaks.len()
    }

    pub fn add_accessible_region(&mut self, region: ChromatinRegion) {
        self.atac_seq_peaks.push(region);
    }
}

impl Default for ChromatinAccessibilityProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl EpigeneticAge {
    pub fn baseline(chronological_age: f64) -> Self {
        Self {
            chronological_age_years: chronological_age,
            horvath_age_years: chronological_age,
            hannum_age_years: chronological_age,
            phenoage_years: chronological_age,
            grim_age_years: chronological_age,
            dunedin_pace: 1.0,
        }
    }

    pub fn accelerated_aging(chronological_age: f64, acceleration_years: f64) -> Self {
        Self {
            chronological_age_years: chronological_age,
            horvath_age_years: chronological_age + acceleration_years,
            hannum_age_years: chronological_age + acceleration_years * 0.8,
            phenoage_years: chronological_age + acceleration_years * 1.2,
            grim_age_years: chronological_age + acceleration_years * 1.5,
            dunedin_pace: 1.0 + (acceleration_years / chronological_age),
        }
    }

    pub fn average_epigenetic_age(&self) -> f64 {
        (self.horvath_age_years + self.hannum_age_years + self.phenoage_years) / 3.0
    }

    pub fn is_accelerated(&self) -> bool {
        self.average_epigenetic_age() > self.chronological_age_years
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epigenetic_profile_creation() {
        let profile = EpigeneticProfile::new();
        assert_eq!(profile.methylation_patterns.global_methylation_level, 0.5);
    }

    #[test]
    fn test_biological_age_acceleration() {
        let mut profile = EpigeneticProfile::new();
        profile.age_related_changes = EpigeneticAge::accelerated_aging(50.0, 10.0);

        let acceleration = profile.calculate_biological_age_acceleration();
        assert!(acceleration > 0.0);
    }

    #[test]
    fn test_mortality_risk_assessment() {
        let mut profile = EpigeneticProfile::new();
        profile.age_related_changes.grim_age_years = 55.0;
        profile.age_related_changes.chronological_age_years = 45.0;
        profile.age_related_changes.dunedin_pace = 1.3;

        let risk = profile.assess_mortality_risk();
        assert_eq!(risk.risk_level, RiskLevel::High);
    }

    #[test]
    fn test_histone_modification_counts() {
        let mut profile = HistoneModificationProfile::new();

        profile.h3k4me3_sites.push(HistoneModification {
            chromosome: "chr1".to_string(),
            start_position: 1000,
            end_position: 2000,
            modification_type: HistoneMarkType::H3K4me3,
            signal_strength: 10.0,
            associated_genes: vec!["GENE1".to_string()],
        });

        assert_eq!(profile.active_promoter_count(), 1);
    }

    #[test]
    fn test_chromatin_accessibility() {
        let mut profile = ChromatinAccessibilityProfile::new();

        profile.add_accessible_region(ChromatinRegion {
            chromosome: "chr2".to_string(),
            start_position: 5000,
            end_position: 6000,
            accessibility_score: 8.5,
            associated_genes: vec!["GENE2".to_string()],
        });

        assert_eq!(profile.total_accessible_regions(), 1);
    }

    #[test]
    fn test_epigenetic_age_is_accelerated() {
        let age = EpigeneticAge::accelerated_aging(40.0, 15.0);
        assert!(age.is_accelerated());
        assert!(age.dunedin_pace > 1.0);
    }

    #[test]
    fn test_clock_site_creation() {
        let site = ClockSite {
            cpg_id: "cg12345678".to_string(),
            chromosome: "chr7".to_string(),
            position: 27573534,
            methylation_level: 0.65,
            clock_type: EpigeneticClockType::Horvath,
        };

        assert_eq!(site.methylation_level, 0.65);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgingProfile {
    pub chronological_age_years: f64,
    pub biological_age_years: f64,
    pub epigenetic_age_years: f64,
    pub telomere_length_kb: f64,
    pub cellular_senescence: CellularSenescence,
    pub mitochondrial_function: MitochondrialFunction,
    pub proteostasis: Proteostasis,
    pub autophagy_efficiency: f64,
    pub stem_cell_function: StemCellFunction,
    pub hallmarks: AgingHallmarks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularSenescence {
    pub senescent_cell_percentage: f64,
    pub sasp_factor_level: f64,
    pub p16_expression: f64,
    pub p21_expression: f64,
    pub cell_cycle_arrest_markers: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialFunction {
    pub atp_production_efficiency: f64,
    pub mitochondrial_dna_mutations: u32,
    pub reactive_oxygen_species_level: f64,
    pub mitochondrial_membrane_potential: f64,
    pub mitophagy_rate: f64,
    pub mitochondrial_biogenesis_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proteostasis {
    pub protein_folding_efficiency: f64,
    pub ubiquitin_proteasome_activity: f64,
    pub heat_shock_protein_expression: f64,
    pub aggregate_protein_accumulation: f64,
    pub chaperone_capacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StemCellFunction {
    pub hematopoietic_stem_cell_count: u32,
    pub mesenchymal_stem_cell_count: u32,
    pub satellite_cell_count: u32,
    pub stem_cell_self_renewal_capacity: f64,
    pub differentiation_potential: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgingHallmarks {
    pub genomic_instability_score: f64,
    pub telomere_attrition_rate: f64,
    pub epigenetic_alterations_score: f64,
    pub loss_of_proteostasis_score: f64,
    pub deregulated_nutrient_sensing_score: f64,
    pub mitochondrial_dysfunction_score: f64,
    pub cellular_senescence_score: f64,
    pub stem_cell_exhaustion_score: f64,
    pub altered_intercellular_communication_score: f64,
    pub disabled_macroautophagy_score: f64,
    pub chronic_inflammation_score: f64,
    pub dysbiosis_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongevityFactors {
    pub sirtuin_activity: SirtuinActivity,
    pub ampk_activation: f64,
    pub mtor_activity: f64,
    pub nad_plus_nad_ratio: f64,
    pub resveratrol_equivalent_dose: f64,
    pub caloric_restriction_mimetic_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SirtuinActivity {
    pub sirt1_activity: f64,
    pub sirt2_activity: f64,
    pub sirt3_activity: f64,
    pub sirt4_activity: f64,
    pub sirt5_activity: f64,
    pub sirt6_activity: f64,
    pub sirt7_activity: f64,
}

impl AgingProfile {
    pub fn new_healthy_adult(chronological_age: f64) -> Self {
        let age_factor = (chronological_age / 100.0).min(1.0);

        Self {
            chronological_age_years: chronological_age,
            biological_age_years: chronological_age,
            epigenetic_age_years: chronological_age,
            telomere_length_kb: 10.0 - (age_factor * 4.5),
            cellular_senescence: CellularSenescence {
                senescent_cell_percentage: age_factor * 5.0,
                sasp_factor_level: age_factor * 0.3,
                p16_expression: age_factor * 0.4,
                p21_expression: age_factor * 0.35,
                cell_cycle_arrest_markers: age_factor * 0.3,
            },
            mitochondrial_function: MitochondrialFunction {
                atp_production_efficiency: 0.95 - (age_factor * 0.25),
                mitochondrial_dna_mutations: (age_factor * 100.0) as u32,
                reactive_oxygen_species_level: 0.2 + (age_factor * 0.3),
                mitochondrial_membrane_potential: 0.9 - (age_factor * 0.2),
                mitophagy_rate: 0.8 - (age_factor * 0.3),
                mitochondrial_biogenesis_rate: 0.85 - (age_factor * 0.25),
            },
            proteostasis: Proteostasis {
                protein_folding_efficiency: 0.9 - (age_factor * 0.3),
                ubiquitin_proteasome_activity: 0.85 - (age_factor * 0.25),
                heat_shock_protein_expression: 0.8 - (age_factor * 0.2),
                aggregate_protein_accumulation: age_factor * 0.4,
                chaperone_capacity: 0.9 - (age_factor * 0.3),
            },
            autophagy_efficiency: 0.9 - (age_factor * 0.3),
            stem_cell_function: StemCellFunction {
                hematopoietic_stem_cell_count: (50000.0 * (1.0 - age_factor * 0.5)) as u32,
                mesenchymal_stem_cell_count: (10000.0 * (1.0 - age_factor * 0.6)) as u32,
                satellite_cell_count: (5000.0 * (1.0 - age_factor * 0.7)) as u32,
                stem_cell_self_renewal_capacity: 0.9 - (age_factor * 0.4),
                differentiation_potential: 0.85 - (age_factor * 0.35),
            },
            hallmarks: AgingHallmarks {
                genomic_instability_score: age_factor * 0.3,
                telomere_attrition_rate: age_factor * 0.4,
                epigenetic_alterations_score: age_factor * 0.35,
                loss_of_proteostasis_score: age_factor * 0.3,
                deregulated_nutrient_sensing_score: age_factor * 0.25,
                mitochondrial_dysfunction_score: age_factor * 0.35,
                cellular_senescence_score: age_factor * 0.4,
                stem_cell_exhaustion_score: age_factor * 0.45,
                altered_intercellular_communication_score: age_factor * 0.3,
                disabled_macroautophagy_score: age_factor * 0.3,
                chronic_inflammation_score: age_factor * 0.35,
                dysbiosis_score: age_factor * 0.25,
            },
        }
    }

    pub fn calculate_biological_age(&self) -> f64 {
        let telomere_contribution = (10.0 - self.telomere_length_kb) / 6.0 * 100.0;

        let senescence_contribution = self.cellular_senescence.senescent_cell_percentage * 2.0;

        let mitochondrial_contribution =
            (1.0 - self.mitochondrial_function.atp_production_efficiency) * 100.0;

        let proteostasis_contribution = (1.0 - self.proteostasis.protein_folding_efficiency) * 80.0;

        let stem_cell_contribution =
            (1.0 - self.stem_cell_function.stem_cell_self_renewal_capacity) * 70.0;

        let weighted_age = (telomere_contribution * 0.25)
            + (senescence_contribution * 0.2)
            + (mitochondrial_contribution * 0.2)
            + (proteostasis_contribution * 0.2)
            + (stem_cell_contribution * 0.15);

        weighted_age.max(self.chronological_age_years * 0.7)
    }

    pub fn calculate_aging_rate(&self) -> f64 {
        let base_rate = 1.0;

        let senescence_factor = 1.0 + (self.cellular_senescence.senescent_cell_percentage / 10.0);
        let mitochondrial_factor = 2.0 - self.mitochondrial_function.atp_production_efficiency;
        let proteostasis_factor = 2.0 - self.proteostasis.protein_folding_efficiency;
        let autophagy_factor = 2.0 - self.autophagy_efficiency;

        base_rate
            * senescence_factor
            * mitochondrial_factor
            * proteostasis_factor
            * autophagy_factor
            / 16.0
    }

    pub fn calculate_overall_hallmark_score(&self) -> f64 {
        let scores = [
            self.hallmarks.genomic_instability_score,
            self.hallmarks.telomere_attrition_rate,
            self.hallmarks.epigenetic_alterations_score,
            self.hallmarks.loss_of_proteostasis_score,
            self.hallmarks.deregulated_nutrient_sensing_score,
            self.hallmarks.mitochondrial_dysfunction_score,
            self.hallmarks.cellular_senescence_score,
            self.hallmarks.stem_cell_exhaustion_score,
            self.hallmarks.altered_intercellular_communication_score,
            self.hallmarks.disabled_macroautophagy_score,
            self.hallmarks.chronic_inflammation_score,
            self.hallmarks.dysbiosis_score,
        ];

        scores.iter().sum::<f64>() / scores.len() as f64
    }

    pub fn assess_longevity_potential(&self) -> LongevityAssessment {
        let biological_age = self.calculate_biological_age();
        let age_delta = biological_age - self.chronological_age_years;

        let hallmark_score = self.calculate_overall_hallmark_score();
        let aging_rate = self.calculate_aging_rate();

        let health_span_estimate = if age_delta < -5.0 {
            self.chronological_age_years + 30.0
        } else if age_delta < 0.0 {
            self.chronological_age_years + 25.0
        } else if age_delta < 5.0 {
            self.chronological_age_years + 20.0
        } else {
            self.chronological_age_years + 15.0
        };

        let remaining_healthy_years =
            (health_span_estimate - self.chronological_age_years).max(0.0);

        LongevityAssessment {
            biological_age,
            age_delta,
            aging_rate,
            overall_hallmark_score: hallmark_score,
            estimated_health_span_years: health_span_estimate,
            remaining_healthy_years,
            longevity_category: Self::categorize_longevity(age_delta),
        }
    }

    fn categorize_longevity(age_delta: f64) -> LongevityCategory {
        if age_delta < -10.0 {
            LongevityCategory::ExceptionalLongevity
        } else if age_delta < -5.0 {
            LongevityCategory::AboveAverage
        } else if age_delta.abs() < 5.0 {
            LongevityCategory::Average
        } else if age_delta < 10.0 {
            LongevityCategory::BelowAverage
        } else {
            LongevityCategory::AcceleratedAging
        }
    }

    pub fn apply_senolytic_intervention(&mut self, efficacy: f64) {
        self.cellular_senescence.senescent_cell_percentage *= 1.0 - efficacy;
        self.cellular_senescence.sasp_factor_level *= 1.0 - efficacy * 0.8;
        self.hallmarks.cellular_senescence_score *= 1.0 - efficacy * 0.7;
        self.hallmarks.chronic_inflammation_score *= 1.0 - efficacy * 0.5;
    }

    pub fn apply_nad_boosting_intervention(&mut self, boost_factor: f64) {
        self.mitochondrial_function.atp_production_efficiency =
            (self.mitochondrial_function.atp_production_efficiency + boost_factor * 0.1).min(1.0);
        self.mitochondrial_function.mitochondrial_biogenesis_rate =
            (self.mitochondrial_function.mitochondrial_biogenesis_rate + boost_factor * 0.15)
                .min(1.0);
        self.hallmarks.mitochondrial_dysfunction_score *= 1.0 - boost_factor * 0.3;
    }

    pub fn apply_autophagy_enhancement(&mut self, enhancement: f64) {
        self.autophagy_efficiency = (self.autophagy_efficiency + enhancement).min(1.0);
        self.proteostasis.aggregate_protein_accumulation *= 1.0 - enhancement * 0.5;
        self.hallmarks.disabled_macroautophagy_score *= 1.0 - enhancement * 0.6;
        self.hallmarks.loss_of_proteostasis_score *= 1.0 - enhancement * 0.4;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongevityAssessment {
    pub biological_age: f64,
    pub age_delta: f64,
    pub aging_rate: f64,
    pub overall_hallmark_score: f64,
    pub estimated_health_span_years: f64,
    pub remaining_healthy_years: f64,
    pub longevity_category: LongevityCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LongevityCategory {
    ExceptionalLongevity,
    AboveAverage,
    Average,
    BelowAverage,
    AcceleratedAging,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_young_adult_aging_profile() {
        let profile = AgingProfile::new_healthy_adult(25.0);
        assert_eq!(profile.chronological_age_years, 25.0);
        assert!(profile.telomere_length_kb > 8.0);
        assert!(profile.cellular_senescence.senescent_cell_percentage < 2.0);
    }

    #[test]
    fn test_middle_aged_aging_profile() {
        let profile = AgingProfile::new_healthy_adult(50.0);
        assert!(profile.telomere_length_kb < 8.0);
        assert!(profile.cellular_senescence.senescent_cell_percentage > 1.0);
    }

    #[test]
    fn test_biological_age_calculation() {
        let profile = AgingProfile::new_healthy_adult(30.0);
        let bio_age = profile.calculate_biological_age();
        assert!(bio_age >= 21.0);
        assert!(bio_age <= 60.0);
    }

    #[test]
    fn test_aging_rate() {
        let young = AgingProfile::new_healthy_adult(25.0);
        let old = AgingProfile::new_healthy_adult(70.0);

        let young_rate = young.calculate_aging_rate();
        let old_rate = old.calculate_aging_rate();

        assert!(old_rate > young_rate);
    }

    #[test]
    fn test_hallmark_score() {
        let profile = AgingProfile::new_healthy_adult(40.0);
        let score = profile.calculate_overall_hallmark_score();
        assert!(score > 0.0);
        assert!(score < 1.0);
    }

    #[test]
    fn test_longevity_assessment() {
        let profile = AgingProfile::new_healthy_adult(35.0);
        let assessment = profile.assess_longevity_potential();

        assert!(assessment.remaining_healthy_years > 0.0);
        assert!(assessment.estimated_health_span_years > 35.0);
    }

    #[test]
    fn test_senolytic_intervention() {
        let mut profile = AgingProfile::new_healthy_adult(60.0);
        let initial_senescent = profile.cellular_senescence.senescent_cell_percentage;

        profile.apply_senolytic_intervention(0.5);

        assert!(profile.cellular_senescence.senescent_cell_percentage < initial_senescent);
    }

    #[test]
    fn test_nad_boosting() {
        let mut profile = AgingProfile::new_healthy_adult(55.0);
        let initial_atp = profile.mitochondrial_function.atp_production_efficiency;

        profile.apply_nad_boosting_intervention(0.6);

        assert!(profile.mitochondrial_function.atp_production_efficiency > initial_atp);
    }

    #[test]
    fn test_autophagy_enhancement() {
        let mut profile = AgingProfile::new_healthy_adult(50.0);
        let initial_autophagy = profile.autophagy_efficiency;

        profile.apply_autophagy_enhancement(0.1);

        assert!(profile.autophagy_efficiency > initial_autophagy);
    }

    #[test]
    fn test_longevity_categories() {
        let young_healthy = AgingProfile::new_healthy_adult(25.0);
        let assessment = young_healthy.assess_longevity_potential();

        assert!(matches!(
            assessment.longevity_category,
            LongevityCategory::Average | LongevityCategory::AboveAverage
        ));
    }
}

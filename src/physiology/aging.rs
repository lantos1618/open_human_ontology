use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgingSystem {
    pub chronological_age_years: f64,
    pub biological_age: BiologicalAge,
    pub cellular_aging: CellularAging,
    pub organ_aging: OrganAging,
    pub hallmarks_of_aging: HallmarksOfAging,
    pub longevity_factors: LongevityFactors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicalAge {
    pub epigenetic_age_years: f64,
    pub telomere_age_years: f64,
    pub metabolic_age_years: f64,
    pub immune_age_years: f64,
    pub phenotypic_age_years: f64,
    pub aging_acceleration_years: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularAging {
    pub telomere_attrition: TelomereStatus,
    pub cellular_senescence: CellularSenescence,
    pub mitochondrial_dysfunction: MitochondrialDysfunction,
    pub stem_cell_exhaustion: StemCellExhaustion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelomereStatus {
    pub average_telomere_length_kb: f64,
    pub shortest_telomere_length_kb: f64,
    pub telomerase_activity: f64,
    pub telomere_shortening_rate_bp_per_year: f64,
    pub critically_short_telomeres_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularSenescence {
    pub senescent_cell_burden_percent: f64,
    pub p16_expression: f64,
    pub p21_expression: f64,
    pub sa_beta_gal_activity: f64,
    pub sasp_factors: SASPFactors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SASPFactors {
    pub il6_pg_ml: f64,
    pub il8_pg_ml: f64,
    pub il1_beta_pg_ml: f64,
    pub mmp3_ng_ml: f64,
    pub vegf_pg_ml: f64,
    pub tgf_beta_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialDysfunction {
    pub mitochondrial_count: u32,
    pub atp_production_pmol_s: f64,
    pub oxidative_capacity: f64,
    pub membrane_potential_mv: f64,
    pub ros_production: f64,
    pub mitochondrial_dna_copy_number: f64,
    pub mitochondrial_dna_deletions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StemCellExhaustion {
    pub hematopoietic_stem_cells: f64,
    pub mesenchymal_stem_cells: f64,
    pub neural_stem_cells: f64,
    pub satellite_cells: f64,
    pub stem_cell_niche_health: f64,
    pub differentiation_capacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganAging {
    pub brain_aging: BrainAging,
    pub cardiovascular_aging: CardiovascularAging,
    pub musculoskeletal_aging: MusculoskeletalAging,
    pub immune_aging: ImmuneAging,
    pub metabolic_aging: MetabolicAging,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainAging {
    pub brain_volume_ml: f64,
    pub white_matter_hyperintensities_ml: f64,
    pub hippocampal_volume_ml: f64,
    pub amyloid_beta_burden: f64,
    pub tau_burden: f64,
    pub cognitive_reserve: f64,
    pub neurogenesis_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularAging {
    pub arterial_stiffness_pwv: f64,
    pub carotid_intima_media_thickness_mm: f64,
    pub ejection_fraction_percent: f64,
    pub diastolic_dysfunction: f64,
    pub endothelial_function: f64,
    pub coronary_artery_calcification_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusculoskeletalAging {
    pub muscle_mass_kg: f64,
    pub muscle_strength_kg: f64,
    pub bone_mineral_density_g_cm2: f64,
    pub sarcopenia_score: f64,
    pub osteoporosis_risk: f64,
    pub joint_degeneration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneAging {
    pub thymic_output: f64,
    pub naive_t_cell_count: f64,
    pub cd4_cd8_ratio: f64,
    pub inflammaging_score: f64,
    pub immunosenescence_score: f64,
    pub vaccine_response: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicAging {
    pub basal_metabolic_rate: f64,
    pub insulin_sensitivity: f64,
    pub nad_plus_levels: f64,
    pub autophagy_activity: f64,
    pub metabolic_flexibility: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallmarksOfAging {
    pub genomic_instability: f64,
    pub telomere_attrition: f64,
    pub epigenetic_alterations: f64,
    pub loss_of_proteostasis: f64,
    pub deregulated_nutrient_sensing: f64,
    pub mitochondrial_dysfunction: f64,
    pub cellular_senescence: f64,
    pub stem_cell_exhaustion: f64,
    pub altered_intercellular_communication: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongevityFactors {
    pub sirtuin_activity: SirtuinActivity,
    pub mtor_activity: f64,
    pub ampk_activity: f64,
    pub foxo_activity: f64,
    pub nrf2_activity: f64,
    pub growth_hormone_igf1_axis: GHAxis,
    pub caloric_restriction_mimetics: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SirtuinActivity {
    pub sirt1: f64,
    pub sirt3: f64,
    pub sirt6: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHAxis {
    pub growth_hormone_ng_ml: f64,
    pub igf1_ng_ml: f64,
    pub igfbp3_ng_ml: f64,
    pub gh_sensitivity: f64,
}

impl AgingSystem {
    pub fn new(chronological_age: f64) -> Self {
        Self {
            chronological_age_years: chronological_age,
            biological_age: BiologicalAge::new(chronological_age),
            cellular_aging: CellularAging::new(chronological_age),
            organ_aging: OrganAging::new(chronological_age),
            hallmarks_of_aging: HallmarksOfAging::new(chronological_age),
            longevity_factors: LongevityFactors::new_optimal(),
        }
    }

    pub fn calculate_biological_age(&mut self) {
        let telomere_age = self.calculate_telomere_age();
        let epigenetic_age = self.calculate_epigenetic_age();
        let metabolic_age = self.calculate_metabolic_age();
        let immune_age = self.calculate_immune_age();

        self.biological_age.telomere_age_years = telomere_age;
        self.biological_age.epigenetic_age_years = epigenetic_age;
        self.biological_age.metabolic_age_years = metabolic_age;
        self.biological_age.immune_age_years = immune_age;

        self.biological_age.phenotypic_age_years =
            telomere_age * 0.25 + epigenetic_age * 0.35 + metabolic_age * 0.2 + immune_age * 0.2;

        self.biological_age.aging_acceleration_years =
            self.biological_age.phenotypic_age_years - self.chronological_age_years;
    }

    fn calculate_telomere_age(&self) -> f64 {
        let tl = self.cellular_aging.telomere_attrition.average_telomere_length_kb;
        let expected_tl_at_birth = 11.0;
        let loss_per_year = 0.05;

        let implied_age = (expected_tl_at_birth - tl) / loss_per_year;
        implied_age.max(0.0)
    }

    fn calculate_epigenetic_age(&self) -> f64 {
        let hallmark_score = self.hallmarks_of_aging.epigenetic_alterations;
        self.chronological_age_years + (hallmark_score * 10.0)
    }

    fn calculate_metabolic_age(&self) -> f64 {
        let bmr_decline = 1.0 - self.organ_aging.metabolic_aging.basal_metabolic_rate / 1800.0;
        let insulin_decline = 1.0 - self.organ_aging.metabolic_aging.insulin_sensitivity;

        self.chronological_age_years + ((bmr_decline + insulin_decline) * 15.0)
    }

    fn calculate_immune_age(&self) -> f64 {
        let immune_score = self.organ_aging.immune_aging.immunosenescence_score;
        self.chronological_age_years + (immune_score * 12.0)
    }

    pub fn update_aging_over_time(&mut self, years_elapsed: f64) {
        self.chronological_age_years += years_elapsed;

        self.cellular_aging.telomere_attrition.average_telomere_length_kb -=
            self.cellular_aging.telomere_attrition.telomere_shortening_rate_bp_per_year * years_elapsed / 1000.0;

        self.cellular_aging.cellular_senescence.senescent_cell_burden_percent +=
            years_elapsed * 0.5;

        self.organ_aging.brain_aging.brain_volume_ml -=
            years_elapsed * 2.0;

        self.organ_aging.musculoskeletal_aging.muscle_mass_kg -=
            years_elapsed * 0.3;

        self.calculate_biological_age();
    }

    pub fn assess_aging_rate(&self) -> AgingRate {
        let acceleration = self.biological_age.aging_acceleration_years;

        if acceleration < -5.0 {
            AgingRate::SlowAging
        } else if acceleration < 0.0 {
            AgingRate::NormalAging
        } else if acceleration < 5.0 {
            AgingRate::AcceleratedAging
        } else {
            AgingRate::RapidAging
        }
    }

    pub fn calculate_frailty_index(&self) -> f64 {
        let mut deficits: f64 = 0.0;
        let total_items: f64 = 40.0;

        if self.organ_aging.musculoskeletal_aging.sarcopenia_score > 0.5 { deficits += 1.0; }
        if self.organ_aging.brain_aging.cognitive_reserve < 0.7 { deficits += 1.0; }
        if self.organ_aging.cardiovascular_aging.ejection_fraction_percent < 55.0 { deficits += 1.0; }
        if self.cellular_aging.cellular_senescence.senescent_cell_burden_percent > 5.0 { deficits += 1.0; }
        if self.cellular_aging.telomere_attrition.average_telomere_length_kb < 7.0 { deficits += 1.0; }

        (deficits / total_items).clamp(0.0, 1.0)
    }

    pub fn longevity_potential_score(&self) -> f64 {
        let telomere_score = (self.cellular_aging.telomere_attrition.average_telomere_length_kb / 11.0).clamp(0.0, 1.0);
        let sirtuin_score = (self.longevity_factors.sirtuin_activity.sirt1 +
                            self.longevity_factors.sirtuin_activity.sirt3 +
                            self.longevity_factors.sirtuin_activity.sirt6) / 3.0;
        let ampk_score = self.longevity_factors.ampk_activity;
        let autophagy_score = self.organ_aging.metabolic_aging.autophagy_activity;

        ((telomere_score + sirtuin_score + ampk_score + autophagy_score) / 4.0 * 100.0).clamp(0.0, 100.0)
    }
}

impl BiologicalAge {
    pub fn new(chronological_age: f64) -> Self {
        Self {
            epigenetic_age_years: chronological_age,
            telomere_age_years: chronological_age,
            metabolic_age_years: chronological_age,
            immune_age_years: chronological_age,
            phenotypic_age_years: chronological_age,
            aging_acceleration_years: 0.0,
        }
    }
}

impl CellularAging {
    pub fn new(age: f64) -> Self {
        let telomere_length = 11.0 - (age * 0.05);

        Self {
            telomere_attrition: TelomereStatus {
                average_telomere_length_kb: telomere_length.max(5.0),
                shortest_telomere_length_kb: (telomere_length - 2.0).max(3.0),
                telomerase_activity: 0.1,
                telomere_shortening_rate_bp_per_year: 50.0,
                critically_short_telomeres_percent: (age * 0.1).min(15.0),
            },
            cellular_senescence: CellularSenescence {
                senescent_cell_burden_percent: (age * 0.1).min(10.0),
                p16_expression: age * 0.05,
                p21_expression: age * 0.03,
                sa_beta_gal_activity: age * 0.04,
                sasp_factors: SASPFactors {
                    il6_pg_ml: 1.0 + age * 0.1,
                    il8_pg_ml: 5.0 + age * 0.2,
                    il1_beta_pg_ml: 0.5 + age * 0.05,
                    mmp3_ng_ml: 10.0 + age * 0.5,
                    vegf_pg_ml: 100.0 + age * 2.0,
                    tgf_beta_pg_ml: 500.0 + age * 10.0,
                },
            },
            mitochondrial_dysfunction: MitochondrialDysfunction {
                mitochondrial_count: (1000 - (age * 5.0) as u32).max(500),
                atp_production_pmol_s: 100.0 - (age * 0.5),
                oxidative_capacity: 1.0 - (age * 0.01),
                membrane_potential_mv: -180.0 + (age * 0.3),
                ros_production: 1.0 + (age * 0.02),
                mitochondrial_dna_copy_number: 1000.0 - (age * 5.0),
                mitochondrial_dna_deletions: (age * 2.0) as u32,
            },
            stem_cell_exhaustion: StemCellExhaustion {
                hematopoietic_stem_cells: 1.0 - (age * 0.01),
                mesenchymal_stem_cells: 1.0 - (age * 0.015),
                neural_stem_cells: 1.0 - (age * 0.02),
                satellite_cells: 1.0 - (age * 0.015),
                stem_cell_niche_health: 1.0 - (age * 0.01),
                differentiation_capacity: 1.0 - (age * 0.012),
            },
        }
    }
}

impl OrganAging {
    pub fn new(age: f64) -> Self {
        Self {
            brain_aging: BrainAging {
                brain_volume_ml: 1400.0 - (age * 2.0),
                white_matter_hyperintensities_ml: age * 0.5,
                hippocampal_volume_ml: 3.5 - (age * 0.02),
                amyloid_beta_burden: (age - 40.0).max(0.0) * 0.05,
                tau_burden: (age - 50.0).max(0.0) * 0.03,
                cognitive_reserve: 1.0 - (age * 0.005),
                neurogenesis_rate: 1.0 - (age * 0.015),
            },
            cardiovascular_aging: CardiovascularAging {
                arterial_stiffness_pwv: 7.0 + (age * 0.1),
                carotid_intima_media_thickness_mm: 0.5 + (age * 0.01),
                ejection_fraction_percent: 65.0 - (age * 0.1),
                diastolic_dysfunction: age * 0.01,
                endothelial_function: 1.0 - (age * 0.01),
                coronary_artery_calcification_score: (age - 40.0).max(0.0) * 5.0,
            },
            musculoskeletal_aging: MusculoskeletalAging {
                muscle_mass_kg: 30.0 - ((age - 30.0).max(0.0) * 0.3),
                muscle_strength_kg: 100.0 - ((age - 30.0).max(0.0) * 1.0),
                bone_mineral_density_g_cm2: 1.2 - ((age - 30.0).max(0.0) * 0.01),
                sarcopenia_score: ((age - 60.0).max(0.0) * 0.05).min(1.0),
                osteoporosis_risk: ((age - 50.0).max(0.0) * 0.02).min(1.0),
                joint_degeneration: (age * 0.01).min(1.0),
            },
            immune_aging: ImmuneAging {
                thymic_output: 1.0 - (age * 0.015),
                naive_t_cell_count: 1.0 - (age * 0.012),
                cd4_cd8_ratio: 2.0 - (age * 0.02),
                inflammaging_score: age * 0.015,
                immunosenescence_score: age * 0.012,
                vaccine_response: 1.0 - (age * 0.01),
            },
            metabolic_aging: MetabolicAging {
                basal_metabolic_rate: 1800.0 - (age * 5.0),
                insulin_sensitivity: 1.0 - (age * 0.008),
                nad_plus_levels: 100.0 - (age * 1.2),
                autophagy_activity: 1.0 - (age * 0.01),
                metabolic_flexibility: 1.0 - (age * 0.01),
            },
        }
    }
}

impl HallmarksOfAging {
    pub fn new(age: f64) -> Self {
        Self {
            genomic_instability: (age * 0.015).min(1.0),
            telomere_attrition: (age * 0.018).min(1.0),
            epigenetic_alterations: (age * 0.016).min(1.0),
            loss_of_proteostasis: (age * 0.014).min(1.0),
            deregulated_nutrient_sensing: (age * 0.012).min(1.0),
            mitochondrial_dysfunction: (age * 0.015).min(1.0),
            cellular_senescence: (age * 0.013).min(1.0),
            stem_cell_exhaustion: (age * 0.017).min(1.0),
            altered_intercellular_communication: (age * 0.014).min(1.0),
        }
    }
}

impl LongevityFactors {
    pub fn new_optimal() -> Self {
        Self {
            sirtuin_activity: SirtuinActivity {
                sirt1: 1.0,
                sirt3: 1.0,
                sirt6: 1.0,
            },
            mtor_activity: 0.5,
            ampk_activity: 1.0,
            foxo_activity: 1.0,
            nrf2_activity: 1.0,
            growth_hormone_igf1_axis: GHAxis {
                growth_hormone_ng_ml: 2.0,
                igf1_ng_ml: 150.0,
                igfbp3_ng_ml: 3500.0,
                gh_sensitivity: 1.0,
            },
            caloric_restriction_mimetics: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgingRate {
    SlowAging,
    NormalAging,
    AcceleratedAging,
    RapidAging,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aging_system_creation() {
        let aging = AgingSystem::new(30.0);
        assert_eq!(aging.chronological_age_years, 30.0);
        assert!(aging.cellular_aging.telomere_attrition.average_telomere_length_kb > 8.0);
    }

    #[test]
    fn test_biological_age_calculation() {
        let mut aging = AgingSystem::new(50.0);
        aging.calculate_biological_age();

        assert!(aging.biological_age.phenotypic_age_years > 0.0);
    }

    #[test]
    fn test_aging_over_time() {
        let mut aging = AgingSystem::new(30.0);
        let initial_telomere = aging.cellular_aging.telomere_attrition.average_telomere_length_kb;

        aging.update_aging_over_time(10.0);

        assert_eq!(aging.chronological_age_years, 40.0);
        assert!(aging.cellular_aging.telomere_attrition.average_telomere_length_kb < initial_telomere);
    }

    #[test]
    fn test_aging_rate_assessment() {
        let aging = AgingSystem::new(30.0);
        let rate = aging.assess_aging_rate();

        assert!(matches!(rate, AgingRate::SlowAging | AgingRate::NormalAging | AgingRate::AcceleratedAging));
    }

    #[test]
    fn test_frailty_index() {
        let aging = AgingSystem::new(80.0);
        let frailty = aging.calculate_frailty_index();

        assert!(frailty >= 0.0);
        assert!(frailty <= 1.0);
    }

    #[test]
    fn test_longevity_potential() {
        let aging = AgingSystem::new(30.0);
        let potential = aging.longevity_potential_score();

        assert!(potential > 50.0);
        assert!(potential <= 100.0);
    }

    #[test]
    fn test_telomere_age_calculation() {
        let aging = AgingSystem::new(40.0);
        let telomere_age = aging.calculate_telomere_age();

        assert!(telomere_age > 0.0);
    }
}

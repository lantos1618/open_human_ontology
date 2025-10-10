use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressResponseSystem {
    pub hpa_axis: HPAAxis,
    pub sam_axis: SAMAxis,
    pub allostatic_load: AllostaticLoad,
    pub stress_biomarkers: StressBiomarkers,
    pub resilience_factors: ResilienceFactors,
    pub chronic_stress_effects: ChronicStressEffects,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HPAAxis {
    pub hypothalamus: HypothalamicResponse,
    pub pituitary: PituitaryResponse,
    pub adrenal: AdrenalResponse,
    pub cortisol_awakening_response: f64,
    pub cortisol_slope_ug_dl_per_hour: f64,
    pub negative_feedback_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypothalamicResponse {
    pub pvn_activation: f64,
    pub crh_release_pg_ml: f64,
    pub avp_release_pg_ml: f64,
    pub crh_neurons_active_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PituitaryResponse {
    pub acth_release_pg_ml: f64,
    pub crh_receptor_density: f64,
    pub corticotroph_count: u32,
    pub acth_pulsatility_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdrenalResponse {
    pub cortisol_ug_dl: f64,
    pub dhea_ug_dl: f64,
    pub aldosterone_ng_dl: f64,
    pub cortisol_to_dhea_ratio: f64,
    pub adrenal_capacity_percent: f64,
    pub zona_fasciculata_health: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAMAxis {
    pub sympathetic_activation: f64,
    pub epinephrine_pg_ml: f64,
    pub norepinephrine_pg_ml: f64,
    pub heart_rate_variability_ms: f64,
    pub blood_pressure_response_mmhg: f64,
    pub vagal_tone: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllostaticLoad {
    pub cumulative_stress_score: f64,
    pub primary_mediators: PrimaryMediators,
    pub secondary_outcomes: SecondaryOutcomes,
    pub tertiary_outcomes: TertiaryOutcomes,
    pub total_allostatic_load_index: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryMediators {
    pub cortisol_dysregulation: f64,
    pub catecholamine_dysregulation: f64,
    pub dhea_s_deficiency: f64,
    pub inflammatory_burden: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryOutcomes {
    pub metabolic_dysregulation: MetabolicDysregulation,
    pub cardiovascular_strain: CardiovascularStrain,
    pub immune_dysregulation: ImmuneDysregulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicDysregulation {
    pub insulin_resistance_homa_ir: f64,
    pub waist_hip_ratio: f64,
    pub triglycerides_mg_dl: f64,
    pub hdl_mg_dl: f64,
    pub fasting_glucose_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularStrain {
    pub systolic_bp_mmhg: f64,
    pub diastolic_bp_mmhg: f64,
    pub resting_heart_rate_bpm: f64,
    pub arterial_stiffness_pwv: f64,
    pub endothelial_dysfunction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneDysregulation {
    pub il6_pg_ml: f64,
    pub tnf_alpha_pg_ml: f64,
    pub crp_mg_l: f64,
    pub nk_cell_activity: f64,
    pub antibody_response: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TertiaryOutcomes {
    pub cardiovascular_disease_risk: f64,
    pub metabolic_syndrome_risk: f64,
    pub cognitive_decline_risk: f64,
    pub depression_risk: f64,
    pub overall_morbidity_risk: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressBiomarkers {
    pub salivary_cortisol_awakening: f64,
    pub salivary_cortisol_evening: f64,
    pub cortisol_slope: f64,
    pub dhea_s_ug_dl: f64,
    pub cortisol_dhea_ratio: f64,
    pub hair_cortisol_pg_mg: f64,
    pub alpha_amylase_u_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceFactors {
    pub coping_strategies_score: f64,
    pub social_support_score: f64,
    pub exercise_frequency_per_week: f64,
    pub sleep_quality_score: f64,
    pub mindfulness_practice_score: f64,
    pub perceived_control_score: f64,
    pub optimism_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicStressEffects {
    pub hippocampal_volume_ml: f64,
    pub prefrontal_cortex_volume_ml: f64,
    pub amygdala_reactivity: f64,
    pub telomere_length_kb: f64,
    pub neuroplasticity_score: f64,
    pub immune_aging_score: f64,
}

impl StressResponseSystem {
    pub fn new_normal() -> Self {
        Self {
            hpa_axis: HPAAxis::new_normal(),
            sam_axis: SAMAxis::new_normal(),
            allostatic_load: AllostaticLoad::new_low(),
            stress_biomarkers: StressBiomarkers::new_normal(),
            resilience_factors: ResilienceFactors::new_high(),
            chronic_stress_effects: ChronicStressEffects::new_normal(),
        }
    }

    pub fn activate_acute_stress(&mut self, stressor_intensity: f64) {
        let intensity = stressor_intensity.clamp(0.0, 10.0);

        self.hpa_axis.hypothalamus.pvn_activation = intensity / 10.0;
        self.hpa_axis.hypothalamus.crh_release_pg_ml = 50.0 + (intensity * 20.0);
        self.hpa_axis.pituitary.acth_release_pg_ml = 30.0 + (intensity * 40.0);
        self.hpa_axis.adrenal.cortisol_ug_dl = 10.0 + (intensity * 5.0);

        self.sam_axis.sympathetic_activation = intensity / 10.0;
        self.sam_axis.epinephrine_pg_ml = 50.0 + (intensity * 100.0);
        self.sam_axis.norepinephrine_pg_ml = 150.0 + (intensity * 200.0);
        self.sam_axis.heart_rate_variability_ms = 60.0 - (intensity * 3.0);
    }

    pub fn chronic_stress_exposure(&mut self, duration_months: f64, intensity: f64) {
        self.allostatic_load.cumulative_stress_score += duration_months * intensity;

        let load_factor = (duration_months * intensity / 100.0).min(1.0);

        self.hpa_axis.negative_feedback_sensitivity *= 1.0 - (load_factor * 0.3);
        self.hpa_axis.adrenal.adrenal_capacity_percent *= 1.0 - (load_factor * 0.2);

        self.chronic_stress_effects.hippocampal_volume_ml -= load_factor * 0.5;
        self.chronic_stress_effects.telomere_length_kb -= load_factor * 2.0;

        self.allostatic_load
            .secondary_outcomes
            .immune_dysregulation
            .il6_pg_ml += load_factor * 5.0;
        self.allostatic_load
            .secondary_outcomes
            .immune_dysregulation
            .crp_mg_l += load_factor * 3.0;

        self.update_allostatic_load_index();
    }

    pub fn update_allostatic_load_index(&mut self) {
        let primary_score = (self
            .allostatic_load
            .primary_mediators
            .cortisol_dysregulation
            + self
                .allostatic_load
                .primary_mediators
                .catecholamine_dysregulation
            + self.allostatic_load.primary_mediators.inflammatory_burden)
            / 3.0;

        let secondary_score = self.calculate_secondary_score();
        let tertiary_score = self.calculate_tertiary_score();

        self.allostatic_load.total_allostatic_load_index =
            (primary_score * 0.4 + secondary_score * 0.4 + tertiary_score * 0.2).clamp(0.0, 10.0);
    }

    fn calculate_secondary_score(&self) -> f64 {
        let metabolic = self
            .allostatic_load
            .secondary_outcomes
            .metabolic_dysregulation
            .metabolic_syndrome_score();
        let cv = self
            .allostatic_load
            .secondary_outcomes
            .cardiovascular_strain
            .cardiovascular_risk_score();
        let immune = self
            .allostatic_load
            .secondary_outcomes
            .immune_dysregulation
            .immune_dysfunction_score();

        (metabolic + cv + immune) / 3.0
    }

    fn calculate_tertiary_score(&self) -> f64 {
        let outcomes = &self.allostatic_load.tertiary_outcomes;
        (outcomes.cardiovascular_disease_risk
            + outcomes.metabolic_syndrome_risk
            + outcomes.cognitive_decline_risk
            + outcomes.depression_risk
            + outcomes.overall_morbidity_risk)
            / 5.0
    }

    pub fn assess_stress_classification(&self) -> StressClassification {
        let al_index = self.allostatic_load.total_allostatic_load_index;

        if al_index < 2.0 {
            StressClassification::LowStress
        } else if al_index < 4.0 {
            StressClassification::ModerateStress
        } else if al_index < 6.0 {
            StressClassification::HighStress
        } else {
            StressClassification::SevereStress
        }
    }

    pub fn calculate_resilience_score(&self) -> f64 {
        let factors = &self.resilience_factors;

        (factors.coping_strategies_score * 0.2
            + factors.social_support_score * 0.2
            + factors.exercise_frequency_per_week * 2.0
            + factors.sleep_quality_score * 0.15
            + factors.mindfulness_practice_score * 0.15
            + factors.perceived_control_score * 0.15
            + factors.optimism_score * 0.15)
            .clamp(0.0, 100.0)
    }

    pub fn hpa_axis_dysregulation(&self) -> HPADysregulation {
        let car = self.hpa_axis.cortisol_awakening_response;
        let slope = self.hpa_axis.cortisol_slope_ug_dl_per_hour;
        let feedback = self.hpa_axis.negative_feedback_sensitivity;

        if car < 2.5 && slope > -0.5 {
            HPADysregulation::Hypocortisolism
        } else if car > 9.0 || slope < -3.0 {
            HPADysregulation::Hypercortisolism
        } else if feedback < 0.5 {
            HPADysregulation::FeedbackDysfunction
        } else {
            HPADysregulation::Normal
        }
    }
}

impl HPAAxis {
    pub fn new_normal() -> Self {
        Self {
            hypothalamus: HypothalamicResponse {
                pvn_activation: 0.3,
                crh_release_pg_ml: 50.0,
                avp_release_pg_ml: 2.0,
                crh_neurons_active_percent: 20.0,
            },
            pituitary: PituitaryResponse {
                acth_release_pg_ml: 30.0,
                crh_receptor_density: 1.0,
                corticotroph_count: 100000,
                acth_pulsatility_score: 1.0,
            },
            adrenal: AdrenalResponse {
                cortisol_ug_dl: 12.0,
                dhea_ug_dl: 200.0,
                aldosterone_ng_dl: 8.0,
                cortisol_to_dhea_ratio: 0.06,
                adrenal_capacity_percent: 100.0,
                zona_fasciculata_health: 1.0,
            },
            cortisol_awakening_response: 5.5,
            cortisol_slope_ug_dl_per_hour: -1.5,
            negative_feedback_sensitivity: 1.0,
        }
    }
}

impl SAMAxis {
    pub fn new_normal() -> Self {
        Self {
            sympathetic_activation: 0.3,
            epinephrine_pg_ml: 50.0,
            norepinephrine_pg_ml: 200.0,
            heart_rate_variability_ms: 60.0,
            blood_pressure_response_mmhg: 10.0,
            vagal_tone: 0.7,
        }
    }
}

impl AllostaticLoad {
    pub fn new_low() -> Self {
        Self {
            cumulative_stress_score: 0.0,
            primary_mediators: PrimaryMediators {
                cortisol_dysregulation: 0.0,
                catecholamine_dysregulation: 0.0,
                dhea_s_deficiency: 0.0,
                inflammatory_burden: 0.0,
            },
            secondary_outcomes: SecondaryOutcomes {
                metabolic_dysregulation: MetabolicDysregulation::new_normal(),
                cardiovascular_strain: CardiovascularStrain::new_normal(),
                immune_dysregulation: ImmuneDysregulation::new_normal(),
            },
            tertiary_outcomes: TertiaryOutcomes {
                cardiovascular_disease_risk: 0.05,
                metabolic_syndrome_risk: 0.05,
                cognitive_decline_risk: 0.05,
                depression_risk: 0.05,
                overall_morbidity_risk: 0.05,
            },
            total_allostatic_load_index: 0.5,
        }
    }
}

impl MetabolicDysregulation {
    pub fn new_normal() -> Self {
        Self {
            insulin_resistance_homa_ir: 1.0,
            waist_hip_ratio: 0.85,
            triglycerides_mg_dl: 100.0,
            hdl_mg_dl: 60.0,
            fasting_glucose_mg_dl: 90.0,
        }
    }

    pub fn metabolic_syndrome_score(&self) -> f64 {
        let mut score = 0.0;

        if self.waist_hip_ratio > 0.90 {
            score += 2.0;
        }
        if self.triglycerides_mg_dl > 150.0 {
            score += 2.0;
        }
        if self.hdl_mg_dl < 40.0 {
            score += 2.0;
        }
        if self.fasting_glucose_mg_dl > 100.0 {
            score += 2.0;
        }
        if self.insulin_resistance_homa_ir > 2.5 {
            score += 2.0;
        }

        score
    }
}

impl CardiovascularStrain {
    pub fn new_normal() -> Self {
        Self {
            systolic_bp_mmhg: 120.0,
            diastolic_bp_mmhg: 80.0,
            resting_heart_rate_bpm: 70.0,
            arterial_stiffness_pwv: 7.0,
            endothelial_dysfunction: 0.0,
        }
    }

    pub fn cardiovascular_risk_score(&self) -> f64 {
        let mut score = 0.0;

        if self.systolic_bp_mmhg > 130.0 {
            score += 2.0;
        }
        if self.diastolic_bp_mmhg > 85.0 {
            score += 1.0;
        }
        if self.resting_heart_rate_bpm > 80.0 {
            score += 1.0;
        }
        if self.arterial_stiffness_pwv > 10.0 {
            score += 2.0;
        }
        if self.endothelial_dysfunction > 0.5 {
            score += 2.0;
        }

        score
    }
}

impl ImmuneDysregulation {
    pub fn new_normal() -> Self {
        Self {
            il6_pg_ml: 1.0,
            tnf_alpha_pg_ml: 5.0,
            crp_mg_l: 1.0,
            nk_cell_activity: 1.0,
            antibody_response: 1.0,
        }
    }

    pub fn immune_dysfunction_score(&self) -> f64 {
        let mut score = 0.0;

        if self.il6_pg_ml > 3.0 {
            score += 2.0;
        }
        if self.tnf_alpha_pg_ml > 12.0 {
            score += 2.0;
        }
        if self.crp_mg_l > 3.0 {
            score += 2.0;
        }
        if self.nk_cell_activity < 0.7 {
            score += 1.0;
        }
        if self.antibody_response < 0.7 {
            score += 1.0;
        }

        score
    }
}

impl StressBiomarkers {
    pub fn new_normal() -> Self {
        Self {
            salivary_cortisol_awakening: 16.0,
            salivary_cortisol_evening: 3.0,
            cortisol_slope: -1.5,
            dhea_s_ug_dl: 200.0,
            cortisol_dhea_ratio: 0.06,
            hair_cortisol_pg_mg: 15.0,
            alpha_amylase_u_ml: 50.0,
        }
    }
}

impl ResilienceFactors {
    pub fn new_high() -> Self {
        Self {
            coping_strategies_score: 80.0,
            social_support_score: 80.0,
            exercise_frequency_per_week: 4.0,
            sleep_quality_score: 85.0,
            mindfulness_practice_score: 70.0,
            perceived_control_score: 75.0,
            optimism_score: 75.0,
        }
    }
}

impl ChronicStressEffects {
    pub fn new_normal() -> Self {
        Self {
            hippocampal_volume_ml: 3.5,
            prefrontal_cortex_volume_ml: 450.0,
            amygdala_reactivity: 1.0,
            telomere_length_kb: 10.0,
            neuroplasticity_score: 1.0,
            immune_aging_score: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StressClassification {
    LowStress,
    ModerateStress,
    HighStress,
    SevereStress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HPADysregulation {
    Normal,
    Hypocortisolism,
    Hypercortisolism,
    FeedbackDysfunction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stress_response_system_creation() {
        let system = StressResponseSystem::new_normal();
        assert_eq!(
            system.assess_stress_classification(),
            StressClassification::LowStress
        );
    }

    #[test]
    fn test_acute_stress_activation() {
        let mut system = StressResponseSystem::new_normal();
        let baseline_cortisol = system.hpa_axis.adrenal.cortisol_ug_dl;

        system.activate_acute_stress(7.0);

        assert!(system.hpa_axis.adrenal.cortisol_ug_dl > baseline_cortisol);
        assert!(system.sam_axis.epinephrine_pg_ml > 100.0);
    }

    #[test]
    fn test_chronic_stress_exposure() {
        let mut system = StressResponseSystem::new_normal();
        let baseline_telomere = system.chronic_stress_effects.telomere_length_kb;

        system.chronic_stress_exposure(12.0, 7.0);

        assert!(system.chronic_stress_effects.telomere_length_kb < baseline_telomere);
        assert!(system.allostatic_load.total_allostatic_load_index > 0.5);
    }

    #[test]
    fn test_resilience_score_calculation() {
        let system = StressResponseSystem::new_normal();
        let resilience = system.calculate_resilience_score();

        assert!(resilience > 50.0);
        assert!(resilience <= 100.0);
    }

    #[test]
    fn test_hpa_axis_dysregulation() {
        let system = StressResponseSystem::new_normal();
        assert_eq!(system.hpa_axis_dysregulation(), HPADysregulation::Normal);
    }

    #[test]
    fn test_allostatic_load_calculation() {
        let mut system = StressResponseSystem::new_normal();
        system.update_allostatic_load_index();

        assert!(system.allostatic_load.total_allostatic_load_index >= 0.0);
        assert!(system.allostatic_load.total_allostatic_load_index <= 10.0);
    }

    #[test]
    fn test_metabolic_syndrome_scoring() {
        let metabolic = MetabolicDysregulation {
            insulin_resistance_homa_ir: 3.0,
            waist_hip_ratio: 0.95,
            triglycerides_mg_dl: 180.0,
            hdl_mg_dl: 35.0,
            fasting_glucose_mg_dl: 110.0,
        };

        let score = metabolic.metabolic_syndrome_score();
        assert!(score >= 8.0);
    }

    #[test]
    fn test_cardiovascular_risk_scoring() {
        let cv_strain = CardiovascularStrain {
            systolic_bp_mmhg: 140.0,
            diastolic_bp_mmhg: 90.0,
            resting_heart_rate_bpm: 85.0,
            arterial_stiffness_pwv: 12.0,
            endothelial_dysfunction: 0.6,
        };

        let score = cv_strain.cardiovascular_risk_score();
        assert!(score >= 6.0);
    }

    #[test]
    fn test_immune_dysfunction_scoring() {
        let immune = ImmuneDysregulation {
            il6_pg_ml: 5.0,
            tnf_alpha_pg_ml: 15.0,
            crp_mg_l: 5.0,
            nk_cell_activity: 0.5,
            antibody_response: 0.6,
        };

        let score = immune.immune_dysfunction_score();
        assert!(score >= 6.0);
    }
}

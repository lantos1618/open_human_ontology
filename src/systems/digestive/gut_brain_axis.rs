use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GutBrainAxis {
    pub vagal_tone: f64,
    pub neurotransmitter_production: NeurotransmitterProduction,
    pub hpa_axis_activity: HPAAxisActivity,
    pub immune_signaling: ImmuneSignaling,
    pub gut_permeability: f64,
    pub stress_response: StressResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurotransmitterProduction {
    pub serotonin_ng_ml: f64,
    pub gaba_nmol_l: f64,
    pub dopamine_pg_ml: f64,
    pub norepinephrine_pg_ml: f64,
    pub tryptophan_umol_l: f64,
    pub bacterial_contribution_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HPAAxisActivity {
    pub cortisol_ug_dl: f64,
    pub acth_pg_ml: f64,
    pub crh_pg_ml: f64,
    pub feedback_sensitivity: f64,
    pub diurnal_rhythm_integrity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneSignaling {
    pub cytokines: HashMap<String, f64>,
    pub inflammatory_markers: InflammatoryMarkers,
    pub blood_brain_barrier_integrity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflammatoryMarkers {
    pub il6_pg_ml: f64,
    pub il1beta_pg_ml: f64,
    pub tnf_alpha_pg_ml: f64,
    pub crp_mg_l: f64,
    pub lipopolysaccharide_eu_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressResponse {
    pub perceived_stress_score: f64,
    pub gut_motility_change: f64,
    pub secretion_change: f64,
    pub microbiome_shift_rate: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GutBrainCommunicationPathway {
    VagusNerve,
    EntericNervousSystem,
    MicrobialMetabolites,
    ImmuneMediated,
    EndocrineSignaling,
    NeurotransmitterProduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GutBrainDisorder {
    pub disorder_type: GutBrainDisorderType,
    pub severity: f64,
    pub primary_pathway_affected: GutBrainCommunicationPathway,
    pub symptoms: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GutBrainDisorderType {
    IrritableBowelSyndrome,
    InflammatoryBowelDisease,
    FunctionalDyspepsia,
    Depression,
    Anxiety,
    AutismSpectrumDisorder,
    Parkinsons,
}

impl GutBrainAxis {
    pub fn new_healthy() -> Self {
        let mut cytokines = HashMap::new();
        cytokines.insert("IL-6".to_string(), 1.5);
        cytokines.insert("IL-1β".to_string(), 0.5);
        cytokines.insert("TNF-α".to_string(), 2.0);
        cytokines.insert("IL-10".to_string(), 5.0);

        Self {
            vagal_tone: 70.0,
            neurotransmitter_production: NeurotransmitterProduction {
                serotonin_ng_ml: 150.0,
                gaba_nmol_l: 80.0,
                dopamine_pg_ml: 50.0,
                norepinephrine_pg_ml: 200.0,
                tryptophan_umol_l: 60.0,
                bacterial_contribution_percent: 90.0,
            },
            hpa_axis_activity: HPAAxisActivity {
                cortisol_ug_dl: 12.0,
                acth_pg_ml: 25.0,
                crh_pg_ml: 10.0,
                feedback_sensitivity: 0.8,
                diurnal_rhythm_integrity: 0.9,
            },
            immune_signaling: ImmuneSignaling {
                cytokines,
                inflammatory_markers: InflammatoryMarkers {
                    il6_pg_ml: 1.5,
                    il1beta_pg_ml: 0.5,
                    tnf_alpha_pg_ml: 2.0,
                    crp_mg_l: 0.8,
                    lipopolysaccharide_eu_ml: 0.05,
                },
                blood_brain_barrier_integrity: 0.95,
            },
            gut_permeability: 0.2,
            stress_response: StressResponse {
                perceived_stress_score: 3.0,
                gut_motility_change: 0.0,
                secretion_change: 0.0,
                microbiome_shift_rate: 0.05,
            },
        }
    }

    pub fn assess_axis_health(&self) -> f64 {
        let vagal_score = (self.vagal_tone / 100.0).min(1.0);
        let permeability_score = 1.0 - self.gut_permeability;
        let inflammation_score = self.immune_signaling.assess_inflammation();
        let hpa_score = self.hpa_axis_activity.assess_function();
        let stress_score = 1.0 - (self.stress_response.perceived_stress_score / 10.0);

        vagal_score * 0.25
            + permeability_score * 0.25
            + inflammation_score * 0.2
            + hpa_score * 0.2
            + stress_score * 0.1
    }

    pub fn serotonin_gut_percentage(&self) -> f64 {
        95.0
    }

    pub fn apply_stress(&mut self, stress_level: f64) {
        self.stress_response.perceived_stress_score =
            (self.stress_response.perceived_stress_score + stress_level).min(10.0);
        self.hpa_axis_activity.cortisol_ug_dl += stress_level * 2.0;
        self.gut_motility_change(stress_level);
        self.gut_permeability += stress_level * 0.05;
        self.vagal_tone -= stress_level * 2.0;
    }

    fn gut_motility_change(&mut self, stress_level: f64) {
        self.stress_response.gut_motility_change = if stress_level > 5.0 {
            -0.3
        } else if stress_level > 3.0 {
            0.2
        } else {
            0.0
        };
    }

    pub fn probiotic_intervention(&mut self, duration_days: u32) {
        let improvement_factor = (duration_days as f64 / 30.0).min(1.0);

        self.neurotransmitter_production.serotonin_ng_ml += 20.0 * improvement_factor;
        self.neurotransmitter_production.gaba_nmol_l += 15.0 * improvement_factor;
        self.gut_permeability = (self.gut_permeability - 0.1 * improvement_factor).max(0.1);
        self.immune_signaling
            .inflammatory_markers
            .reduce_inflammation(improvement_factor);
        self.vagal_tone = (self.vagal_tone + 10.0 * improvement_factor).min(100.0);
    }

    pub fn vagus_nerve_stimulation(&mut self) {
        self.vagal_tone = (self.vagal_tone + 15.0).min(100.0);
        self.hpa_axis_activity.cortisol_ug_dl *= 0.85;
        self.immune_signaling.inflammatory_markers.il6_pg_ml *= 0.8;
    }
}

impl NeurotransmitterProduction {
    pub fn assess_balance(&self) -> f64 {
        let serotonin_normal = self.serotonin_ng_ml > 100.0 && self.serotonin_ng_ml < 250.0;
        let gaba_normal = self.gaba_nmol_l > 50.0 && self.gaba_nmol_l < 120.0;
        let dopamine_normal = self.dopamine_pg_ml > 30.0 && self.dopamine_pg_ml < 100.0;

        let score =
            (serotonin_normal as u32 + gaba_normal as u32 + dopamine_normal as u32) as f64 / 3.0;
        score
    }
}

impl HPAAxisActivity {
    pub fn assess_function(&self) -> f64 {
        let cortisol_normal = self.cortisol_ug_dl > 5.0 && self.cortisol_ug_dl < 25.0;
        let rhythm_intact = self.diurnal_rhythm_integrity > 0.7;
        let feedback_working = self.feedback_sensitivity > 0.6;

        let mut score = 0.0;
        if cortisol_normal {
            score += 0.4;
        }
        if rhythm_intact {
            score += 0.3;
        }
        if feedback_working {
            score += 0.3;
        }
        score
    }

    pub fn is_dysregulated(&self) -> bool {
        self.cortisol_ug_dl > 30.0
            || self.feedback_sensitivity < 0.5
            || self.diurnal_rhythm_integrity < 0.6
    }
}

impl ImmuneSignaling {
    pub fn assess_inflammation(&self) -> f64 {
        let il6_elevated = self.inflammatory_markers.il6_pg_ml > 5.0;
        let crp_elevated = self.inflammatory_markers.crp_mg_l > 3.0;
        let lps_elevated = self.inflammatory_markers.lipopolysaccharide_eu_ml > 0.2;

        let inflammation_count = il6_elevated as u32 + crp_elevated as u32 + lps_elevated as u32;

        1.0 - (inflammation_count as f64 * 0.25)
    }
}

impl InflammatoryMarkers {
    pub fn reduce_inflammation(&mut self, factor: f64) {
        self.il6_pg_ml *= 1.0 - (0.3 * factor);
        self.il1beta_pg_ml *= 1.0 - (0.3 * factor);
        self.tnf_alpha_pg_ml *= 1.0 - (0.3 * factor);
        self.crp_mg_l *= 1.0 - (0.4 * factor);
        self.lipopolysaccharide_eu_ml *= 1.0 - (0.5 * factor);
    }
}

impl GutBrainDisorder {
    pub fn ibs() -> Self {
        Self {
            disorder_type: GutBrainDisorderType::IrritableBowelSyndrome,
            severity: 0.6,
            primary_pathway_affected: GutBrainCommunicationPathway::VagusNerve,
            symptoms: vec![
                "Abdominal pain".to_string(),
                "Bloating".to_string(),
                "Altered bowel habits".to_string(),
                "Anxiety".to_string(),
            ],
        }
    }

    pub fn depression_with_gi() -> Self {
        Self {
            disorder_type: GutBrainDisorderType::Depression,
            severity: 0.7,
            primary_pathway_affected: GutBrainCommunicationPathway::MicrobialMetabolites,
            symptoms: vec![
                "Low mood".to_string(),
                "Anhedonia".to_string(),
                "Constipation".to_string(),
                "Appetite changes".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gut_brain_axis_creation() {
        let gba = GutBrainAxis::new_healthy();
        assert!(gba.vagal_tone > 0.0);
        assert!(gba.gut_permeability < 0.5);
        assert!(gba.neurotransmitter_production.serotonin_ng_ml > 0.0);
    }

    #[test]
    fn test_axis_health_assessment() {
        let gba = GutBrainAxis::new_healthy();
        let health = gba.assess_axis_health();
        assert!(health > 0.7);
        assert!(health <= 1.0);
    }

    #[test]
    fn test_stress_application() {
        let mut gba = GutBrainAxis::new_healthy();
        let initial_cortisol = gba.hpa_axis_activity.cortisol_ug_dl;
        gba.apply_stress(3.0);
        assert!(gba.hpa_axis_activity.cortisol_ug_dl > initial_cortisol);
        assert!(gba.vagal_tone < 70.0);
    }

    #[test]
    fn test_probiotic_intervention() {
        let mut gba = GutBrainAxis::new_healthy();
        let initial_serotonin = gba.neurotransmitter_production.serotonin_ng_ml;
        gba.probiotic_intervention(30);
        assert!(gba.neurotransmitter_production.serotonin_ng_ml > initial_serotonin);
    }

    #[test]
    fn test_vagus_nerve_stimulation() {
        let mut gba = GutBrainAxis::new_healthy();
        gba.apply_stress(5.0);
        let stressed_cortisol = gba.hpa_axis_activity.cortisol_ug_dl;
        gba.vagus_nerve_stimulation();
        assert!(gba.hpa_axis_activity.cortisol_ug_dl < stressed_cortisol);
        assert!(gba.vagal_tone > 70.0);
    }

    #[test]
    fn test_neurotransmitter_balance() {
        let gba = GutBrainAxis::new_healthy();
        let balance = gba.neurotransmitter_production.assess_balance();
        assert!(balance > 0.5);
    }

    #[test]
    fn test_hpa_axis_function() {
        let gba = GutBrainAxis::new_healthy();
        assert!(!gba.hpa_axis_activity.is_dysregulated());
        let function = gba.hpa_axis_activity.assess_function();
        assert!(function > 0.8);
    }

    #[test]
    fn test_serotonin_gut_percentage() {
        let gba = GutBrainAxis::new_healthy();
        assert_eq!(gba.serotonin_gut_percentage(), 95.0);
    }

    #[test]
    fn test_inflammation_assessment() {
        let gba = GutBrainAxis::new_healthy();
        let inflammation = gba.immune_signaling.assess_inflammation();
        assert!(inflammation > 0.7);
    }

    #[test]
    fn test_ibs_disorder() {
        let ibs = GutBrainDisorder::ibs();
        assert_eq!(
            ibs.disorder_type,
            GutBrainDisorderType::IrritableBowelSyndrome
        );
        assert!(!ibs.symptoms.is_empty());
    }

    #[test]
    fn test_depression_disorder() {
        let depression = GutBrainDisorder::depression_with_gi();
        assert_eq!(depression.disorder_type, GutBrainDisorderType::Depression);
        assert!(depression.severity > 0.0);
    }
}

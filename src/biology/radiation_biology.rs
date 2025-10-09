use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RadiationType {
    Photon,
    Electron,
    Proton,
    NeutronBeam,
    CarbonIon,
    AlphaParticle,
    BetaParticle,
    Gamma,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationDose {
    pub dose_gy: f64,
    pub fractions: u32,
    pub dose_per_fraction_gy: f64,
    pub radiation_type: RadiationType,
}

impl RadiationDose {
    pub fn new(total_dose_gy: f64, fractions: u32, radiation_type: RadiationType) -> Self {
        Self {
            dose_gy: total_dose_gy,
            fractions,
            dose_per_fraction_gy: total_dose_gy / fractions as f64,
            radiation_type,
        }
    }

    pub fn biological_effective_dose(&self, alpha_beta_ratio: f64) -> f64 {
        let d = self.dose_per_fraction_gy;
        self.dose_gy * (1.0 + d / alpha_beta_ratio)
    }

    pub fn equivalent_dose_in_2gy_fractions(&self, alpha_beta_ratio: f64) -> f64 {
        let bed = self.biological_effective_dose(alpha_beta_ratio);
        bed / (1.0 + 2.0 / alpha_beta_ratio)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TissueRadiosensitivity {
    pub tissue_name: String,
    pub alpha_beta_ratio: f64,
    pub tolerance_dose_gy: f64,
    pub repair_capacity: f64,
}

impl TissueRadiosensitivity {
    pub fn new_early_responding(tissue_name: String) -> Self {
        Self {
            tissue_name,
            alpha_beta_ratio: 10.0,
            tolerance_dose_gy: 60.0,
            repair_capacity: 0.8,
        }
    }

    pub fn new_late_responding(tissue_name: String) -> Self {
        Self {
            tissue_name,
            alpha_beta_ratio: 3.0,
            tolerance_dose_gy: 45.0,
            repair_capacity: 0.4,
        }
    }

    pub fn complication_probability(&self, dose: &RadiationDose) -> f64 {
        let bed = dose.biological_effective_dose(self.alpha_beta_ratio);
        let tolerance_bed = self.tolerance_dose_gy * (1.0 + 2.0 / self.alpha_beta_ratio);

        if bed <= tolerance_bed {
            0.0
        } else {
            let excess_dose = (bed - tolerance_bed) / tolerance_bed;
            1.0 - (-excess_dose * 5.0).exp()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNADamage {
    pub single_strand_breaks: u32,
    pub double_strand_breaks: u32,
    pub base_damages: u32,
    pub cross_links: u32,
}

impl DNADamage {
    pub fn from_dose(dose_gy: f64) -> Self {
        Self {
            single_strand_breaks: (dose_gy * 1000.0) as u32,
            double_strand_breaks: (dose_gy * 40.0) as u32,
            base_damages: (dose_gy * 3000.0) as u32,
            cross_links: (dose_gy * 150.0) as u32,
        }
    }

    pub fn total_damage(&self) -> u32 {
        self.single_strand_breaks + self.double_strand_breaks + self.base_damages + self.cross_links
    }

    pub fn lethal_damage(&self) -> u32 {
        self.double_strand_breaks + self.cross_links / 10
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellSurvival {
    pub cell_type: String,
    pub surviving_fraction: f64,
    pub d0: f64,
    pub dq: f64,
    pub alpha: f64,
    pub beta: f64,
}

impl CellSurvival {
    pub fn calculate_survival_lq(&self, dose_gy: f64) -> f64 {
        (-self.alpha * dose_gy - self.beta * dose_gy * dose_gy).exp()
    }

    pub fn calculate_survival_multitarget(&self, dose_gy: f64) -> f64 {
        1.0 - (1.0 - (-dose_gy / self.d0).exp()).powf(self.dq)
    }

    pub fn d10_dose(&self) -> f64 {
        let target = 0.1_f64.ln();
        (-self.alpha + (self.alpha * self.alpha - 4.0 * self.beta * target).sqrt()) / (2.0 * self.beta)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellCyclePhase {
    G1,
    S,
    G2,
    M,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiosensitivityByPhase {
    pub phase: CellCyclePhase,
    pub relative_sensitivity: f64,
}

impl CellCyclePhase {
    pub fn radiosensitivity(&self) -> f64 {
        match self {
            CellCyclePhase::M => 1.0,
            CellCyclePhase::G2 => 0.9,
            CellCyclePhase::G1 => 0.7,
            CellCyclePhase::S => 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiotherapyPlan {
    pub target_volume: String,
    pub prescription_dose: RadiationDose,
    pub technique: TreatmentTechnique,
    pub organs_at_risk: HashMap<String, DoseConstraint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TreatmentTechnique {
    ThreeDimensionalConformal,
    IMRT,
    VMAT,
    Stereotactic,
    Brachytherapy,
    ProtonTherapy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseConstraint {
    pub organ: String,
    pub max_dose_gy: Option<f64>,
    pub mean_dose_gy: Option<f64>,
    pub volume_dose_constraint: Option<VolumeDose>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeDose {
    pub volume_percent: f64,
    pub dose_gy: f64,
}

impl RadiotherapyPlan {
    pub fn new(target: String, total_dose_gy: f64, fractions: u32) -> Self {
        Self {
            target_volume: target,
            prescription_dose: RadiationDose::new(total_dose_gy, fractions, RadiationType::Photon),
            technique: TreatmentTechnique::IMRT,
            organs_at_risk: HashMap::new(),
        }
    }

    pub fn add_constraint(&mut self, organ: String, constraint: DoseConstraint) {
        self.organs_at_risk.insert(organ, constraint);
    }

    pub fn estimated_treatment_duration_weeks(&self) -> f64 {
        self.prescription_dose.fractions as f64 / 5.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationToxicity {
    pub organ: String,
    pub toxicity_grade: ToxicityGrade,
    pub acute: bool,
    pub symptoms: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToxicityGrade {
    Grade0,
    Grade1,
    Grade2,
    Grade3,
    Grade4,
    Grade5,
}

impl ToxicityGrade {
    pub fn requires_intervention(&self) -> bool {
        matches!(self, ToxicityGrade::Grade3 | ToxicityGrade::Grade4 | ToxicityGrade::Grade5)
    }

    pub fn is_life_threatening(&self) -> bool {
        matches!(self, ToxicityGrade::Grade4 | ToxicityGrade::Grade5)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TumorControlProbability {
    pub dose_gy: f64,
    pub tcp: f64,
    pub d50: f64,
    pub gamma_50: f64,
}

impl TumorControlProbability {
    pub fn calculate(dose_gy: f64, d50: f64, gamma_50: f64) -> Self {
        let tcp = 1.0 / (1.0 + ((d50 / dose_gy).powf(4.0 * gamma_50)));
        Self {
            dose_gy,
            tcp,
            d50,
            gamma_50,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalTissueComplicationProbability {
    pub organ: String,
    pub dose_gy: f64,
    pub ntcp: f64,
    pub td50: f64,
    pub m: f64,
}

impl NormalTissueComplicationProbability {
    pub fn calculate(organ: String, dose_gy: f64, td50: f64, m: f64) -> Self {
        let t = (dose_gy - td50) / (m * td50);
        let ntcp = 1.0 / (1.0 + (-t).exp());

        Self {
            organ,
            dose_gy,
            ntcp,
            td50,
            m,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxygenEffect {
    pub oxygen_partial_pressure_mmhg: f64,
    pub oxygen_enhancement_ratio: f64,
}

impl OxygenEffect {
    pub fn calculate_oer(po2_mmhg: f64) -> Self {
        let k = 3.0;
        let oer = 1.0 + 2.0 * (k * po2_mmhg) / (k * po2_mmhg + 1.0);

        Self {
            oxygen_partial_pressure_mmhg: po2_mmhg,
            oxygen_enhancement_ratio: oer.min(3.0),
        }
    }

    pub fn is_hypoxic(&self) -> bool {
        self.oxygen_partial_pressure_mmhg < 10.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radiation_dose() {
        let dose = RadiationDose::new(60.0, 30, RadiationType::Photon);
        assert_eq!(dose.dose_per_fraction_gy, 2.0);

        let bed = dose.biological_effective_dose(10.0);
        assert!((bed - 72.0).abs() < 0.1);
    }

    #[test]
    fn test_dna_damage() {
        let damage = DNADamage::from_dose(2.0);
        assert!(damage.double_strand_breaks > 0);
        assert!(damage.total_damage() > damage.lethal_damage());
    }

    #[test]
    fn test_cell_survival() {
        let survival = CellSurvival {
            cell_type: "Tumor".to_string(),
            surviving_fraction: 0.0,
            d0: 2.0,
            dq: 2.0,
            alpha: 0.3,
            beta: 0.03,
        };

        let sf = survival.calculate_survival_lq(2.0);
        assert!(sf < 1.0);
        assert!(sf > 0.0);
    }

    #[test]
    fn test_tissue_radiosensitivity() {
        let tissue = TissueRadiosensitivity::new_early_responding("Tumor".to_string());
        let dose = RadiationDose::new(60.0, 30, RadiationType::Photon);

        let prob = tissue.complication_probability(&dose);
        assert!((0.0..=1.0).contains(&prob));
    }

    #[test]
    fn test_cell_cycle_radiosensitivity() {
        let m_phase = CellCyclePhase::M.radiosensitivity();
        let s_phase = CellCyclePhase::S.radiosensitivity();

        assert!(m_phase > s_phase);
    }

    #[test]
    fn test_radiotherapy_plan() {
        let plan = RadiotherapyPlan::new("Prostate".to_string(), 78.0, 39);
        assert_eq!(plan.prescription_dose.fractions, 39);
        assert!((plan.estimated_treatment_duration_weeks() - 7.8).abs() < 0.1);
    }

    #[test]
    fn test_toxicity_grade() {
        assert!(ToxicityGrade::Grade3.requires_intervention());
        assert!(!ToxicityGrade::Grade1.requires_intervention());
        assert!(ToxicityGrade::Grade4.is_life_threatening());
    }

    #[test]
    fn test_tcp_calculation() {
        let tcp = TumorControlProbability::calculate(70.0, 50.0, 2.0);
        assert!(tcp.tcp > 0.5);
        assert!(tcp.tcp < 1.0);
    }

    #[test]
    fn test_oxygen_effect() {
        let hypoxic = OxygenEffect::calculate_oer(5.0);
        let normoxic = OxygenEffect::calculate_oer(40.0);

        assert!(hypoxic.is_hypoxic());
        assert!(!normoxic.is_hypoxic());
        assert!(normoxic.oxygen_enhancement_ratio > hypoxic.oxygen_enhancement_ratio);
    }
}

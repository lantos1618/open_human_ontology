use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcidBaseBalance {
    pub ph: f64,
    pub pco2_mmhg: f64,
    pub bicarbonate_meq_l: f64,
    pub base_excess_meq_l: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcidBaseDisturbance {
    Normal,
    MetabolicAcidosis,
    MetabolicAlkalosis,
    RespiratoryAcidosis,
    RespiratoryAlkalosis,
    MixedAcidosis,
    MixedAlkalosis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferSystem {
    pub bicarbonate_buffer: BicarbonateBuffer,
    pub phosphate_buffer: PhosphateBuffer,
    pub protein_buffer: ProteinBuffer,
    pub ammonia_buffer: AmmoniaBuffer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicarbonateBuffer {
    pub hco3_meq_l: f64,
    pub h2co3_meq_l: f64,
    pub pka: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhosphateBuffer {
    pub h2po4_meq_l: f64,
    pub hpo4_meq_l: f64,
    pub pka: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinBuffer {
    pub protein_concentration_g_dl: f64,
    pub buffering_capacity_meq_ph_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmoniaBuffer {
    pub nh3_production_meq_day: f64,
    pub nh4_excretion_meq_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalAcidBaseRegulation {
    pub bicarbonate_reabsorption_meq_day: f64,
    pub bicarbonate_regeneration_meq_day: f64,
    pub titratable_acid_excretion_meq_day: f64,
    pub ammonium_excretion_meq_day: f64,
    pub net_acid_excretion_meq_day: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnionGapType {
    Normal,
    High,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnionGapAnalysis {
    pub anion_gap: f64,
    pub gap_type: AnionGapType,
    pub delta_ratio: Option<f64>,
    pub corrected_for_albumin: f64,
}

impl AcidBaseBalance {
    pub fn new_normal() -> Self {
        Self {
            ph: 7.40,
            pco2_mmhg: 40.0,
            bicarbonate_meq_l: 24.0,
            base_excess_meq_l: 0.0,
        }
    }

    pub fn from_abg(ph: f64, pco2_mmhg: f64, bicarbonate_meq_l: f64) -> Self {
        let base_excess = Self::calculate_base_excess(ph, pco2_mmhg, bicarbonate_meq_l);
        Self {
            ph,
            pco2_mmhg,
            bicarbonate_meq_l,
            base_excess_meq_l: base_excess,
        }
    }

    fn calculate_base_excess(ph: f64, pco2: f64, hco3: f64) -> f64 {
        0.93 * (hco3 - 24.4 + (14.8 * (ph - 7.4)))
    }

    pub fn classify_disturbance(&self) -> AcidBaseDisturbance {
        let acidotic = self.ph < 7.35;
        let alkalotic = self.ph > 7.45;
        let high_pco2 = self.pco2_mmhg > 45.0;
        let low_pco2 = self.pco2_mmhg < 35.0;
        let high_hco3 = self.bicarbonate_meq_l > 26.0;
        let low_hco3 = self.bicarbonate_meq_l < 22.0;

        match (acidotic, alkalotic, high_pco2, low_pco2, high_hco3, low_hco3) {
            (true, _, true, _, _, _) => AcidBaseDisturbance::RespiratoryAcidosis,
            (true, _, _, _, _, true) => AcidBaseDisturbance::MetabolicAcidosis,
            (_, true, _, true, _, _) => AcidBaseDisturbance::RespiratoryAlkalosis,
            (_, true, _, _, true, _) => AcidBaseDisturbance::MetabolicAlkalosis,
            (true, _, true, _, _, true) => AcidBaseDisturbance::MixedAcidosis,
            (_, true, _, true, true, _) => AcidBaseDisturbance::MixedAlkalosis,
            _ => AcidBaseDisturbance::Normal,
        }
    }

    pub fn expected_compensation(&self) -> (f64, f64) {
        match self.classify_disturbance() {
            AcidBaseDisturbance::MetabolicAcidosis => {
                let expected_pco2 = 1.5 * self.bicarbonate_meq_l + 8.0;
                (expected_pco2 - 2.0, expected_pco2 + 2.0)
            }
            AcidBaseDisturbance::MetabolicAlkalosis => {
                let expected_pco2 = 0.7 * (self.bicarbonate_meq_l - 24.0) + 40.0;
                (expected_pco2 - 5.0, expected_pco2 + 5.0)
            }
            AcidBaseDisturbance::RespiratoryAcidosis => {
                let hco3_change = (self.pco2_mmhg - 40.0) * 0.35;
                let expected_hco3 = 24.0 + hco3_change;
                (expected_hco3 - 2.0, expected_hco3 + 2.0)
            }
            AcidBaseDisturbance::RespiratoryAlkalosis => {
                let hco3_change = (40.0 - self.pco2_mmhg) * 0.5;
                let expected_hco3 = 24.0 - hco3_change;
                (expected_hco3 - 2.0, expected_hco3 + 2.0)
            }
            _ => (self.bicarbonate_meq_l, self.bicarbonate_meq_l),
        }
    }

    pub fn is_compensated(&self) -> bool {
        let (min, max) = self.expected_compensation();
        match self.classify_disturbance() {
            AcidBaseDisturbance::MetabolicAcidosis | AcidBaseDisturbance::MetabolicAlkalosis => {
                self.pco2_mmhg >= min && self.pco2_mmhg <= max
            }
            AcidBaseDisturbance::RespiratoryAcidosis | AcidBaseDisturbance::RespiratoryAlkalosis => {
                self.bicarbonate_meq_l >= min && self.bicarbonate_meq_l <= max
            }
            _ => true,
        }
    }

    pub fn h_concentration_nmol_l(&self) -> f64 {
        10_f64.powf(9.0 - self.ph)
    }

    pub fn henderson_hasselbalch_check(&self) -> bool {
        let calculated_ph = 6.1 + (self.bicarbonate_meq_l / (0.03 * self.pco2_mmhg)).log10();
        (calculated_ph - self.ph).abs() < 0.05
    }
}

impl BufferSystem {
    pub fn new_normal() -> Self {
        Self {
            bicarbonate_buffer: BicarbonateBuffer::new(),
            phosphate_buffer: PhosphateBuffer::new(),
            protein_buffer: ProteinBuffer::new(),
            ammonia_buffer: AmmoniaBuffer::new(),
        }
    }

    pub fn total_buffering_capacity(&self) -> f64 {
        self.bicarbonate_buffer.buffering_capacity()
            + self.phosphate_buffer.buffering_capacity()
            + self.protein_buffer.buffering_capacity_meq_ph_l
    }

    pub fn buffer_acid_load(&mut self, h_ions_meq: f64) {
        let bicarb_proportion = 0.75;
        let phosphate_proportion = 0.10;
        let protein_proportion = 0.15;

        self.bicarbonate_buffer.hco3_meq_l -= h_ions_meq * bicarb_proportion;
        self.phosphate_buffer.hpo4_meq_l -= h_ions_meq * phosphate_proportion;
    }
}

impl BicarbonateBuffer {
    pub fn new() -> Self {
        Self {
            hco3_meq_l: 24.0,
            h2co3_meq_l: 1.2,
            pka: 6.1,
        }
    }

    pub fn buffering_capacity(&self) -> f64 {
        self.hco3_meq_l * 2.3
    }

    pub fn calculate_ph(&self) -> f64 {
        self.pka + (self.hco3_meq_l / self.h2co3_meq_l).log10()
    }
}

impl Default for BicarbonateBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl PhosphateBuffer {
    pub fn new() -> Self {
        Self {
            h2po4_meq_l: 1.0,
            hpo4_meq_l: 4.0,
            pka: 6.8,
        }
    }

    pub fn buffering_capacity(&self) -> f64 {
        (self.h2po4_meq_l + self.hpo4_meq_l) * 0.7
    }

    pub fn calculate_ph(&self) -> f64 {
        self.pka + (self.hpo4_meq_l / self.h2po4_meq_l).log10()
    }
}

impl Default for PhosphateBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl ProteinBuffer {
    pub fn new() -> Self {
        Self {
            protein_concentration_g_dl: 7.0,
            buffering_capacity_meq_ph_l: 8.0,
        }
    }

    pub fn albumin_contribution(&self) -> f64 {
        self.protein_concentration_g_dl * 0.15
    }
}

impl Default for ProteinBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl AmmoniaBuffer {
    pub fn new() -> Self {
        Self {
            nh3_production_meq_day: 40.0,
            nh4_excretion_meq_day: 30.0,
        }
    }

    pub fn increase_production_acidosis(&mut self) {
        self.nh3_production_meq_day *= 3.0;
        self.nh4_excretion_meq_day = self.nh3_production_meq_day * 0.85;
    }

    pub fn net_acid_excretion(&self) -> f64 {
        self.nh4_excretion_meq_day
    }
}

impl RenalAcidBaseRegulation {
    pub fn new_normal() -> Self {
        Self {
            bicarbonate_reabsorption_meq_day: 4320.0,
            bicarbonate_regeneration_meq_day: 70.0,
            titratable_acid_excretion_meq_day: 30.0,
            ammonium_excretion_meq_day: 40.0,
            net_acid_excretion_meq_day: 70.0,
        }
    }

    pub fn calculate_net_acid_excretion(&self) -> f64 {
        self.ammonium_excretion_meq_day + self.titratable_acid_excretion_meq_day
    }

    pub fn adapt_to_acidosis(&mut self) {
        self.ammonium_excretion_meq_day *= 3.0;
        self.titratable_acid_excretion_meq_day *= 1.5;
        self.bicarbonate_regeneration_meq_day *= 2.0;
        self.net_acid_excretion_meq_day = self.calculate_net_acid_excretion();
    }

    pub fn adapt_to_alkalosis(&mut self) {
        self.bicarbonate_reabsorption_meq_day *= 0.5;
        self.ammonium_excretion_meq_day *= 0.3;
        self.net_acid_excretion_meq_day = self.calculate_net_acid_excretion();
    }

    pub fn bicarbonate_reabsorption_rate(&self) -> f64 {
        self.bicarbonate_reabsorption_meq_day / 4320.0
    }
}

impl AnionGapAnalysis {
    pub fn calculate(sodium: f64, chloride: f64, bicarbonate: f64, albumin_g_dl: f64) -> Self {
        let anion_gap = sodium - (chloride + bicarbonate);
        let corrected_ag = anion_gap + 2.5 * (4.0 - albumin_g_dl);

        let gap_type = if corrected_ag > 12.0 {
            AnionGapType::High
        } else if corrected_ag < 6.0 {
            AnionGapType::Low
        } else {
            AnionGapType::Normal
        };

        Self {
            anion_gap,
            gap_type,
            delta_ratio: None,
            corrected_for_albumin: corrected_ag,
        }
    }

    pub fn calculate_delta_ratio(&mut self, baseline_bicarbonate: f64) {
        if self.anion_gap > 12.0 {
            let delta_ag = self.anion_gap - 12.0;
            let delta_hco3 = 24.0 - baseline_bicarbonate;
            if delta_hco3.abs() > 0.1 {
                self.delta_ratio = Some(delta_ag / delta_hco3);
            }
        }
    }

    pub fn interpret_delta_ratio(&self) -> Option<&str> {
        self.delta_ratio.map(|ratio| {
            if ratio < 0.4 {
                "Normal anion gap metabolic acidosis"
            } else if ratio < 1.0 {
                "Mixed high anion gap and normal anion gap acidosis"
            } else if ratio > 2.0 {
                "Metabolic alkalosis and high anion gap acidosis"
            } else {
                "Pure high anion gap metabolic acidosis"
            }
        })
    }

    pub fn mudpiles_causes(&self) -> Vec<&str> {
        if matches!(self.gap_type, AnionGapType::High) {
            vec![
                "Methanol",
                "Uremia",
                "Diabetic ketoacidosis",
                "Propylene glycol/Paraldehyde",
                "Iron/Isoniazid",
                "Lactic acidosis",
                "Ethylene glycol",
                "Salicylates",
            ]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_acid_base() {
        let ab = AcidBaseBalance::new_normal();
        assert_eq!(ab.ph, 7.40);
        assert_eq!(ab.pco2_mmhg, 40.0);
        assert_eq!(ab.bicarbonate_meq_l, 24.0);
        assert_eq!(ab.classify_disturbance(), AcidBaseDisturbance::Normal);
    }

    #[test]
    fn test_metabolic_acidosis() {
        let ab = AcidBaseBalance::from_abg(7.30, 38.0, 18.0);
        assert_eq!(ab.classify_disturbance(), AcidBaseDisturbance::MetabolicAcidosis);
        let (min, max) = ab.expected_compensation();
        assert!(min < max);
    }

    #[test]
    fn test_respiratory_acidosis() {
        let ab = AcidBaseBalance::from_abg(7.32, 55.0, 28.0);
        assert_eq!(ab.classify_disturbance(), AcidBaseDisturbance::RespiratoryAcidosis);
    }

    #[test]
    fn test_metabolic_alkalosis() {
        let ab = AcidBaseBalance::from_abg(7.50, 48.0, 36.0);
        assert_eq!(ab.classify_disturbance(), AcidBaseDisturbance::MetabolicAlkalosis);
    }

    #[test]
    fn test_respiratory_alkalosis() {
        let ab = AcidBaseBalance::from_abg(7.48, 30.0, 22.0);
        assert_eq!(ab.classify_disturbance(), AcidBaseDisturbance::RespiratoryAlkalosis);
    }

    #[test]
    fn test_henderson_hasselbalch() {
        let ab = AcidBaseBalance::new_normal();
        assert!(ab.henderson_hasselbalch_check());
    }

    #[test]
    fn test_compensation() {
        let ab = AcidBaseBalance::from_abg(7.35, 30.0, 18.0);
        assert!(ab.is_compensated());
    }

    #[test]
    fn test_bicarbonate_buffer() {
        let buffer = BicarbonateBuffer::new();
        let ph = buffer.calculate_ph();
        assert!(ph > 7.0 && ph < 8.0);
        assert!(buffer.buffering_capacity() > 50.0);
    }

    #[test]
    fn test_phosphate_buffer() {
        let buffer = PhosphateBuffer::new();
        let ph = buffer.calculate_ph();
        assert!(ph > 6.0 && ph < 8.0);
    }

    #[test]
    fn test_buffer_system() {
        let system = BufferSystem::new_normal();
        assert!(system.total_buffering_capacity() > 50.0);
    }

    #[test]
    fn test_renal_regulation() {
        let mut reg = RenalAcidBaseRegulation::new_normal();
        let baseline_nae = reg.net_acid_excretion_meq_day;

        reg.adapt_to_acidosis();
        assert!(reg.net_acid_excretion_meq_day > baseline_nae);
    }

    #[test]
    fn test_ammonia_buffer_acidosis() {
        let mut ammonia = AmmoniaBuffer::new();
        let baseline = ammonia.nh3_production_meq_day;

        ammonia.increase_production_acidosis();
        assert!(ammonia.nh3_production_meq_day > baseline);
    }

    #[test]
    fn test_anion_gap() {
        let ag = AnionGapAnalysis::calculate(140.0, 105.0, 24.0, 4.0);
        assert!(ag.anion_gap > 8.0 && ag.anion_gap < 16.0);
        assert_eq!(ag.gap_type, AnionGapType::Normal);
    }

    #[test]
    fn test_high_anion_gap() {
        let ag = AnionGapAnalysis::calculate(140.0, 105.0, 10.0, 4.0);
        assert_eq!(ag.gap_type, AnionGapType::High);
        assert!(!ag.mudpiles_causes().is_empty());
    }

    #[test]
    fn test_delta_ratio() {
        let mut ag = AnionGapAnalysis::calculate(140.0, 100.0, 14.0, 4.0);
        ag.calculate_delta_ratio(14.0);
        let interpretation = ag.interpret_delta_ratio();
        assert!(interpretation.is_some());
    }

    #[test]
    fn test_albumin_correction() {
        let ag_low_albumin = AnionGapAnalysis::calculate(140.0, 105.0, 24.0, 2.0);
        let ag_normal_albumin = AnionGapAnalysis::calculate(140.0, 105.0, 24.0, 4.0);

        assert!(ag_low_albumin.corrected_for_albumin > ag_normal_albumin.corrected_for_albumin);
    }
}

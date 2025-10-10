use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Arrhythmia {
    Bradycardia(BradycardiaType),
    Tachycardia(TachycardiaType),
    PrematureContractions(PrematureContractionType),
    HeartBlock(HeartBlockDegree),
    FibrillationFlutter(FibrillationType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BradycardiaType {
    SinusBradycardia,
    SickSinusSyndrome,
    JunctionalBradycardia,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TachycardiaType {
    SinusTachycardia,
    SupraventricularTachycardia,
    VentricularTachycardia,
    AtrialTachycardia,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrematureContractionType {
    PrematureAtrialContraction,
    PrematureVentricularContraction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeartBlockDegree {
    FirstDegree,
    SecondDegreeMobitzI,
    SecondDegreeMobitzII,
    ThirdDegree,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FibrillationType {
    AtrialFibrillation,
    AtrialFlutter,
    VentricularFibrillation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalConduction {
    pub sa_node_rate_bpm: f64,
    pub av_node_delay_ms: f64,
    pub pr_interval_ms: f64,
    pub qrs_duration_ms: f64,
    pub qt_interval_ms: f64,
    pub refractory_period_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ECGReading {
    pub heart_rate_bpm: f64,
    pub rhythm: ECGRhythm,
    pub pr_interval_ms: f64,
    pub qrs_duration_ms: f64,
    pub qt_interval_ms: f64,
    pub st_segment_deviation_mm: f64,
    pub p_wave_present: bool,
    pub qrs_axis_degrees: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ECGRhythm {
    SinusRhythm,
    SinusBradycardia,
    SinusTachycardia,
    AtrialFibrillation,
    AtrialFlutter,
    VentricularTachycardia,
    VentricularFibrillation,
    Asystole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrhythmiaRisk {
    pub risk_factors: HashMap<String, f64>,
    pub total_risk_score: f64,
    pub predicted_arrhythmias: Vec<Arrhythmia>,
}

impl ElectricalConduction {
    pub fn new() -> Self {
        Self {
            sa_node_rate_bpm: 70.0,
            av_node_delay_ms: 120.0,
            pr_interval_ms: 160.0,
            qrs_duration_ms: 90.0,
            qt_interval_ms: 400.0,
            refractory_period_ms: 200.0,
        }
    }

    pub fn calculate_corrected_qt(&self, heart_rate: f64) -> f64 {
        let rr_interval_s = 60.0 / heart_rate;
        self.qt_interval_ms / rr_interval_s.sqrt()
    }

    pub fn has_prolonged_qt(&self, heart_rate: f64) -> bool {
        self.calculate_corrected_qt(heart_rate) > 450.0
    }

    pub fn has_av_block(&self) -> Option<HeartBlockDegree> {
        if self.pr_interval_ms > 300.0 {
            Some(HeartBlockDegree::SecondDegreeMobitzII)
        } else if self.pr_interval_ms > 200.0 {
            Some(HeartBlockDegree::FirstDegree)
        } else {
            None
        }
    }

    pub fn assess_conduction_abnormalities(&self) -> Vec<String> {
        let mut abnormalities = Vec::new();

        if self.pr_interval_ms > 200.0 {
            abnormalities.push("Prolonged PR interval".to_string());
        }
        if self.qrs_duration_ms > 120.0 {
            abnormalities.push("Wide QRS complex".to_string());
        }
        if self.qt_interval_ms > 450.0 {
            abnormalities.push("Prolonged QT interval".to_string());
        }

        abnormalities
    }
}

impl Default for ElectricalConduction {
    fn default() -> Self {
        Self::new()
    }
}

impl ECGReading {
    pub fn new(heart_rate: f64) -> Self {
        let rhythm = if heart_rate < 60.0 {
            ECGRhythm::SinusBradycardia
        } else if heart_rate > 100.0 {
            ECGRhythm::SinusTachycardia
        } else {
            ECGRhythm::SinusRhythm
        };

        Self {
            heart_rate_bpm: heart_rate,
            rhythm,
            pr_interval_ms: 160.0,
            qrs_duration_ms: 90.0,
            qt_interval_ms: 400.0,
            st_segment_deviation_mm: 0.0,
            p_wave_present: true,
            qrs_axis_degrees: 60.0,
        }
    }

    pub fn detect_arrhythmias(&self) -> Vec<Arrhythmia> {
        let mut arrhythmias = Vec::new();

        match self.rhythm {
            ECGRhythm::SinusBradycardia => {
                arrhythmias.push(Arrhythmia::Bradycardia(BradycardiaType::SinusBradycardia));
            }
            ECGRhythm::SinusTachycardia => {
                arrhythmias.push(Arrhythmia::Tachycardia(TachycardiaType::SinusTachycardia));
            }
            ECGRhythm::AtrialFibrillation => {
                arrhythmias.push(Arrhythmia::FibrillationFlutter(
                    FibrillationType::AtrialFibrillation,
                ));
            }
            ECGRhythm::VentricularTachycardia => {
                arrhythmias.push(Arrhythmia::Tachycardia(
                    TachycardiaType::VentricularTachycardia,
                ));
            }
            _ => {}
        }

        if self.pr_interval_ms > 200.0 {
            arrhythmias.push(Arrhythmia::HeartBlock(HeartBlockDegree::FirstDegree));
        }

        arrhythmias
    }

    pub fn is_life_threatening(&self) -> bool {
        matches!(
            self.rhythm,
            ECGRhythm::VentricularFibrillation
                | ECGRhythm::VentricularTachycardia
                | ECGRhythm::Asystole
        )
    }

    pub fn requires_immediate_treatment(&self) -> bool {
        self.is_life_threatening() || self.st_segment_deviation_mm.abs() > 1.0
    }
}

impl ArrhythmiaRisk {
    pub fn new() -> Self {
        Self {
            risk_factors: HashMap::new(),
            total_risk_score: 0.0,
            predicted_arrhythmias: Vec::new(),
        }
    }

    pub fn add_risk_factor(&mut self, factor: String, weight: f64) {
        self.risk_factors.insert(factor, weight);
        self.calculate_total_risk();
    }

    fn calculate_total_risk(&mut self) {
        self.total_risk_score = self.risk_factors.values().sum();

        self.predicted_arrhythmias.clear();

        if self.risk_factors.contains_key("age_over_65") {
            self.predicted_arrhythmias
                .push(Arrhythmia::FibrillationFlutter(
                    FibrillationType::AtrialFibrillation,
                ));
        }
        if self.risk_factors.contains_key("heart_disease") {
            self.predicted_arrhythmias.push(Arrhythmia::Tachycardia(
                TachycardiaType::VentricularTachycardia,
            ));
        }
    }

    pub fn get_prevention_strategies(&self) -> Vec<String> {
        let mut strategies = Vec::new();

        if self.total_risk_score > 10.0 {
            strategies.push("Regular cardiac monitoring".to_string());
            strategies.push("Antiarrhythmic medication consideration".to_string());
        }
        if self.risk_factors.contains_key("hypertension") {
            strategies.push("Blood pressure control".to_string());
        }
        if self.risk_factors.contains_key("obesity") {
            strategies.push("Weight management".to_string());
        }

        strategies
    }
}

impl Default for ArrhythmiaRisk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electrical_conduction() {
        let conduction = ElectricalConduction::new();
        assert_eq!(conduction.sa_node_rate_bpm, 70.0);
        assert!(conduction.pr_interval_ms < 200.0);
    }

    #[test]
    fn test_corrected_qt() {
        let conduction = ElectricalConduction::new();
        let qtc = conduction.calculate_corrected_qt(70.0);
        assert!(qtc > 0.0);
    }

    #[test]
    fn test_av_block_detection() {
        let mut conduction = ElectricalConduction::new();
        conduction.pr_interval_ms = 250.0;
        let block = conduction.has_av_block();
        assert!(block.is_some());
    }

    #[test]
    fn test_ecg_reading() {
        let ecg = ECGReading::new(70.0);
        assert_eq!(ecg.rhythm, ECGRhythm::SinusRhythm);
        assert!(ecg.p_wave_present);
    }

    #[test]
    fn test_arrhythmia_detection() {
        let mut ecg = ECGReading::new(50.0);
        ecg.rhythm = ECGRhythm::SinusBradycardia;
        let arrhythmias = ecg.detect_arrhythmias();
        assert!(!arrhythmias.is_empty());
    }

    #[test]
    fn test_life_threatening_detection() {
        let mut ecg = ECGReading::new(200.0);
        ecg.rhythm = ECGRhythm::VentricularFibrillation;
        assert!(ecg.is_life_threatening());
    }

    #[test]
    fn test_arrhythmia_risk() {
        let mut risk = ArrhythmiaRisk::new();
        risk.add_risk_factor("age_over_65".to_string(), 5.0);
        assert!(risk.total_risk_score > 0.0);
    }
}

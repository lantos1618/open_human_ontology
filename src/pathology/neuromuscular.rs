use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NeuromuscularDisorder {
    MyastheniaGravis,
    LambertEatonSyndrome,
    MuscularDystrophy(DystrophyType),
    AmyotrophicLateralSclerosis,
    SpinalMuscularAtrophy,
    CharcotMarieTooth,
    GuillainBarreSyndrome,
    ChronicInflammatoryDemyelinatingPolyneuropathy,
    Myositis(MyositisType),
    Myotonia,
    PeriodicParalysis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DystrophyType {
    Duchenne,
    Becker,
    Limb_girdle,
    Facioscapulohumeral,
    Myotonic,
    Oculopharyngeal,
    Emery_dreifuss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MyositisType {
    Polymyositis,
    Dermatomyositis,
    InclusionBodyMyositis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuromuscularProfile {
    pub disorders: Vec<NeuromuscularDisorder>,
    pub muscle_strength: HashMap<MuscleGroup, StrengthGrade>,
    pub nerve_conduction_studies: Option<NerveConduction>,
    pub emg_findings: Option<EMGFindings>,
    pub creatine_kinase_u_l: f64,
    pub acetylcholine_receptor_antibodies: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MuscleGroup {
    NeckFlexors,
    NeckExtensors,
    ShoulderAbductors,
    ElbowFlexors,
    ElbowExtensors,
    WristExtensors,
    WristFlexors,
    FingerFlexors,
    FingerExtensors,
    HipFlexors,
    HipExtensors,
    KneeFlexors,
    KneeExtensors,
    AnkleDorsiflexors,
    AnklePlantarflexors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrengthGrade {
    Zero,
    Trace,
    Poor,
    Fair,
    Good,
    Normal,
}

impl StrengthGrade {
    pub fn to_mrc_scale(&self) -> u8 {
        match self {
            StrengthGrade::Zero => 0,
            StrengthGrade::Trace => 1,
            StrengthGrade::Poor => 2,
            StrengthGrade::Fair => 3,
            StrengthGrade::Good => 4,
            StrengthGrade::Normal => 5,
        }
    }

    pub fn from_mrc_scale(scale: u8) -> Self {
        match scale {
            0 => StrengthGrade::Zero,
            1 => StrengthGrade::Trace,
            2 => StrengthGrade::Poor,
            3 => StrengthGrade::Fair,
            4 => StrengthGrade::Good,
            _ => StrengthGrade::Normal,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NerveConduction {
    pub motor_studies: Vec<MotorStudy>,
    pub sensory_studies: Vec<SensoryStudy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorStudy {
    pub nerve: String,
    pub distal_latency_ms: f64,
    pub amplitude_mv: f64,
    pub conduction_velocity_m_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryStudy {
    pub nerve: String,
    pub peak_latency_ms: f64,
    pub amplitude_uv: f64,
    pub conduction_velocity_m_s: f64,
}

impl MotorStudy {
    pub fn is_abnormal(&self) -> bool {
        self.distal_latency_ms > 4.5 ||
        self.amplitude_mv < 4.0 ||
        self.conduction_velocity_m_s < 45.0
    }
}

impl SensoryStudy {
    pub fn is_abnormal(&self) -> bool {
        self.peak_latency_ms > 3.5 ||
        self.amplitude_uv < 10.0 ||
        self.conduction_velocity_m_s < 40.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMGFindings {
    pub spontaneous_activity: SpontaneousActivity,
    pub motor_unit_potentials: MotorUnitPotentialPattern,
    pub recruitment_pattern: RecruitmentPattern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpontaneousActivity {
    None,
    Fibrillations,
    PositiveSharpWaves,
    Fasciculations,
    MyotonicDischarges,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotorUnitPotentialPattern {
    Normal,
    LowAmplitudeShortDuration,
    HighAmplitudeLongDuration,
    Polyphasic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecruitmentPattern {
    Normal,
    Reduced,
    Increased,
    Absent,
}

impl EMGFindings {
    pub fn suggests_myopathy(&self) -> bool {
        matches!(self.motor_unit_potentials, MotorUnitPotentialPattern::LowAmplitudeShortDuration) &&
        matches!(self.recruitment_pattern, RecruitmentPattern::Increased)
    }

    pub fn suggests_neuropathy(&self) -> bool {
        matches!(self.motor_unit_potentials, MotorUnitPotentialPattern::HighAmplitudeLongDuration) &&
        matches!(self.recruitment_pattern, RecruitmentPattern::Reduced)
    }

    pub fn suggests_motor_neuron_disease(&self) -> bool {
        matches!(self.spontaneous_activity, SpontaneousActivity::Fibrillations | SpontaneousActivity::Fasciculations) &&
        matches!(self.motor_unit_potentials, MotorUnitPotentialPattern::HighAmplitudeLongDuration)
    }
}

impl NeuromuscularProfile {
    pub fn new() -> Self {
        Self {
            disorders: Vec::new(),
            muscle_strength: HashMap::new(),
            nerve_conduction_studies: None,
            emg_findings: None,
            creatine_kinase_u_l: 100.0,
            acetylcholine_receptor_antibodies: false,
        }
    }

    pub fn mrc_sum_score(&self) -> u32 {
        self.muscle_strength.values()
            .map(|g| g.to_mrc_scale() as u32)
            .sum()
    }

    pub fn has_proximal_weakness(&self) -> bool {
        let proximal_groups = [
            MuscleGroup::ShoulderAbductors,
            MuscleGroup::HipFlexors,
            MuscleGroup::HipExtensors,
        ];

        proximal_groups.iter().any(|group| {
            self.muscle_strength.get(group)
                .map(|s| s.to_mrc_scale() < 4)
                .unwrap_or(false)
        })
    }

    pub fn has_distal_weakness(&self) -> bool {
        let distal_groups = [
            MuscleGroup::FingerFlexors,
            MuscleGroup::FingerExtensors,
            MuscleGroup::AnkleDorsiflexors,
            MuscleGroup::AnklePlantarflexors,
        ];

        distal_groups.iter().any(|group| {
            self.muscle_strength.get(group)
                .map(|s| s.to_mrc_scale() < 4)
                .unwrap_or(false)
        })
    }

    pub fn ck_elevation_fold(&self) -> f64 {
        self.creatine_kinase_u_l / 200.0
    }

    pub fn is_ck_elevated(&self) -> bool {
        self.creatine_kinase_u_l > 200.0
    }
}

impl Default for NeuromuscularProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyastheniaGravisProfile {
    pub mgfa_class: MGFAClass,
    pub symptoms: Vec<MyastheniaSymptom>,
    pub acetylcholine_receptor_ab_positive: bool,
    pub musk_ab_positive: bool,
    pub thymoma_present: bool,
    pub ice_pack_test_positive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MGFAClass {
    Class1,
    Class2a,
    Class2b,
    Class3a,
    Class3b,
    Class4a,
    Class4b,
    Class5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MyastheniaSymptom {
    Ptosis,
    Diplopia,
    Dysphagia,
    Dysarthria,
    LimbWeakness,
    NeckWeakness,
    RespiratoryWeakness,
    FatiguableWeakness,
}

impl MyastheniaGravisProfile {
    pub fn new() -> Self {
        Self {
            mgfa_class: MGFAClass::Class1,
            symptoms: Vec::new(),
            acetylcholine_receptor_ab_positive: false,
            musk_ab_positive: false,
            thymoma_present: false,
            ice_pack_test_positive: false,
        }
    }

    pub fn is_generalized(&self) -> bool {
        self.symptoms.contains(&MyastheniaSymptom::LimbWeakness)
    }

    pub fn is_ocular_only(&self) -> bool {
        let ocular_symptoms = self.symptoms.iter()
            .filter(|s| matches!(s, MyastheniaSymptom::Ptosis | MyastheniaSymptom::Diplopia))
            .count();

        ocular_symptoms == self.symptoms.len() && !self.symptoms.is_empty()
    }

    pub fn requires_immunotherapy(&self) -> bool {
        matches!(self.mgfa_class,
            MGFAClass::Class3a | MGFAClass::Class3b |
            MGFAClass::Class4a | MGFAClass::Class4b |
            MGFAClass::Class5
        )
    }
}

impl Default for MyastheniaGravisProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ALSProfile {
    pub onset_region: ALSOnset,
    pub bulbar_involvement: bool,
    pub respiratory_involvement: bool,
    pub upper_motor_neuron_signs: Vec<UMNSign>,
    pub lower_motor_neuron_signs: Vec<LMNSign>,
    pub forced_vital_capacity_percent: f64,
    pub alsfrs_r_score: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ALSOnset {
    Limb,
    Bulbar,
    Respiratory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UMNSign {
    Hyperreflexia,
    Spasticity,
    BabinskiSign,
    Clonus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LMNSign {
    Weakness,
    Atrophy,
    Fasciculations,
    Hyporeflexia,
}

impl ALSProfile {
    pub fn new(onset_region: ALSOnset) -> Self {
        Self {
            onset_region,
            bulbar_involvement: false,
            respiratory_involvement: false,
            upper_motor_neuron_signs: Vec::new(),
            lower_motor_neuron_signs: Vec::new(),
            forced_vital_capacity_percent: 100.0,
            alsfrs_r_score: 48,
        }
    }

    pub fn meets_el_escorial_criteria(&self) -> bool {
        !self.upper_motor_neuron_signs.is_empty() &&
        !self.lower_motor_neuron_signs.is_empty()
    }

    pub fn requires_ventilatory_support(&self) -> bool {
        self.forced_vital_capacity_percent < 50.0 || self.respiratory_involvement
    }

    pub fn disease_progression_rate(&self) -> f64 {
        (48.0 - self.alsfrs_r_score as f64) / 48.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength_grade_conversion() {
        assert_eq!(StrengthGrade::Normal.to_mrc_scale(), 5);
        assert_eq!(StrengthGrade::from_mrc_scale(3), StrengthGrade::Fair);
    }

    #[test]
    fn test_neuromuscular_profile() {
        let mut profile = NeuromuscularProfile::new();
        profile.muscle_strength.insert(MuscleGroup::HipFlexors, StrengthGrade::Poor);
        profile.muscle_strength.insert(MuscleGroup::AnkleDorsiflexors, StrengthGrade::Good);

        assert!(profile.has_proximal_weakness());
    }

    #[test]
    fn test_ck_elevation() {
        let mut profile = NeuromuscularProfile::new();
        profile.creatine_kinase_u_l = 1000.0;

        assert!(profile.is_ck_elevated());
        assert_eq!(profile.ck_elevation_fold(), 5.0);
    }

    #[test]
    fn test_motor_study() {
        let study = MotorStudy {
            nerve: "Median".to_string(),
            distal_latency_ms: 5.0,
            amplitude_mv: 3.0,
            conduction_velocity_m_s: 40.0,
        };

        assert!(study.is_abnormal());
    }

    #[test]
    fn test_emg_patterns() {
        let myopathy = EMGFindings {
            spontaneous_activity: SpontaneousActivity::None,
            motor_unit_potentials: MotorUnitPotentialPattern::LowAmplitudeShortDuration,
            recruitment_pattern: RecruitmentPattern::Increased,
        };

        assert!(myopathy.suggests_myopathy());
        assert!(!myopathy.suggests_neuropathy());
    }

    #[test]
    fn test_myasthenia_gravis() {
        let mut mg = MyastheniaGravisProfile::new();
        mg.symptoms.push(MyastheniaSymptom::Ptosis);
        mg.symptoms.push(MyastheniaSymptom::Diplopia);

        assert!(mg.is_ocular_only());
        assert!(!mg.is_generalized());
    }

    #[test]
    fn test_als_profile() {
        let mut als = ALSProfile::new(ALSOnset::Limb);
        als.upper_motor_neuron_signs.push(UMNSign::Hyperreflexia);
        als.lower_motor_neuron_signs.push(LMNSign::Weakness);
        als.lower_motor_neuron_signs.push(LMNSign::Fasciculations);

        assert!(als.meets_el_escorial_criteria());
    }

    #[test]
    fn test_als_respiratory() {
        let mut als = ALSProfile::new(ALSOnset::Bulbar);
        als.forced_vital_capacity_percent = 45.0;

        assert!(als.requires_ventilatory_support());
    }
}

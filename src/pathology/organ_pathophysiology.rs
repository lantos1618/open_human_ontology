use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganPathophysiology {
    pub cardiac_pathology: CardiacPathology,
    pub pulmonary_pathology: PulmonaryPathology,
    pub hepatic_pathology: HepaticPathology,
    pub renal_pathology: RenalPathology,
    pub neurological_pathology: NeurologicalPathology,
    pub endocrine_pathology: EndocrinePathology,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiacPathology {
    pub ejection_fraction: f64,
    pub cardiac_output_status: CardiacOutputStatus,
    pub arrhythmias: Vec<ArrhythmiaType>,
    pub valvular_dysfunction: Vec<ValvularDysfunction>,
    pub coronary_artery_disease_severity: CADSeverity,
    pub heart_failure_stage: Option<HeartFailureStage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardiacOutputStatus {
    Normal,
    ReducedMild,
    ReducedModerate,
    ReducedSevere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArrhythmiaType {
    SinusBradycardia,
    SinusTachycardia,
    AtrialFibrillation,
    AtrialFlutter,
    VentricularTachycardia,
    VentricularFibrillation,
    PrematureAtrialContractions,
    PrematureVentricularContractions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValvularDysfunction {
    pub valve: ValveType,
    pub dysfunction_type: ValveDysfunction,
    pub severity: DysfunctionSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValveType {
    Mitral,
    Aortic,
    Tricuspid,
    Pulmonary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValveDysfunction {
    Stenosis,
    Regurgitation,
    Prolapse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DysfunctionSeverity {
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CADSeverity {
    None,
    Minimal,
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeartFailureStage {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulmonaryPathology {
    pub fev1_percent_predicted: f64,
    pub fvc_percent_predicted: f64,
    pub fev1_fvc_ratio: f64,
    pub diffusion_capacity: f64,
    pub obstructive_pattern: bool,
    pub restrictive_pattern: bool,
    pub chronic_conditions: Vec<ChronicPulmonaryCondition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChronicPulmonaryCondition {
    COPD,
    Asthma,
    InterstitialLungDisease,
    PulmonaryFibrosis,
    BronchiectaSIS,
    PulmonaryHypertension,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HepaticPathology {
    pub liver_function_tests: LiverFunctionTests,
    pub fibrosis_stage: FibrosisStage,
    pub steatosis_grade: SteatosisGrade,
    pub portal_hypertension: bool,
    pub synthetic_function: SyntheticFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiverFunctionTests {
    pub alt_u_per_l: f64,
    pub ast_u_per_l: f64,
    pub alkaline_phosphatase_u_per_l: f64,
    pub bilirubin_mg_per_dl: f64,
    pub albumin_g_per_dl: f64,
    pub inr: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FibrosisStage {
    F0,
    F1,
    F2,
    F3,
    F4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SteatosisGrade {
    S0,
    S1,
    S2,
    S3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyntheticFunction {
    Normal,
    MildImpairment,
    ModerateImpairment,
    SevereImpairment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalPathology {
    pub gfr_ml_per_min: f64,
    pub ckd_stage: Option<CKDStage>,
    pub proteinuria_mg_per_day: f64,
    pub electrolyte_disturbances: Vec<ElectrolyteDisturbance>,
    pub acid_base_status: AcidBaseStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CKDStage {
    Stage1,
    Stage2,
    Stage3a,
    Stage3b,
    Stage4,
    Stage5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectrolyteDisturbance {
    pub electrolyte: Electrolyte,
    pub level: f64,
    pub abnormality: ElectrolyteAbnormality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Electrolyte {
    Sodium,
    Potassium,
    Calcium,
    Phosphate,
    Magnesium,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElectrolyteAbnormality {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcidBaseStatus {
    pub ph: f64,
    pub bicarbonate_mmol_per_l: f64,
    pub pco2_mmhg: f64,
    pub disorder_type: Option<AcidBaseDisorder>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcidBaseDisorder {
    MetabolicAcidosis,
    MetabolicAlkalosis,
    RespiratoryAcidosis,
    RespiratoryAlkalosis,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurologicalPathology {
    pub cognitive_function: CognitiveFunction,
    pub motor_function: MotorFunction,
    pub sensory_function: SensoryFunction,
    pub neurodegenerative_conditions: Vec<NeurodegenerativeCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveFunction {
    pub mmse_score: Option<u8>,
    pub moca_score: Option<u8>,
    pub memory_impairment: bool,
    pub executive_dysfunction: bool,
    pub language_impairment: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorFunction {
    pub muscle_strength: HashMap<String, u8>,
    pub coordination: CoordinationStatus,
    pub gait_abnormality: bool,
    pub tremor: Option<TremorType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinationStatus {
    Normal,
    MildImpairment,
    ModerateImpairment,
    SevereImpairment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TremorType {
    RestingTremor,
    ActionTremor,
    IntentionTremor,
    PosturalTremor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryFunction {
    pub proprioception: SensoryStatus,
    pub vibration_sense: SensoryStatus,
    pub pain_sensation: SensoryStatus,
    pub temperature_sensation: SensoryStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensoryStatus {
    Normal,
    Decreased,
    Absent,
    Hyperesthetic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeurodegenerativeCondition {
    AlzheimersDisease,
    ParkinsonsDisease,
    LewyBodyDementia,
    FrontotemporalDementia,
    MultipleSclerosis,
    ALS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndocrinePathology {
    pub thyroid_function: ThyroidFunction,
    pub adrenal_function: AdrenalFunction,
    pub glucose_metabolism: GlucoseMetabolismStatus,
    pub bone_metabolism: BoneMetabolismStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThyroidFunction {
    pub tsh_miu_per_l: f64,
    pub free_t4_ng_per_dl: f64,
    pub free_t3_pg_per_ml: f64,
    pub status: ThyroidStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThyroidStatus {
    Euthyroid,
    SubclinicalHypothyroid,
    OvertHypothyroid,
    SubclinicalHyperthyroid,
    OvertHyperthyroid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdrenalFunction {
    pub cortisol_ug_per_dl: f64,
    pub acth_pg_per_ml: f64,
    pub aldosterone_ng_per_dl: f64,
    pub status: AdrenalStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdrenalStatus {
    Normal,
    PrimaryInsufficiency,
    SecondaryInsufficiency,
    Hyperfunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlucoseMetabolismStatus {
    pub fasting_glucose_mg_per_dl: f64,
    pub hba1c_percent: f64,
    pub insulin_level_uiu_per_ml: f64,
    pub homa_ir: f64,
    pub diabetes_status: DiabetesStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiabetesStatus {
    Normal,
    Prediabetes,
    Type1Diabetes,
    Type2Diabetes,
    GestationalDiabetes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneMetabolismStatus {
    pub bone_density_t_score: f64,
    pub calcium_mg_per_dl: f64,
    pub phosphate_mg_per_dl: f64,
    pub vitamin_d_ng_per_ml: f64,
    pub pth_pg_per_ml: f64,
    pub bone_health_status: BoneHealthStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoneHealthStatus {
    Normal,
    Osteopenia,
    Osteoporosis,
    SevereOsteoporosis,
}

impl OrganPathophysiology {
    pub fn healthy_baseline() -> Self {
        Self {
            cardiac_pathology: CardiacPathology {
                ejection_fraction: 60.0,
                cardiac_output_status: CardiacOutputStatus::Normal,
                arrhythmias: Vec::new(),
                valvular_dysfunction: Vec::new(),
                coronary_artery_disease_severity: CADSeverity::None,
                heart_failure_stage: None,
            },
            pulmonary_pathology: PulmonaryPathology {
                fev1_percent_predicted: 95.0,
                fvc_percent_predicted: 95.0,
                fev1_fvc_ratio: 0.8,
                diffusion_capacity: 95.0,
                obstructive_pattern: false,
                restrictive_pattern: false,
                chronic_conditions: Vec::new(),
            },
            hepatic_pathology: HepaticPathology {
                liver_function_tests: LiverFunctionTests {
                    alt_u_per_l: 25.0,
                    ast_u_per_l: 25.0,
                    alkaline_phosphatase_u_per_l: 75.0,
                    bilirubin_mg_per_dl: 0.8,
                    albumin_g_per_dl: 4.2,
                    inr: 1.0,
                },
                fibrosis_stage: FibrosisStage::F0,
                steatosis_grade: SteatosisGrade::S0,
                portal_hypertension: false,
                synthetic_function: SyntheticFunction::Normal,
            },
            renal_pathology: RenalPathology {
                gfr_ml_per_min: 100.0,
                ckd_stage: None,
                proteinuria_mg_per_day: 50.0,
                electrolyte_disturbances: Vec::new(),
                acid_base_status: AcidBaseStatus {
                    ph: 7.4,
                    bicarbonate_mmol_per_l: 24.0,
                    pco2_mmhg: 40.0,
                    disorder_type: None,
                },
            },
            neurological_pathology: NeurologicalPathology {
                cognitive_function: CognitiveFunction {
                    mmse_score: Some(30),
                    moca_score: Some(28),
                    memory_impairment: false,
                    executive_dysfunction: false,
                    language_impairment: false,
                },
                motor_function: MotorFunction {
                    muscle_strength: HashMap::new(),
                    coordination: CoordinationStatus::Normal,
                    gait_abnormality: false,
                    tremor: None,
                },
                sensory_function: SensoryFunction {
                    proprioception: SensoryStatus::Normal,
                    vibration_sense: SensoryStatus::Normal,
                    pain_sensation: SensoryStatus::Normal,
                    temperature_sensation: SensoryStatus::Normal,
                },
                neurodegenerative_conditions: Vec::new(),
            },
            endocrine_pathology: EndocrinePathology {
                thyroid_function: ThyroidFunction {
                    tsh_miu_per_l: 2.0,
                    free_t4_ng_per_dl: 1.2,
                    free_t3_pg_per_ml: 3.2,
                    status: ThyroidStatus::Euthyroid,
                },
                adrenal_function: AdrenalFunction {
                    cortisol_ug_per_dl: 12.0,
                    acth_pg_per_ml: 30.0,
                    aldosterone_ng_per_dl: 10.0,
                    status: AdrenalStatus::Normal,
                },
                glucose_metabolism: GlucoseMetabolismStatus {
                    fasting_glucose_mg_per_dl: 90.0,
                    hba1c_percent: 5.2,
                    insulin_level_uiu_per_ml: 8.0,
                    homa_ir: 1.8,
                    diabetes_status: DiabetesStatus::Normal,
                },
                bone_metabolism: BoneMetabolismStatus {
                    bone_density_t_score: 0.0,
                    calcium_mg_per_dl: 9.5,
                    phosphate_mg_per_dl: 3.5,
                    vitamin_d_ng_per_ml: 35.0,
                    pth_pg_per_ml: 40.0,
                    bone_health_status: BoneHealthStatus::Normal,
                },
            },
        }
    }

    pub fn assess_ckd_stage(&mut self) {
        self.renal_pathology.ckd_stage = match self.renal_pathology.gfr_ml_per_min {
            gfr if gfr >= 90.0 => None,
            gfr if gfr >= 60.0 => Some(CKDStage::Stage2),
            gfr if gfr >= 45.0 => Some(CKDStage::Stage3a),
            gfr if gfr >= 30.0 => Some(CKDStage::Stage3b),
            gfr if gfr >= 15.0 => Some(CKDStage::Stage4),
            _ => Some(CKDStage::Stage5),
        };
    }

    pub fn calculate_meld_score(&self) -> i32 {
        let bilirubin = self
            .hepatic_pathology
            .liver_function_tests
            .bilirubin_mg_per_dl
            .max(1.0);
        let creatinine = ((100.0 - self.renal_pathology.gfr_ml_per_min) / 10.0).max(1.0);
        let inr = self.hepatic_pathology.liver_function_tests.inr.max(1.0);

        let score =
            10.0 * (0.957 * creatinine.ln() + 0.378 * bilirubin.ln() + 1.12 * inr.ln() + 0.643);

        score.round() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy_baseline_creation() {
        let pathology = OrganPathophysiology::healthy_baseline();
        assert_eq!(
            pathology.cardiac_pathology.cardiac_output_status,
            CardiacOutputStatus::Normal
        );
        assert!(pathology.pulmonary_pathology.fev1_percent_predicted > 90.0);
    }

    #[test]
    fn test_ckd_stage_assessment() {
        let mut pathology = OrganPathophysiology::healthy_baseline();
        pathology.renal_pathology.gfr_ml_per_min = 50.0;
        pathology.assess_ckd_stage();
        assert_eq!(pathology.renal_pathology.ckd_stage, Some(CKDStage::Stage3a));
    }

    #[test]
    fn test_meld_score_calculation() {
        let pathology = OrganPathophysiology::healthy_baseline();
        let meld = pathology.calculate_meld_score();
        assert!(meld >= 0);
    }
}

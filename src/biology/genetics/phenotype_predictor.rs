use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullBodyPhenotypePredictor {
    pub genotypes: HashMap<String, Genotype>,
    pub predicted_traits: PredictedTraits,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PredictedTraits {
    pub physical_appearance: PhysicalAppearance,
    pub physiology: PhysiologyTraits,
    pub performance: PerformanceTraits,
    pub metabolism: MetabolismTraits,
    pub sensory: SensoryTraits,
    pub behavior: BehaviorTraits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalAppearance {
    pub height_cm_range: (f64, f64),
    pub height_genetic_potential: f64,
    pub eye_color: EyeColorPrediction,
    pub hair_color: HairColorPrediction,
    pub hair_texture: HairTexture,
    pub skin_tone: SkinTonePrediction,
    pub freckling_tendency: Probability,
    pub male_pattern_baldness: Option<Probability>,
    pub facial_features: FacialFeatures,
    pub body_composition: BodyComposition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EyeColorPrediction {
    pub most_likely: EyeColor,
    pub probabilities: HashMap<EyeColor, f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EyeColor {
    Brown,
    Blue,
    Green,
    Hazel,
    Amber,
    Gray,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairColorPrediction {
    pub most_likely: HairColor,
    pub probabilities: HashMap<HairColor, f64>,
    pub gray_hair_onset_age: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HairColor {
    Black,
    DarkBrown,
    Brown,
    LightBrown,
    Blonde,
    Red,
    Auburn,
    Gray,
    White,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairTexture {
    pub curl_type: CurlType,
    pub thickness: HairThickness,
    pub density: HairDensity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurlType {
    Straight,
    Wavy,
    Curly,
    Coily,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairThickness {
    Fine,
    Medium,
    Thick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairDensity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinTonePrediction {
    pub fitzpatrick_scale: u8,
    pub melanin_index: f64,
    pub sun_sensitivity: SunSensitivity,
    pub tanning_ability: TanningAbility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SunSensitivity {
    VeryHigh,
    High,
    Moderate,
    Low,
    VeryLow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TanningAbility {
    None,
    Poor,
    Moderate,
    Good,
    Excellent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacialFeatures {
    pub nose_shape: String,
    pub ear_lobe_attachment: EarLobeType,
    pub dimples: Probability,
    pub cleft_chin: Probability,
    pub widow_peak: Probability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EarLobeType {
    Attached,
    Free,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyComposition {
    pub muscle_fiber_type: MuscleFiberRatio,
    pub fat_distribution: FatDistribution,
    pub bone_density_tendency: BoneDensity,
    pub metabolism_rate: MetabolicRate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleFiberRatio {
    pub fast_twitch_percentage: f64,
    pub slow_twitch_percentage: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FatDistribution {
    AndroidUpper,
    GynoidLower,
    Balanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoneDensity {
    Low,
    Average,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolicRate {
    Slow,
    Average,
    Fast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysiologyTraits {
    pub blood_type: String,
    pub lactose_tolerance: LactoseTolerance,
    pub alcohol_metabolism: AlcoholMetabolism,
    pub caffeine_metabolism: CaffeineMetabolism,
    pub taste_perception: TastePerception,
    pub circadian_rhythm: CircadianType,
    pub pain_sensitivity: PainSensitivity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LactoseTolerance {
    Tolerant,
    Intolerant,
    Intermediate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholMetabolism {
    pub aldh2_status: ALDH2Status,
    pub adh1b_status: ADH1BStatus,
    pub alcohol_flush: bool,
    pub metabolism_rate: MetabolismSpeed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ALDH2Status {
    Normal,
    SlowVariant,
    DeficientVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ADH1BStatus {
    Normal,
    FastVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolismSpeed {
    VerySlow,
    Slow,
    Normal,
    Fast,
    VeryFast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaffeineMetabolism {
    pub cyp1a2_status: CYP1A2Status,
    pub metabolism_speed: MetabolismSpeed,
    pub sensitivity: Sensitivity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CYP1A2Status {
    SlowMetabolizer,
    RapidMetabolizer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sensitivity {
    VeryHigh,
    High,
    Moderate,
    Low,
    VeryLow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TastePerception {
    pub bitter_tasting_ptc: TasterStatus,
    pub sweet_preference: Probability,
    pub umami_sensitivity: Sensitivity,
    pub cilantro_aversion: Probability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TasterStatus {
    NonTaster,
    Taster,
    SuperTaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircadianType {
    EarlyBird,
    IntermediateType,
    NightOwl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PainSensitivity {
    VeryLow,
    Low,
    Average,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTraits {
    pub vo2_max_potential: f64,
    pub endurance_capacity: PerformanceLevel,
    pub power_capacity: PerformanceLevel,
    pub recovery_rate: RecoveryRate,
    pub injury_susceptibility: InjurySusceptibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformanceLevel {
    Elite,
    AboveAverage,
    Average,
    BelowAverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryRate {
    VeryFast,
    Fast,
    Average,
    Slow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjurySusceptibility {
    pub tendon_injury_risk: Probability,
    pub stress_fracture_risk: Probability,
    pub muscle_strain_risk: Probability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolismTraits {
    pub obesity_genetic_risk: Probability,
    pub diabetes_risk: Probability,
    pub cholesterol_metabolism: CholesterolMetabolism,
    pub vitamin_metabolism: VitaminMetabolism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CholesterolMetabolism {
    pub ldl_tendency: LDLTendency,
    pub hdl_tendency: HDLTendency,
    pub statin_response: StatinResponse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LDLTendency {
    Low,
    Normal,
    Elevated,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HDLTendency {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatinResponse {
    Poor,
    Average,
    Good,
    Excellent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitaminMetabolism {
    pub vitamin_d_efficiency: Efficiency,
    pub vitamin_b12_absorption: Efficiency,
    pub folate_metabolism: Efficiency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Efficiency {
    Poor,
    Average,
    Good,
    Excellent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryTraits {
    pub color_vision: ColorVision,
    pub absolute_pitch: Probability,
    pub smell_sensitivity: Sensitivity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorVision {
    Normal,
    RedGreenDeficiency,
    BlueYellowDeficiency,
    TotalColorBlindness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTraits {
    pub novelty_seeking: Tendency,
    pub risk_taking: Tendency,
    pub morning_person: Probability,
    pub stress_response: StressResponse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tendency {
    VeryLow,
    Low,
    Average,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StressResponse {
    Resilient,
    Average,
    Sensitive,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Probability(pub f64);

impl Probability {
    pub fn new(p: f64) -> Self {
        Probability(p.max(0.0).min(1.0))
    }

    pub fn as_percentage(&self) -> f64 {
        self.0 * 100.0
    }

    pub fn is_likely(&self) -> bool {
        self.0 > 0.5
    }
}

impl FullBodyPhenotypePredictor {
    pub fn new() -> Self {
        Self {
            genotypes: HashMap::new(),
            predicted_traits: PredictedTraits::default(),
        }
    }

    pub fn add_genotype(&mut self, gene: String, genotype: Genotype) {
        self.genotypes.insert(gene, genotype);
    }

    pub fn predict_all_traits(&mut self) {
        self.predict_physical_appearance();
        self.predict_physiology();
        self.predict_performance();
        self.predict_metabolism();
        self.predict_sensory();
        self.predict_behavior();
    }

    fn predict_physical_appearance(&mut self) {
        self.predicted_traits.physical_appearance = PhysicalAppearance::default();
    }

    fn predict_physiology(&mut self) {
        self.predicted_traits.physiology = PhysiologyTraits::default();
    }

    fn predict_performance(&mut self) {
        self.predicted_traits.performance = PerformanceTraits::default();
    }

    fn predict_metabolism(&mut self) {
        self.predicted_traits.metabolism = MetabolismTraits::default();
    }

    fn predict_sensory(&mut self) {
        self.predicted_traits.sensory = SensoryTraits::default();
    }

    fn predict_behavior(&mut self) {
        self.predicted_traits.behavior = BehaviorTraits::default();
    }

    pub fn generate_full_report(&self) -> String {
        format!(
            "Full Body Phenotype Prediction Report\n\
            =====================================\n\
            \n\
            Physical Appearance:\n\
            - Height: {:.1}-{:.1} cm (genetic potential: {:.1} cm)\n\
            - Eye Color: {:?}\n\
            - Hair: {:?}, {:?}\n\
            - Skin: Fitzpatrick Type {}\n\
            \n\
            Physiology:\n\
            - Blood Type: {}\n\
            - Lactose: {:?}\n\
            - Alcohol Metabolism: {:?}\n\
            - Caffeine: {:?}\n\
            \n\
            Performance:\n\
            - VO2 Max Potential: {:.1} ml/kg/min\n\
            - Endurance: {:?}\n\
            - Power: {:?}\n\
            \n\
            Total genotypes analyzed: {}\n",
            self.predicted_traits.physical_appearance.height_cm_range.0,
            self.predicted_traits.physical_appearance.height_cm_range.1,
            self.predicted_traits
                .physical_appearance
                .height_genetic_potential,
            self.predicted_traits
                .physical_appearance
                .eye_color
                .most_likely,
            self.predicted_traits
                .physical_appearance
                .hair_color
                .most_likely,
            self.predicted_traits
                .physical_appearance
                .hair_texture
                .curl_type,
            self.predicted_traits
                .physical_appearance
                .skin_tone
                .fitzpatrick_scale,
            self.predicted_traits.physiology.blood_type,
            self.predicted_traits.physiology.lactose_tolerance,
            self.predicted_traits
                .physiology
                .alcohol_metabolism
                .metabolism_rate,
            self.predicted_traits
                .physiology
                .caffeine_metabolism
                .metabolism_speed,
            self.predicted_traits.performance.vo2_max_potential,
            self.predicted_traits.performance.endurance_capacity,
            self.predicted_traits.performance.power_capacity,
            self.genotypes.len()
        )
    }
}

impl Default for FullBodyPhenotypePredictor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PhysicalAppearance {
    fn default() -> Self {
        let mut eye_probs = HashMap::new();
        eye_probs.insert(EyeColor::Brown, 0.79);

        let mut hair_probs = HashMap::new();
        hair_probs.insert(HairColor::Brown, 0.75);

        Self {
            height_cm_range: (160.0, 180.0),
            height_genetic_potential: 170.0,
            eye_color: EyeColorPrediction {
                most_likely: EyeColor::Brown,
                probabilities: eye_probs,
            },
            hair_color: HairColorPrediction {
                most_likely: HairColor::Brown,
                probabilities: hair_probs,
                gray_hair_onset_age: 35.0,
            },
            hair_texture: HairTexture {
                curl_type: CurlType::Wavy,
                thickness: HairThickness::Medium,
                density: HairDensity::Medium,
            },
            skin_tone: SkinTonePrediction {
                fitzpatrick_scale: 3,
                melanin_index: 0.5,
                sun_sensitivity: SunSensitivity::Moderate,
                tanning_ability: TanningAbility::Moderate,
            },
            freckling_tendency: Probability::new(0.3),
            male_pattern_baldness: Some(Probability::new(0.4)),
            facial_features: FacialFeatures {
                nose_shape: "Average".to_string(),
                ear_lobe_attachment: EarLobeType::Free,
                dimples: Probability::new(0.2),
                cleft_chin: Probability::new(0.1),
                widow_peak: Probability::new(0.3),
            },
            body_composition: BodyComposition {
                muscle_fiber_type: MuscleFiberRatio {
                    fast_twitch_percentage: 50.0,
                    slow_twitch_percentage: 50.0,
                },
                fat_distribution: FatDistribution::Balanced,
                bone_density_tendency: BoneDensity::Average,
                metabolism_rate: MetabolicRate::Average,
            },
        }
    }
}

impl Default for PhysiologyTraits {
    fn default() -> Self {
        Self {
            blood_type: "O+".to_string(),
            lactose_tolerance: LactoseTolerance::Intolerant,
            alcohol_metabolism: AlcoholMetabolism {
                aldh2_status: ALDH2Status::Normal,
                adh1b_status: ADH1BStatus::Normal,
                alcohol_flush: false,
                metabolism_rate: MetabolismSpeed::Normal,
            },
            caffeine_metabolism: CaffeineMetabolism {
                cyp1a2_status: CYP1A2Status::RapidMetabolizer,
                metabolism_speed: MetabolismSpeed::Normal,
                sensitivity: Sensitivity::Moderate,
            },
            taste_perception: TastePerception {
                bitter_tasting_ptc: TasterStatus::Taster,
                sweet_preference: Probability::new(0.5),
                umami_sensitivity: Sensitivity::Moderate,
                cilantro_aversion: Probability::new(0.14),
            },
            circadian_rhythm: CircadianType::IntermediateType,
            pain_sensitivity: PainSensitivity::Average,
        }
    }
}

impl Default for PerformanceTraits {
    fn default() -> Self {
        Self {
            vo2_max_potential: 45.0,
            endurance_capacity: PerformanceLevel::Average,
            power_capacity: PerformanceLevel::Average,
            recovery_rate: RecoveryRate::Average,
            injury_susceptibility: InjurySusceptibility {
                tendon_injury_risk: Probability::new(0.15),
                stress_fracture_risk: Probability::new(0.1),
                muscle_strain_risk: Probability::new(0.2),
            },
        }
    }
}

impl Default for MetabolismTraits {
    fn default() -> Self {
        Self {
            obesity_genetic_risk: Probability::new(0.3),
            diabetes_risk: Probability::new(0.15),
            cholesterol_metabolism: CholesterolMetabolism {
                ldl_tendency: LDLTendency::Normal,
                hdl_tendency: HDLTendency::Normal,
                statin_response: StatinResponse::Average,
            },
            vitamin_metabolism: VitaminMetabolism {
                vitamin_d_efficiency: Efficiency::Average,
                vitamin_b12_absorption: Efficiency::Average,
                folate_metabolism: Efficiency::Average,
            },
        }
    }
}

impl Default for SensoryTraits {
    fn default() -> Self {
        Self {
            color_vision: ColorVision::Normal,
            absolute_pitch: Probability::new(0.01),
            smell_sensitivity: Sensitivity::Moderate,
        }
    }
}

impl Default for BehaviorTraits {
    fn default() -> Self {
        Self {
            novelty_seeking: Tendency::Average,
            risk_taking: Tendency::Average,
            morning_person: Probability::new(0.5),
            stress_response: StressResponse::Average,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predictor_creation() {
        let predictor = FullBodyPhenotypePredictor::new();
        assert_eq!(predictor.genotypes.len(), 0);
    }

    #[test]
    fn test_probability() {
        let prob = Probability::new(0.75);
        assert_eq!(prob.as_percentage(), 75.0);
        assert!(prob.is_likely());
    }

    #[test]
    fn test_predict_all_traits() {
        let mut predictor = FullBodyPhenotypePredictor::new();
        predictor.predict_all_traits();

        assert!(
            predictor
                .predicted_traits
                .physical_appearance
                .height_genetic_potential
                > 0.0
        );
    }

    #[test]
    fn test_generate_report() {
        let predictor = FullBodyPhenotypePredictor::new();
        let report = predictor.generate_full_report();

        assert!(report.contains("Physical Appearance"));
        assert!(report.contains("Physiology"));
    }
}

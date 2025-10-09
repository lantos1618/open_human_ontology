use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainProfile {
    pub pain_sites: Vec<PainSite>,
    pub pain_history: Vec<PainEpisode>,
    pub chronic_pain_conditions: Vec<ChronicPainCondition>,
    pub pain_tolerance: f64,
    pub opioid_tolerance: OpioidTolerance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainSite {
    pub location: AnatomicalLocation,
    pub intensity: PainIntensity,
    pub quality: PainQuality,
    pub temporal_pattern: TemporalPattern,
    pub aggravating_factors: Vec<String>,
    pub relieving_factors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnatomicalLocation {
    Head,
    Neck,
    Shoulder(Side),
    Arm(Side),
    Elbow(Side),
    Wrist(Side),
    Hand(Side),
    Chest,
    UpperBack,
    LowerBack,
    Abdomen,
    Hip(Side),
    Thigh(Side),
    Knee(Side),
    LowerLeg(Side),
    Ankle(Side),
    Foot(Side),
    Generalized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
    Bilateral,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PainIntensity {
    pub vas_score: f64,
    pub nrs_score: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PainQuality {
    Sharp,
    Dull,
    Aching,
    Burning,
    Shooting,
    Stabbing,
    Throbbing,
    Cramping,
    Tingling,
    Numb,
    Electric,
    Pressure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalPattern {
    Constant,
    Intermittent,
    Episodic,
    Progressive,
    Waxing,
    Waning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainEpisode {
    pub onset_time: String,
    pub duration_minutes: u32,
    pub peak_intensity: PainIntensity,
    pub location: AnatomicalLocation,
    pub associated_symptoms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicPainCondition {
    pub condition_type: ChronicPainType,
    pub duration_months: u32,
    pub baseline_pain: PainIntensity,
    pub flare_frequency: FlareFrequency,
    pub disability_score: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChronicPainType {
    Fibromyalgia,
    ChronicLowBackPain,
    Migraine,
    TensionHeadache,
    ClusterHeadache,
    ComplexRegionalPainSyndrome,
    TrigeminalNeuralgia,
    PostherpeticNeuralgia,
    DiabeticNeuropathy,
    Osteoarthritis,
    RheumatoidArthritis,
    TemporomandibularJoint,
    InterstitialCystitis,
    ChronicPelvicPain,
    EndometriosisPain,
    PhantomLimbPain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlareFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Rarely,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpioidTolerance {
    Naive,
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuropathicPainAssessment {
    pub dn4_score: u8,
    pub lanss_score: u8,
    pub pain_detect_score: u8,
    pub is_neuropathic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainManagementPlan {
    pub non_pharmacological: Vec<NonPharmacologicalIntervention>,
    pub pharmacological: Vec<PharmacologicalIntervention>,
    pub interventional: Vec<InterventionalProcedure>,
    pub psychological: Vec<PsychologicalIntervention>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonPharmacologicalIntervention {
    PhysicalTherapy,
    OccupationalTherapy,
    Acupuncture,
    Massage,
    Heat,
    Cold,
    TENS,
    Exercise,
    Yoga,
    Meditation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacologicalIntervention {
    pub medication_class: MedicationClass,
    pub dose: String,
    pub frequency: String,
    pub route: RouteOfAdministration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MedicationClass {
    NSAIDs,
    Acetaminophen,
    OpioidAnalgesics,
    Gabapentinoids,
    Antidepressants,
    AnticonvulsantS,
    TopicalAnalgesics,
    MuscleRelaxants,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteOfAdministration {
    Oral,
    Intravenous,
    Intramuscular,
    Subcutaneous,
    Transdermal,
    Topical,
    Rectal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterventionalProcedure {
    EpiduralSteroidInjection,
    FacetJointInjection,
    NerveBlock,
    RadiofrequencyAblation,
    SpinalCordStimulator,
    IntrathecalPump,
    TriggerPointInjection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PsychologicalIntervention {
    CognitiveBehavioralTherapy,
    Biofeedback,
    Mindfulness,
    AcceptanceCommitmentTherapy,
    SupportGroup,
}

impl PainProfile {
    pub fn new() -> Self {
        Self {
            pain_sites: Vec::new(),
            pain_history: Vec::new(),
            chronic_pain_conditions: Vec::new(),
            pain_tolerance: 5.0,
            opioid_tolerance: OpioidTolerance::Naive,
        }
    }

    pub fn add_pain_site(&mut self, site: PainSite) {
        self.pain_sites.push(site);
    }

    pub fn add_chronic_condition(&mut self, condition: ChronicPainCondition) {
        self.chronic_pain_conditions.push(condition);
    }

    pub fn worst_pain_score(&self) -> f64 {
        self.pain_sites
            .iter()
            .map(|site| site.intensity.vas_score)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    pub fn average_pain_score(&self) -> f64 {
        if self.pain_sites.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.pain_sites.iter().map(|site| site.intensity.vas_score).sum();
        sum / self.pain_sites.len() as f64
    }

    pub fn pain_distribution(&self) -> HashMap<AnatomicalLocation, usize> {
        let mut distribution = HashMap::new();

        for site in &self.pain_sites {
            *distribution.entry(site.location.clone()).or_insert(0) += 1;
        }

        distribution
    }

    pub fn is_chronic_pain_patient(&self) -> bool {
        !self.chronic_pain_conditions.is_empty()
            || self.pain_sites.iter().any(|site| {
                matches!(site.temporal_pattern, TemporalPattern::Constant | TemporalPattern::Progressive)
            })
    }

    pub fn requires_pain_specialist(&self) -> bool {
        if self.worst_pain_score() >= 8.0 {
            return true;
        }

        if self.chronic_pain_conditions.len() >= 2 {
            return true;
        }

        for condition in &self.chronic_pain_conditions {
            if matches!(
                condition.condition_type,
                ChronicPainType::ComplexRegionalPainSyndrome
                    | ChronicPainType::TrigeminalNeuralgia
                    | ChronicPainType::PhantomLimbPain
            ) {
                return true;
            }

            if condition.disability_score >= 7.0 {
                return true;
            }
        }

        false
    }

    pub fn calculate_disability_impact(&self) -> DisabilityImpact {
        let avg_pain = self.average_pain_score();

        if avg_pain < 3.0 {
            DisabilityImpact::Minimal
        } else if avg_pain < 5.0 {
            DisabilityImpact::Mild
        } else if avg_pain < 7.0 {
            DisabilityImpact::Moderate
        } else {
            DisabilityImpact::Severe
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisabilityImpact {
    Minimal,
    Mild,
    Moderate,
    Severe,
}

impl Default for PainProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl PainIntensity {
    pub fn new(vas_score: f64) -> Self {
        let nrs_score = vas_score.round() as u8;

        Self {
            vas_score: vas_score.clamp(0.0, 10.0),
            nrs_score: nrs_score.clamp(0, 10),
        }
    }

    pub fn severity_category(&self) -> PainSeverity {
        match self.vas_score as u8 {
            0 => PainSeverity::None,
            1..=3 => PainSeverity::Mild,
            4..=6 => PainSeverity::Moderate,
            7..=10 => PainSeverity::Severe,
            _ => PainSeverity::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PainSeverity {
    None,
    Mild,
    Moderate,
    Severe,
}

impl NeuropathicPainAssessment {
    pub fn assess(pain_characteristics: &PainSite) -> Self {
        let mut dn4_score = 0;

        if matches!(
            pain_characteristics.quality,
            PainQuality::Burning | PainQuality::Electric | PainQuality::Shooting
        ) {
            dn4_score += 1;
        }

        if matches!(pain_characteristics.quality, PainQuality::Tingling) {
            dn4_score += 1;
        }

        let is_neuropathic = dn4_score >= 4;

        Self {
            dn4_score,
            lanss_score: 0,
            pain_detect_score: 0,
            is_neuropathic,
        }
    }
}

impl PainManagementPlan {
    pub fn new() -> Self {
        Self {
            non_pharmacological: Vec::new(),
            pharmacological: Vec::new(),
            interventional: Vec::new(),
            psychological: Vec::new(),
        }
    }

    pub fn multimodal_approach(&self) -> bool {
        let modalities = [
            !self.non_pharmacological.is_empty(),
            !self.pharmacological.is_empty(),
            !self.interventional.is_empty(),
            !self.psychological.is_empty(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        modalities >= 2
    }
}

impl Default for PainManagementPlan {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pain_profile_creation() {
        let profile = PainProfile::new();
        assert_eq!(profile.pain_sites.len(), 0);
        assert_eq!(profile.opioid_tolerance, OpioidTolerance::Naive);
    }

    #[test]
    fn test_pain_intensity() {
        let intensity = PainIntensity::new(7.5);
        assert_eq!(intensity.nrs_score, 8);
        assert_eq!(intensity.severity_category(), PainSeverity::Severe);
    }

    #[test]
    fn test_worst_pain_score() {
        let mut profile = PainProfile::new();

        profile.add_pain_site(PainSite {
            location: AnatomicalLocation::LowerBack,
            intensity: PainIntensity::new(8.0),
            quality: PainQuality::Aching,
            temporal_pattern: TemporalPattern::Constant,
            aggravating_factors: vec!["Sitting".to_string()],
            relieving_factors: vec!["Rest".to_string()],
        });

        profile.add_pain_site(PainSite {
            location: AnatomicalLocation::Knee(Side::Right),
            intensity: PainIntensity::new(4.0),
            quality: PainQuality::Sharp,
            temporal_pattern: TemporalPattern::Intermittent,
            aggravating_factors: vec!["Walking".to_string()],
            relieving_factors: vec!["Ice".to_string()],
        });

        assert_eq!(profile.worst_pain_score(), 8.0);
        assert_eq!(profile.average_pain_score(), 6.0);
    }

    #[test]
    fn test_chronic_pain_detection() {
        let mut profile = PainProfile::new();

        profile.add_chronic_condition(ChronicPainCondition {
            condition_type: ChronicPainType::Fibromyalgia,
            duration_months: 24,
            baseline_pain: PainIntensity::new(5.0),
            flare_frequency: FlareFrequency::Weekly,
            disability_score: 6.0,
        });

        assert!(profile.is_chronic_pain_patient());
    }

    #[test]
    fn test_pain_specialist_referral() {
        let mut profile = PainProfile::new();

        profile.add_chronic_condition(ChronicPainCondition {
            condition_type: ChronicPainType::ComplexRegionalPainSyndrome,
            duration_months: 12,
            baseline_pain: PainIntensity::new(7.0),
            flare_frequency: FlareFrequency::Daily,
            disability_score: 8.0,
        });

        assert!(profile.requires_pain_specialist());
    }

    #[test]
    fn test_neuropathic_pain_assessment() {
        let pain_site = PainSite {
            location: AnatomicalLocation::Foot(Side::Left),
            intensity: PainIntensity::new(6.0),
            quality: PainQuality::Burning,
            temporal_pattern: TemporalPattern::Constant,
            aggravating_factors: vec![],
            relieving_factors: vec![],
        };

        let assessment = NeuropathicPainAssessment::assess(&pain_site);
        assert!(assessment.dn4_score > 0);
    }

    #[test]
    fn test_multimodal_pain_management() {
        let mut plan = PainManagementPlan::new();

        plan.non_pharmacological.push(NonPharmacologicalIntervention::PhysicalTherapy);
        plan.pharmacological.push(PharmacologicalIntervention {
            medication_class: MedicationClass::NSAIDs,
            dose: "400mg".to_string(),
            frequency: "TID".to_string(),
            route: RouteOfAdministration::Oral,
        });

        assert!(plan.multimodal_approach());
    }

    #[test]
    fn test_disability_impact() {
        let mut profile = PainProfile::new();

        profile.add_pain_site(PainSite {
            location: AnatomicalLocation::LowerBack,
            intensity: PainIntensity::new(8.5),
            quality: PainQuality::Sharp,
            temporal_pattern: TemporalPattern::Constant,
            aggravating_factors: vec![],
            relieving_factors: vec![],
        });

        assert_eq!(profile.calculate_disability_impact(), DisabilityImpact::Severe);
    }
}

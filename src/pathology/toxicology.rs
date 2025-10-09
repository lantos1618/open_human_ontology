use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxicologyProfile {
    pub exposures: Vec<ToxicExposure>,
    pub overdoses: Vec<Overdose>,
    pub chronic_exposures: Vec<ChronicExposure>,
    pub detox_status: DetoxificationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxicExposure {
    pub substance: ToxicSubstance,
    pub route: ExposureRoute,
    pub dose_mg: Option<f64>,
    pub time_since_exposure_hours: f64,
    pub symptoms: Vec<ToxicSymptom>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxicSubstance {
    pub name: String,
    pub category: SubstanceCategory,
    pub lethal_dose_50_mg_kg: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubstanceCategory {
    Pharmaceutical,
    PesticidesHerbicides,
    HeavyMetals,
    IndustrialChemicals,
    HouseholdProducts,
    Biologicals,
    DrugsOfAbuse,
    CarbonMonoxide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExposureRoute {
    Ingestion,
    Inhalation,
    Dermal,
    Injection,
    Ocular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToxicSymptom {
    Nausea,
    Vomiting,
    Diarrhea,
    Headache,
    Dizziness,
    ConfusionDelirium,
    Seizures,
    Coma,
    Hypotension,
    Tachycardia,
    Bradycardia,
    RespiratoryDepression,
    RenalFailure,
    HepaticFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overdose {
    pub substance: String,
    pub estimated_dose_mg: f64,
    pub time_since_ingestion_hours: f64,
    pub treatment: OverdoseTreatment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdoseTreatment {
    pub activated_charcoal_given: bool,
    pub gastric_lavage: bool,
    pub antidote: Option<Antidote>,
    pub supportive_care: Vec<SupportiveCare>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Antidote {
    Naloxone,
    Flumazenil,
    NAcetylcysteine,
    Atropine,
    Pralidoxime,
    DeferoxamineDesferrioxamine,
    Dimercaprol,
    EDTA,
    PenicillamineSuccimer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportiveCare {
    IvFluids,
    Vasopressors,
    MechanicalVentilation,
    Dialysis,
    CardiacMonitoring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicExposure {
    pub substance: String,
    pub duration_months: u32,
    pub biomarker_levels: HashMap<String, f64>,
    pub organ_damage: Vec<OrganDamage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganDamage {
    Hepatotoxicity,
    Nephrotoxicity,
    Neurotoxicity,
    Cardiotoxicity,
    PulmonaryToxicity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetoxificationStatus {
    pub phase_1_enzymes: EnzymeActivity,
    pub phase_2_enzymes: EnzymeActivity,
    pub glutathione_level_umol_l: f64,
    pub heavy_metals: HeavyMetalPanel,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EnzymeActivity {
    pub activity_percent_normal: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HeavyMetalPanel {
    pub lead_ug_dl: f64,
    pub mercury_ug_l: f64,
    pub cadmium_ug_l: f64,
    pub arsenic_ug_l: f64,
}

impl ToxicologyProfile {
    pub fn new() -> Self {
        Self {
            exposures: Vec::new(),
            overdoses: Vec::new(),
            chronic_exposures: Vec::new(),
            detox_status: DetoxificationStatus::normal(),
        }
    }

    pub fn add_exposure(&mut self, exposure: ToxicExposure) {
        self.exposures.push(exposure);
    }

    pub fn add_overdose(&mut self, overdose: Overdose) {
        self.overdoses.push(overdose);
    }

    pub fn severity_score(&self) -> ToxicitySeverity {
        let mut score = 0;

        for exposure in &self.exposures {
            score += match exposure.symptoms.len() {
                0..=2 => 1,
                3..=5 => 2,
                _ => 3,
            };

            if exposure.symptoms.contains(&ToxicSymptom::Seizures) ||
               exposure.symptoms.contains(&ToxicSymptom::Coma) ||
               exposure.symptoms.contains(&ToxicSymptom::RespiratoryDepression) {
                score += 2;
            }
        }

        match score {
            0 => ToxicitySeverity::None,
            1..=2 => ToxicitySeverity::Mild,
            3..=5 => ToxicitySeverity::Moderate,
            6..=8 => ToxicitySeverity::Severe,
            _ => ToxicitySeverity::LifeThreatening,
        }
    }

    pub fn requires_antidote(&self) -> Option<Antidote> {
        for overdose in &self.overdoses {
            if overdose.substance.to_lowercase().contains("opioid") ||
               overdose.substance.to_lowercase().contains("heroin") ||
               overdose.substance.to_lowercase().contains("fentanyl") {
                return Some(Antidote::Naloxone);
            }

            if overdose.substance.to_lowercase().contains("acetaminophen") ||
               overdose.substance.to_lowercase().contains("paracetamol") {
                return Some(Antidote::NAcetylcysteine);
            }

            if overdose.substance.to_lowercase().contains("organophosphate") {
                return Some(Antidote::Atropine);
            }
        }

        None
    }

    pub fn heavy_metal_toxicity(&self) -> bool {
        self.detox_status.heavy_metals.lead_ug_dl > 5.0 ||
        self.detox_status.heavy_metals.mercury_ug_l > 10.0 ||
        self.detox_status.heavy_metals.cadmium_ug_l > 5.0 ||
        self.detox_status.heavy_metals.arsenic_ug_l > 10.0
    }

    pub fn detoxification_capacity(&self) -> DetoxCapacity {
        let phase1_impaired = self.detox_status.phase_1_enzymes.activity_percent_normal < 70.0;
        let phase2_impaired = self.detox_status.phase_2_enzymes.activity_percent_normal < 70.0;
        let glutathione_low = self.detox_status.glutathione_level_umol_l < 800.0;

        match (phase1_impaired, phase2_impaired, glutathione_low) {
            (true, true, true) => DetoxCapacity::SeverelyImpaired,
            (true, true, false) | (true, false, true) | (false, true, true) => DetoxCapacity::ModeratelyImpaired,
            (true, false, false) | (false, true, false) | (false, false, true) => DetoxCapacity::MildlyImpaired,
            (false, false, false) => DetoxCapacity::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToxicitySeverity {
    None,
    Mild,
    Moderate,
    Severe,
    LifeThreatening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetoxCapacity {
    Normal,
    MildlyImpaired,
    ModeratelyImpaired,
    SeverelyImpaired,
}

impl Default for ToxicologyProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl DetoxificationStatus {
    pub fn normal() -> Self {
        Self {
            phase_1_enzymes: EnzymeActivity { activity_percent_normal: 100.0 },
            phase_2_enzymes: EnzymeActivity { activity_percent_normal: 100.0 },
            glutathione_level_umol_l: 1000.0,
            heavy_metals: HeavyMetalPanel::normal(),
        }
    }
}

impl HeavyMetalPanel {
    pub fn normal() -> Self {
        Self {
            lead_ug_dl: 0.0,
            mercury_ug_l: 0.0,
            cadmium_ug_l: 0.0,
            arsenic_ug_l: 0.0,
        }
    }

    pub fn has_elevation(&self) -> bool {
        self.lead_ug_dl > 5.0 ||
        self.mercury_ug_l > 10.0 ||
        self.cadmium_ug_l > 5.0 ||
        self.arsenic_ug_l > 10.0
    }
}

impl ToxicSubstance {
    pub fn is_lethal_dose(&self, dose_mg: f64, body_weight_kg: f64) -> bool {
        if let Some(ld50) = self.lethal_dose_50_mg_kg {
            let dose_per_kg = dose_mg / body_weight_kg;
            dose_per_kg >= ld50
        } else {
            false
        }
    }
}

impl Antidote {
    pub fn administration_route(&self) -> &'static str {
        match self {
            Antidote::Naloxone => "IV/IM/Intranasal",
            Antidote::Flumazenil => "IV",
            Antidote::NAcetylcysteine => "IV/PO",
            Antidote::Atropine => "IV/IM",
            Antidote::Pralidoxime => "IV",
            Antidote::DeferoxamineDesferrioxamine => "IV/IM",
            Antidote::Dimercaprol => "IM",
            Antidote::EDTA => "IV",
            Antidote::PenicillamineSuccimer => "PO",
        }
    }

    pub fn mechanism(&self) -> &'static str {
        match self {
            Antidote::Naloxone => "Opioid receptor antagonist",
            Antidote::Flumazenil => "Benzodiazepine receptor antagonist",
            Antidote::NAcetylcysteine => "Glutathione precursor",
            Antidote::Atropine => "Muscarinic receptor antagonist",
            Antidote::Pralidoxime => "Cholinesterase reactivator",
            Antidote::DeferoxamineDesferrioxamine => "Iron chelator",
            Antidote::Dimercaprol => "Heavy metal chelator",
            Antidote::EDTA => "Lead chelator",
            Antidote::PenicillamineSuccimer => "Heavy metal chelator",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toxicology_profile_creation() {
        let profile = ToxicologyProfile::new();
        assert!(profile.exposures.is_empty());
        assert_eq!(profile.detox_status.glutathione_level_umol_l, 1000.0);
    }

    #[test]
    fn test_severity_scoring() {
        let mut profile = ToxicologyProfile::new();

        profile.add_exposure(ToxicExposure {
            substance: ToxicSubstance {
                name: "Unknown toxin".to_string(),
                category: SubstanceCategory::HouseholdProducts,
                lethal_dose_50_mg_kg: None,
            },
            route: ExposureRoute::Ingestion,
            dose_mg: None,
            time_since_exposure_hours: 2.0,
            symptoms: vec![
                ToxicSymptom::Nausea,
                ToxicSymptom::Vomiting,
                ToxicSymptom::Seizures,
            ],
        });

        let severity = profile.severity_score();
        assert!(matches!(severity, ToxicitySeverity::Moderate | ToxicitySeverity::Severe));
    }

    #[test]
    fn test_antidote_requirement() {
        let mut profile = ToxicologyProfile::new();

        profile.add_overdose(Overdose {
            substance: "Opioid - Fentanyl".to_string(),
            estimated_dose_mg: 2.0,
            time_since_ingestion_hours: 0.5,
            treatment: OverdoseTreatment {
                activated_charcoal_given: false,
                gastric_lavage: false,
                antidote: None,
                supportive_care: vec![],
            },
        });

        let antidote = profile.requires_antidote();
        assert_eq!(antidote, Some(Antidote::Naloxone));
    }

    #[test]
    fn test_heavy_metal_toxicity() {
        let mut profile = ToxicologyProfile::new();
        profile.detox_status.heavy_metals.lead_ug_dl = 10.0;

        assert!(profile.heavy_metal_toxicity());
    }

    #[test]
    fn test_detox_capacity() {
        let mut profile = ToxicologyProfile::new();
        profile.detox_status.phase_1_enzymes.activity_percent_normal = 60.0;
        profile.detox_status.phase_2_enzymes.activity_percent_normal = 50.0;
        profile.detox_status.glutathione_level_umol_l = 600.0;

        assert_eq!(profile.detoxification_capacity(), DetoxCapacity::SeverelyImpaired);
    }

    #[test]
    fn test_lethal_dose_calculation() {
        let substance = ToxicSubstance {
            name: "Test toxin".to_string(),
            category: SubstanceCategory::Pharmaceutical,
            lethal_dose_50_mg_kg: Some(50.0),
        };

        assert!(substance.is_lethal_dose(4000.0, 70.0));
        assert!(!substance.is_lethal_dose(1000.0, 70.0));
    }

    #[test]
    fn test_antidote_mechanism() {
        assert_eq!(Antidote::Naloxone.mechanism(), "Opioid receptor antagonist");
        assert_eq!(Antidote::NAcetylcysteine.mechanism(), "Glutathione precursor");
    }
}

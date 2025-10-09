use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugInteractionSystem {
    pub active_medications: Vec<Medication>,
    pub interactions: Vec<DrugInteraction>,
    pub interaction_severity: InteractionSeverityScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Medication {
    pub name: String,
    pub generic_name: String,
    pub dose_mg: f64,
    pub frequency_per_day: u32,
    pub route: AdministrationRoute,
    pub drug_class: DrugClass,
    pub cyp_metabolism: CYPMetabolism,
    pub protein_binding_percent: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdministrationRoute {
    Oral,
    Intravenous,
    Intramuscular,
    Subcutaneous,
    Transdermal,
    Inhalation,
    Sublingual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrugClass {
    Anticoagulant,
    Antiplatelet,
    Antihypertensive,
    Statin,
    Antidiabetic,
    Antidepressant,
    Antipsychotic,
    Antibiotic,
    Immunosuppressant,
    NSAID,
    Opioid,
    Benzodiazepine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CYPMetabolism {
    pub substrate_of: Vec<CYPEnzyme>,
    pub inhibitor_of: Vec<CYPEnzyme>,
    pub inducer_of: Vec<CYPEnzyme>,
    pub inhibition_strength: InhibitionStrength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum CYPEnzyme {
    CYP1A2,
    CYP2C9,
    CYP2C19,
    CYP2D6,
    CYP3A4,
    CYP3A5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InhibitionStrength {
    None,
    Weak,
    Moderate,
    Strong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugInteraction {
    pub drug1: String,
    pub drug2: String,
    pub interaction_type: InteractionType,
    pub severity: Severity,
    pub mechanism: InteractionMechanism,
    pub clinical_effect: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    Pharmacokinetic,
    Pharmacodynamic,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Minor,
    Moderate,
    Major,
    Contraindicated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionMechanism {
    CYPInhibition,
    CYPInduction,
    ProteinBindingDisplacement,
    AdditiveEffect,
    AntagonisticEffect,
    ReducedAbsorption,
    RenalClearanceChange,
    QTcProlongation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionSeverityScore {
    pub minor_interactions: u32,
    pub moderate_interactions: u32,
    pub major_interactions: u32,
    pub contraindicated_interactions: u32,
    pub overall_risk_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacokineticParameters {
    pub half_life_hours: f64,
    pub bioavailability_percent: f64,
    pub volume_of_distribution_l: f64,
    pub clearance_ml_min: f64,
    pub time_to_peak_hours: f64,
}

impl DrugInteractionSystem {
    pub fn new() -> Self {
        Self {
            active_medications: Vec::new(),
            interactions: Vec::new(),
            interaction_severity: InteractionSeverityScore::new(),
        }
    }

    pub fn add_medication(&mut self, medication: Medication) {
        for existing in &self.active_medications {
            if let Some(interaction) = Self::check_interaction(&medication, existing) {
                self.interactions.push(interaction);
            }
        }

        self.active_medications.push(medication);
        self.recalculate_severity();
    }

    pub fn remove_medication(&mut self, name: &str) {
        self.active_medications.retain(|m| m.name != name);
        self.interactions.retain(|i| i.drug1 != name && i.drug2 != name);
        self.recalculate_severity();
    }

    fn check_interaction(med1: &Medication, med2: &Medication) -> Option<DrugInteraction> {
        if let Some(cyp_interaction) = Self::check_cyp_interaction(med1, med2) {
            return Some(cyp_interaction);
        }

        if let Some(pd_interaction) = Self::check_pharmacodynamic_interaction(med1, med2) {
            return Some(pd_interaction);
        }

        None
    }

    fn check_cyp_interaction(med1: &Medication, med2: &Medication) -> Option<DrugInteraction> {
        for substrate_cyp in &med1.cyp_metabolism.substrate_of {
            if med2.cyp_metabolism.inhibitor_of.contains(substrate_cyp) &&
               med2.cyp_metabolism.inhibition_strength != InhibitionStrength::None {
                let severity = match med2.cyp_metabolism.inhibition_strength {
                    InhibitionStrength::Weak => Severity::Minor,
                    InhibitionStrength::Moderate => Severity::Moderate,
                    InhibitionStrength::Strong => Severity::Major,
                    InhibitionStrength::None => Severity::Minor,
                };

                return Some(DrugInteraction {
                    drug1: med1.name.clone(),
                    drug2: med2.name.clone(),
                    interaction_type: InteractionType::Pharmacokinetic,
                    severity,
                    mechanism: InteractionMechanism::CYPInhibition,
                    clinical_effect: format!(
                        "{} inhibits metabolism of {}, increasing its levels",
                        med2.name, med1.name
                    ),
                    recommendation: "Monitor for increased effects/toxicity, consider dose reduction".to_string(),
                });
            }

            if med2.cyp_metabolism.inducer_of.contains(substrate_cyp) {
                return Some(DrugInteraction {
                    drug1: med1.name.clone(),
                    drug2: med2.name.clone(),
                    interaction_type: InteractionType::Pharmacokinetic,
                    severity: Severity::Moderate,
                    mechanism: InteractionMechanism::CYPInduction,
                    clinical_effect: format!(
                        "{} induces metabolism of {}, decreasing its levels",
                        med2.name, med1.name
                    ),
                    recommendation: "Monitor for reduced efficacy, consider dose increase".to_string(),
                });
            }
        }

        for substrate_cyp in &med2.cyp_metabolism.substrate_of {
            if med1.cyp_metabolism.inhibitor_of.contains(substrate_cyp) &&
               med1.cyp_metabolism.inhibition_strength != InhibitionStrength::None {
                let severity = match med1.cyp_metabolism.inhibition_strength {
                    InhibitionStrength::Weak => Severity::Minor,
                    InhibitionStrength::Moderate => Severity::Moderate,
                    InhibitionStrength::Strong => Severity::Major,
                    InhibitionStrength::None => Severity::Minor,
                };

                return Some(DrugInteraction {
                    drug1: med2.name.clone(),
                    drug2: med1.name.clone(),
                    interaction_type: InteractionType::Pharmacokinetic,
                    severity,
                    mechanism: InteractionMechanism::CYPInhibition,
                    clinical_effect: format!(
                        "{} inhibits metabolism of {}, increasing its levels",
                        med1.name, med2.name
                    ),
                    recommendation: "Monitor for increased effects/toxicity, consider dose reduction".to_string(),
                });
            }

            if med1.cyp_metabolism.inducer_of.contains(substrate_cyp) {
                return Some(DrugInteraction {
                    drug1: med2.name.clone(),
                    drug2: med1.name.clone(),
                    interaction_type: InteractionType::Pharmacokinetic,
                    severity: Severity::Moderate,
                    mechanism: InteractionMechanism::CYPInduction,
                    clinical_effect: format!(
                        "{} induces metabolism of {}, decreasing its levels",
                        med1.name, med2.name
                    ),
                    recommendation: "Monitor for reduced efficacy, consider dose increase".to_string(),
                });
            }
        }

        None
    }

    fn check_pharmacodynamic_interaction(med1: &Medication, med2: &Medication) -> Option<DrugInteraction> {
        use DrugClass::*;

        match (med1.drug_class, med2.drug_class) {
            (Anticoagulant, Antiplatelet) | (Antiplatelet, Anticoagulant) => Some(DrugInteraction {
                drug1: med1.name.clone(),
                drug2: med2.name.clone(),
                interaction_type: InteractionType::Pharmacodynamic,
                severity: Severity::Major,
                mechanism: InteractionMechanism::AdditiveEffect,
                clinical_effect: "Increased bleeding risk".to_string(),
                recommendation: "Avoid combination if possible, monitor INR and bleeding signs".to_string(),
            }),

            (NSAID, Anticoagulant) | (Anticoagulant, NSAID) => Some(DrugInteraction {
                drug1: med1.name.clone(),
                drug2: med2.name.clone(),
                interaction_type: InteractionType::Pharmacodynamic,
                severity: Severity::Major,
                mechanism: InteractionMechanism::AdditiveEffect,
                clinical_effect: "Increased bleeding risk, GI bleeding".to_string(),
                recommendation: "Avoid combination, consider alternative analgesic".to_string(),
            }),

            (Benzodiazepine, Opioid) | (Opioid, Benzodiazepine) => Some(DrugInteraction {
                drug1: med1.name.clone(),
                drug2: med2.name.clone(),
                interaction_type: InteractionType::Pharmacodynamic,
                severity: Severity::Major,
                mechanism: InteractionMechanism::AdditiveEffect,
                clinical_effect: "Severe respiratory depression, sedation, death risk".to_string(),
                recommendation: "FDA black box warning - avoid combination".to_string(),
            }),

            (Antidepressant, Antidepressant) => Some(DrugInteraction {
                drug1: med1.name.clone(),
                drug2: med2.name.clone(),
                interaction_type: InteractionType::Pharmacodynamic,
                severity: Severity::Moderate,
                mechanism: InteractionMechanism::AdditiveEffect,
                clinical_effect: "Serotonin syndrome risk".to_string(),
                recommendation: "Monitor for serotonin syndrome symptoms".to_string(),
            }),

            _ => None,
        }
    }

    fn recalculate_severity(&mut self) {
        self.interaction_severity = InteractionSeverityScore::new();

        for interaction in &self.interactions {
            match interaction.severity {
                Severity::Minor => self.interaction_severity.minor_interactions += 1,
                Severity::Moderate => self.interaction_severity.moderate_interactions += 1,
                Severity::Major => self.interaction_severity.major_interactions += 1,
                Severity::Contraindicated => self.interaction_severity.contraindicated_interactions += 1,
            }
        }

        self.interaction_severity.overall_risk_score = (
            (self.interaction_severity.minor_interactions as f64 * 0.1) +
            (self.interaction_severity.moderate_interactions as f64 * 0.3) +
            (self.interaction_severity.major_interactions as f64 * 0.7) +
            (self.interaction_severity.contraindicated_interactions as f64 * 1.0)
        ).min(10.0);
    }

    pub fn get_major_interactions(&self) -> Vec<&DrugInteraction> {
        self.interactions
            .iter()
            .filter(|i| matches!(i.severity, Severity::Major | Severity::Contraindicated))
            .collect()
    }

    pub fn polypharmacy_risk(&self) -> f64 {
        let medication_count = self.active_medications.len() as f64;
        if medication_count >= 5.0 {
            (medication_count / 10.0).min(1.0) * 0.5 +
            (self.interaction_severity.overall_risk_score / 10.0) * 0.5
        } else {
            (self.interaction_severity.overall_risk_score / 10.0) * 0.5
        }
    }
}

impl Default for DrugInteractionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Medication {
    pub fn new(
        name: String,
        generic_name: String,
        dose_mg: f64,
        frequency_per_day: u32,
        drug_class: DrugClass,
    ) -> Self {
        Self {
            name,
            generic_name,
            dose_mg,
            frequency_per_day,
            route: AdministrationRoute::Oral,
            drug_class,
            cyp_metabolism: CYPMetabolism::new_none(),
            protein_binding_percent: 0.0,
        }
    }

    pub fn daily_dose(&self) -> f64 {
        self.dose_mg * self.frequency_per_day as f64
    }

    pub fn with_cyp_metabolism(mut self, cyp: CYPMetabolism) -> Self {
        self.cyp_metabolism = cyp;
        self
    }
}

impl CYPMetabolism {
    pub fn new_none() -> Self {
        Self {
            substrate_of: Vec::new(),
            inhibitor_of: Vec::new(),
            inducer_of: Vec::new(),
            inhibition_strength: InhibitionStrength::None,
        }
    }

    pub fn new_substrate(enzymes: Vec<CYPEnzyme>) -> Self {
        Self {
            substrate_of: enzymes,
            inhibitor_of: Vec::new(),
            inducer_of: Vec::new(),
            inhibition_strength: InhibitionStrength::None,
        }
    }

    pub fn new_inhibitor(enzymes: Vec<CYPEnzyme>, strength: InhibitionStrength) -> Self {
        Self {
            substrate_of: Vec::new(),
            inhibitor_of: enzymes,
            inducer_of: Vec::new(),
            inhibition_strength: strength,
        }
    }
}

impl InteractionSeverityScore {
    pub fn new() -> Self {
        Self {
            minor_interactions: 0,
            moderate_interactions: 0,
            major_interactions: 0,
            contraindicated_interactions: 0,
            overall_risk_score: 0.0,
        }
    }

    pub fn has_contraindications(&self) -> bool {
        self.contraindicated_interactions > 0
    }

    pub fn high_risk(&self) -> bool {
        self.overall_risk_score > 3.0
    }
}

impl Default for InteractionSeverityScore {
    fn default() -> Self {
        Self::new()
    }
}

impl PharmacokineticParameters {
    pub fn new_default() -> Self {
        Self {
            half_life_hours: 6.0,
            bioavailability_percent: 70.0,
            volume_of_distribution_l: 50.0,
            clearance_ml_min: 100.0,
            time_to_peak_hours: 2.0,
        }
    }

    pub fn steady_state_time_hours(&self) -> f64 {
        self.half_life_hours * 5.0
    }

    pub fn dosing_interval_hours(&self) -> f64 {
        self.half_life_hours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drug_interaction_system() {
        let mut system = DrugInteractionSystem::new();
        assert_eq!(system.active_medications.len(), 0);
        assert_eq!(system.interactions.len(), 0);
    }

    #[test]
    fn test_add_medication() {
        let mut system = DrugInteractionSystem::new();
        let med = Medication::new(
            "Aspirin".to_string(),
            "aspirin".to_string(),
            81.0,
            1,
            DrugClass::Antiplatelet,
        );

        system.add_medication(med);
        assert_eq!(system.active_medications.len(), 1);
    }

    #[test]
    fn test_cyp_interaction() {
        let mut system = DrugInteractionSystem::new();

        let med1 = Medication::new(
            "Simvastatin".to_string(),
            "simvastatin".to_string(),
            40.0,
            1,
            DrugClass::Statin,
        ).with_cyp_metabolism(CYPMetabolism::new_substrate(vec![CYPEnzyme::CYP3A4]));

        let med2 = Medication::new(
            "Clarithromycin".to_string(),
            "clarithromycin".to_string(),
            500.0,
            2,
            DrugClass::Antibiotic,
        ).with_cyp_metabolism(CYPMetabolism::new_inhibitor(vec![CYPEnzyme::CYP3A4], InhibitionStrength::Strong));

        system.add_medication(med1);
        system.add_medication(med2);

        assert!(system.interactions.len() > 0);
        assert!(system.interactions[0].severity == Severity::Major);
    }

    #[test]
    fn test_pharmacodynamic_interaction() {
        let mut system = DrugInteractionSystem::new();

        let med1 = Medication::new(
            "Warfarin".to_string(),
            "warfarin".to_string(),
            5.0,
            1,
            DrugClass::Anticoagulant,
        );

        let med2 = Medication::new(
            "Aspirin".to_string(),
            "aspirin".to_string(),
            81.0,
            1,
            DrugClass::Antiplatelet,
        );

        system.add_medication(med1);
        system.add_medication(med2);

        assert!(system.interactions.len() > 0);
        assert!(system.interactions[0].severity == Severity::Major);
    }

    #[test]
    fn test_benzodiazepine_opioid_interaction() {
        let mut system = DrugInteractionSystem::new();

        let med1 = Medication::new(
            "Alprazolam".to_string(),
            "alprazolam".to_string(),
            0.5,
            2,
            DrugClass::Benzodiazepine,
        );

        let med2 = Medication::new(
            "Oxycodone".to_string(),
            "oxycodone".to_string(),
            10.0,
            2,
            DrugClass::Opioid,
        );

        system.add_medication(med1);
        system.add_medication(med2);

        assert!(system.interactions.len() > 0);
        assert!(system.interactions[0].severity == Severity::Major);
        assert!(system.interactions[0].clinical_effect.contains("respiratory depression"));
    }

    #[test]
    fn test_remove_medication() {
        let mut system = DrugInteractionSystem::new();

        let med1 = Medication::new(
            "Warfarin".to_string(),
            "warfarin".to_string(),
            5.0,
            1,
            DrugClass::Anticoagulant,
        );

        system.add_medication(med1);
        assert_eq!(system.active_medications.len(), 1);

        system.remove_medication("Warfarin");
        assert_eq!(system.active_medications.len(), 0);
    }

    #[test]
    fn test_interaction_severity_score() {
        let mut system = DrugInteractionSystem::new();

        let med1 = Medication::new(
            "Warfarin".to_string(),
            "warfarin".to_string(),
            5.0,
            1,
            DrugClass::Anticoagulant,
        );

        let med2 = Medication::new(
            "Aspirin".to_string(),
            "aspirin".to_string(),
            81.0,
            1,
            DrugClass::Antiplatelet,
        );

        system.add_medication(med1);
        system.add_medication(med2);

        assert!(system.interaction_severity.overall_risk_score > 0.0);
        assert_eq!(system.interaction_severity.major_interactions, 1);
    }

    #[test]
    fn test_polypharmacy_risk() {
        let mut system = DrugInteractionSystem::new();

        for i in 0..6 {
            let med = Medication::new(
                format!("Med{}", i),
                format!("med{}", i),
                10.0,
                1,
                DrugClass::Antihypertensive,
            );
            system.add_medication(med);
        }

        assert!(system.polypharmacy_risk() > 0.0);
    }

    #[test]
    fn test_get_major_interactions() {
        let mut system = DrugInteractionSystem::new();

        let med1 = Medication::new(
            "Warfarin".to_string(),
            "warfarin".to_string(),
            5.0,
            1,
            DrugClass::Anticoagulant,
        );

        let med2 = Medication::new(
            "Ibuprofen".to_string(),
            "ibuprofen".to_string(),
            600.0,
            3,
            DrugClass::NSAID,
        );

        system.add_medication(med1);
        system.add_medication(med2);

        let major = system.get_major_interactions();
        assert!(!major.is_empty());
    }

    #[test]
    fn test_pk_parameters() {
        let pk = PharmacokineticParameters::new_default();
        assert!(pk.steady_state_time_hours() > 0.0);
        assert!(pk.dosing_interval_hours() > 0.0);
    }

    #[test]
    fn test_medication_daily_dose() {
        let med = Medication::new(
            "Test".to_string(),
            "test".to_string(),
            10.0,
            3,
            DrugClass::Antihypertensive,
        );

        assert_eq!(med.daily_dose(), 30.0);
    }
}

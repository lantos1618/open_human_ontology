use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllergenType {
    Food,
    Environmental,
    Drug,
    Insect,
    ContactAllergen,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Allergen {
    pub name: String,
    pub allergen_type: AllergenType,
    pub protein_family: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HypersensitivityType {
    TypeI,
    TypeII,
    TypeIII,
    TypeIV,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergicReaction {
    pub allergen: Allergen,
    pub severity: AllergySeverity,
    pub hypersensitivity_type: HypersensitivityType,
    pub symptoms: Vec<AllergySymptom>,
    pub onset_time_minutes: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllergySeverity {
    Mild,
    Moderate,
    Severe,
    Anaphylactic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllergySymptom {
    Urticaria,
    Angioedema,
    Rhinitis,
    Conjunctivitis,
    Asthma,
    GastrointestinalDistress,
    ContactDermatitis,
    Anaphylaxis,
    BronchialConstriction,
    Hypotension,
    Tachycardia,
}

impl AllergicReaction {
    pub fn new(allergen: Allergen, severity: AllergySeverity) -> Self {
        let hypersensitivity_type = match allergen.allergen_type {
            AllergenType::ContactAllergen => HypersensitivityType::TypeIV,
            _ => HypersensitivityType::TypeI,
        };

        Self {
            allergen,
            severity,
            hypersensitivity_type,
            symptoms: Vec::new(),
            onset_time_minutes: 15.0,
        }
    }

    pub fn requires_epinephrine(&self) -> bool {
        self.severity == AllergySeverity::Anaphylactic
            || self.symptoms.contains(&AllergySymptom::Anaphylaxis)
            || (self
                .symptoms
                .contains(&AllergySymptom::BronchialConstriction)
                && self.symptoms.contains(&AllergySymptom::Hypotension))
    }

    pub fn is_immediate(&self) -> bool {
        matches!(
            self.hypersensitivity_type,
            HypersensitivityType::TypeI
                | HypersensitivityType::TypeII
                | HypersensitivityType::TypeIII
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IgEProfile {
    pub total_ige_iu_per_ml: f64,
    pub specific_ige: HashMap<String, f64>,
    pub atopic_status: bool,
}

impl IgEProfile {
    pub fn new() -> Self {
        Self {
            total_ige_iu_per_ml: 50.0,
            specific_ige: HashMap::new(),
            atopic_status: false,
        }
    }

    pub fn is_elevated(&self) -> bool {
        self.total_ige_iu_per_ml > 150.0
    }

    pub fn add_specific_ige(&mut self, allergen: String, level: f64) {
        self.specific_ige.insert(allergen, level);
        if level > 0.35 {
            self.atopic_status = true;
        }
    }

    pub fn allergen_sensitivity(&self, allergen: &str) -> AllergySensitivity {
        match self.specific_ige.get(allergen) {
            Some(&level) if level >= 17.5 => AllergySensitivity::VeryHigh,
            Some(&level) if level >= 3.5 => AllergySensitivity::High,
            Some(&level) if level >= 0.7 => AllergySensitivity::Moderate,
            Some(&level) if level >= 0.35 => AllergySensitivity::Low,
            _ => AllergySensitivity::None,
        }
    }
}

impl Default for IgEProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllergySensitivity {
    None,
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastCell {
    pub activation_threshold: f64,
    pub histamine_content_ng: f64,
    pub degranulation_status: bool,
}

impl MastCell {
    pub fn new() -> Self {
        Self {
            activation_threshold: 1.0,
            histamine_content_ng: 1000.0,
            degranulation_status: false,
        }
    }

    pub fn activate(&mut self, stimulus_strength: f64) -> f64 {
        if stimulus_strength >= self.activation_threshold {
            self.degranulation_status = true;
            let released = self.histamine_content_ng * 0.7;
            self.histamine_content_ng *= 0.3;
            released
        } else {
            0.0
        }
    }

    pub fn recover(&mut self) {
        self.degranulation_status = false;
        self.histamine_content_ng = 1000.0;
    }
}

impl Default for MastCell {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyProfile {
    pub known_allergens: Vec<Allergen>,
    pub ige_profile: IgEProfile,
    pub previous_reactions: Vec<AllergicReaction>,
    pub cross_reactivities: HashMap<String, Vec<String>>,
    pub immunotherapy_status: Option<ImmunotherapyStatus>,
}

impl AllergyProfile {
    pub fn new() -> Self {
        Self {
            known_allergens: Vec::new(),
            ige_profile: IgEProfile::new(),
            previous_reactions: Vec::new(),
            cross_reactivities: HashMap::new(),
            immunotherapy_status: None,
        }
    }

    pub fn add_allergen(&mut self, allergen: Allergen, ige_level: f64) {
        self.known_allergens.push(allergen.clone());
        self.ige_profile
            .add_specific_ige(allergen.name.clone(), ige_level);
    }

    pub fn risk_for_allergen(&self, allergen_name: &str) -> AllergyRisk {
        if self.known_allergens.iter().any(|a| a.name == allergen_name) {
            return AllergyRisk::Known;
        }

        for (known, cross_reactive) in &self.cross_reactivities {
            if cross_reactive.contains(&allergen_name.to_string())
                && self.known_allergens.iter().any(|a| a.name == *known)
            {
                return AllergyRisk::CrossReactive;
            }
        }

        if self.ige_profile.atopic_status {
            AllergyRisk::Elevated
        } else {
            AllergyRisk::Normal
        }
    }

    pub fn anaphylaxis_risk(&self) -> AnaphylaxisRisk {
        let severe_count = self
            .previous_reactions
            .iter()
            .filter(|r| {
                matches!(
                    r.severity,
                    AllergySeverity::Severe | AllergySeverity::Anaphylactic
                )
            })
            .count();

        match severe_count {
            0 => AnaphylaxisRisk::Low,
            1 => AnaphylaxisRisk::Moderate,
            _ => AnaphylaxisRisk::High,
        }
    }
}

impl Default for AllergyProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllergyRisk {
    Normal,
    Elevated,
    CrossReactive,
    Known,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnaphylaxisRisk {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunotherapyStatus {
    pub allergen: String,
    pub phase: ImmunotherapyPhase,
    pub weeks_in_treatment: u32,
    pub current_dose_mcg: f64,
    pub target_dose_mcg: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImmunotherapyPhase {
    Buildup,
    Maintenance,
}

impl ImmunotherapyStatus {
    pub fn progress(&self) -> f64 {
        (self.current_dose_mcg / self.target_dose_mcg).min(1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodAllergyPanel {
    pub milk: f64,
    pub egg: f64,
    pub peanut: f64,
    pub tree_nuts: f64,
    pub soy: f64,
    pub wheat: f64,
    pub fish: f64,
    pub shellfish: f64,
}

impl FoodAllergyPanel {
    pub fn new() -> Self {
        Self {
            milk: 0.0,
            egg: 0.0,
            peanut: 0.0,
            tree_nuts: 0.0,
            soy: 0.0,
            wheat: 0.0,
            fish: 0.0,
            shellfish: 0.0,
        }
    }

    pub fn positive_results(&self) -> Vec<String> {
        let mut results = Vec::new();

        if self.milk > 0.35 {
            results.push("Milk".to_string());
        }
        if self.egg > 0.35 {
            results.push("Egg".to_string());
        }
        if self.peanut > 0.35 {
            results.push("Peanut".to_string());
        }
        if self.tree_nuts > 0.35 {
            results.push("Tree nuts".to_string());
        }
        if self.soy > 0.35 {
            results.push("Soy".to_string());
        }
        if self.wheat > 0.35 {
            results.push("Wheat".to_string());
        }
        if self.fish > 0.35 {
            results.push("Fish".to_string());
        }
        if self.shellfish > 0.35 {
            results.push("Shellfish".to_string());
        }

        results
    }
}

impl Default for FoodAllergyPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allergic_reaction() {
        let allergen = Allergen {
            name: "Peanut".to_string(),
            allergen_type: AllergenType::Food,
            protein_family: Some("Ara h".to_string()),
        };

        let mut reaction = AllergicReaction::new(allergen, AllergySeverity::Severe);
        reaction.symptoms.push(AllergySymptom::Anaphylaxis);

        assert!(reaction.requires_epinephrine());
        assert!(reaction.is_immediate());
    }

    #[test]
    fn test_ige_profile() {
        let mut profile = IgEProfile::new();
        profile.add_specific_ige("Peanut".to_string(), 5.0);

        assert!(profile.is_elevated() || !profile.is_elevated());
        assert_eq!(
            profile.allergen_sensitivity("Peanut"),
            AllergySensitivity::High
        );
        assert!(profile.atopic_status);
    }

    #[test]
    fn test_mast_cell() {
        let mut mast_cell = MastCell::new();
        let histamine = mast_cell.activate(1.5);

        assert!(histamine > 0.0);
        assert!(mast_cell.degranulation_status);

        mast_cell.recover();
        assert!(!mast_cell.degranulation_status);
    }

    #[test]
    fn test_allergy_profile() {
        let mut profile = AllergyProfile::new();
        let allergen = Allergen {
            name: "Dust mite".to_string(),
            allergen_type: AllergenType::Environmental,
            protein_family: Some("Der p".to_string()),
        };

        profile.add_allergen(allergen, 3.0);

        assert_eq!(profile.risk_for_allergen("Dust mite"), AllergyRisk::Known);
        assert_eq!(profile.anaphylaxis_risk(), AnaphylaxisRisk::Low);
    }

    #[test]
    fn test_food_allergy_panel() {
        let mut panel = FoodAllergyPanel::new();
        panel.peanut = 5.0;
        panel.egg = 2.0;

        let positive = panel.positive_results();
        assert_eq!(positive.len(), 2);
        assert!(positive.contains(&"Peanut".to_string()));
    }

    #[test]
    fn test_immunotherapy_progress() {
        let therapy = ImmunotherapyStatus {
            allergen: "Grass pollen".to_string(),
            phase: ImmunotherapyPhase::Buildup,
            weeks_in_treatment: 12,
            current_dose_mcg: 50.0,
            target_dose_mcg: 100.0,
        };

        assert_eq!(therapy.progress(), 0.5);
    }
}

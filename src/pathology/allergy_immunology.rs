use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyImmunologyProfile {
    pub allergies: Vec<Allergy>,
    pub immunodeficiency: Option<Immunodeficiency>,
    pub autoimmune_conditions: Vec<AutoimmuneCondition>,
    pub ig_levels: ImmunoglobulinLevels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allergy {
    pub allergen: Allergen,
    pub reaction_type: AllergyType,
    pub severity: AllergySeverity,
    pub onset_age: Option<f64>,
    pub symptoms: Vec<AllergySymptom>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allergen {
    pub name: String,
    pub category: AllergenCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllergenCategory {
    Food,
    Environmental,
    Medication,
    Venom,
    ContactAllergen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllergyType {
    IgEMediated,
    NonIgEMediated,
    Mixed,
    CellMediated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllergySeverity {
    Mild,
    Moderate,
    Severe,
    Anaphylactic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllergySymptom {
    Urticaria,
    Angioedema,
    Rhinitis,
    Conjunctivitis,
    Wheezing,
    Dyspnea,
    Nausea,
    Vomiting,
    Diarrhea,
    Hypotension,
    Tachycardia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Immunodeficiency {
    pub deficiency_type: ImmunodeficiencyType,
    pub affected_components: Vec<ImmuneComponent>,
    pub recurrent_infections: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImmunodeficiencyType {
    PrimaryImmunodeficiency,
    SecondaryImmunodeficiency,
    CVID,
    SelectiveIgA,
    SCID,
    HivAids,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImmuneComponent {
    BCell,
    TCell,
    NaturalKiller,
    Complement,
    Phagocyte,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoimmuneCondition {
    pub condition: AutoimmuneDisorder,
    pub autoantibodies: Vec<Autoantibody>,
    pub affected_organs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutoimmuneDisorder {
    SystemicLupusErythematosus,
    RheumatoidArthritis,
    SjogrensSyndrome,
    Scleroderma,
    Type1Diabetes,
    Thyroiditis,
    MultiplSclerosis,
    InflammatoryBowelDisease,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Autoantibody {
    ANA,
    AntiDsDNA,
    AntiSm,
    RheumatoidFactor,
    AntiCCP,
    AntiTPO,
    AntiGAD,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImmunoglobulinLevels {
    pub iga_mg_dl: f64,
    pub igg_mg_dl: f64,
    pub igm_mg_dl: f64,
    pub ige_iu_ml: f64,
}

impl AllergyImmunologyProfile {
    pub fn new() -> Self {
        Self {
            allergies: Vec::new(),
            immunodeficiency: None,
            autoimmune_conditions: Vec::new(),
            ig_levels: ImmunoglobulinLevels::normal(),
        }
    }

    pub fn add_allergy(&mut self, allergy: Allergy) {
        self.allergies.push(allergy);
    }

    pub fn add_autoimmune_condition(&mut self, condition: AutoimmuneCondition) {
        self.autoimmune_conditions.push(condition);
    }

    pub fn anaphylaxis_risk(&self) -> AnaphylaxisRisk {
        let has_anaphylactic = self
            .allergies
            .iter()
            .any(|a| a.severity == AllergySeverity::Anaphylactic);

        let multiple_severe = self
            .allergies
            .iter()
            .filter(|a| {
                matches!(
                    a.severity,
                    AllergySeverity::Severe | AllergySeverity::Anaphylactic
                )
            })
            .count()
            > 1;

        match (has_anaphylactic, multiple_severe) {
            (true, _) => AnaphylaxisRisk::High,
            (false, true) => AnaphylaxisRisk::Moderate,
            (false, false) => AnaphylaxisRisk::Low,
        }
    }

    pub fn requires_epipen(&self) -> bool {
        self.allergies.iter().any(|a| {
            matches!(a.severity, AllergySeverity::Anaphylactic)
                || (matches!(a.severity, AllergySeverity::Severe)
                    && a.symptoms.iter().any(|s| {
                        matches!(
                            s,
                            AllergySymptom::Dyspnea
                                | AllergySymptom::Hypotension
                                | AllergySymptom::Angioedema
                        )
                    }))
        })
    }

    pub fn immunoglobulin_deficiency(&self) -> Vec<IgDeficiency> {
        let mut deficiencies = Vec::new();

        if self.ig_levels.iga_mg_dl < 70.0 {
            deficiencies.push(IgDeficiency::IgA);
        }
        if self.ig_levels.igg_mg_dl < 700.0 {
            deficiencies.push(IgDeficiency::IgG);
        }
        if self.ig_levels.igm_mg_dl < 40.0 {
            deficiencies.push(IgDeficiency::IgM);
        }

        deficiencies
    }

    pub fn atopic_march_present(&self) -> bool {
        let has_atopic_dermatitis = self
            .allergies
            .iter()
            .any(|a| matches!(a.allergen.category, AllergenCategory::ContactAllergen));

        let has_food_allergy = self
            .allergies
            .iter()
            .any(|a| matches!(a.allergen.category, AllergenCategory::Food));

        let has_environmental = self
            .allergies
            .iter()
            .any(|a| matches!(a.allergen.category, AllergenCategory::Environmental));

        has_atopic_dermatitis && has_food_allergy && has_environmental
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnaphylaxisRisk {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IgDeficiency {
    IgA,
    IgG,
    IgM,
}

impl Default for AllergyImmunologyProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl ImmunoglobulinLevels {
    pub fn normal() -> Self {
        Self {
            iga_mg_dl: 200.0,
            igg_mg_dl: 1000.0,
            igm_mg_dl: 100.0,
            ige_iu_ml: 50.0,
        }
    }

    pub fn total_ig(&self) -> f64 {
        self.iga_mg_dl + self.igg_mg_dl + self.igm_mg_dl
    }

    pub fn has_hypergammaglobulinemia(&self) -> bool {
        self.igg_mg_dl > 1600.0
    }

    pub fn has_elevated_ige(&self) -> bool {
        self.ige_iu_ml > 150.0
    }
}

impl Allergy {
    pub fn requires_avoidance(&self) -> bool {
        matches!(
            self.severity,
            AllergySeverity::Severe | AllergySeverity::Anaphylactic
        )
    }

    pub fn suitable_for_immunotherapy(&self) -> bool {
        matches!(self.reaction_type, AllergyType::IgEMediated)
            && matches!(
                self.allergen.category,
                AllergenCategory::Environmental | AllergenCategory::Venom
            )
            && !matches!(self.severity, AllergySeverity::Anaphylactic)
    }
}

impl AllergenCategory {
    pub fn common_allergens(&self) -> Vec<&'static str> {
        match self {
            AllergenCategory::Food => vec![
                "Peanuts",
                "Tree nuts",
                "Milk",
                "Eggs",
                "Wheat",
                "Soy",
                "Fish",
                "Shellfish",
            ],
            AllergenCategory::Environmental => {
                vec!["Pollen", "Dust mites", "Mold", "Pet dander", "Cockroach"]
            }
            AllergenCategory::Medication => vec!["Penicillin", "Sulfa drugs", "NSAIDs", "Aspirin"],
            AllergenCategory::Venom => vec!["Bee sting", "Wasp sting", "Fire ant"],
            AllergenCategory::ContactAllergen => {
                vec!["Nickel", "Latex", "Fragrances", "Preservatives"]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allergy_immunology_profile_creation() {
        let profile = AllergyImmunologyProfile::new();
        assert!(profile.allergies.is_empty());
        assert_eq!(profile.ig_levels.iga_mg_dl, 200.0);
    }

    #[test]
    fn test_anaphylaxis_risk() {
        let mut profile = AllergyImmunologyProfile::new();

        profile.add_allergy(Allergy {
            allergen: Allergen {
                name: "Peanuts".to_string(),
                category: AllergenCategory::Food,
            },
            reaction_type: AllergyType::IgEMediated,
            severity: AllergySeverity::Anaphylactic,
            onset_age: Some(5.0),
            symptoms: vec![AllergySymptom::Angioedema, AllergySymptom::Dyspnea],
        });

        assert_eq!(profile.anaphylaxis_risk(), AnaphylaxisRisk::High);
        assert!(profile.requires_epipen());
    }

    #[test]
    fn test_ig_deficiency_detection() {
        let mut profile = AllergyImmunologyProfile::new();
        profile.ig_levels.iga_mg_dl = 30.0;

        let deficiencies = profile.immunoglobulin_deficiency();
        assert!(deficiencies.contains(&IgDeficiency::IgA));
    }

    #[test]
    fn test_immunotherapy_eligibility() {
        let allergy = Allergy {
            allergen: Allergen {
                name: "Dust mites".to_string(),
                category: AllergenCategory::Environmental,
            },
            reaction_type: AllergyType::IgEMediated,
            severity: AllergySeverity::Moderate,
            onset_age: Some(10.0),
            symptoms: vec![AllergySymptom::Rhinitis, AllergySymptom::Conjunctivitis],
        };

        assert!(allergy.suitable_for_immunotherapy());
    }

    #[test]
    fn test_atopic_march() {
        let mut profile = AllergyImmunologyProfile::new();

        profile.add_allergy(Allergy {
            allergen: Allergen {
                name: "Nickel".to_string(),
                category: AllergenCategory::ContactAllergen,
            },
            reaction_type: AllergyType::CellMediated,
            severity: AllergySeverity::Mild,
            onset_age: Some(2.0),
            symptoms: vec![AllergySymptom::Urticaria],
        });

        profile.add_allergy(Allergy {
            allergen: Allergen {
                name: "Milk".to_string(),
                category: AllergenCategory::Food,
            },
            reaction_type: AllergyType::IgEMediated,
            severity: AllergySeverity::Moderate,
            onset_age: Some(3.0),
            symptoms: vec![AllergySymptom::Vomiting],
        });

        profile.add_allergy(Allergy {
            allergen: Allergen {
                name: "Pollen".to_string(),
                category: AllergenCategory::Environmental,
            },
            reaction_type: AllergyType::IgEMediated,
            severity: AllergySeverity::Moderate,
            onset_age: Some(8.0),
            symptoms: vec![AllergySymptom::Rhinitis],
        });

        assert!(profile.atopic_march_present());
    }

    #[test]
    fn test_ig_levels() {
        let levels = ImmunoglobulinLevels::normal();
        assert!(!levels.has_hypergammaglobulinemia());
        assert!(!levels.has_elevated_ige());
    }
}

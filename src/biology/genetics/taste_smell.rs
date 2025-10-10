use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tas2r38Genotype {
    PavPav,
    PavAvi,
    AviAvi,
}

impl Tas2r38Genotype {
    pub fn ptc_tasting_ability(&self) -> TasteAbility {
        match self {
            Tas2r38Genotype::PavPav => TasteAbility::SuperTaster,
            Tas2r38Genotype::PavAvi => TasteAbility::NormalTaster,
            Tas2r38Genotype::AviAvi => TasteAbility::NonTaster,
        }
    }

    pub fn bitter_sensitivity(&self) -> f64 {
        match self {
            Tas2r38Genotype::PavPav => 2.0,
            Tas2r38Genotype::PavAvi => 1.0,
            Tas2r38Genotype::AviAvi => 0.1,
        }
    }

    pub fn vegetable_preference_likelihood(&self) -> f64 {
        match self {
            Tas2r38Genotype::PavPav => 0.4,
            Tas2r38Genotype::PavAvi => 0.7,
            Tas2r38Genotype::AviAvi => 0.9,
        }
    }

    pub fn affected_foods(&self) -> Vec<&'static str> {
        vec![
            "Broccoli",
            "Brussels sprouts",
            "Kale",
            "Cabbage",
            "Coffee",
            "Dark chocolate",
            "Grapefruit",
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TasteAbility {
    SuperTaster,
    NormalTaster,
    NonTaster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteReceptorGenetics {
    pub tas2r38_genotype: Tas2r38Genotype,
    pub tas1r2_functional: bool,
    pub tas1r3_functional: bool,
    pub sweet_sensitivity: f64,
    pub umami_sensitivity: f64,
}

impl TasteReceptorGenetics {
    pub fn new(tas2r38: Tas2r38Genotype, tas1r2: bool, tas1r3: bool) -> Self {
        let sweet = if tas1r2 && tas1r3 {
            1.0
        } else if tas1r2 || tas1r3 {
            0.7
        } else {
            0.3
        };
        let umami = if tas1r3 { 1.0 } else { 0.5 };

        Self {
            tas2r38_genotype: tas2r38,
            tas1r2_functional: tas1r2,
            tas1r3_functional: tas1r3,
            sweet_sensitivity: sweet,
            umami_sensitivity: umami,
        }
    }

    pub fn taste_profile(&self) -> TasteProfile {
        TasteProfile {
            bitter: self.tas2r38_genotype.bitter_sensitivity(),
            sweet: self.sweet_sensitivity,
            umami: self.umami_sensitivity,
            salty: 1.0,
            sour: 1.0,
        }
    }

    pub fn food_preferences(&self) -> FoodPreferences {
        let vegetable_liking = self.tas2r38_genotype.vegetable_preference_likelihood();
        let sweet_preference = self.sweet_sensitivity;

        FoodPreferences {
            cruciferous_vegetables: vegetable_liking,
            sweet_foods: sweet_preference,
            bitter_beverages: 1.0 - self.tas2r38_genotype.bitter_sensitivity() / 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TasteProfile {
    pub bitter: f64,
    pub sweet: f64,
    pub umami: f64,
    pub salty: f64,
    pub sour: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FoodPreferences {
    pub cruciferous_vegetables: f64,
    pub sweet_foods: f64,
    pub bitter_beverages: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Or11h7pGenotype {
    Functional,
    Defective,
}

impl Or11h7pGenotype {
    pub fn can_smell_androstenone(&self) -> bool {
        matches!(self, Or11h7pGenotype::Functional)
    }

    pub fn affected_perception(&self) -> &'static str {
        match self {
            Or11h7pGenotype::Functional => "Can detect musky/urinous smell in pork",
            Or11h7pGenotype::Defective => "Cannot detect androstenone odor",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Or2j3Genotype {
    Sensitive,
    Normal,
    Insensitive,
}

impl Or2j3Genotype {
    pub fn cilantro_perception(&self) -> &'static str {
        match self {
            Or2j3Genotype::Sensitive => "Strong soapy/unpleasant taste from cilantro",
            Or2j3Genotype::Normal => "Mild soapy notes in cilantro",
            Or2j3Genotype::Insensitive => "Enjoys cilantro, no soapy taste",
        }
    }

    pub fn aldehyde_sensitivity(&self) -> f64 {
        match self {
            Or2j3Genotype::Sensitive => 2.0,
            Or2j3Genotype::Normal => 1.0,
            Or2j3Genotype::Insensitive => 0.3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryReceptorGenetics {
    pub or11h7p_genotype: Or11h7pGenotype,
    pub or2j3_genotype: Or2j3Genotype,
    pub or5a1_functional: bool,
    pub or7d4_functional: bool,
    pub total_functional_receptors: u32,
}

impl OlfactoryReceptorGenetics {
    pub fn new(
        or11h7p: Or11h7pGenotype,
        or2j3: Or2j3Genotype,
        or5a1: bool,
        or7d4: bool,
        total_functional: u32,
    ) -> Self {
        Self {
            or11h7p_genotype: or11h7p,
            or2j3_genotype: or2j3,
            or5a1_functional: or5a1,
            or7d4_functional: or7d4,
            total_functional_receptors: total_functional,
        }
    }

    pub fn can_smell_asparagus_metabolites(&self) -> bool {
        self.or5a1_functional
    }

    pub fn can_smell_androstenone(&self) -> bool {
        self.or11h7p_genotype.can_smell_androstenone()
    }

    pub fn olfactory_acuity(&self) -> f64 {
        let max_receptors = 400.0;
        self.total_functional_receptors as f64 / max_receptors
    }

    pub fn smell_blindness_to(&self) -> Vec<&'static str> {
        let mut compounds = Vec::new();

        if !self.or11h7p_genotype.can_smell_androstenone() {
            compounds.push("Androstenone");
        }

        if !self.or5a1_functional {
            compounds.push("Asparagus metabolites");
        }

        if !self.or7d4_functional {
            compounds.push("Certain floral notes");
        }

        compounds
    }

    pub fn cilantro_aversion_likelihood(&self) -> f64 {
        self.or2j3_genotype.aldehyde_sensitivity() / 2.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChemosensoryProfile {
    pub taste_genetics: TasteReceptorGenetics,
    pub olfactory_genetics: OlfactoryReceptorGenetics,
}

impl ChemosensoryProfile {
    pub fn new(taste: TasteReceptorGenetics, olfactory: OlfactoryReceptorGenetics) -> Self {
        Self {
            taste_genetics: taste,
            olfactory_genetics: olfactory,
        }
    }

    pub fn dietary_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        match self.taste_genetics.tas2r38_genotype.ptc_tasting_ability() {
            TasteAbility::SuperTaster => {
                recommendations.push(
                    "Consider cooking cruciferous vegetables to reduce bitterness".to_string(),
                );
                recommendations
                    .push("May prefer milder coffee and chocolate varieties".to_string());
            }
            TasteAbility::NonTaster => {
                recommendations.push("Can easily consume bitter vegetables raw".to_string());
                recommendations.push("May need to monitor vegetable intake variety".to_string());
            }
            TasteAbility::NormalTaster => {
                recommendations.push("Balanced taste perception for most foods".to_string());
            }
        }

        if self.olfactory_genetics.cilantro_aversion_likelihood() > 0.8 {
            recommendations
                .push("Strong genetic predisposition to dislike cilantro/coriander".to_string());
        }

        if !self.olfactory_genetics.can_smell_asparagus_metabolites() {
            recommendations
                .push("Genetically unable to smell asparagus metabolites in urine".to_string());
        }

        recommendations
    }

    pub fn flavor_perception_index(&self) -> f64 {
        let taste_score = (self.taste_genetics.sweet_sensitivity
            + self.taste_genetics.umami_sensitivity
            + self.taste_genetics.tas2r38_genotype.bitter_sensitivity())
            / 3.0;

        let smell_score = self.olfactory_genetics.olfactory_acuity();

        (taste_score * 0.3 + smell_score * 0.7).min(2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tas2r38_genotype() {
        let super_taster = Tas2r38Genotype::PavPav;
        assert_eq!(
            super_taster.ptc_tasting_ability(),
            TasteAbility::SuperTaster
        );
        assert!(super_taster.bitter_sensitivity() > 1.5);
        assert!(super_taster.vegetable_preference_likelihood() < 0.5);

        let non_taster = Tas2r38Genotype::AviAvi;
        assert_eq!(non_taster.ptc_tasting_ability(), TasteAbility::NonTaster);
        assert!(non_taster.bitter_sensitivity() < 0.2);
        assert!(non_taster.vegetable_preference_likelihood() > 0.8);
    }

    #[test]
    fn test_taste_receptor_genetics() {
        let genetics = TasteReceptorGenetics::new(Tas2r38Genotype::PavAvi, true, true);

        assert_eq!(genetics.sweet_sensitivity, 1.0);
        assert_eq!(genetics.umami_sensitivity, 1.0);

        let profile = genetics.taste_profile();
        assert!(profile.bitter > 0.5);
        assert_eq!(profile.sweet, 1.0);
    }

    #[test]
    fn test_olfactory_genetics() {
        let genetics = OlfactoryReceptorGenetics::new(
            Or11h7pGenotype::Functional,
            Or2j3Genotype::Sensitive,
            true,
            true,
            350,
        );

        assert!(genetics.can_smell_androstenone());
        assert!(genetics.can_smell_asparagus_metabolites());
        assert!(genetics.cilantro_aversion_likelihood() > 0.5);
        assert!(genetics.olfactory_acuity() > 0.8);
    }

    #[test]
    fn test_chemosensory_profile() {
        let taste = TasteReceptorGenetics::new(Tas2r38Genotype::PavPav, true, true);

        let olfactory = OlfactoryReceptorGenetics::new(
            Or11h7pGenotype::Functional,
            Or2j3Genotype::Sensitive,
            true,
            true,
            350,
        );

        let profile = ChemosensoryProfile::new(taste, olfactory);
        let recommendations = profile.dietary_recommendations();

        assert!(!recommendations.is_empty());
        assert!(profile.flavor_perception_index() > 0.5);
    }

    #[test]
    fn test_cilantro_perception() {
        let sensitive = Or2j3Genotype::Sensitive;
        assert!(sensitive.aldehyde_sensitivity() > 1.5);

        let insensitive = Or2j3Genotype::Insensitive;
        assert!(insensitive.aldehyde_sensitivity() < 0.5);
    }

    #[test]
    fn test_smell_blindness() {
        let genetics = OlfactoryReceptorGenetics::new(
            Or11h7pGenotype::Defective,
            Or2j3Genotype::Normal,
            false,
            false,
            300,
        );

        let blind_to = genetics.smell_blindness_to();
        assert!(blind_to.contains(&"Androstenone"));
        assert!(blind_to.contains(&"Asparagus metabolites"));
        assert!(blind_to.contains(&"Certain floral notes"));
    }
}

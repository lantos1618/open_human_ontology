use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrugInteraction {
    pub drug_a: String,
    pub drug_b: String,
    pub interaction_type: InteractionType,
    pub severity: InteractionSeverity,
    pub mechanism: String,
    pub clinical_effect: String,
    pub management: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    Pharmacokinetic,
    Pharmacodynamic,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionSeverity {
    Minor,
    Moderate,
    Major,
    Contraindicated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PharmacokineticInteraction {
    AbsorptionChange,
    DistributionChange,
    MetabolismInhibition,
    MetabolismInduction,
    ExcretionChange,
    ProteinBindingDisplacement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PharmacodynamicInteraction {
    Synergistic,
    Additive,
    Antagonistic,
    Potentiation,
}

impl DrugInteraction {
    pub fn new(
        drug_a: String,
        drug_b: String,
        interaction_type: InteractionType,
        severity: InteractionSeverity,
        mechanism: String,
    ) -> Self {
        DrugInteraction {
            drug_a,
            drug_b,
            interaction_type,
            severity,
            mechanism,
            clinical_effect: String::new(),
            management: String::new(),
        }
    }

    pub fn with_effect(mut self, effect: String) -> Self {
        self.clinical_effect = effect;
        self
    }

    pub fn with_management(mut self, management: String) -> Self {
        self.management = management;
        self
    }

    pub fn is_contraindicated(&self) -> bool {
        matches!(self.severity, InteractionSeverity::Contraindicated)
    }

    pub fn requires_monitoring(&self) -> bool {
        matches!(
            self.severity,
            InteractionSeverity::Moderate | InteractionSeverity::Major
        )
    }

    pub fn is_clinically_significant(&self) -> bool {
        !matches!(self.severity, InteractionSeverity::Minor)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionChecker {
    pub known_interactions: Vec<DrugInteraction>,
}

impl InteractionChecker {
    pub fn new() -> Self {
        InteractionChecker {
            known_interactions: Vec::new(),
        }
    }

    pub fn add_interaction(&mut self, interaction: DrugInteraction) {
        self.known_interactions.push(interaction);
    }

    pub fn check_interaction(&self, drug_a: &str, drug_b: &str) -> Option<&DrugInteraction> {
        self.known_interactions.iter()
            .find(|i|
                (i.drug_a == drug_a && i.drug_b == drug_b) ||
                (i.drug_a == drug_b && i.drug_b == drug_a)
            )
    }

    pub fn check_multiple(&self, drugs: &[String]) -> Vec<&DrugInteraction> {
        let mut interactions = Vec::new();

        for i in 0..drugs.len() {
            for j in (i + 1)..drugs.len() {
                if let Some(interaction) = self.check_interaction(&drugs[i], &drugs[j]) {
                    interactions.push(interaction);
                }
            }
        }

        interactions
    }

    pub fn has_contraindications(&self, drugs: &[String]) -> bool {
        self.check_multiple(drugs)
            .iter()
            .any(|i| i.is_contraindicated())
    }

    pub fn major_interactions(&self, drugs: &[String]) -> Vec<&DrugInteraction> {
        self.check_multiple(drugs)
            .into_iter()
            .filter(|i| matches!(i.severity, InteractionSeverity::Major))
            .collect()
    }
}

impl Default for InteractionChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drug_interaction() {
        let interaction = DrugInteraction::new(
            "Warfarin".to_string(),
            "Aspirin".to_string(),
            InteractionType::Pharmacodynamic,
            InteractionSeverity::Major,
            "Additive anticoagulant effect".to_string(),
        )
        .with_effect("Increased bleeding risk".to_string())
        .with_management("Monitor INR closely".to_string());

        assert!(!interaction.is_contraindicated());
        assert!(interaction.requires_monitoring());
        assert!(interaction.is_clinically_significant());
    }

    #[test]
    fn test_interaction_checker() {
        let mut checker = InteractionChecker::new();

        checker.add_interaction(DrugInteraction::new(
            "Simvastatin".to_string(),
            "Clarithromycin".to_string(),
            InteractionType::Pharmacokinetic,
            InteractionSeverity::Contraindicated,
            "CYP3A4 inhibition".to_string(),
        ));

        checker.add_interaction(DrugInteraction::new(
            "Lisinopril".to_string(),
            "Spironolactone".to_string(),
            InteractionType::Pharmacodynamic,
            InteractionSeverity::Major,
            "Hyperkalemia risk".to_string(),
        ));

        let drugs = vec![
            "Simvastatin".to_string(),
            "Clarithromycin".to_string(),
        ];

        assert!(checker.has_contraindications(&drugs));

        let interaction = checker.check_interaction("Simvastatin", "Clarithromycin");
        assert!(interaction.is_some());
        assert!(interaction.unwrap().is_contraindicated());
    }

    #[test]
    fn test_multiple_drug_check() {
        let mut checker = InteractionChecker::new();

        checker.add_interaction(DrugInteraction::new(
            "DrugA".to_string(),
            "DrugB".to_string(),
            InteractionType::Both,
            InteractionSeverity::Moderate,
            "Test".to_string(),
        ));

        checker.add_interaction(DrugInteraction::new(
            "DrugB".to_string(),
            "DrugC".to_string(),
            InteractionType::Pharmacokinetic,
            InteractionSeverity::Major,
            "Test".to_string(),
        ));

        let drugs = vec![
            "DrugA".to_string(),
            "DrugB".to_string(),
            "DrugC".to_string(),
        ];

        let interactions = checker.check_multiple(&drugs);
        assert_eq!(interactions.len(), 2);

        let major = checker.major_interactions(&drugs);
        assert_eq!(major.len(), 1);
    }
}

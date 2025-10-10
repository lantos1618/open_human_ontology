use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DrugClass {
    Analgesic,
    Antibiotic,
    Antiviral,
    Antifungal,
    Antihistamine,
    Antihypertensive,
    Antidiabetic,
    Anticoagulant,
    Immunosuppressant,
    Chemotherapy,
    Antidepressant,
    Anxiolytic,
    Antipsychotic,
    Anticonvulsant,
    Bronchodilator,
    Corticosteroid,
    BetaBlocker,
    CalciumChannelBlocker,
    ACEInhibitor,
    Statin,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drug {
    pub name: String,
    pub generic_name: String,
    pub drug_class: DrugClass,
    pub molecular_weight: f64,
    pub mechanism_of_action: String,
    pub indications: Vec<String>,
    pub contraindications: Vec<String>,
    pub side_effects: Vec<SideEffect>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SideEffect {
    pub effect: String,
    pub frequency: Frequency,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Frequency {
    VeryCommon,
    Common,
    Uncommon,
    Rare,
    VeryRare,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Mild,
    Moderate,
    Severe,
    LifeThreatening,
}

impl Drug {
    pub fn new(
        name: String,
        generic_name: String,
        drug_class: DrugClass,
        mechanism_of_action: String,
    ) -> Self {
        Drug {
            name,
            generic_name,
            drug_class,
            molecular_weight: 0.0,
            mechanism_of_action,
            indications: Vec::new(),
            contraindications: Vec::new(),
            side_effects: Vec::new(),
        }
    }

    pub fn add_indication(&mut self, indication: String) {
        if !self.indications.contains(&indication) {
            self.indications.push(indication);
        }
    }

    pub fn add_contraindication(&mut self, contraindication: String) {
        if !self.contraindications.contains(&contraindication) {
            self.contraindications.push(contraindication);
        }
    }

    pub fn add_side_effect(&mut self, side_effect: SideEffect) {
        self.side_effects.push(side_effect);
    }

    pub fn has_severe_side_effects(&self) -> bool {
        self.side_effects
            .iter()
            .any(|se| matches!(se.severity, Severity::Severe | Severity::LifeThreatening))
    }

    pub fn common_side_effects(&self) -> Vec<&SideEffect> {
        self.side_effects
            .iter()
            .filter(|se| matches!(se.frequency, Frequency::VeryCommon | Frequency::Common))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dosage {
    pub amount: f64,
    pub unit: DosageUnit,
    pub route: RouteOfAdministration,
    pub frequency: DosageFrequency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DosageUnit {
    Milligram,
    Gram,
    Microgram,
    InternationalUnit,
    Milliliter,
    Tablet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteOfAdministration {
    Oral,
    Intravenous,
    Intramuscular,
    Subcutaneous,
    Topical,
    Inhalation,
    Sublingual,
    Rectal,
    Transdermal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DosageFrequency {
    OnceDaily,
    TwiceDaily,
    ThreeTimesDaily,
    FourTimesDaily,
    EveryNHours(f64),
    AsNeeded,
    Weekly,
    Monthly,
}

impl Dosage {
    pub fn new(amount: f64, unit: DosageUnit, route: RouteOfAdministration) -> Self {
        Dosage {
            amount,
            unit,
            route,
            frequency: DosageFrequency::OnceDaily,
        }
    }

    pub fn with_frequency(mut self, frequency: DosageFrequency) -> Self {
        self.frequency = frequency;
        self
    }

    pub fn daily_dose(&self) -> Option<f64> {
        match &self.frequency {
            DosageFrequency::OnceDaily => Some(self.amount),
            DosageFrequency::TwiceDaily => Some(self.amount * 2.0),
            DosageFrequency::ThreeTimesDaily => Some(self.amount * 3.0),
            DosageFrequency::FourTimesDaily => Some(self.amount * 4.0),
            DosageFrequency::EveryNHours(hours) => Some(self.amount * (24.0 / hours)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drug_creation() {
        let mut drug = Drug::new(
            "Aspirin".to_string(),
            "Acetylsalicylic acid".to_string(),
            DrugClass::Analgesic,
            "COX inhibitor".to_string(),
        );

        drug.add_indication("Pain relief".to_string());
        drug.add_indication("Fever reduction".to_string());

        drug.add_side_effect(SideEffect {
            effect: "Stomach upset".to_string(),
            frequency: Frequency::Common,
            severity: Severity::Mild,
        });

        assert_eq!(drug.indications.len(), 2);
        assert!(!drug.has_severe_side_effects());
        assert_eq!(drug.common_side_effects().len(), 1);
    }

    #[test]
    fn test_dosage() {
        let dosage = Dosage::new(500.0, DosageUnit::Milligram, RouteOfAdministration::Oral)
            .with_frequency(DosageFrequency::TwiceDaily);

        assert_eq!(dosage.daily_dose(), Some(1000.0));
    }

    #[test]
    fn test_dosage_every_n_hours() {
        let dosage = Dosage::new(200.0, DosageUnit::Milligram, RouteOfAdministration::Oral)
            .with_frequency(DosageFrequency::EveryNHours(6.0));

        assert_eq!(dosage.daily_dose(), Some(800.0));
    }
}

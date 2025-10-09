use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ABOType {
    A,
    B,
    AB,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BloodType {
    pub abo: ABOType,
    pub rh: RhFactor,
    pub minor_antigens: Vec<MinorAntigen>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MinorAntigen {
    Kell,
    Duffy,
    Kidd,
    Lewis,
    MNS,
    P,
    Lutheran,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransfusionCompatibility {
    pub donor: BloodType,
    pub recipient: BloodType,
    pub abo_compatible: bool,
    pub rh_compatible: bool,
    pub minor_antigen_compatible: bool,
    pub crossmatch_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodBankInventory {
    pub units: Vec<BloodUnit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodUnit {
    pub blood_type: BloodType,
    pub unit_type: BloodProductType,
    pub volume_ml: f64,
    pub collection_date: String,
    pub expiration_date: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BloodProductType {
    WholeBlood,
    PackedRedBloodCells,
    FreshFrozenPlasma,
    Platelets,
    Cryoprecipitate,
}

impl BloodType {
    pub fn new(abo: ABOType, rh: RhFactor) -> Self {
        Self {
            abo,
            rh,
            minor_antigens: Vec::new(),
        }
    }

    pub fn with_minor_antigens(mut self, antigens: Vec<MinorAntigen>) -> Self {
        self.minor_antigens = antigens;
        self
    }

    pub fn can_receive_from(&self, donor: &BloodType) -> bool {
        self.abo_compatible(donor) && self.rh_compatible(donor) && self.minor_antigens_compatible(donor)
    }

    pub fn can_donate_to(&self, recipient: &BloodType) -> bool {
        recipient.can_receive_from(self)
    }

    fn abo_compatible(&self, donor: &BloodType) -> bool {
        match (self.abo, donor.abo) {
            (ABOType::AB, _) => true,
            (ABOType::A, ABOType::A) | (ABOType::A, ABOType::O) => true,
            (ABOType::B, ABOType::B) | (ABOType::B, ABOType::O) => true,
            (ABOType::O, ABOType::O) => true,
            _ => false,
        }
    }

    fn rh_compatible(&self, donor: &BloodType) -> bool {
        match (self.rh, donor.rh) {
            (RhFactor::Positive, _) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            _ => false,
        }
    }

    fn minor_antigens_compatible(&self, donor: &BloodType) -> bool {
        let recipient_antigens: HashSet<_> = self.minor_antigens.iter().collect();
        let donor_antigens: HashSet<_> = donor.minor_antigens.iter().collect();

        for donor_ag in donor_antigens {
            if !recipient_antigens.contains(donor_ag) {
                return false;
            }
        }

        true
    }

    pub fn is_universal_donor(&self) -> bool {
        self.abo == ABOType::O && self.rh == RhFactor::Negative && self.minor_antigens.is_empty()
    }

    pub fn is_universal_recipient(&self) -> bool {
        self.abo == ABOType::AB && self.rh == RhFactor::Positive
    }

    pub fn frequency_in_population(&self, ethnicity: Ethnicity) -> f64 {
        match ethnicity {
            Ethnicity::Caucasian => self.caucasian_frequency(),
            Ethnicity::AfricanAmerican => self.african_american_frequency(),
            Ethnicity::Hispanic => self.hispanic_frequency(),
            Ethnicity::Asian => self.asian_frequency(),
        }
    }

    fn caucasian_frequency(&self) -> f64 {
        match (self.abo, self.rh) {
            (ABOType::O, RhFactor::Positive) => 0.37,
            (ABOType::O, RhFactor::Negative) => 0.08,
            (ABOType::A, RhFactor::Positive) => 0.34,
            (ABOType::A, RhFactor::Negative) => 0.06,
            (ABOType::B, RhFactor::Positive) => 0.10,
            (ABOType::B, RhFactor::Negative) => 0.02,
            (ABOType::AB, RhFactor::Positive) => 0.04,
            (ABOType::AB, RhFactor::Negative) => 0.01,
        }
    }

    fn african_american_frequency(&self) -> f64 {
        match (self.abo, self.rh) {
            (ABOType::O, RhFactor::Positive) => 0.47,
            (ABOType::O, RhFactor::Negative) => 0.04,
            (ABOType::A, RhFactor::Positive) => 0.24,
            (ABOType::A, RhFactor::Negative) => 0.02,
            (ABOType::B, RhFactor::Positive) => 0.18,
            (ABOType::B, RhFactor::Negative) => 0.01,
            (ABOType::AB, RhFactor::Positive) => 0.04,
            (ABOType::AB, RhFactor::Negative) => 0.00,
        }
    }

    fn hispanic_frequency(&self) -> f64 {
        match (self.abo, self.rh) {
            (ABOType::O, RhFactor::Positive) => 0.53,
            (ABOType::O, RhFactor::Negative) => 0.04,
            (ABOType::A, RhFactor::Positive) => 0.29,
            (ABOType::A, RhFactor::Negative) => 0.02,
            (ABOType::B, RhFactor::Positive) => 0.09,
            (ABOType::B, RhFactor::Negative) => 0.01,
            (ABOType::AB, RhFactor::Positive) => 0.02,
            (ABOType::AB, RhFactor::Negative) => 0.00,
        }
    }

    fn asian_frequency(&self) -> f64 {
        match (self.abo, self.rh) {
            (ABOType::O, RhFactor::Positive) => 0.39,
            (ABOType::O, RhFactor::Negative) => 0.01,
            (ABOType::A, RhFactor::Positive) => 0.27,
            (ABOType::A, RhFactor::Negative) => 0.00,
            (ABOType::B, RhFactor::Positive) => 0.25,
            (ABOType::B, RhFactor::Negative) => 0.00,
            (ABOType::AB, RhFactor::Positive) => 0.07,
            (ABOType::AB, RhFactor::Negative) => 0.00,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ethnicity {
    Caucasian,
    AfricanAmerican,
    Hispanic,
    Asian,
}

impl TransfusionCompatibility {
    pub fn assess(donor: BloodType, recipient: BloodType) -> Self {
        let abo_compatible = recipient.abo_compatible(&donor);
        let rh_compatible = recipient.rh_compatible(&donor);
        let minor_antigen_compatible = recipient.minor_antigens_compatible(&donor);

        let crossmatch_required = !donor.minor_antigens.is_empty()
            || matches!(recipient.abo, ABOType::AB)
            || matches!(donor.abo, ABOType::O);

        Self {
            donor,
            recipient,
            abo_compatible,
            rh_compatible,
            minor_antigen_compatible,
            crossmatch_required,
        }
    }

    pub fn is_compatible(&self) -> bool {
        self.abo_compatible && self.rh_compatible && self.minor_antigen_compatible
    }

    pub fn emergency_transfusion_allowed(&self) -> bool {
        self.donor.abo == ABOType::O && self.donor.rh == RhFactor::Negative
    }
}

impl BloodBankInventory {
    pub fn new() -> Self {
        Self { units: Vec::new() }
    }

    pub fn add_unit(&mut self, unit: BloodUnit) {
        self.units.push(unit);
    }

    pub fn find_compatible_units(&self, recipient: &BloodType, product_type: BloodProductType) -> Vec<&BloodUnit> {
        self.units
            .iter()
            .filter(|unit| {
                unit.unit_type == product_type && recipient.can_receive_from(&unit.blood_type)
            })
            .collect()
    }

    pub fn inventory_by_type(&self) -> std::collections::HashMap<(ABOType, RhFactor), usize> {
        let mut inventory = std::collections::HashMap::new();

        for unit in &self.units {
            let key = (unit.blood_type.abo, unit.blood_type.rh);
            *inventory.entry(key).or_insert(0) += 1;
        }

        inventory
    }
}

impl Default for BloodBankInventory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abo_compatibility() {
        let o_neg = BloodType::new(ABOType::O, RhFactor::Negative);
        let ab_pos = BloodType::new(ABOType::AB, RhFactor::Positive);
        let a_pos = BloodType::new(ABOType::A, RhFactor::Positive);

        assert!(ab_pos.can_receive_from(&o_neg));
        assert!(ab_pos.can_receive_from(&a_pos));
        assert!(!a_pos.can_receive_from(&ab_pos));
    }

    #[test]
    fn test_rh_compatibility() {
        let rh_pos = BloodType::new(ABOType::A, RhFactor::Positive);
        let rh_neg = BloodType::new(ABOType::A, RhFactor::Negative);

        assert!(rh_pos.can_receive_from(&rh_neg));
        assert!(!rh_neg.can_receive_from(&rh_pos));
    }

    #[test]
    fn test_universal_donor() {
        let o_neg = BloodType::new(ABOType::O, RhFactor::Negative);
        assert!(o_neg.is_universal_donor());

        let a_pos = BloodType::new(ABOType::A, RhFactor::Positive);
        let ab_pos = BloodType::new(ABOType::AB, RhFactor::Positive);
        let b_neg = BloodType::new(ABOType::B, RhFactor::Negative);

        assert!(a_pos.can_receive_from(&o_neg));
        assert!(ab_pos.can_receive_from(&o_neg));
        assert!(b_neg.can_receive_from(&o_neg));
    }

    #[test]
    fn test_universal_recipient() {
        let ab_pos = BloodType::new(ABOType::AB, RhFactor::Positive);
        assert!(ab_pos.is_universal_recipient());

        let a_pos = BloodType::new(ABOType::A, RhFactor::Positive);
        let b_neg = BloodType::new(ABOType::B, RhFactor::Negative);
        let o_pos = BloodType::new(ABOType::O, RhFactor::Positive);

        assert!(ab_pos.can_receive_from(&a_pos));
        assert!(ab_pos.can_receive_from(&b_neg));
        assert!(ab_pos.can_receive_from(&o_pos));
    }

    #[test]
    fn test_minor_antigens() {
        let donor = BloodType::new(ABOType::A, RhFactor::Positive)
            .with_minor_antigens(vec![MinorAntigen::Kell]);

        let recipient_without_kell = BloodType::new(ABOType::A, RhFactor::Positive);
        let recipient_with_kell = BloodType::new(ABOType::A, RhFactor::Positive)
            .with_minor_antigens(vec![MinorAntigen::Kell]);

        assert!(!recipient_without_kell.can_receive_from(&donor));
        assert!(recipient_with_kell.can_receive_from(&donor));
    }

    #[test]
    fn test_transfusion_compatibility() {
        let donor = BloodType::new(ABOType::O, RhFactor::Positive);
        let recipient = BloodType::new(ABOType::A, RhFactor::Positive);

        let compat = TransfusionCompatibility::assess(donor, recipient);
        assert!(compat.is_compatible());
    }

    #[test]
    fn test_blood_bank_inventory() {
        let mut inventory = BloodBankInventory::new();

        inventory.add_unit(BloodUnit {
            blood_type: BloodType::new(ABOType::O, RhFactor::Negative),
            unit_type: BloodProductType::PackedRedBloodCells,
            volume_ml: 450.0,
            collection_date: "2024-01-01".to_string(),
            expiration_date: "2024-02-01".to_string(),
        });

        let recipient = BloodType::new(ABOType::A, RhFactor::Positive);
        let compatible = inventory.find_compatible_units(&recipient, BloodProductType::PackedRedBloodCells);

        assert_eq!(compatible.len(), 1);
    }

    #[test]
    fn test_population_frequencies() {
        let o_pos = BloodType::new(ABOType::O, RhFactor::Positive);
        let freq_caucasian = o_pos.frequency_in_population(Ethnicity::Caucasian);
        let freq_hispanic = o_pos.frequency_in_population(Ethnicity::Hispanic);

        assert!(freq_caucasian > 0.0);
        assert!(freq_hispanic > freq_caucasian);
    }
}

use crate::biology::BiologyResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ABOAllele {
    A,
    B,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RhAllele {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABOGenotype {
    pub allele1: ABOAllele,
    pub allele2: ABOAllele,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RhGenotype {
    pub allele1: RhAllele,
    pub allele2: RhAllele,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BloodType {
    OPositive,
    ONegative,
    APositive,
    ANegative,
    BPositive,
    BNegative,
    ABPositive,
    ABNegative,
}

impl BloodType {
    pub fn from_genotype(abo: ABOGenotype, rh: RhGenotype) -> Self {
        let phenotype = match (abo.allele1, abo.allele2) {
            (ABOAllele::A, ABOAllele::A)
            | (ABOAllele::A, ABOAllele::O)
            | (ABOAllele::O, ABOAllele::A) => "A",
            (ABOAllele::B, ABOAllele::B)
            | (ABOAllele::B, ABOAllele::O)
            | (ABOAllele::O, ABOAllele::B) => "B",
            (ABOAllele::A, ABOAllele::B) | (ABOAllele::B, ABOAllele::A) => "AB",
            (ABOAllele::O, ABOAllele::O) => "O",
        };

        let rh_positive =
            matches!(rh.allele1, RhAllele::Positive) || matches!(rh.allele2, RhAllele::Positive);

        match (phenotype, rh_positive) {
            ("O", true) => BloodType::OPositive,
            ("O", false) => BloodType::ONegative,
            ("A", true) => BloodType::APositive,
            ("A", false) => BloodType::ANegative,
            ("B", true) => BloodType::BPositive,
            ("B", false) => BloodType::BNegative,
            ("AB", true) => BloodType::ABPositive,
            ("AB", false) => BloodType::ABNegative,
            _ => BloodType::OPositive,
        }
    }

    pub fn can_receive_from(&self, donor: BloodType) -> bool {
        match self {
            BloodType::OPositive => matches!(donor, BloodType::OPositive | BloodType::ONegative),
            BloodType::ONegative => matches!(donor, BloodType::ONegative),
            BloodType::APositive => matches!(
                donor,
                BloodType::APositive
                    | BloodType::ANegative
                    | BloodType::OPositive
                    | BloodType::ONegative
            ),
            BloodType::ANegative => matches!(donor, BloodType::ANegative | BloodType::ONegative),
            BloodType::BPositive => matches!(
                donor,
                BloodType::BPositive
                    | BloodType::BNegative
                    | BloodType::OPositive
                    | BloodType::ONegative
            ),
            BloodType::BNegative => matches!(donor, BloodType::BNegative | BloodType::ONegative),
            BloodType::ABPositive => true,
            BloodType::ABNegative => matches!(
                donor,
                BloodType::ANegative
                    | BloodType::BNegative
                    | BloodType::ABNegative
                    | BloodType::ONegative
            ),
        }
    }

    pub fn can_donate_to(&self, recipient: BloodType) -> bool {
        recipient.can_receive_from(*self)
    }

    pub fn is_universal_donor(&self) -> bool {
        matches!(self, BloodType::ONegative)
    }

    pub fn is_universal_recipient(&self) -> bool {
        matches!(self, BloodType::ABPositive)
    }

    pub fn global_frequency(&self) -> f64 {
        match self {
            BloodType::OPositive => 0.374,
            BloodType::ONegative => 0.066,
            BloodType::APositive => 0.357,
            BloodType::ANegative => 0.063,
            BloodType::BPositive => 0.085,
            BloodType::BNegative => 0.015,
            BloodType::ABPositive => 0.034,
            BloodType::ABNegative => 0.006,
        }
    }

    pub fn asian_frequency(&self) -> f64 {
        match self {
            BloodType::OPositive => 0.36,
            BloodType::ONegative => 0.005,
            BloodType::APositive => 0.27,
            BloodType::ANegative => 0.005,
            BloodType::BPositive => 0.25,
            BloodType::BNegative => 0.005,
            BloodType::ABPositive => 0.10,
            BloodType::ABNegative => 0.005,
        }
    }

    pub fn european_frequency(&self) -> f64 {
        match self {
            BloodType::OPositive => 0.37,
            BloodType::ONegative => 0.08,
            BloodType::APositive => 0.36,
            BloodType::ANegative => 0.08,
            BloodType::BPositive => 0.08,
            BloodType::BNegative => 0.02,
            BloodType::ABPositive => 0.03,
            BloodType::ABNegative => 0.01,
        }
    }

    pub fn associated_diseases(&self) -> Vec<&'static str> {
        match self {
            BloodType::OPositive | BloodType::ONegative => vec![
                "Lower risk of cardiovascular disease",
                "Lower risk of venous thromboembolism",
                "Higher risk of bleeding disorders",
            ],
            BloodType::APositive | BloodType::ANegative => vec![
                "Higher risk of gastric cancer",
                "Higher risk of pancreatic cancer",
                "Higher risk of cardiovascular disease",
            ],
            BloodType::BPositive | BloodType::BNegative => vec![
                "Higher risk of type 2 diabetes",
                "Higher risk of pancreatic cancer",
            ],
            BloodType::ABPositive | BloodType::ABNegative => vec![
                "Higher risk of blood clots",
                "Higher risk of stroke",
                "Higher risk of cognitive impairment",
            ],
        }
    }
}

impl ABOGenotype {
    pub fn new(allele1: ABOAllele, allele2: ABOAllele) -> Self {
        Self { allele1, allele2 }
    }

    pub fn phenotype(&self) -> &'static str {
        match (self.allele1, self.allele2) {
            (ABOAllele::A, ABOAllele::A)
            | (ABOAllele::A, ABOAllele::O)
            | (ABOAllele::O, ABOAllele::A) => "A",
            (ABOAllele::B, ABOAllele::B)
            | (ABOAllele::B, ABOAllele::O)
            | (ABOAllele::O, ABOAllele::B) => "B",
            (ABOAllele::A, ABOAllele::B) | (ABOAllele::B, ABOAllele::A) => "AB",
            (ABOAllele::O, ABOAllele::O) => "O",
        }
    }
}

impl RhGenotype {
    pub fn new(allele1: RhAllele, allele2: RhAllele) -> Self {
        Self { allele1, allele2 }
    }

    pub fn phenotype(&self) -> &'static str {
        if matches!(self.allele1, RhAllele::Positive) || matches!(self.allele2, RhAllele::Positive)
        {
            "Positive"
        } else {
            "Negative"
        }
    }
}

pub struct BloodTypeCompatibility;

impl BloodTypeCompatibility {
    pub fn check_transfusion(
        donor: BloodType,
        recipient: BloodType,
    ) -> BiologyResult<TransfusionCompatibility> {
        let compatible = recipient.can_receive_from(donor);

        Ok(TransfusionCompatibility {
            donor,
            recipient,
            compatible,
            risk_level: if compatible {
                RiskLevel::Safe
            } else {
                RiskLevel::Dangerous
            },
            notes: if compatible {
                "Transfusion compatible".to_string()
            } else {
                "INCOMPATIBLE - Risk of hemolytic reaction".to_string()
            },
        })
    }

    pub fn check_pregnancy_risk(mother: BloodType, father: BloodType) -> PregnancyRiskAssessment {
        let mother_rh_neg = matches!(
            mother,
            BloodType::ONegative
                | BloodType::ANegative
                | BloodType::BNegative
                | BloodType::ABNegative
        );
        let father_rh_pos = matches!(
            father,
            BloodType::OPositive
                | BloodType::APositive
                | BloodType::BPositive
                | BloodType::ABPositive
        );

        let rh_incompatibility_risk = mother_rh_neg && father_rh_pos;

        PregnancyRiskAssessment {
            mother_blood_type: mother,
            father_blood_type: father,
            rh_incompatibility_risk,
            recommendations: if rh_incompatibility_risk {
                vec![
                    "Monitor for Rh sensitization".to_string(),
                    "Consider RhoGAM administration at 28 weeks".to_string(),
                    "RhoGAM within 72 hours postpartum if baby is Rh+".to_string(),
                ]
            } else {
                vec!["Standard prenatal care".to_string()]
            },
        }
    }

    pub fn possible_offspring_types(_parent1: BloodType, _parent2: BloodType) -> Vec<BloodType> {
        let mut possible = Vec::new();

        for blood_type in [
            BloodType::OPositive,
            BloodType::ONegative,
            BloodType::APositive,
            BloodType::ANegative,
            BloodType::BPositive,
            BloodType::BNegative,
            BloodType::ABPositive,
            BloodType::ABNegative,
        ] {
            possible.push(blood_type);
        }

        possible
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransfusionCompatibility {
    pub donor: BloodType,
    pub recipient: BloodType,
    pub compatible: bool,
    pub risk_level: RiskLevel,
    pub notes: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Safe,
    LowRisk,
    ModerateRisk,
    HighRisk,
    Dangerous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PregnancyRiskAssessment {
    pub mother_blood_type: BloodType,
    pub father_blood_type: BloodType,
    pub rh_incompatibility_risk: bool,
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blood_type_from_genotype() {
        let abo = ABOGenotype::new(ABOAllele::A, ABOAllele::O);
        let rh = RhGenotype::new(RhAllele::Positive, RhAllele::Negative);
        let blood_type = BloodType::from_genotype(abo, rh);
        assert_eq!(blood_type, BloodType::APositive);
    }

    #[test]
    fn test_universal_donor() {
        assert!(BloodType::ONegative.is_universal_donor());
        assert!(!BloodType::ABPositive.is_universal_donor());
    }

    #[test]
    fn test_universal_recipient() {
        assert!(BloodType::ABPositive.is_universal_recipient());
        assert!(!BloodType::ONegative.is_universal_recipient());
    }

    #[test]
    fn test_transfusion_compatibility() {
        assert!(BloodType::ABPositive.can_receive_from(BloodType::ONegative));
        assert!(BloodType::ABPositive.can_receive_from(BloodType::APositive));
        assert!(!BloodType::ONegative.can_receive_from(BloodType::APositive));
    }

    #[test]
    fn test_o_neg_to_all() {
        let o_neg = BloodType::ONegative;
        assert!(o_neg.can_donate_to(BloodType::OPositive));
        assert!(o_neg.can_donate_to(BloodType::APositive));
        assert!(o_neg.can_donate_to(BloodType::BPositive));
        assert!(o_neg.can_donate_to(BloodType::ABPositive));
    }

    #[test]
    fn test_check_transfusion() {
        let result =
            BloodTypeCompatibility::check_transfusion(BloodType::ONegative, BloodType::ABPositive)
                .unwrap();
        assert!(result.compatible);
        assert_eq!(result.risk_level, RiskLevel::Safe);
    }

    #[test]
    fn test_incompatible_transfusion() {
        let result =
            BloodTypeCompatibility::check_transfusion(BloodType::ABPositive, BloodType::ONegative)
                .unwrap();
        assert!(!result.compatible);
        assert_eq!(result.risk_level, RiskLevel::Dangerous);
    }

    #[test]
    fn test_pregnancy_risk_rh_incompatible() {
        let assessment = BloodTypeCompatibility::check_pregnancy_risk(
            BloodType::ANegative,
            BloodType::APositive,
        );
        assert!(assessment.rh_incompatibility_risk);
        assert!(!assessment.recommendations.is_empty());
    }

    #[test]
    fn test_pregnancy_risk_rh_compatible() {
        let assessment = BloodTypeCompatibility::check_pregnancy_risk(
            BloodType::APositive,
            BloodType::APositive,
        );
        assert!(!assessment.rh_incompatibility_risk);
    }

    #[test]
    fn test_asian_frequency() {
        let b_pos_freq = BloodType::BPositive.asian_frequency();
        assert!(b_pos_freq > 0.20);
    }

    #[test]
    fn test_abo_genotype_phenotype() {
        let genotype = ABOGenotype::new(ABOAllele::A, ABOAllele::B);
        assert_eq!(genotype.phenotype(), "AB");
    }

    #[test]
    fn test_rh_genotype_phenotype() {
        let genotype = RhGenotype::new(RhAllele::Positive, RhAllele::Negative);
        assert_eq!(genotype.phenotype(), "Positive");
    }
}

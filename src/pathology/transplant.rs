use crate::biology::blood_typing::{BloodType, ABOType, RhFactor};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrganType {
    Heart,
    Lung,
    Liver,
    Kidney,
    Pancreas,
    SmallIntestine,
    Cornea,
    BoneMarrow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HLATyping {
    pub hla_a: Vec<HLAAllele>,
    pub hla_b: Vec<HLAAllele>,
    pub hla_c: Vec<HLAAllele>,
    pub hla_dr: Vec<HLAAllele>,
    pub hla_dq: Vec<HLAAllele>,
    pub hla_dp: Vec<HLAAllele>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HLAAllele {
    pub locus: String,
    pub allele: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransplantCandidate {
    pub blood_type: BloodType,
    pub hla_typing: HLATyping,
    pub organ_needed: OrganType,
    pub urgency: UrgencyStatus,
    pub wait_time_days: u32,
    pub panel_reactive_antibody: f64,
    pub body_size: BodySize,
    pub medical_urgency_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganDonor {
    pub blood_type: BloodType,
    pub hla_typing: HLATyping,
    pub organs_available: Vec<OrganType>,
    pub body_size: BodySize,
    pub viral_status: ViralStatus,
    pub age: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UrgencyStatus {
    Status1A,
    Status1B,
    Status2,
    Status3,
    Status4,
    Status7,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodySize {
    pub height_cm: f64,
    pub weight_kg: f64,
    pub bsa_m2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViralStatus {
    pub cmv: SerologyStatus,
    pub ebv: SerologyStatus,
    pub hiv: SerologyStatus,
    pub hep_b: SerologyStatus,
    pub hep_c: SerologyStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerologyStatus {
    Positive,
    Negative,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchAnalysis {
    pub abo_compatible: bool,
    pub hla_match_grade: HLAMatchGrade,
    pub crossmatch_compatible: bool,
    pub size_compatible: bool,
    pub viral_risk: ViralTransmissionRisk,
    pub overall_compatibility_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HLAMatchGrade {
    SixOfSix,
    FiveOfSix,
    FourOfSix,
    ThreeOfSix,
    TwoOfSix,
    OneOfSix,
    ZeroOfSix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViralTransmissionRisk {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunosuppressionRegimen {
    pub induction: Vec<InductionAgent>,
    pub maintenance: Vec<MaintenanceAgent>,
    pub rejection_prophylaxis: Vec<RejectionProphylaxis>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InductionAgent {
    Basiliximab,
    Daclizumab,
    AntiThymocyteGlobulin,
    Alemtuzumab,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceAgent {
    Tacrolimus,
    Cyclosporine,
    Sirolimus,
    Everolimus,
    Mycophenolate,
    Azathioprine,
    Prednisone,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RejectionProphylaxis {
    CMVProphylaxis,
    PneumocystisProphylaxis,
    FungalProphylaxis,
}

impl HLATyping {
    pub fn new() -> Self {
        Self {
            hla_a: Vec::new(),
            hla_b: Vec::new(),
            hla_c: Vec::new(),
            hla_dr: Vec::new(),
            hla_dq: Vec::new(),
            hla_dp: Vec::new(),
        }
    }

    pub fn calculate_match_with(&self, other: &HLATyping) -> usize {
        let mut matches = 0;

        matches += self.count_allele_matches(&self.hla_a, &other.hla_a);
        matches += self.count_allele_matches(&self.hla_b, &other.hla_b);
        matches += self.count_allele_matches(&self.hla_dr, &other.hla_dr);

        matches
    }

    fn count_allele_matches(&self, alleles1: &[HLAAllele], alleles2: &[HLAAllele]) -> usize {
        let set1: HashSet<_> = alleles1.iter().collect();
        let set2: HashSet<_> = alleles2.iter().collect();

        set1.intersection(&set2).count()
    }

    pub fn is_six_antigen_match(&self, other: &HLATyping) -> bool {
        self.calculate_match_with(other) >= 6
    }
}

impl Default for HLATyping {
    fn default() -> Self {
        Self::new()
    }
}

impl TransplantCandidate {
    pub fn calculate_priority_score(&self) -> f64 {
        let urgency_score = match self.urgency {
            UrgencyStatus::Status1A => 10.0,
            UrgencyStatus::Status1B => 8.0,
            UrgencyStatus::Status2 => 6.0,
            UrgencyStatus::Status3 => 4.0,
            UrgencyStatus::Status4 => 2.0,
            UrgencyStatus::Status7 => 0.0,
        };

        let wait_time_score = (self.wait_time_days as f64 / 365.0).min(5.0);
        let pra_penalty = self.panel_reactive_antibody * 2.0;

        urgency_score + wait_time_score + self.medical_urgency_score - pra_penalty
    }

    pub fn analyze_match_with(&self, donor: &OrganDonor) -> MatchAnalysis {
        let abo_compatible = self.blood_type.can_receive_from(&donor.blood_type);

        let hla_matches = self.hla_typing.calculate_match_with(&donor.hla_typing);
        let hla_match_grade = match hla_matches {
            6.. => HLAMatchGrade::SixOfSix,
            5 => HLAMatchGrade::FiveOfSix,
            4 => HLAMatchGrade::FourOfSix,
            3 => HLAMatchGrade::ThreeOfSix,
            2 => HLAMatchGrade::TwoOfSix,
            1 => HLAMatchGrade::OneOfSix,
            _ => HLAMatchGrade::ZeroOfSix,
        };

        let crossmatch_compatible = self.panel_reactive_antibody < 0.8;

        let size_compatible = self.body_size.is_compatible_with(&donor.body_size, self.organ_needed);

        let viral_risk = ViralStatus::assess_transmission_risk(
            &self.calculate_recipient_viral_status(),
            &donor.viral_status,
        );

        let mut compatibility_score = 0.0;
        if abo_compatible {
            compatibility_score += 30.0;
        }
        compatibility_score += hla_matches as f64 * 10.0;
        if crossmatch_compatible {
            compatibility_score += 20.0;
        }
        if size_compatible {
            compatibility_score += 10.0;
        }

        match viral_risk {
            ViralTransmissionRisk::Low => compatibility_score += 10.0,
            ViralTransmissionRisk::Moderate => compatibility_score += 5.0,
            ViralTransmissionRisk::High => compatibility_score -= 10.0,
        }

        MatchAnalysis {
            abo_compatible,
            hla_match_grade,
            crossmatch_compatible,
            size_compatible,
            viral_risk,
            overall_compatibility_score: compatibility_score,
        }
    }

    fn calculate_recipient_viral_status(&self) -> ViralStatus {
        ViralStatus {
            cmv: SerologyStatus::Unknown,
            ebv: SerologyStatus::Unknown,
            hiv: SerologyStatus::Negative,
            hep_b: SerologyStatus::Negative,
            hep_c: SerologyStatus::Negative,
        }
    }
}

impl BodySize {
    pub fn calculate_bsa(&self) -> f64 {
        0.007184 * self.height_cm.powf(0.725) * self.weight_kg.powf(0.425)
    }

    pub fn is_compatible_with(&self, donor_size: &BodySize, organ: OrganType) -> bool {
        let size_ratio = self.bsa_m2 / donor_size.bsa_m2;

        match organ {
            OrganType::Heart | OrganType::Lung => (0.7..=1.3).contains(&size_ratio),
            OrganType::Liver => (0.6..=2.0).contains(&size_ratio),
            OrganType::Kidney => (0.5..=2.5).contains(&size_ratio),
            OrganType::Pancreas => (0.6..=1.5).contains(&size_ratio),
            OrganType::SmallIntestine => (0.7..=1.3).contains(&size_ratio),
            OrganType::Cornea | OrganType::BoneMarrow => true,
        }
    }
}

impl ViralStatus {
    pub fn assess_transmission_risk(recipient: &ViralStatus, donor: &ViralStatus) -> ViralTransmissionRisk {
        let mut risk_factors = 0;

        if donor.hiv == SerologyStatus::Positive {
            return ViralTransmissionRisk::High;
        }

        if donor.hep_b == SerologyStatus::Positive && recipient.hep_b == SerologyStatus::Negative {
            risk_factors += 2;
        }

        if donor.hep_c == SerologyStatus::Positive && recipient.hep_c == SerologyStatus::Negative {
            risk_factors += 2;
        }

        if donor.cmv == SerologyStatus::Positive && recipient.cmv == SerologyStatus::Negative {
            risk_factors += 1;
        }

        match risk_factors {
            0..=1 => ViralTransmissionRisk::Low,
            2..=3 => ViralTransmissionRisk::Moderate,
            _ => ViralTransmissionRisk::High,
        }
    }
}

impl OrganType {
    pub fn cold_ischemia_time_hours(&self) -> f64 {
        match self {
            OrganType::Heart => 4.0,
            OrganType::Lung => 6.0,
            OrganType::Liver => 12.0,
            OrganType::Kidney => 24.0,
            OrganType::Pancreas => 12.0,
            OrganType::SmallIntestine => 8.0,
            OrganType::Cornea => 168.0,
            OrganType::BoneMarrow => 48.0,
        }
    }

    pub fn one_year_survival_rate(&self) -> f64 {
        match self {
            OrganType::Heart => 0.91,
            OrganType::Lung => 0.89,
            OrganType::Liver => 0.92,
            OrganType::Kidney => 0.98,
            OrganType::Pancreas => 0.95,
            OrganType::SmallIntestine => 0.85,
            OrganType::Cornea => 0.99,
            OrganType::BoneMarrow => 0.80,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hla_matching() {
        let typing1 = HLATyping::new();
        let typing2 = HLATyping::new();

        let matches = typing1.calculate_match_with(&typing2);
        assert_eq!(matches, 0);
    }

    #[test]
    fn test_body_size_compatibility() {
        let recipient_size = BodySize {
            height_cm: 170.0,
            weight_kg: 70.0,
            bsa_m2: 1.8,
        };

        let donor_size = BodySize {
            height_cm: 175.0,
            weight_kg: 75.0,
            bsa_m2: 1.9,
        };

        assert!(recipient_size.is_compatible_with(&donor_size, OrganType::Heart));
    }

    #[test]
    fn test_viral_risk_assessment() {
        let recipient = ViralStatus {
            cmv: SerologyStatus::Negative,
            ebv: SerologyStatus::Positive,
            hiv: SerologyStatus::Negative,
            hep_b: SerologyStatus::Negative,
            hep_c: SerologyStatus::Negative,
        };

        let donor = ViralStatus {
            cmv: SerologyStatus::Positive,
            ebv: SerologyStatus::Positive,
            hiv: SerologyStatus::Negative,
            hep_b: SerologyStatus::Negative,
            hep_c: SerologyStatus::Negative,
        };

        let risk = ViralStatus::assess_transmission_risk(&recipient, &donor);
        assert_eq!(risk, ViralTransmissionRisk::Low);
    }

    #[test]
    fn test_transplant_priority() {
        let candidate = TransplantCandidate {
            blood_type: BloodType::new(ABOType::A, RhFactor::Positive),
            hla_typing: HLATyping::new(),
            organ_needed: OrganType::Heart,
            urgency: UrgencyStatus::Status1A,
            wait_time_days: 365,
            panel_reactive_antibody: 0.2,
            body_size: BodySize {
                height_cm: 170.0,
                weight_kg: 70.0,
                bsa_m2: 1.8,
            },
            medical_urgency_score: 5.0,
        };

        let score = candidate.calculate_priority_score();
        assert!(score > 10.0);
    }

    #[test]
    fn test_cold_ischemia_times() {
        assert_eq!(OrganType::Heart.cold_ischemia_time_hours(), 4.0);
        assert_eq!(OrganType::Kidney.cold_ischemia_time_hours(), 24.0);
    }

    #[test]
    fn test_survival_rates() {
        assert!(OrganType::Kidney.one_year_survival_rate() > 0.9);
    }
}

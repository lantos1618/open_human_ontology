use serde::{Deserialize, Serialize};
use super::cancer_data_loader::CANCER_DATA;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brca1Status {
    WildType,
    Pathogenic185delAG,
    Pathogenic5382insC,
    OtherPathogenic,
    VariantOfUncertainSignificance,
}

impl Brca1Status {
    pub fn breast_cancer_risk_by_age(&self, age: f64) -> f64 {
        CANCER_DATA.brca1.get_breast_cancer_risk(age, self.is_pathogenic())
    }

    pub fn ovarian_cancer_lifetime_risk(&self) -> f64 {
        if self.is_pathogenic() {
            CANCER_DATA.brca1.ovarian_cancer_risk.pathogenic
        } else {
            CANCER_DATA.brca1.ovarian_cancer_risk.wild_type
        }
    }

    pub fn screening_recommendations(&self) -> Vec<String> {
        match self {
            Brca1Status::Pathogenic185delAG
            | Brca1Status::Pathogenic5382insC
            | Brca1Status::OtherPathogenic => CANCER_DATA.brca1.screening.pathogenic.clone(),
            Brca1Status::WildType => CANCER_DATA.brca1.screening.wild_type.clone(),
            Brca1Status::VariantOfUncertainSignificance => CANCER_DATA.brca1.screening.vus.clone(),
        }
    }

    pub fn is_pathogenic(&self) -> bool {
        matches!(
            self,
            Brca1Status::Pathogenic185delAG
                | Brca1Status::Pathogenic5382insC
                | Brca1Status::OtherPathogenic
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brca2Status {
    WildType,
    Pathogenic6174delT,
    OtherPathogenic,
    VariantOfUncertainSignificance,
}

impl Brca2Status {
    pub fn breast_cancer_risk_by_age(&self, age: f64) -> f64 {
        CANCER_DATA.brca2.get_breast_cancer_risk(age, self.is_pathogenic())
    }

    pub fn ovarian_cancer_lifetime_risk(&self) -> f64 {
        if self.is_pathogenic() {
            CANCER_DATA.brca2.ovarian_cancer_risk.pathogenic
        } else {
            CANCER_DATA.brca2.ovarian_cancer_risk.wild_type
        }
    }

    pub fn prostate_cancer_risk(&self) -> f64 {
        if self.is_pathogenic() {
            CANCER_DATA.brca2.prostate_cancer_risk.pathogenic
        } else {
            CANCER_DATA.brca2.prostate_cancer_risk.wild_type
        }
    }

    pub fn pancreatic_cancer_risk(&self) -> f64 {
        if self.is_pathogenic() {
            CANCER_DATA.brca2.pancreatic_cancer_risk.pathogenic
        } else {
            CANCER_DATA.brca2.pancreatic_cancer_risk.wild_type
        }
    }

    pub fn is_pathogenic(&self) -> bool {
        matches!(
            self,
            Brca2Status::Pathogenic6174delT | Brca2Status::OtherPathogenic
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tp53Status {
    WildType,
    Pathogenic,
}

impl Tp53Status {
    pub fn li_fraumeni_syndrome(&self) -> bool {
        matches!(self, Tp53Status::Pathogenic)
    }

    pub fn cancer_risk_by_age(&self, age: f64) -> f64 {
        CANCER_DATA.tp53.get_cancer_risk(age, matches!(self, Tp53Status::Pathogenic))
    }

    pub fn screening_protocol(&self) -> Vec<String> {
        if self.li_fraumeni_syndrome() {
            CANCER_DATA.tp53.screening.pathogenic.clone()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MlhMshStatus {
    WildType,
    Mlh1Pathogenic,
    Msh2Pathogenic,
    Msh6Pathogenic,
    Pms2Pathogenic,
}

impl MlhMshStatus {
    pub fn lynch_syndrome(&self) -> bool {
        !matches!(self, MlhMshStatus::WildType)
    }

    pub fn colorectal_cancer_lifetime_risk(&self) -> f64 {
        let variant = match self {
            MlhMshStatus::Mlh1Pathogenic => "Mlh1Pathogenic",
            MlhMshStatus::Msh2Pathogenic => "Msh2Pathogenic",
            MlhMshStatus::Msh6Pathogenic => "Msh6Pathogenic",
            MlhMshStatus::Pms2Pathogenic => "Pms2Pathogenic",
            MlhMshStatus::WildType => "WildType",
        };
        CANCER_DATA.mlh_msh.get_colorectal_risk(variant)
    }

    pub fn endometrial_cancer_lifetime_risk(&self) -> f64 {
        let variant = match self {
            MlhMshStatus::Mlh1Pathogenic => "Mlh1Pathogenic",
            MlhMshStatus::Msh2Pathogenic => "Msh2Pathogenic",
            MlhMshStatus::Msh6Pathogenic => "Msh6Pathogenic",
            MlhMshStatus::Pms2Pathogenic => "Pms2Pathogenic",
            MlhMshStatus::WildType => "WildType",
        };
        CANCER_DATA.mlh_msh.get_endometrial_risk(variant)
    }

    pub fn screening_recommendations(&self) -> Vec<String> {
        if self.lynch_syndrome() {
            CANCER_DATA.mlh_msh.screening.pathogenic.clone()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApcStatus {
    WildType,
    Pathogenic,
}

impl ApcStatus {
    pub fn familial_adenomatous_polyposis(&self) -> bool {
        matches!(self, ApcStatus::Pathogenic)
    }

    pub fn colorectal_cancer_risk(&self) -> f64 {
        if self.familial_adenomatous_polyposis() {
            CANCER_DATA.apc.colorectal_cancer_risk.pathogenic
        } else {
            CANCER_DATA.apc.colorectal_cancer_risk.wild_type
        }
    }

    pub fn polyp_burden(&self) -> &str {
        if self.familial_adenomatous_polyposis() {
            &CANCER_DATA.apc.polyp_burden.pathogenic
        } else {
            &CANCER_DATA.apc.polyp_burden.wild_type
        }
    }

    pub fn management(&self) -> Vec<String> {
        if self.familial_adenomatous_polyposis() {
            CANCER_DATA.apc.management.pathogenic.clone()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PtenStatus {
    WildType,
    Pathogenic,
}

impl PtenStatus {
    pub fn cowden_syndrome(&self) -> bool {
        matches!(self, PtenStatus::Pathogenic)
    }

    pub fn breast_cancer_lifetime_risk(&self) -> f64 {
        if self.cowden_syndrome() {
            CANCER_DATA.pten.breast_cancer_risk.pathogenic
        } else {
            CANCER_DATA.pten.breast_cancer_risk.wild_type
        }
    }

    pub fn thyroid_cancer_risk(&self) -> f64 {
        if self.cowden_syndrome() {
            CANCER_DATA.pten.thyroid_cancer_risk.pathogenic
        } else {
            CANCER_DATA.pten.thyroid_cancer_risk.wild_type
        }
    }

    pub fn endometrial_cancer_risk(&self) -> f64 {
        if self.cowden_syndrome() {
            CANCER_DATA.pten.endometrial_cancer_risk.pathogenic
        } else {
            CANCER_DATA.pten.endometrial_cancer_risk.wild_type
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerGeneticProfile {
    pub brca1: Brca1Status,
    pub brca2: Brca2Status,
    pub tp53: Tp53Status,
    pub mlh_msh: MlhMshStatus,
    pub apc: ApcStatus,
    pub pten: PtenStatus,
}

impl Default for CancerGeneticProfile {
    fn default() -> Self {
        Self {
            brca1: Brca1Status::WildType,
            brca2: Brca2Status::WildType,
            tp53: Tp53Status::WildType,
            mlh_msh: MlhMshStatus::WildType,
            apc: ApcStatus::WildType,
            pten: PtenStatus::WildType,
        }
    }
}

impl CancerGeneticProfile {
    pub fn comprehensive_cancer_screening(&self, _age: f64, is_female: bool) -> Vec<String> {
        let mut screening = Vec::new();

        if self.brca1.is_pathogenic() {
            screening.extend(self.brca1.screening_recommendations());
        }

        if self.brca2.is_pathogenic() && is_female {
            screening.extend(self.brca1.screening_recommendations());
        }

        if !is_female && self.brca2.is_pathogenic() {
            screening.push("Annual prostate PSA and exam starting age 40".to_string());
        }

        if self.tp53.li_fraumeni_syndrome() {
            screening.extend(self.tp53.screening_protocol());
        }

        if self.mlh_msh.lynch_syndrome() {
            screening.extend(self.mlh_msh.screening_recommendations());
        }

        if self.apc.familial_adenomatous_polyposis() {
            screening.extend(self.apc.management());
        }

        if self.pten.cowden_syndrome() {
            screening.extend(CANCER_DATA.pten.screening.pathogenic.clone());
        }

        if screening.is_empty() {
            screening.push("Standard population screening appropriate".to_string());
        }

        screening
    }

    pub fn hereditary_cancer_syndromes(&self) -> Vec<String> {
        let mut syndromes = Vec::new();

        if self.brca1.is_pathogenic() || self.brca2.is_pathogenic() {
            syndromes.push("Hereditary Breast and Ovarian Cancer Syndrome".to_string());
        }

        if self.tp53.li_fraumeni_syndrome() {
            syndromes.push("Li-Fraumeni Syndrome".to_string());
        }

        if self.mlh_msh.lynch_syndrome() {
            syndromes.push("Lynch Syndrome".to_string());
        }

        if self.apc.familial_adenomatous_polyposis() {
            syndromes.push("Familial Adenomatous Polyposis".to_string());
        }

        if self.pten.cowden_syndrome() {
            syndromes.push("Cowden Syndrome".to_string());
        }

        syndromes
    }

    pub fn lifetime_cancer_risk_summary(&self, age: f64, is_female: bool) -> Vec<(String, f64)> {
        let mut risks = Vec::new();

        if is_female {
            let breast_risk = if self.brca1.is_pathogenic() {
                self.brca1.breast_cancer_risk_by_age(age)
            } else if self.brca2.is_pathogenic() {
                self.brca2.breast_cancer_risk_by_age(age)
            } else if self.pten.cowden_syndrome() {
                self.pten.breast_cancer_lifetime_risk()
            } else {
                0.125
            };
            risks.push(("Breast Cancer".to_string(), breast_risk));

            let ovarian_risk = if self.brca1.is_pathogenic() {
                self.brca1.ovarian_cancer_lifetime_risk()
            } else if self.brca2.is_pathogenic() {
                self.brca2.ovarian_cancer_lifetime_risk()
            } else {
                0.012
            };
            risks.push(("Ovarian Cancer".to_string(), ovarian_risk));

            let endometrial_risk = if self.mlh_msh.lynch_syndrome() {
                self.mlh_msh.endometrial_cancer_lifetime_risk()
            } else {
                0.028
            };
            risks.push(("Endometrial Cancer".to_string(), endometrial_risk));
        } else if self.brca2.is_pathogenic() {
            risks.push((
                "Prostate Cancer".to_string(),
                self.brca2.prostate_cancer_risk() * 0.12,
            ));
        }

        let colorectal_risk = if self.mlh_msh.lynch_syndrome() {
            self.mlh_msh.colorectal_cancer_lifetime_risk()
        } else if self.apc.familial_adenomatous_polyposis() {
            self.apc.colorectal_cancer_risk()
        } else {
            0.045
        };
        risks.push(("Colorectal Cancer".to_string(), colorectal_risk));

        if self.tp53.li_fraumeni_syndrome() {
            risks.push((
                "Any Cancer (Li-Fraumeni)".to_string(),
                self.tp53.cancer_risk_by_age(age),
            ));
        }

        risks
    }

    pub fn parp_inhibitor_eligibility(&self) -> bool {
        self.brca1.is_pathogenic() || self.brca2.is_pathogenic()
    }

    pub fn immunotherapy_biomarkers(&self) -> Vec<String> {
        let mut biomarkers = Vec::new();

        if self.mlh_msh.lynch_syndrome() {
            biomarkers
                .push("MSI-High / dMMR: excellent response to checkpoint inhibitors".to_string());
        }

        biomarkers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brca1_pathogenic() {
        let status = Brca1Status::Pathogenic185delAG;
        assert!(status.is_pathogenic());
        assert!(status.breast_cancer_risk_by_age(50.0) > 0.4);
        assert!(status.ovarian_cancer_lifetime_risk() > 0.3);
    }

    #[test]
    fn test_brca2_prostate() {
        let status = Brca2Status::Pathogenic6174delT;
        assert!(status.prostate_cancer_risk() > 5.0);
        assert!(status.pancreatic_cancer_risk() > 3.0);
    }

    #[test]
    fn test_tp53_li_fraumeni() {
        let status = Tp53Status::Pathogenic;
        assert!(status.li_fraumeni_syndrome());
        assert!(status.cancer_risk_by_age(30.0) > 0.3);
        assert!(!status.screening_protocol().is_empty());
    }

    #[test]
    fn test_lynch_syndrome() {
        let status = MlhMshStatus::Mlh1Pathogenic;
        assert!(status.lynch_syndrome());
        assert!(status.colorectal_cancer_lifetime_risk() > 0.4);
        assert!(status.endometrial_cancer_lifetime_risk() > 0.4);
    }

    #[test]
    fn test_apc_fap() {
        let status = ApcStatus::Pathogenic;
        assert!(status.familial_adenomatous_polyposis());
        assert_eq!(status.colorectal_cancer_risk(), 1.0);
    }

    #[test]
    fn test_pten_cowden() {
        let status = PtenStatus::Pathogenic;
        assert!(status.cowden_syndrome());
        assert!(status.breast_cancer_lifetime_risk() > 0.7);
        assert!(status.thyroid_cancer_risk() > 20.0);
    }

    #[test]
    fn test_comprehensive_profile() {
        let mut profile = CancerGeneticProfile::default();
        profile.brca1 = Brca1Status::Pathogenic5382insC;
        profile.mlh_msh = MlhMshStatus::Msh2Pathogenic;

        let syndromes = profile.hereditary_cancer_syndromes();
        assert_eq!(syndromes.len(), 2);

        let screening = profile.comprehensive_cancer_screening(35.0, true);
        assert!(!screening.is_empty());

        assert!(profile.parp_inhibitor_eligibility());
    }

    #[test]
    fn test_lifetime_risks() {
        let mut profile = CancerGeneticProfile::default();
        profile.brca1 = Brca1Status::OtherPathogenic;

        let risks = profile.lifetime_cancer_risk_summary(40.0, true);
        assert!(!risks.is_empty());

        let breast_risk = risks.iter().find(|(name, _)| name == "Breast Cancer");
        assert!(breast_risk.is_some());
        assert!(breast_risk.unwrap().1 > 0.3);
    }
}

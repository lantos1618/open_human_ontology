use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct RiskBracket {
    pub max_age: f64,
    pub wild_type: f64,
    pub pathogenic: f64,
}

#[derive(Debug, Deserialize)]
pub struct Brca1Data {
    pub gene_name: String,
    pub chromosome: String,
    pub variants: HashMap<String, String>,
    pub breast_cancer_risk: BreastCancerRisk,
    pub ovarian_cancer_risk: OvarianCancerRisk,
    pub screening: ScreeningData,
}

#[derive(Debug, Deserialize)]
pub struct BreastCancerRisk {
    pub brackets: Vec<RiskBracket>,
}

#[derive(Debug, Deserialize)]
pub struct OvarianCancerRisk {
    pub wild_type: f64,
    pub pathogenic: f64,
}

#[derive(Debug, Deserialize)]
pub struct ScreeningData {
    pub pathogenic: Vec<String>,
    pub wild_type: Vec<String>,
    pub vus: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Brca2Data {
    pub gene_name: String,
    pub chromosome: String,
    pub variants: HashMap<String, String>,
    pub breast_cancer_risk: BreastCancerRisk,
    pub ovarian_cancer_risk: OvarianCancerRisk,
    pub prostate_cancer_risk: SimpleRisk,
    pub pancreatic_cancer_risk: SimpleRisk,
}

#[derive(Debug, Deserialize)]
pub struct SimpleRisk {
    pub wild_type: f64,
    pub pathogenic: f64,
}

#[derive(Debug, Deserialize)]
pub struct Tp53RiskBracket {
    pub max_age: f64,
    pub pathogenic: f64,
}

#[derive(Debug, Deserialize)]
pub struct Tp53CancerRisk {
    pub wild_type: f64,
    pub brackets: Vec<Tp53RiskBracket>,
}

#[derive(Debug, Deserialize)]
pub struct Tp53Data {
    pub gene_name: String,
    pub chromosome: String,
    pub syndrome: String,
    pub cancer_risk: Tp53CancerRisk,
    pub screening: Tp53Screening,
}

#[derive(Debug, Deserialize)]
pub struct Tp53Screening {
    pub pathogenic: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MlhMshData {
    pub gene_name: String,
    pub syndrome: String,
    pub colorectal_cancer_risk: MlhMshCancerRisk,
    pub endometrial_cancer_risk: MlhMshCancerRisk,
    pub screening: MlhMshScreening,
}

#[derive(Debug, Deserialize)]
pub struct MlhMshCancerRisk {
    pub wild_type: f64,
    pub mlh1_pathogenic: f64,
    pub msh2_pathogenic: f64,
    pub msh6_pathogenic: f64,
    pub pms2_pathogenic: f64,
}

#[derive(Debug, Deserialize)]
pub struct MlhMshScreening {
    pub pathogenic: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApcData {
    pub gene_name: String,
    pub chromosome: String,
    pub syndrome: String,
    pub colorectal_cancer_risk: SimpleRisk,
    pub polyp_burden: PolypBurden,
    pub management: ApcManagement,
}

#[derive(Debug, Deserialize)]
pub struct PolypBurden {
    pub wild_type: String,
    pub pathogenic: String,
}

#[derive(Debug, Deserialize)]
pub struct ApcManagement {
    pub pathogenic: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PtenData {
    pub gene_name: String,
    pub chromosome: String,
    pub syndrome: String,
    pub breast_cancer_risk: SimpleRisk,
    pub thyroid_cancer_risk: SimpleRisk,
    pub endometrial_cancer_risk: SimpleRisk,
    pub screening: PtenScreening,
}

#[derive(Debug, Deserialize)]
pub struct PtenScreening {
    pub pathogenic: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CancerVariantsData {
    pub brca1: Brca1Data,
    pub brca2: Brca2Data,
    pub tp53: Tp53Data,
    pub mlh_msh: MlhMshData,
    pub apc: ApcData,
    pub pten: PtenData,
}

pub static CANCER_DATA: Lazy<CancerVariantsData> = Lazy::new(|| {
    let toml_str = include_str!("../../../data/genetics/cancer_variants.toml");
    toml::from_str(toml_str).expect("Failed to parse cancer_variants.toml")
});

impl Brca1Data {
    pub fn get_breast_cancer_risk(&self, age: f64, is_pathogenic: bool) -> f64 {
        for bracket in &self.breast_cancer_risk.brackets {
            if age < bracket.max_age {
                return if is_pathogenic {
                    bracket.pathogenic
                } else {
                    bracket.wild_type
                };
            }
        }
        self.breast_cancer_risk.brackets.last().unwrap().wild_type
    }
}

impl Brca2Data {
    pub fn get_breast_cancer_risk(&self, age: f64, is_pathogenic: bool) -> f64 {
        for bracket in &self.breast_cancer_risk.brackets {
            if age < bracket.max_age {
                return if is_pathogenic {
                    bracket.pathogenic
                } else {
                    bracket.wild_type
                };
            }
        }
        self.breast_cancer_risk.brackets.last().unwrap().wild_type
    }
}

impl Tp53Data {
    pub fn get_cancer_risk(&self, age: f64, is_pathogenic: bool) -> f64 {
        if !is_pathogenic {
            return self.cancer_risk.wild_type;
        }

        for bracket in &self.cancer_risk.brackets {
            if age < bracket.max_age {
                return bracket.pathogenic;
            }
        }
        self.cancer_risk.brackets.last().unwrap().pathogenic
    }
}

impl MlhMshData {
    pub fn get_colorectal_risk(&self, variant: &str) -> f64 {
        match variant {
            "Mlh1Pathogenic" => self.colorectal_cancer_risk.mlh1_pathogenic,
            "Msh2Pathogenic" => self.colorectal_cancer_risk.msh2_pathogenic,
            "Msh6Pathogenic" => self.colorectal_cancer_risk.msh6_pathogenic,
            "Pms2Pathogenic" => self.colorectal_cancer_risk.pms2_pathogenic,
            _ => self.colorectal_cancer_risk.wild_type,
        }
    }

    pub fn get_endometrial_risk(&self, variant: &str) -> f64 {
        match variant {
            "Mlh1Pathogenic" => self.endometrial_cancer_risk.mlh1_pathogenic,
            "Msh2Pathogenic" => self.endometrial_cancer_risk.msh2_pathogenic,
            "Msh6Pathogenic" => self.endometrial_cancer_risk.msh6_pathogenic,
            "Pms2Pathogenic" => self.endometrial_cancer_risk.pms2_pathogenic,
            _ => self.endometrial_cancer_risk.wild_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_cancer_data() {
        let data = &*CANCER_DATA;
        assert_eq!(data.brca1.gene_name, "BRCA1");
        assert_eq!(data.brca2.gene_name, "BRCA2");
        assert_eq!(data.tp53.syndrome, "Li-Fraumeni Syndrome");
    }

    #[test]
    fn test_brca1_risk_calculation() {
        let data = &*CANCER_DATA;
        let risk_30 = data.brca1.get_breast_cancer_risk(30.0, true);
        assert!(risk_30 > 0.15 && risk_30 < 0.20);

        let risk_55 = data.brca1.get_breast_cancer_risk(55.0, true);
        assert!(risk_55 > 0.60);
    }

    #[test]
    fn test_tp53_risk_calculation() {
        let data = &*CANCER_DATA;
        let risk_25 = data.tp53.get_cancer_risk(25.0, true);
        assert!(risk_25 > 0.20 && risk_25 < 0.45);

        let risk_wt = data.tp53.get_cancer_risk(50.0, false);
        assert_eq!(risk_wt, 0.01);
    }
}

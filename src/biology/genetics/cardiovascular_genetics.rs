use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApoeAllele {
    E2,
    E3,
    E4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApoeGenotype {
    pub allele1: ApoeAllele,
    pub allele2: ApoeAllele,
}

impl ApoeGenotype {
    pub fn new(allele1: ApoeAllele, allele2: ApoeAllele) -> Self {
        Self { allele1, allele2 }
    }

    pub fn e3_e3() -> Self {
        Self::new(ApoeAllele::E3, ApoeAllele::E3)
    }

    pub fn e4_e4() -> Self {
        Self::new(ApoeAllele::E4, ApoeAllele::E4)
    }

    pub fn e3_e4() -> Self {
        Self::new(ApoeAllele::E3, ApoeAllele::E4)
    }

    pub fn e2_e3() -> Self {
        Self::new(ApoeAllele::E2, ApoeAllele::E3)
    }

    pub fn e2_e4() -> Self {
        Self::new(ApoeAllele::E2, ApoeAllele::E4)
    }

    pub fn e2_e2() -> Self {
        Self::new(ApoeAllele::E2, ApoeAllele::E2)
    }

    pub fn has_e4(&self) -> bool {
        self.allele1 == ApoeAllele::E4 || self.allele2 == ApoeAllele::E4
    }

    pub fn e4_count(&self) -> u8 {
        let mut count = 0;
        if self.allele1 == ApoeAllele::E4 {
            count += 1;
        }
        if self.allele2 == ApoeAllele::E4 {
            count += 1;
        }
        count
    }

    pub fn alzheimers_relative_risk(&self) -> f64 {
        match (self.allele1, self.allele2) {
            (ApoeAllele::E2, ApoeAllele::E2) => 0.6,
            (ApoeAllele::E2, ApoeAllele::E3) | (ApoeAllele::E3, ApoeAllele::E2) => 0.8,
            (ApoeAllele::E3, ApoeAllele::E3) => 1.0,
            (ApoeAllele::E2, ApoeAllele::E4) | (ApoeAllele::E4, ApoeAllele::E2) => 2.6,
            (ApoeAllele::E3, ApoeAllele::E4) | (ApoeAllele::E4, ApoeAllele::E3) => 3.2,
            (ApoeAllele::E4, ApoeAllele::E4) => 12.0,
        }
    }

    pub fn cardiovascular_disease_relative_risk(&self) -> f64 {
        match (self.allele1, self.allele2) {
            (ApoeAllele::E2, ApoeAllele::E2) => 0.8,
            (ApoeAllele::E2, ApoeAllele::E3) | (ApoeAllele::E3, ApoeAllele::E2) => 0.9,
            (ApoeAllele::E3, ApoeAllele::E3) => 1.0,
            (ApoeAllele::E2, ApoeAllele::E4) | (ApoeAllele::E4, ApoeAllele::E2) => 1.1,
            (ApoeAllele::E3, ApoeAllele::E4) | (ApoeAllele::E4, ApoeAllele::E3) => 1.4,
            (ApoeAllele::E4, ApoeAllele::E4) => 1.8,
        }
    }

    pub fn ldl_cholesterol_effect(&self) -> f64 {
        match (self.allele1, self.allele2) {
            (ApoeAllele::E2, ApoeAllele::E2) => -20.0,
            (ApoeAllele::E2, ApoeAllele::E3) | (ApoeAllele::E3, ApoeAllele::E2) => -10.0,
            (ApoeAllele::E3, ApoeAllele::E3) => 0.0,
            (ApoeAllele::E2, ApoeAllele::E4) | (ApoeAllele::E4, ApoeAllele::E2) => 5.0,
            (ApoeAllele::E3, ApoeAllele::E4) | (ApoeAllele::E4, ApoeAllele::E3) => 8.0,
            (ApoeAllele::E4, ApoeAllele::E4) => 15.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pcsk9Variant {
    WildType,
    LossOfFunction,
    GainOfFunction,
}

impl Pcsk9Variant {
    pub fn ldl_effect(&self) -> f64 {
        match self {
            Pcsk9Variant::WildType => 0.0,
            Pcsk9Variant::LossOfFunction => -28.0,
            Pcsk9Variant::GainOfFunction => 15.0,
        }
    }

    pub fn cardiovascular_risk(&self) -> f64 {
        match self {
            Pcsk9Variant::WildType => 1.0,
            Pcsk9Variant::LossOfFunction => 0.72,
            Pcsk9Variant::GainOfFunction => 1.3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LdlrVariant {
    WildType,
    Heterozygous,
    Homozygous,
}

impl LdlrVariant {
    pub fn familial_hypercholesterolemia_risk(&self) -> f64 {
        match self {
            LdlrVariant::WildType => 1.0,
            LdlrVariant::Heterozygous => 100.0,
            LdlrVariant::Homozygous => 1000.0,
        }
    }

    pub fn ldl_level_multiplier(&self) -> f64 {
        match self {
            LdlrVariant::WildType => 1.0,
            LdlrVariant::Heterozygous => 2.5,
            LdlrVariant::Homozygous => 6.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Factor5LeidenStatus {
    Negative,
    Heterozygous,
    Homozygous,
}

impl Factor5LeidenStatus {
    pub fn thrombosis_risk(&self) -> f64 {
        match self {
            Factor5LeidenStatus::Negative => 1.0,
            Factor5LeidenStatus::Heterozygous => 5.0,
            Factor5LeidenStatus::Homozygous => 50.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mthfr677Variant {
    CC,
    CT,
    TT,
}

impl Mthfr677Variant {
    pub fn homocysteine_level_multiplier(&self) -> f64 {
        match self {
            Mthfr677Variant::CC => 1.0,
            Mthfr677Variant::CT => 1.25,
            Mthfr677Variant::TT => 1.7,
        }
    }

    pub fn cardiovascular_risk(&self) -> f64 {
        match self {
            Mthfr677Variant::CC => 1.0,
            Mthfr677Variant::CT => 1.2,
            Mthfr677Variant::TT => 1.6,
        }
    }

    pub fn requires_folate_supplementation(&self) -> bool {
        matches!(self, Mthfr677Variant::TT)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cyp2c19Phenotype {
    UltraRapid,
    Rapid,
    Normal,
    Intermediate,
    Poor,
}

impl Cyp2c19Phenotype {
    pub fn clopidogrel_response(&self) -> &'static str {
        match self {
            Cyp2c19Phenotype::UltraRapid => "Enhanced response, increased bleeding risk",
            Cyp2c19Phenotype::Rapid => "Enhanced response",
            Cyp2c19Phenotype::Normal => "Normal response",
            Cyp2c19Phenotype::Intermediate => "Reduced response, consider alternative",
            Cyp2c19Phenotype::Poor => "Minimal response, use alternative antiplatelet",
        }
    }

    pub fn ppi_metabolism(&self) -> &'static str {
        match self {
            Cyp2c19Phenotype::UltraRapid | Cyp2c19Phenotype::Rapid => {
                "Rapid metabolism, may need higher doses"
            }
            Cyp2c19Phenotype::Normal => "Normal metabolism",
            Cyp2c19Phenotype::Intermediate => "Slower metabolism, standard dosing effective",
            Cyp2c19Phenotype::Poor => "Very slow metabolism, lower doses recommended",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularGeneticProfile {
    pub apoe: ApoeGenotype,
    pub pcsk9: Pcsk9Variant,
    pub ldlr: LdlrVariant,
    pub factor5_leiden: Factor5LeidenStatus,
    pub mthfr_677: Mthfr677Variant,
    pub cyp2c19: Cyp2c19Phenotype,
}

impl Default for CardiovascularGeneticProfile {
    fn default() -> Self {
        Self {
            apoe: ApoeGenotype::e3_e3(),
            pcsk9: Pcsk9Variant::WildType,
            ldlr: LdlrVariant::WildType,
            factor5_leiden: Factor5LeidenStatus::Negative,
            mthfr_677: Mthfr677Variant::CC,
            cyp2c19: Cyp2c19Phenotype::Normal,
        }
    }
}

impl CardiovascularGeneticProfile {
    pub fn comprehensive_cardiovascular_risk(&self) -> f64 {
        let mut risk = 1.0;
        risk *= self.apoe.cardiovascular_disease_relative_risk();
        risk *= self.pcsk9.cardiovascular_risk();
        risk *= if self.ldlr != LdlrVariant::WildType {
            2.0
        } else {
            1.0
        };
        risk *= if self.factor5_leiden != Factor5LeidenStatus::Negative {
            1.5
        } else {
            1.0
        };
        risk *= self.mthfr_677.cardiovascular_risk();
        risk
    }

    pub fn estimated_ldl_effect(&self) -> f64 {
        let mut effect = 0.0;
        effect += self.apoe.ldl_cholesterol_effect();
        effect += self.pcsk9.ldl_effect();
        effect
    }

    pub fn statin_considerations(&self) -> Vec<String> {
        let mut considerations = Vec::new();

        if self.ldlr == LdlrVariant::Heterozygous || self.ldlr == LdlrVariant::Homozygous {
            considerations.push("High-intensity statin therapy indicated".to_string());
        }

        if self.apoe.e4_count() > 0 {
            considerations.push("APOE E4 carrier: monitor cognitive function".to_string());
        }

        if self.pcsk9 == Pcsk9Variant::GainOfFunction {
            considerations.push("May require PCSK9 inhibitor therapy".to_string());
        }

        considerations
    }

    pub fn antiplatelet_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        match self.cyp2c19 {
            Cyp2c19Phenotype::Poor | Cyp2c19Phenotype::Intermediate => {
                recommendations.push("Avoid clopidogrel, use prasugrel or ticagrelor".to_string());
            }
            Cyp2c19Phenotype::UltraRapid => {
                recommendations.push("Clopidogrel OK, monitor for bleeding".to_string());
            }
            _ => {
                recommendations.push("Standard antiplatelet therapy appropriate".to_string());
            }
        }

        if self.factor5_leiden != Factor5LeidenStatus::Negative {
            recommendations.push("Increased thrombosis risk, monitor closely".to_string());
        }

        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apoe_e4_risk() {
        let genotype = ApoeGenotype::e4_e4();
        assert!(genotype.has_e4());
        assert_eq!(genotype.e4_count(), 2);
        assert!(genotype.alzheimers_relative_risk() > 10.0);
    }

    #[test]
    fn test_apoe_protective() {
        let genotype = ApoeGenotype::e2_e2();
        assert!(!genotype.has_e4());
        assert!(genotype.alzheimers_relative_risk() < 1.0);
    }

    #[test]
    fn test_pcsk9_lof() {
        let variant = Pcsk9Variant::LossOfFunction;
        assert!(variant.ldl_effect() < 0.0);
        assert!(variant.cardiovascular_risk() < 1.0);
    }

    #[test]
    fn test_ldlr_homozygous() {
        let variant = LdlrVariant::Homozygous;
        assert!(variant.familial_hypercholesterolemia_risk() > 100.0);
        assert!(variant.ldl_level_multiplier() > 5.0);
    }

    #[test]
    fn test_factor5_leiden() {
        let status = Factor5LeidenStatus::Heterozygous;
        assert!(status.thrombosis_risk() > 3.0);
    }

    #[test]
    fn test_mthfr() {
        let variant = Mthfr677Variant::TT;
        assert!(variant.homocysteine_level_multiplier() > 1.5);
        assert!(variant.requires_folate_supplementation());
    }

    #[test]
    fn test_cyp2c19_poor() {
        let phenotype = Cyp2c19Phenotype::Poor;
        assert!(phenotype.clopidogrel_response().contains("alternative"));
    }

    #[test]
    fn test_comprehensive_profile() {
        let mut profile = CardiovascularGeneticProfile::default();
        profile.apoe = ApoeGenotype::e3_e4();
        profile.ldlr = LdlrVariant::Heterozygous;

        let risk = profile.comprehensive_cardiovascular_risk();
        assert!(risk > 1.0);

        let considerations = profile.statin_considerations();
        assert!(!considerations.is_empty());
    }
}

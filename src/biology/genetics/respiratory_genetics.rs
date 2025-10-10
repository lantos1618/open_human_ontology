use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CftrStatus {
    WildType,
    DeltaF508Heterozygous,
    DeltaF508Homozygous,
    G551DHeterozygous,
    OtherPathogenicCompound,
    Carrier,
}

impl CftrStatus {
    pub fn cystic_fibrosis(&self) -> bool {
        matches!(
            self,
            CftrStatus::DeltaF508Homozygous | CftrStatus::OtherPathogenicCompound
        )
    }

    pub fn carrier(&self) -> bool {
        matches!(
            self,
            CftrStatus::DeltaF508Heterozygous | CftrStatus::G551DHeterozygous | CftrStatus::Carrier
        )
    }

    pub fn ivacaftor_responsive(&self) -> bool {
        matches!(self, CftrStatus::G551DHeterozygous)
    }

    pub fn disease_severity(&self) -> &'static str {
        match self {
            CftrStatus::WildType | CftrStatus::Carrier => "No disease",
            CftrStatus::DeltaF508Heterozygous | CftrStatus::G551DHeterozygous => "Carrier only",
            CftrStatus::DeltaF508Homozygous => "Classic CF, severe",
            CftrStatus::OtherPathogenicCompound => "Variable severity",
        }
    }

    pub fn treatment_options(&self) -> Vec<String> {
        let mut treatments = Vec::new();

        if self.cystic_fibrosis() {
            treatments.push("Airway clearance techniques".to_string());
            treatments.push("Inhaled mucolytics (dornase alfa)".to_string());
            treatments.push("Inhaled antibiotics as needed".to_string());
            treatments.push("Pancreatic enzyme replacement".to_string());
            treatments.push("High-calorie diet".to_string());
        }

        if matches!(self, CftrStatus::DeltaF508Homozygous) {
            treatments.push("Elexacaftor/tezacaftor/ivacaftor (Trikafta)".to_string());
        }

        if self.ivacaftor_responsive() {
            treatments.push("Ivacaftor monotherapy".to_string());
        }

        treatments
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerpinA1Genotype {
    MM,
    MZ,
    ZZ,
    SS,
    SZ,
}

impl SerpinA1Genotype {
    pub fn alpha1_antitrypsin_deficiency(&self) -> bool {
        matches!(self, SerpinA1Genotype::ZZ | SerpinA1Genotype::SZ)
    }

    pub fn carrier(&self) -> bool {
        matches!(self, SerpinA1Genotype::MZ)
    }

    pub fn serum_aat_level_percent_normal(&self) -> f64 {
        match self {
            SerpinA1Genotype::MM => 100.0,
            SerpinA1Genotype::MZ => 60.0,
            SerpinA1Genotype::ZZ => 15.0,
            SerpinA1Genotype::SS => 60.0,
            SerpinA1Genotype::SZ => 40.0,
        }
    }

    pub fn copd_risk(&self) -> f64 {
        match self {
            SerpinA1Genotype::MM => 1.0,
            SerpinA1Genotype::MZ => 1.5,
            SerpinA1Genotype::ZZ => 20.0,
            SerpinA1Genotype::SS => 1.2,
            SerpinA1Genotype::SZ => 5.0,
        }
    }

    pub fn liver_disease_risk(&self) -> f64 {
        match self {
            SerpinA1Genotype::MM => 1.0,
            SerpinA1Genotype::MZ => 1.5,
            SerpinA1Genotype::ZZ => 10.0,
            SerpinA1Genotype::SS => 1.0,
            SerpinA1Genotype::SZ => 3.0,
        }
    }

    pub fn recommendations(&self) -> Vec<String> {
        let mut recs = Vec::new();

        if self.alpha1_antitrypsin_deficiency() {
            recs.push("Avoid smoking and secondhand smoke".to_string());
            recs.push("Avoid environmental/occupational lung irritants".to_string());
            recs.push("Annual pulmonary function tests".to_string());
            recs.push("Hepatitis A and B vaccination".to_string());
            recs.push("Consider AAT augmentation therapy".to_string());
            recs.push("Annual liver function monitoring".to_string());
        } else if self.carrier() {
            recs.push("Smoking avoidance particularly important".to_string());
            recs.push("Monitor pulmonary function if symptomatic".to_string());
        }

        recs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Adrb2Genotype {
    Arg16Arg,
    Arg16Gly,
    Gly16Gly,
}

impl Adrb2Genotype {
    pub fn asthma_severity_modifier(&self) -> &'static str {
        match self {
            Adrb2Genotype::Arg16Arg => "Better response to beta-agonists",
            Adrb2Genotype::Arg16Gly => "Intermediate response",
            Adrb2Genotype::Gly16Gly => "Reduced response to beta-agonists, may desensitize",
        }
    }

    pub fn beta_agonist_response(&self) -> f64 {
        match self {
            Adrb2Genotype::Arg16Arg => 1.2,
            Adrb2Genotype::Arg16Gly => 1.0,
            Adrb2Genotype::Gly16Gly => 0.7,
        }
    }

    pub fn treatment_considerations(&self) -> Vec<String> {
        let mut considerations = Vec::new();

        match self {
            Adrb2Genotype::Gly16Gly => {
                considerations.push("May benefit from leukotriene modifiers".to_string());
                considerations.push("Consider limiting regular LABA use".to_string());
                considerations.push("Inhaled corticosteroids particularly important".to_string());
            }
            _ => {
                considerations.push("Standard asthma treatment appropriate".to_string());
            }
        }

        considerations
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Il4raVariant {
    WildType,
    Q576RHeterozygous,
    Q576RHomozygous,
}

impl Il4raVariant {
    pub fn asthma_risk(&self) -> f64 {
        match self {
            Il4raVariant::WildType => 1.0,
            Il4raVariant::Q576RHeterozygous => 1.4,
            Il4raVariant::Q576RHomozygous => 1.8,
        }
    }

    pub fn ige_levels_modifier(&self) -> f64 {
        match self {
            Il4raVariant::WildType => 1.0,
            Il4raVariant::Q576RHeterozygous => 1.3,
            Il4raVariant::Q576RHomozygous => 1.6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tnfa308Variant {
    GG,
    GA,
    AA,
}

impl Tnfa308Variant {
    pub fn tnf_alpha_production(&self) -> &'static str {
        match self {
            Tnfa308Variant::GG => "Normal TNF-alpha production",
            Tnfa308Variant::GA => "Increased TNF-alpha production",
            Tnfa308Variant::AA => "Significantly increased TNF-alpha production",
        }
    }

    pub fn asthma_severity_modifier(&self) -> f64 {
        match self {
            Tnfa308Variant::GG => 1.0,
            Tnfa308Variant::GA => 1.3,
            Tnfa308Variant::AA => 1.6,
        }
    }

    pub fn copd_risk(&self) -> f64 {
        match self {
            Tnfa308Variant::GG => 1.0,
            Tnfa308Variant::GA => 1.5,
            Tnfa308Variant::AA => 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratoryGeneticProfile {
    pub cftr: CftrStatus,
    pub serpina1: SerpinA1Genotype,
    pub adrb2: Adrb2Genotype,
    pub il4ra: Il4raVariant,
    pub tnfa: Tnfa308Variant,
}

impl Default for RespiratoryGeneticProfile {
    fn default() -> Self {
        Self {
            cftr: CftrStatus::WildType,
            serpina1: SerpinA1Genotype::MM,
            adrb2: Adrb2Genotype::Arg16Gly,
            il4ra: Il4raVariant::WildType,
            tnfa: Tnfa308Variant::GG,
        }
    }
}

impl RespiratoryGeneticProfile {
    pub fn asthma_genetic_risk(&self) -> f64 {
        let mut risk = 1.0;
        risk *= self.il4ra.asthma_risk();
        risk *= self.tnfa.asthma_severity_modifier();
        risk
    }

    pub fn copd_genetic_risk(&self, is_smoker: bool) -> f64 {
        let mut risk = 1.0;
        risk *= self.serpina1.copd_risk();
        risk *= self.tnfa.copd_risk();

        if is_smoker {
            risk *= 3.0;
            if self.serpina1.alpha1_antitrypsin_deficiency() {
                risk *= 5.0;
            }
        }

        risk
    }

    pub fn asthma_treatment_plan(&self) -> Vec<String> {
        let mut plan = Vec::new();

        plan.push("Inhaled corticosteroid (ICS) as controller".to_string());

        match self.adrb2 {
            Adrb2Genotype::Gly16Gly => {
                plan.push("Short-acting beta-agonist PRN (limited regular use)".to_string());
                plan.push("Consider leukotriene modifier as add-on".to_string());
            }
            _ => {
                plan.push("Short-acting beta-agonist PRN".to_string());
                plan.push("LABA/ICS combination if needed".to_string());
            }
        }

        if self.il4ra != Il4raVariant::WildType {
            plan.push("May benefit from anti-IgE therapy (omalizumab) if severe".to_string());
        }

        plan
    }

    pub fn high_priority_screening(&self) -> Vec<String> {
        let mut screening = Vec::new();

        if self.cftr.cystic_fibrosis() {
            screening.push("Quarterly pulmonary function tests".to_string());
            screening.push("Annual sputum culture".to_string());
            screening.push("Sweat chloride monitoring".to_string());
        }

        if self.serpina1.alpha1_antitrypsin_deficiency() {
            screening.push("Annual spirometry".to_string());
            screening.push("Serum AAT level".to_string());
            screening.push("Annual liver function tests".to_string());
            screening.push("CT chest if symptomatic".to_string());
        }

        screening
    }

    pub fn carrier_screening_implications(&self) -> Vec<String> {
        let mut implications = Vec::new();

        if self.cftr.carrier() {
            implications.push("CFTR carrier: partner testing recommended".to_string());
        }

        if self.serpina1.carrier() {
            implications.push("AAT carrier: avoid smoking, monitor if symptomatic".to_string());
        }

        implications
    }

    pub fn personalized_recommendations(&self) -> Vec<String> {
        let mut recs = Vec::new();

        if self.cftr.cystic_fibrosis() {
            recs.extend(self.cftr.treatment_options());
        }

        if self.serpina1.alpha1_antitrypsin_deficiency() || self.serpina1 == SerpinA1Genotype::MZ {
            recs.extend(self.serpina1.recommendations());
        }

        recs.extend(self.adrb2.treatment_considerations());

        if recs.is_empty() {
            recs.push("Standard respiratory health maintenance".to_string());
        }

        recs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cftr_homozygous() {
        let status = CftrStatus::DeltaF508Homozygous;
        assert!(status.cystic_fibrosis());
        assert!(!status.carrier());
        let treatments = status.treatment_options();
        assert!(!treatments.is_empty());
    }

    #[test]
    fn test_cftr_carrier() {
        let status = CftrStatus::DeltaF508Heterozygous;
        assert!(!status.cystic_fibrosis());
        assert!(status.carrier());
    }

    #[test]
    fn test_cftr_ivacaftor() {
        let status = CftrStatus::G551DHeterozygous;
        assert!(status.carrier());
        assert!(status.ivacaftor_responsive());
    }

    #[test]
    fn test_serpina1_zz() {
        let genotype = SerpinA1Genotype::ZZ;
        assert!(genotype.alpha1_antitrypsin_deficiency());
        assert!(genotype.serum_aat_level_percent_normal() < 20.0);
        assert!(genotype.copd_risk() > 10.0);
        assert!(genotype.liver_disease_risk() > 5.0);
    }

    #[test]
    fn test_serpina1_carrier() {
        let genotype = SerpinA1Genotype::MZ;
        assert!(!genotype.alpha1_antitrypsin_deficiency());
        assert!(genotype.carrier());
        assert!(genotype.copd_risk() > 1.0);
    }

    #[test]
    fn test_adrb2() {
        let arg = Adrb2Genotype::Arg16Arg;
        let gly = Adrb2Genotype::Gly16Gly;

        assert!(arg.beta_agonist_response() > gly.beta_agonist_response());
    }

    #[test]
    fn test_il4ra_asthma() {
        let variant = Il4raVariant::Q576RHomozygous;
        assert!(variant.asthma_risk() > 1.5);
        assert!(variant.ige_levels_modifier() > 1.3);
    }

    #[test]
    fn test_tnfa_inflammation() {
        let variant = Tnfa308Variant::AA;
        assert!(variant.asthma_severity_modifier() > 1.3);
        assert!(variant.copd_risk() > 1.5);
    }

    #[test]
    fn test_comprehensive_profile() {
        let mut profile = RespiratoryGeneticProfile::default();
        profile.serpina1 = SerpinA1Genotype::ZZ;
        profile.adrb2 = Adrb2Genotype::Gly16Gly;

        let copd_risk = profile.copd_genetic_risk(true);
        assert!(copd_risk > 10.0);

        let screening = profile.high_priority_screening();
        assert!(!screening.is_empty());
    }

    #[test]
    fn test_asthma_treatment_gly16gly() {
        let mut profile = RespiratoryGeneticProfile::default();
        profile.adrb2 = Adrb2Genotype::Gly16Gly;

        let plan = profile.asthma_treatment_plan();
        assert!(plan
            .iter()
            .any(|s| s.contains("leukotriene") || s.contains("limited")));
    }
}

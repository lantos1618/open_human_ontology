use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lrrk2Status {
    WildType,
    G2019S,
    OtherPathogenic,
}

impl Lrrk2Status {
    pub fn parkinsons_risk(&self) -> f64 {
        match self {
            Lrrk2Status::WildType => 1.0,
            Lrrk2Status::G2019S => 28.0,
            Lrrk2Status::OtherPathogenic => 15.0,
        }
    }

    pub fn penetrance(&self, age: f64) -> f64 {
        match self {
            Lrrk2Status::WildType => 0.01,
            Lrrk2Status::G2019S => {
                if age < 50.0 {
                    0.17
                } else if age < 70.0 {
                    0.24
                } else {
                    0.33
                }
            }
            Lrrk2Status::OtherPathogenic => {
                if age < 70.0 {
                    0.15
                } else {
                    0.25
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SncaStatus {
    WildType,
    A53T,
    A30P,
    E46K,
    Duplication,
    Triplication,
}

impl SncaStatus {
    pub fn parkinsons_risk(&self) -> f64 {
        match self {
            SncaStatus::WildType => 1.0,
            SncaStatus::A53T => 100.0,
            SncaStatus::A30P => 100.0,
            SncaStatus::E46K => 100.0,
            SncaStatus::Duplication => 200.0,
            SncaStatus::Triplication => 500.0,
        }
    }

    pub fn age_of_onset_estimate(&self) -> Option<f64> {
        match self {
            SncaStatus::WildType => None,
            SncaStatus::A53T => Some(45.0),
            SncaStatus::A30P => Some(50.0),
            SncaStatus::E46K => Some(55.0),
            SncaStatus::Duplication => Some(35.0),
            SncaStatus::Triplication => Some(30.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gba1Status {
    WildType,
    N370S,
    L444P,
    OtherMutation,
}

impl Gba1Status {
    pub fn parkinsons_risk(&self) -> f64 {
        match self {
            Gba1Status::WildType => 1.0,
            Gba1Status::N370S => 5.4,
            Gba1Status::L444P => 9.4,
            Gba1Status::OtherMutation => 7.0,
        }
    }

    pub fn gaucher_disease_carrier(&self) -> bool {
        !matches!(self, Gba1Status::WildType)
    }

    pub fn cognitive_decline_risk(&self) -> f64 {
        match self {
            Gba1Status::WildType => 1.0,
            Gba1Status::N370S => 2.3,
            Gba1Status::L444P => 3.1,
            Gba1Status::OtherMutation => 2.5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HlaDrb1Status {
    NoRiskAlleles,
    DR15Positive,
    DR4Positive,
    BothPositive,
}

impl HlaDrb1Status {
    pub fn multiple_sclerosis_risk(&self) -> f64 {
        match self {
            HlaDrb1Status::NoRiskAlleles => 1.0,
            HlaDrb1Status::DR15Positive => 3.1,
            HlaDrb1Status::DR4Positive => 0.7,
            HlaDrb1Status::BothPositive => 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Il7raVariant {
    CC,
    CT,
    TT,
}

impl Il7raVariant {
    pub fn multiple_sclerosis_risk(&self) -> f64 {
        match self {
            Il7raVariant::CC => 1.0,
            Il7raVariant::CT => 1.18,
            Il7raVariant::TT => 1.2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bdnf66Variant {
    ValVal,
    ValMet,
    MetMet,
}

impl Bdnf66Variant {
    pub fn memory_performance(&self) -> &'static str {
        match self {
            Bdnf66Variant::ValVal => "Typical hippocampal function",
            Bdnf66Variant::ValMet => "Slightly reduced hippocampal function",
            Bdnf66Variant::MetMet => "Reduced hippocampal function, lower BDNF secretion",
        }
    }

    pub fn depression_risk(&self) -> f64 {
        match self {
            Bdnf66Variant::ValVal => 1.0,
            Bdnf66Variant::ValMet => 1.3,
            Bdnf66Variant::MetMet => 1.7,
        }
    }

    pub fn response_to_exercise(&self) -> &'static str {
        match self {
            Bdnf66Variant::ValVal => "Normal cognitive benefits from exercise",
            Bdnf66Variant::ValMet => "Moderate cognitive benefits from exercise",
            Bdnf66Variant::MetMet => "Reduced cognitive benefits from exercise",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Comt158Variant {
    ValVal,
    ValMet,
    MetMet,
}

impl Comt158Variant {
    pub fn dopamine_metabolism(&self) -> &'static str {
        match self {
            Comt158Variant::ValVal => "Fast COMT, low prefrontal dopamine (warrior)",
            Comt158Variant::ValMet => "Intermediate COMT activity",
            Comt158Variant::MetMet => "Slow COMT, high prefrontal dopamine (worrier)",
        }
    }

    pub fn stress_response(&self) -> &'static str {
        match self {
            Comt158Variant::ValVal => "Better performance under stress, worse at baseline",
            Comt158Variant::ValMet => "Balanced stress response",
            Comt158Variant::MetMet => "Better baseline performance, worse under stress",
        }
    }

    pub fn pain_sensitivity(&self) -> &'static str {
        match self {
            Comt158Variant::ValVal => "Lower pain sensitivity",
            Comt158Variant::ValMet => "Average pain sensitivity",
            Comt158Variant::MetMet => "Higher pain sensitivity",
        }
    }

    pub fn opioid_requirement_multiplier(&self) -> f64 {
        match self {
            Comt158Variant::ValVal => 0.8,
            Comt158Variant::ValMet => 1.0,
            Comt158Variant::MetMet => 1.3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Slc6a4Variant {
    LL,
    LS,
    SS,
}

impl Slc6a4Variant {
    pub fn serotonin_transporter_activity(&self) -> &'static str {
        match self {
            Slc6a4Variant::LL => "High serotonin reuptake",
            Slc6a4Variant::LS => "Intermediate serotonin reuptake",
            Slc6a4Variant::SS => "Low serotonin reuptake",
        }
    }

    pub fn depression_risk_with_stress(&self) -> f64 {
        match self {
            Slc6a4Variant::LL => 1.0,
            Slc6a4Variant::LS => 1.4,
            Slc6a4Variant::SS => 2.0,
        }
    }

    pub fn ssri_response(&self) -> &'static str {
        match self {
            Slc6a4Variant::LL => "Typical SSRI response",
            Slc6a4Variant::LS => "Variable SSRI response",
            Slc6a4Variant::SS => "May have reduced or delayed SSRI response",
        }
    }

    pub fn anxiety_risk(&self) -> f64 {
        match self {
            Slc6a4Variant::LL => 1.0,
            Slc6a4Variant::LS => 1.5,
            Slc6a4Variant::SS => 2.2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurologicalGeneticProfile {
    pub lrrk2: Lrrk2Status,
    pub snca: SncaStatus,
    pub gba1: Gba1Status,
    pub hla_drb1: HlaDrb1Status,
    pub il7ra: Il7raVariant,
    pub bdnf: Bdnf66Variant,
    pub comt: Comt158Variant,
    pub slc6a4: Slc6a4Variant,
}

impl Default for NeurologicalGeneticProfile {
    fn default() -> Self {
        Self {
            lrrk2: Lrrk2Status::WildType,
            snca: SncaStatus::WildType,
            gba1: Gba1Status::WildType,
            hla_drb1: HlaDrb1Status::NoRiskAlleles,
            il7ra: Il7raVariant::CC,
            bdnf: Bdnf66Variant::ValVal,
            comt: Comt158Variant::ValMet,
            slc6a4: Slc6a4Variant::LS,
        }
    }
}

impl NeurologicalGeneticProfile {
    pub fn parkinsons_risk(&self, age: f64) -> f64 {
        let mut risk = 1.0;
        risk *= self.lrrk2.parkinsons_risk();
        risk *= self.snca.parkinsons_risk();
        risk *= self.gba1.parkinsons_risk();

        let penetrance = self.lrrk2.penetrance(age);
        if penetrance > 0.1 {
            risk *= penetrance * 10.0;
        }

        risk
    }

    pub fn multiple_sclerosis_risk(&self) -> f64 {
        let mut risk = 1.0;
        risk *= self.hla_drb1.multiple_sclerosis_risk();
        risk *= self.il7ra.multiple_sclerosis_risk();
        risk
    }

    pub fn cognitive_profile(&self) -> Vec<String> {
        let mut profile = Vec::new();

        profile.push(format!("Memory: {}", self.bdnf.memory_performance()));
        profile.push(format!("Stress: {}", self.comt.stress_response()));
        profile.push(format!(
            "Serotonin: {}",
            self.slc6a4.serotonin_transporter_activity()
        ));

        if self.gba1.cognitive_decline_risk() > 1.5 {
            profile.push("Increased cognitive decline risk with GBA1 variant".to_string());
        }

        profile
    }

    pub fn psychiatric_risk_factors(&self) -> Vec<String> {
        let mut factors = Vec::new();

        let depression_risk =
            self.bdnf.depression_risk() * self.slc6a4.depression_risk_with_stress();
        if depression_risk > 2.0 {
            factors.push(format!(
                "Elevated depression risk with stress ({:.1}x)",
                depression_risk
            ));
        }

        let anxiety_risk = self.slc6a4.anxiety_risk();
        if anxiety_risk > 1.5 {
            factors.push(format!("Increased anxiety risk ({:.1}x)", anxiety_risk));
        }

        factors
    }

    pub fn pain_management_profile(&self) -> Vec<String> {
        let mut profile = Vec::new();

        profile.push(format!(
            "Pain sensitivity: {}",
            self.comt.pain_sensitivity()
        ));
        profile.push(format!(
            "Opioid requirement: {:.0}% of average",
            self.comt.opioid_requirement_multiplier() * 100.0
        ));

        profile
    }

    pub fn ssri_response_prediction(&self) -> String {
        self.slc6a4.ssri_response().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lrrk2_g2019s() {
        let variant = Lrrk2Status::G2019S;
        assert!(variant.parkinsons_risk() > 20.0);
        assert!(variant.penetrance(70.0) > 0.2);
    }

    #[test]
    fn test_snca_triplication() {
        let variant = SncaStatus::Triplication;
        assert!(variant.parkinsons_risk() > 100.0);
        assert!(variant.age_of_onset_estimate().unwrap() < 40.0);
    }

    #[test]
    fn test_gba1() {
        let variant = Gba1Status::L444P;
        assert!(variant.parkinsons_risk() > 5.0);
        assert!(variant.gaucher_disease_carrier());
        assert!(variant.cognitive_decline_risk() > 2.0);
    }

    #[test]
    fn test_hla_ms() {
        let status = HlaDrb1Status::DR15Positive;
        assert!(status.multiple_sclerosis_risk() > 2.0);
    }

    #[test]
    fn test_bdnf() {
        let variant = Bdnf66Variant::MetMet;
        assert!(variant.depression_risk() > 1.5);
        assert!(variant.memory_performance().contains("Reduced"));
    }

    #[test]
    fn test_comt_warrior_worrier() {
        let warrior = Comt158Variant::ValVal;
        let worrier = Comt158Variant::MetMet;

        assert!(warrior
            .stress_response()
            .contains("Better performance under stress"));
        assert!(worrier.stress_response().contains("worse under stress"));
        assert!(worrier.opioid_requirement_multiplier() > 1.0);
    }

    #[test]
    fn test_slc6a4_ss() {
        let variant = Slc6a4Variant::SS;
        assert!(variant.depression_risk_with_stress() > 1.5);
        assert!(variant.anxiety_risk() > 2.0);
    }

    #[test]
    fn test_comprehensive_profile() {
        let mut profile = NeurologicalGeneticProfile::default();
        profile.lrrk2 = Lrrk2Status::G2019S;
        profile.gba1 = Gba1Status::N370S;

        let pd_risk = profile.parkinsons_risk(60.0);
        assert!(pd_risk > 1.0);

        let cognitive = profile.cognitive_profile();
        assert!(!cognitive.is_empty());
    }

    #[test]
    fn test_psychiatric_risk() {
        let mut profile = NeurologicalGeneticProfile::default();
        profile.bdnf = Bdnf66Variant::MetMet;
        profile.slc6a4 = Slc6a4Variant::SS;

        let factors = profile.psychiatric_risk_factors();
        assert!(!factors.is_empty());
    }
}

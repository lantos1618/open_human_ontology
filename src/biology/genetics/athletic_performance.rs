use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Actn3Genotype {
    RR,
    RX,
    XX,
}

impl Actn3Genotype {
    pub fn fast_twitch_fiber_function(&self) -> &'static str {
        match self {
            Actn3Genotype::RR => "Optimal fast-twitch (type IIx) muscle fiber function",
            Actn3Genotype::RX => "Normal fast-twitch fiber function",
            Actn3Genotype::XX => "No alpha-actinin-3 in fast-twitch fibers",
        }
    }

    pub fn power_performance(&self) -> &'static str {
        match self {
            Actn3Genotype::RR => "Enhanced sprint/power performance",
            Actn3Genotype::RX => "Balanced power/endurance",
            Actn3Genotype::XX => "Reduced sprint capacity, enhanced endurance adaptation",
        }
    }

    pub fn sport_suitability(&self) -> Vec<&'static str> {
        match self {
            Actn3Genotype::RR => vec!["Sprinting", "Powerlifting", "Football", "Rugby"],
            Actn3Genotype::RX => {
                vec!["Middle distance", "Soccer", "Basketball", "All-around"]
            }
            Actn3Genotype::XX => vec!["Long distance", "Cycling", "Swimming endurance"],
        }
    }

    pub fn muscle_damage_resistance(&self) -> f64 {
        match self {
            Actn3Genotype::RR => 0.9,
            Actn3Genotype::RX => 1.0,
            Actn3Genotype::XX => 1.2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AceGenotype {
    II,
    ID,
    DD,
}

impl AceGenotype {
    pub fn endurance_performance(&self) -> &'static str {
        match self {
            AceGenotype::II => "Enhanced endurance capacity",
            AceGenotype::ID => "Intermediate endurance",
            AceGenotype::DD => "Better power, reduced endurance",
        }
    }

    pub fn vo2_max_potential(&self) -> f64 {
        match self {
            AceGenotype::II => 1.15,
            AceGenotype::ID => 1.0,
            AceGenotype::DD => 0.9,
        }
    }

    pub fn altitude_adaptation(&self) -> &'static str {
        match self {
            AceGenotype::II => "Better adaptation to high altitude",
            AceGenotype::ID => "Normal altitude adaptation",
            AceGenotype::DD => "Reduced altitude adaptation",
        }
    }

    pub fn training_response(&self) -> &'static str {
        match self {
            AceGenotype::II => "High response to endurance training",
            AceGenotype::ID => "Moderate training response",
            AceGenotype::DD => "Lower endurance training response, better strength gains",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ppargc1aVariant {
    GG,
    GS,
    SS,
}

impl Ppargc1aVariant {
    pub fn mitochondrial_biogenesis(&self) -> &'static str {
        match self {
            Ppargc1aVariant::GG => "Enhanced mitochondrial biogenesis (Gly482 isoform)",
            Ppargc1aVariant::GS => "Intermediate mitochondrial function",
            Ppargc1aVariant::SS => "Reduced aerobic capacity",
        }
    }

    pub fn endurance_training_response(&self) -> f64 {
        match self {
            Ppargc1aVariant::GG => 1.3,
            Ppargc1aVariant::GS => 1.0,
            Ppargc1aVariant::SS => 0.8,
        }
    }

    pub fn type_2_diabetes_risk(&self) -> f64 {
        match self {
            Ppargc1aVariant::GG => 1.0,
            Ppargc1aVariant::GS => 1.2,
            Ppargc1aVariant::SS => 1.4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Col5a1Genotype {
    CC,
    CT,
    TT,
}

impl Col5a1Genotype {
    pub fn tendon_injury_risk(&self) -> &'static str {
        match self {
            Col5a1Genotype::CC => "Lower risk of tendon/ligament injury",
            Col5a1Genotype::CT => "Moderate injury risk",
            Col5a1Genotype::TT => "Increased risk of tendon/ligament injury",
        }
    }

    pub fn injury_risk_multiplier(&self) -> f64 {
        match self {
            Col5a1Genotype::CC => 0.7,
            Col5a1Genotype::CT => 1.0,
            Col5a1Genotype::TT => 1.5,
        }
    }

    pub fn recommendations(&self) -> Vec<String> {
        let mut recs = Vec::new();

        match self {
            Col5a1Genotype::TT => {
                recs.push("Extended warm-up essential".to_string());
                recs.push("Progressive loading important".to_string());
                recs.push("Consider additional recovery time".to_string());
                recs.push("Focus on eccentric strengthening".to_string());
            }
            Col5a1Genotype::CT => {
                recs.push("Standard warm-up and recovery protocols".to_string());
            }
            Col5a1Genotype::CC => {
                recs.push("Standard training protocols appropriate".to_string());
            }
        }

        recs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmpD1Genotype {
    CC,
    CT,
    TT,
}

impl AmpD1Genotype {
    pub fn metabolic_efficiency(&self) -> &'static str {
        match self {
            AmpD1Genotype::CC => "Normal purine metabolism",
            AmpD1Genotype::CT => "Enhanced metabolic efficiency during exercise",
            AmpD1Genotype::TT => "Myoadenylate deaminase deficiency, improved endurance",
        }
    }

    pub fn exercise_induced_ammonia(&self) -> &'static str {
        match self {
            AmpD1Genotype::CC => "Normal ammonia production",
            AmpD1Genotype::CT => "Reduced ammonia during exercise",
            AmpD1Genotype::TT => "Minimal ammonia production, reduced fatigue",
        }
    }

    pub fn post_exercise_muscle_pain(&self) -> &'static str {
        match self {
            AmpD1Genotype::CC => "Normal recovery",
            AmpD1Genotype::CT => "Potentially faster recovery",
            AmpD1Genotype::TT => "May experience cramping, but less fatigue",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AthleticPerformanceProfile {
    pub actn3: Actn3Genotype,
    pub ace: AceGenotype,
    pub ppargc1a: Ppargc1aVariant,
    pub col5a1: Col5a1Genotype,
    pub ampd1: AmpD1Genotype,
}

impl Default for AthleticPerformanceProfile {
    fn default() -> Self {
        Self {
            actn3: Actn3Genotype::RX,
            ace: AceGenotype::ID,
            ppargc1a: Ppargc1aVariant::GS,
            col5a1: Col5a1Genotype::CT,
            ampd1: AmpD1Genotype::CC,
        }
    }
}

impl AthleticPerformanceProfile {
    pub fn power_vs_endurance_score(&self) -> f64 {
        let mut score = 0.0;

        match self.actn3 {
            Actn3Genotype::RR => score += 2.0,
            Actn3Genotype::RX => score += 0.0,
            Actn3Genotype::XX => score -= 2.0,
        }

        match self.ace {
            AceGenotype::DD => score += 1.5,
            AceGenotype::ID => score += 0.0,
            AceGenotype::II => score -= 1.5,
        }

        match self.ppargc1a {
            Ppargc1aVariant::SS => score += 1.0,
            Ppargc1aVariant::GS => score += 0.0,
            Ppargc1aVariant::GG => score -= 1.0,
        }

        score
    }

    pub fn athletic_profile_summary(&self) -> String {
        let score = self.power_vs_endurance_score();

        if score > 2.0 {
            "Power/Sprint Athlete Profile".to_string()
        } else if score > 0.5 {
            "Power-Endurance Balanced, slight power bias".to_string()
        } else if score > -0.5 {
            "Well-balanced Power/Endurance".to_string()
        } else if score > -2.0 {
            "Endurance-Power Balanced, slight endurance bias".to_string()
        } else {
            "Endurance Athlete Profile".to_string()
        }
    }

    pub fn optimal_sports(&self) -> Vec<String> {
        let score = self.power_vs_endurance_score();
        let mut sports = Vec::new();

        if score > 1.5 {
            sports.extend(
                [
                    "100m-400m sprinting",
                    "Powerlifting",
                    "Olympic weightlifting",
                    "American football",
                    "Rugby",
                ]
                .iter()
                .map(|s| s.to_string()),
            );
        } else if score > 0.0 {
            sports.extend(
                [
                    "800m-1500m middle distance",
                    "Soccer",
                    "Basketball",
                    "Tennis",
                    "CrossFit",
                ]
                .iter()
                .map(|s| s.to_string()),
            );
        } else {
            sports.extend(
                [
                    "Marathon",
                    "Long-distance cycling",
                    "Triathlon",
                    "Cross-country skiing",
                    "Swimming distance events",
                ]
                .iter()
                .map(|s| s.to_string()),
            );
        }

        sports
    }

    pub fn injury_prevention_priorities(&self) -> Vec<String> {
        let mut priorities = Vec::new();

        let injury_risk = self.col5a1.injury_risk_multiplier();
        if injury_risk > 1.2 {
            priorities.push("HIGH PRIORITY: Tendon/ligament injury prevention".to_string());
            priorities.extend(self.col5a1.recommendations());
        }

        priorities
    }

    pub fn training_recommendations(&self) -> Vec<String> {
        let mut recs = Vec::new();

        match self.ace {
            AceGenotype::II => {
                recs.push("Excellent endurance training responder".to_string());
                recs.push("Focus on aerobic base building".to_string());
            }
            AceGenotype::DD => {
                recs.push("Focus on strength and power development".to_string());
                recs.push("Shorter, higher intensity intervals".to_string());
            }
            AceGenotype::ID => {
                recs.push("Balanced training approach appropriate".to_string());
            }
        }

        if self.ppargc1a == Ppargc1aVariant::GG {
            recs.push("High mitochondrial adaptation potential".to_string());
            recs.push("Respond well to high-volume training".to_string());
        }

        if matches!(self.ampd1, AmpD1Genotype::TT) {
            recs.push("Reduced fatigue but may experience cramping".to_string());
            recs.push("Ensure adequate hydration and electrolytes".to_string());
        }

        recs
    }

    pub fn recovery_profile(&self) -> Vec<String> {
        let mut profile = Vec::new();

        let muscle_damage = self.actn3.muscle_damage_resistance();
        if muscle_damage > 1.1 {
            profile.push("Lower exercise-induced muscle damage".to_string());
        } else if muscle_damage < 0.95 {
            profile.push("Higher exercise-induced muscle damage, need more recovery".to_string());
        }

        match self.ampd1 {
            AmpD1Genotype::TT | AmpD1Genotype::CT => {
                profile.push("Enhanced metabolic recovery".to_string());
            }
            _ => {}
        }

        if profile.is_empty() {
            profile.push("Standard recovery protocols".to_string());
        }

        profile
    }

    pub fn vo2_max_genetic_potential(&self, baseline_vo2_max: f64) -> f64 {
        baseline_vo2_max
            * self.ace.vo2_max_potential()
            * self.ppargc1a.endurance_training_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actn3_rr_power() {
        let genotype = Actn3Genotype::RR;
        assert!(genotype.power_performance().contains("Enhanced"));
        assert!(genotype.sport_suitability().contains(&"Sprinting"));
    }

    #[test]
    fn test_actn3_xx_endurance() {
        let genotype = Actn3Genotype::XX;
        assert!(genotype.power_performance().contains("endurance"));
        assert!(genotype.sport_suitability().contains(&"Long distance"));
        assert!(genotype.muscle_damage_resistance() > 1.0);
    }

    #[test]
    fn test_ace_ii_endurance() {
        let genotype = AceGenotype::II;
        assert!(genotype.vo2_max_potential() > 1.0);
        assert!(genotype.endurance_performance().contains("Enhanced"));
    }

    #[test]
    fn test_ace_dd_power() {
        let genotype = AceGenotype::DD;
        assert!(genotype.vo2_max_potential() < 1.0);
        assert!(genotype.endurance_performance().contains("power"));
    }

    #[test]
    fn test_ppargc1a_gg() {
        let variant = Ppargc1aVariant::GG;
        assert!(variant.endurance_training_response() > 1.0);
        assert!(variant.type_2_diabetes_risk() == 1.0);
    }

    #[test]
    fn test_col5a1_tt_injury() {
        let genotype = Col5a1Genotype::TT;
        assert!(genotype.injury_risk_multiplier() > 1.2);
        assert!(!genotype.recommendations().is_empty());
    }

    #[test]
    fn test_ampd1_deficiency() {
        let genotype = AmpD1Genotype::TT;
        assert!(genotype.metabolic_efficiency().contains("deficiency"));
        assert!(genotype.exercise_induced_ammonia().contains("Minimal"));
    }

    #[test]
    fn test_power_athlete_profile() {
        let profile = AthleticPerformanceProfile {
            actn3: Actn3Genotype::RR,
            ace: AceGenotype::DD,
            ppargc1a: Ppargc1aVariant::SS,
            col5a1: Col5a1Genotype::CC,
            ampd1: AmpD1Genotype::CC,
        };

        let score = profile.power_vs_endurance_score();
        assert!(score > 2.0);
        assert!(profile.athletic_profile_summary().contains("Power"));
    }

    #[test]
    fn test_endurance_athlete_profile() {
        let profile = AthleticPerformanceProfile {
            actn3: Actn3Genotype::XX,
            ace: AceGenotype::II,
            ppargc1a: Ppargc1aVariant::GG,
            col5a1: Col5a1Genotype::CT,
            ampd1: AmpD1Genotype::TT,
        };

        let score = profile.power_vs_endurance_score();
        assert!(score < -2.0);
        assert!(profile.athletic_profile_summary().contains("Endurance"));
    }

    #[test]
    fn test_vo2_max_potential() {
        let profile = AthleticPerformanceProfile {
            actn3: Actn3Genotype::XX,
            ace: AceGenotype::II,
            ppargc1a: Ppargc1aVariant::GG,
            col5a1: Col5a1Genotype::CT,
            ampd1: AmpD1Genotype::CC,
        };

        let baseline = 50.0;
        let potential = profile.vo2_max_genetic_potential(baseline);
        assert!(potential > baseline);
    }

    #[test]
    fn test_injury_prevention() {
        let mut profile = AthleticPerformanceProfile::default();
        profile.col5a1 = Col5a1Genotype::TT;

        let priorities = profile.injury_prevention_priorities();
        assert!(!priorities.is_empty());
    }
}

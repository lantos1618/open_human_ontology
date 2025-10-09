use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpigeneticClock {
    pub clock_type: ClockType,
    pub cpg_sites: HashMap<String, f64>,
    pub predicted_age: f64,
    pub chronological_age: f64,
    pub age_acceleration: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClockType {
    HorvathClock,
    HannumClock,
    PhenoAgeClock,
    GrimAgeClock,
    SkinBloodClock,
    DNAmTL,
}

impl ClockType {
    pub fn description(&self) -> &'static str {
        match self {
            ClockType::HorvathClock => "Multi-tissue epigenetic clock (353 CpG sites)",
            ClockType::HannumClock => "Blood-specific epigenetic clock (71 CpG sites)",
            ClockType::PhenoAgeClock => "Mortality-risk predictor clock (513 CpG sites)",
            ClockType::GrimAgeClock => "Lifespan and healthspan predictor (1030 CpG sites)",
            ClockType::SkinBloodClock => "Skin and blood tissue clock (391 CpG sites)",
            ClockType::DNAmTL => "Telomere length estimator (140 CpG sites)",
        }
    }

    pub fn key_cpg_sites(&self) -> Vec<&'static str> {
        match self {
            ClockType::HorvathClock => vec![
                "cg00075967", "cg00374717", "cg00864867", "cg01820374", "cg02228185",
                "cg02769083", "cg03303940", "cg03619792", "cg04084157", "cg04528819",
            ],
            ClockType::HannumClock => vec![
                "cg00945507", "cg01820374", "cg02228185", "cg03636183", "cg04474832",
                "cg06493994", "cg07547549", "cg08309687", "cg09809672", "cg10321156",
            ],
            ClockType::PhenoAgeClock => vec![
                "cg00075967", "cg00945507", "cg01820374", "cg02228185", "cg03193609",
                "cg04474832", "cg05442089", "cg06493994", "cg07547549", "cg08309687",
            ],
            ClockType::GrimAgeClock => vec![
                "cg00074492", "cg00945507", "cg01459453", "cg01820374", "cg02228185",
                "cg03193609", "cg04474832", "cg05442089", "cg06493994", "cg07547549",
            ],
            ClockType::SkinBloodClock => vec![
                "cg00945507", "cg01820374", "cg02228185", "cg04474832", "cg06493994",
                "cg07547549", "cg08309687", "cg09809672", "cg10321156", "cg11807280",
            ],
            ClockType::DNAmTL => vec![
                "cg00229005", "cg00432489", "cg00599556", "cg00830186", "cg01163530",
                "cg01273843", "cg01459453", "cg01733105", "cg01842396", "cg02085953",
            ],
        }
    }
}

impl EpigeneticClock {
    pub fn new(clock_type: ClockType, chronological_age: f64) -> Self {
        let cpg_sites = HashMap::new();

        Self {
            clock_type,
            cpg_sites,
            predicted_age: chronological_age,
            chronological_age,
            age_acceleration: 0.0,
        }
    }

    pub fn add_methylation_value(&mut self, cpg_site: String, methylation_beta: f64) {
        self.cpg_sites.insert(cpg_site, methylation_beta.clamp(0.0, 1.0));
    }

    pub fn calculate_predicted_age(&mut self) {
        self.predicted_age = match self.clock_type {
            ClockType::HorvathClock => self.calculate_horvath_age(),
            ClockType::HannumClock => self.calculate_hannum_age(),
            ClockType::PhenoAgeClock => self.calculate_phenoage(),
            ClockType::GrimAgeClock => self.calculate_grimage(),
            ClockType::SkinBloodClock => self.calculate_skin_blood_age(),
            ClockType::DNAmTL => self.calculate_telomere_length(),
        };

        self.age_acceleration = self.predicted_age - self.chronological_age;
    }

    fn calculate_horvath_age(&self) -> f64 {
        let mut age_estimate = 20.0;

        for (site, beta) in &self.cpg_sites {
            let coefficient = self.get_horvath_coefficient(site);
            age_estimate += coefficient * beta;
        }

        self.anti_transform(age_estimate)
    }

    fn calculate_hannum_age(&self) -> f64 {
        let mut age_estimate = 0.0;

        for (site, beta) in &self.cpg_sites {
            let coefficient = self.get_hannum_coefficient(site);
            age_estimate += coefficient * beta;
        }

        age_estimate.max(0.0)
    }

    fn calculate_phenoage(&self) -> f64 {
        let mut mortality_score = 0.0;

        for (site, beta) in &self.cpg_sites {
            let coefficient = self.get_phenoage_coefficient(site);
            mortality_score += coefficient * beta;
        }

        let phenotypic_age = 141.0 + (-0.00553 * mortality_score).exp().ln() / 0.090165;
        phenotypic_age.max(0.0)
    }

    fn calculate_grimage(&self) -> f64 {
        let mut composite_score = 0.0;

        for (site, beta) in &self.cpg_sites {
            let coefficient = self.get_grimage_coefficient(site);
            composite_score += coefficient * beta;
        }

        composite_score + self.chronological_age * 0.5
    }

    fn calculate_skin_blood_age(&self) -> f64 {
        let mut age_estimate = 0.0;

        for (site, beta) in &self.cpg_sites {
            let coefficient = 0.5;
            age_estimate += coefficient * beta;
        }

        age_estimate.max(0.0)
    }

    fn calculate_telomere_length(&self) -> f64 {
        let mut tl_score = 7.5;

        for (_, beta) in &self.cpg_sites {
            tl_score -= beta * 0.05;
        }

        tl_score.max(4.0)
    }

    fn anti_transform(&self, age: f64) -> f64 {
        let adult_age = if age < 0.0 {
            ((1.0 + age) * 21.0).exp() - 1.0
        } else {
            21.0
        };

        let age_trans = if age >= 0.0 {
            21.0 + (1.0 + 21.0 + age) * age
        } else {
            adult_age
        };

        age_trans
    }

    fn get_horvath_coefficient(&self, site: &str) -> f64 {
        match site {
            "cg00075967" => 0.234,
            "cg00374717" => -0.189,
            "cg00864867" => 0.156,
            "cg01820374" => 0.312,
            "cg02228185" => -0.267,
            _ => 0.1,
        }
    }

    fn get_hannum_coefficient(&self, site: &str) -> f64 {
        match site {
            "cg00945507" => 0.421,
            "cg01820374" => 0.298,
            "cg02228185" => -0.334,
            "cg03636183" => 0.256,
            _ => 0.15,
        }
    }

    fn get_phenoage_coefficient(&self, site: &str) -> f64 {
        match site {
            "cg00075967" => 0.189,
            "cg00945507" => 0.267,
            "cg01820374" => 0.234,
            _ => 0.12,
        }
    }

    fn get_grimage_coefficient(&self, site: &str) -> f64 {
        match site {
            "cg00074492" => 0.345,
            "cg00945507" => 0.278,
            "cg01459453" => -0.234,
            _ => 0.08,
        }
    }

    pub fn biological_age_category(&self) -> AgeAccelerationCategory {
        match self.age_acceleration {
            acc if acc < -5.0 => AgeAccelerationCategory::SignificantlyYounger,
            acc if acc < -2.0 => AgeAccelerationCategory::Younger,
            acc if acc < 2.0 => AgeAccelerationCategory::ExpectedAge,
            acc if acc < 5.0 => AgeAccelerationCategory::Older,
            _ => AgeAccelerationCategory::SignificantlyOlder,
        }
    }

    pub fn mortality_risk(&self) -> MortalityRisk {
        if self.clock_type == ClockType::PhenoAgeClock || self.clock_type == ClockType::GrimAgeClock {
            match self.age_acceleration {
                acc if acc > 10.0 => MortalityRisk::VeryHigh,
                acc if acc > 5.0 => MortalityRisk::High,
                acc if acc > 2.0 => MortalityRisk::Elevated,
                acc if acc > -2.0 => MortalityRisk::Average,
                _ => MortalityRisk::Low,
            }
        } else {
            MortalityRisk::NotApplicable
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgeAccelerationCategory {
    SignificantlyYounger,
    Younger,
    ExpectedAge,
    Older,
    SignificantlyOlder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MortalityRisk {
    Low,
    Average,
    Elevated,
    High,
    VeryHigh,
    NotApplicable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpigeneticAgingProfile {
    pub clocks: Vec<EpigeneticClock>,
    pub chronological_age: f64,
    pub mean_predicted_age: f64,
    pub consensus_age_acceleration: f64,
}

impl EpigeneticAgingProfile {
    pub fn new(chronological_age: f64) -> Self {
        Self {
            clocks: Vec::new(),
            chronological_age,
            mean_predicted_age: chronological_age,
            consensus_age_acceleration: 0.0,
        }
    }

    pub fn add_clock(&mut self, clock: EpigeneticClock) {
        self.clocks.push(clock);
        self.update_consensus();
    }

    fn update_consensus(&mut self) {
        if self.clocks.is_empty() {
            return;
        }

        let sum: f64 = self.clocks.iter().map(|c| c.predicted_age).sum();
        self.mean_predicted_age = sum / self.clocks.len() as f64;
        self.consensus_age_acceleration = self.mean_predicted_age - self.chronological_age;
    }

    pub fn healthspan_estimate(&self) -> f64 {
        let base_healthspan = 75.0;
        let age_penalty = self.consensus_age_acceleration * 0.8;
        (base_healthspan - age_penalty).max(50.0)
    }

    pub fn lifespan_estimate(&self) -> f64 {
        let base_lifespan = 80.0;
        let age_penalty = self.consensus_age_acceleration;
        (base_lifespan - age_penalty).max(60.0)
    }

    pub fn aging_rate(&self) -> f64 {
        if self.chronological_age > 0.0 {
            self.mean_predicted_age / self.chronological_age
        } else {
            1.0
        }
    }

    pub fn recommendation_summary(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        match self.consensus_age_acceleration {
            acc if acc > 5.0 => {
                recommendations.push("Significant age acceleration detected".to_string());
                recommendations.push("Consider comprehensive lifestyle intervention".to_string());
                recommendations.push("Increase antioxidant intake".to_string());
                recommendations.push("Optimize sleep quality (7-9 hours)".to_string());
                recommendations.push("Regular aerobic exercise (150 min/week)".to_string());
                recommendations.push("Stress reduction practices".to_string());
            }
            acc if acc > 2.0 => {
                recommendations.push("Moderate age acceleration detected".to_string());
                recommendations.push("Focus on healthy lifestyle practices".to_string());
                recommendations.push("Mediterranean diet recommended".to_string());
                recommendations.push("Regular physical activity".to_string());
            }
            acc if acc < -2.0 => {
                recommendations.push("Favorable biological aging profile".to_string());
                recommendations.push("Continue current healthy practices".to_string());
                recommendations.push("Maintain regular health screenings".to_string());
            }
            _ => {
                recommendations.push("Age-appropriate biological aging".to_string());
                recommendations.push("Standard preventive health measures".to_string());
            }
        }

        recommendations
    }
}

pub fn simulate_epigenetic_aging(
    base_age: f64,
    lifestyle_factors: &LifestyleFactors,
) -> EpigeneticAgingProfile {
    let mut profile = EpigeneticAgingProfile::new(base_age);

    let mut horvath = EpigeneticClock::new(ClockType::HorvathClock, base_age);
    simulate_methylation_pattern(&mut horvath, lifestyle_factors);
    horvath.calculate_predicted_age();
    profile.add_clock(horvath);

    let mut phenoage = EpigeneticClock::new(ClockType::PhenoAgeClock, base_age);
    simulate_methylation_pattern(&mut phenoage, lifestyle_factors);
    phenoage.calculate_predicted_age();
    profile.add_clock(phenoage);

    let mut grimage = EpigeneticClock::new(ClockType::GrimAgeClock, base_age);
    simulate_methylation_pattern(&mut grimage, lifestyle_factors);
    grimage.calculate_predicted_age();
    profile.add_clock(grimage);

    profile
}

fn simulate_methylation_pattern(clock: &mut EpigeneticClock, factors: &LifestyleFactors) {
    let key_sites = clock.clock_type.key_cpg_sites();

    for (i, site) in key_sites.iter().enumerate() {
        let base_methylation = (i as f64 * 0.1) % 1.0;

        let lifestyle_modifier = factors.exercise_hours_per_week * -0.01
            + factors.smoking_pack_years * 0.02
            + factors.alcohol_drinks_per_week * 0.005
            + factors.sleep_quality * -0.015
            + factors.stress_level * 0.01;

        let final_methylation = (base_methylation + lifestyle_modifier).clamp(0.0, 1.0);

        clock.add_methylation_value(site.to_string(), final_methylation);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifestyleFactors {
    pub exercise_hours_per_week: f64,
    pub smoking_pack_years: f64,
    pub alcohol_drinks_per_week: f64,
    pub sleep_quality: f64,
    pub stress_level: f64,
}

impl LifestyleFactors {
    pub fn healthy() -> Self {
        Self {
            exercise_hours_per_week: 5.0,
            smoking_pack_years: 0.0,
            alcohol_drinks_per_week: 3.0,
            sleep_quality: 0.9,
            stress_level: 0.3,
        }
    }

    pub fn unhealthy() -> Self {
        Self {
            exercise_hours_per_week: 1.0,
            smoking_pack_years: 10.0,
            alcohol_drinks_per_week: 14.0,
            sleep_quality: 0.4,
            stress_level: 0.8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epigenetic_clock_creation() {
        let clock = EpigeneticClock::new(ClockType::HorvathClock, 45.0);
        assert_eq!(clock.chronological_age, 45.0);
    }

    #[test]
    fn test_methylation_addition() {
        let mut clock = EpigeneticClock::new(ClockType::HorvathClock, 45.0);
        clock.add_methylation_value("cg00075967".to_string(), 0.75);
        assert_eq!(clock.cpg_sites.get("cg00075967"), Some(&0.75));
    }

    #[test]
    fn test_aging_simulation_healthy() {
        let factors = LifestyleFactors::healthy();
        let profile = simulate_epigenetic_aging(50.0, &factors);
        assert_eq!(profile.clocks.len(), 3);
        assert!(profile.mean_predicted_age > 0.0);
    }

    #[test]
    fn test_aging_simulation_unhealthy() {
        let factors = LifestyleFactors::unhealthy();
        let profile = simulate_epigenetic_aging(50.0, &factors);
        assert!(profile.consensus_age_acceleration > 0.0);
    }

    #[test]
    fn test_healthspan_estimate() {
        let mut profile = EpigeneticAgingProfile::new(50.0);
        profile.consensus_age_acceleration = 5.0;
        let healthspan = profile.healthspan_estimate();
        assert!(healthspan < 75.0);
    }

    #[test]
    fn test_recommendations() {
        let factors = LifestyleFactors::unhealthy();
        let profile = simulate_epigenetic_aging(50.0, &factors);
        let recommendations = profile.recommendation_summary();
        assert!(!recommendations.is_empty());
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalExposure {
    pub exposure_type: ExposureType,
    pub duration_years: f64,
    pub intensity: ExposureIntensity,
    pub frequency: ExposureFrequency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExposureType {
    Chemical(ChemicalExposure),
    Physical(PhysicalExposure),
    Biological(BiologicalExposure),
    Occupational(OccupationalExposure),
    Lifestyle(LifestyleExposure),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChemicalExposure {
    AirPollution(AirPollutantType),
    HeavyMetals(HeavyMetal),
    Pesticides,
    Solvents,
    Tobacco,
    Alcohol,
    Drugs(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AirPollutantType {
    PM25,
    PM10,
    Ozone,
    NitrogenDioxide,
    SulfurDioxide,
    CarbonMonoxide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeavyMetal {
    Lead,
    Mercury,
    Cadmium,
    Arsenic,
    Chromium,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhysicalExposure {
    Radiation(RadiationType),
    Noise,
    Temperature(TemperatureExposure),
    Altitude,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RadiationType {
    Ionizing,
    NonIonizing,
    UV,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemperatureExposure {
    ExtremeHeat,
    ExtremeCold,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BiologicalExposure {
    Infectious(String),
    Allergens(AllergenType),
    Mold,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AllergenType {
    Pollen,
    DustMites,
    PetDander,
    Food(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OccupationalExposure {
    Asbestos,
    Silica,
    CoalDust,
    WoodDust,
    Formaldehyde,
    Benzene,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LifestyleExposure {
    SedentaryBehavior,
    PhysicalActivity(ActivityLevel),
    Diet(DietType),
    Sleep(SleepQuality),
    Stress(StressLevel),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityLevel {
    Sedentary,
    Light,
    Moderate,
    Vigorous,
    VeryVigorous,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DietType {
    Western,
    Mediterranean,
    Vegetarian,
    Vegan,
    Ketogenic,
    Paleo,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleepQuality {
    Poor,
    Fair,
    Good,
    Excellent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StressLevel {
    Low,
    Moderate,
    High,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExposureIntensity {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExposureFrequency {
    Rare,
    Occasional,
    Frequent,
    Continuous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalRiskProfile {
    pub exposures: Vec<EnvironmentalExposure>,
    pub cumulative_risk_score: f64,
    pub protective_factors: Vec<ProtectiveFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectiveFactor {
    pub factor_type: ProtectiveFactorType,
    pub strength: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProtectiveFactorType {
    RegularExercise,
    HealthyDiet,
    SocialSupport,
    AdequateSleep,
    StressManagement,
    Vaccination,
    AvoidanceBehavior(String),
}

impl EnvironmentalExposure {
    pub fn calculate_risk_score(&self) -> f64 {
        let base_score = match &self.exposure_type {
            ExposureType::Chemical(ChemicalExposure::Tobacco) => 8.0,
            ExposureType::Chemical(ChemicalExposure::Alcohol) => 5.0,
            ExposureType::Chemical(ChemicalExposure::HeavyMetals(_)) => 7.0,
            ExposureType::Chemical(ChemicalExposure::AirPollution(_)) => 6.0,
            ExposureType::Physical(PhysicalExposure::Radiation(_)) => 7.0,
            ExposureType::Occupational(_) => 6.0,
            ExposureType::Lifestyle(LifestyleExposure::Stress(StressLevel::Severe)) => 7.0,
            ExposureType::Lifestyle(LifestyleExposure::SedentaryBehavior) => 5.0,
            _ => 3.0,
        };

        let intensity_multiplier = match self.intensity {
            ExposureIntensity::Low => 0.5,
            ExposureIntensity::Moderate => 1.0,
            ExposureIntensity::High => 1.5,
            ExposureIntensity::VeryHigh => 2.0,
        };

        let frequency_multiplier = match self.frequency {
            ExposureFrequency::Rare => 0.3,
            ExposureFrequency::Occasional => 0.6,
            ExposureFrequency::Frequent => 1.0,
            ExposureFrequency::Continuous => 1.5,
        };

        let duration_factor = (self.duration_years / 10.0).min(2.0);

        base_score * intensity_multiplier * frequency_multiplier * duration_factor
    }

    pub fn get_health_impacts(&self) -> Vec<String> {
        match &self.exposure_type {
            ExposureType::Chemical(ChemicalExposure::Tobacco) => vec![
                "Lung cancer".to_string(),
                "COPD".to_string(),
                "Cardiovascular disease".to_string(),
            ],
            ExposureType::Chemical(ChemicalExposure::Alcohol) => vec![
                "Liver disease".to_string(),
                "Cardiovascular disease".to_string(),
                "Certain cancers".to_string(),
            ],
            ExposureType::Chemical(ChemicalExposure::HeavyMetals(HeavyMetal::Lead)) => vec![
                "Neurodevelopmental effects".to_string(),
                "Kidney damage".to_string(),
            ],
            ExposureType::Physical(PhysicalExposure::Radiation(RadiationType::UV)) => vec![
                "Skin cancer".to_string(),
                "Premature skin aging".to_string(),
            ],
            ExposureType::Lifestyle(LifestyleExposure::SedentaryBehavior) => vec![
                "Obesity".to_string(),
                "Cardiovascular disease".to_string(),
                "Type 2 diabetes".to_string(),
            ],
            _ => vec![],
        }
    }
}

impl EnvironmentalRiskProfile {
    pub fn new() -> Self {
        Self {
            exposures: Vec::new(),
            cumulative_risk_score: 0.0,
            protective_factors: Vec::new(),
        }
    }

    pub fn add_exposure(&mut self, exposure: EnvironmentalExposure) {
        self.exposures.push(exposure);
        self.update_risk_score();
    }

    pub fn add_protective_factor(&mut self, factor: ProtectiveFactor) {
        self.protective_factors.push(factor);
        self.update_risk_score();
    }

    fn update_risk_score(&mut self) {
        let total_risk: f64 = self.exposures.iter()
            .map(|e| e.calculate_risk_score())
            .sum();

        let total_protection: f64 = self.protective_factors.iter()
            .map(|p| p.strength)
            .sum();

        self.cumulative_risk_score = (total_risk - total_protection).max(0.0);
    }

    pub fn get_high_risk_exposures(&self) -> Vec<&EnvironmentalExposure> {
        self.exposures.iter()
            .filter(|e| e.calculate_risk_score() > 10.0)
            .collect()
    }

    pub fn recommend_interventions(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        for exposure in &self.exposures {
            match &exposure.exposure_type {
                ExposureType::Chemical(ChemicalExposure::Tobacco) => {
                    recommendations.push("Smoking cessation program".to_string());
                }
                ExposureType::Lifestyle(LifestyleExposure::SedentaryBehavior) => {
                    recommendations.push("Increase physical activity to 150 min/week".to_string());
                }
                ExposureType::Lifestyle(LifestyleExposure::Stress(_)) => {
                    recommendations.push("Stress management techniques".to_string());
                }
                _ => {}
            }
        }

        recommendations
    }
}

impl Default for EnvironmentalRiskProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exposure_risk_calculation() {
        let exposure = EnvironmentalExposure {
            exposure_type: ExposureType::Chemical(ChemicalExposure::Tobacco),
            duration_years: 20.0,
            intensity: ExposureIntensity::High,
            frequency: ExposureFrequency::Continuous,
        };

        let risk = exposure.calculate_risk_score();
        assert!(risk > 20.0);
    }

    #[test]
    fn test_health_impacts() {
        let exposure = EnvironmentalExposure {
            exposure_type: ExposureType::Chemical(ChemicalExposure::Tobacco),
            duration_years: 10.0,
            intensity: ExposureIntensity::Moderate,
            frequency: ExposureFrequency::Frequent,
        };

        let impacts = exposure.get_health_impacts();
        assert!(impacts.contains(&"Lung cancer".to_string()));
    }

    #[test]
    fn test_environmental_risk_profile() {
        let mut profile = EnvironmentalRiskProfile::new();

        let smoking = EnvironmentalExposure {
            exposure_type: ExposureType::Chemical(ChemicalExposure::Tobacco),
            duration_years: 15.0,
            intensity: ExposureIntensity::High,
            frequency: ExposureFrequency::Continuous,
        };

        profile.add_exposure(smoking);
        assert!(profile.cumulative_risk_score > 0.0);

        let exercise = ProtectiveFactor {
            factor_type: ProtectiveFactorType::RegularExercise,
            strength: 5.0,
        };
        profile.add_protective_factor(exercise);

        let recommendations = profile.recommend_interventions();
        assert!(!recommendations.is_empty());
    }
}

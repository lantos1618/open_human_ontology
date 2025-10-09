use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LongevityGene {
    APOE,
    FOXO3,
    CETP,
    APOC3,
    IGF1R,
    GHR,
    SIRT1,
    SIRT3,
    SIRT6,
    KLOTHO,
    TERC,
    TERT,
    TP53,
    MTOR,
    AMPK,
    NAD,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongevityVariant {
    pub gene: LongevityGene,
    pub rs_id: String,
    pub variant: String,
    pub effect: LongevityEffect,
    pub lifespan_impact_years: f64,
    pub healthspan_impact: HealthspanEffect,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LongevityEffect {
    ProLongevity,
    Neutral,
    AntiLongevity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthspanEffect {
    ExtendedHealthspan,
    Normal,
    ReducedHealthspan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgingProfile {
    pub biological_age_modifier: f64,
    pub longevity_score: f64,
    pub healthspan_score: f64,
    pub cellular_aging_rate: f64,
    pub telomere_attrition_rate: f64,
    pub oxidative_stress_resistance: f64,
    pub inflammation_tendency: f64,
    pub metabolic_efficiency: f64,
    pub variants: Vec<LongevityVariant>,
}

impl AgingProfile {
    pub fn new() -> Self {
        Self {
            biological_age_modifier: 1.0,
            longevity_score: 50.0,
            healthspan_score: 50.0,
            cellular_aging_rate: 1.0,
            telomere_attrition_rate: 1.0,
            oxidative_stress_resistance: 1.0,
            inflammation_tendency: 1.0,
            metabolic_efficiency: 1.0,
            variants: Vec::new(),
        }
    }

    pub fn add_variant(&mut self, variant: LongevityVariant) {
        match variant.effect {
            LongevityEffect::ProLongevity => {
                self.longevity_score += 10.0;
                self.cellular_aging_rate *= 0.9;
            }
            LongevityEffect::AntiLongevity => {
                self.longevity_score -= 10.0;
                self.cellular_aging_rate *= 1.1;
            }
            LongevityEffect::Neutral => {}
        }

        match variant.gene {
            LongevityGene::FOXO3 => {
                self.oxidative_stress_resistance *= 1.3;
                self.inflammation_tendency *= 0.8;
            }
            LongevityGene::KLOTHO => {
                self.metabolic_efficiency *= 1.2;
                self.biological_age_modifier *= 0.95;
            }
            LongevityGene::SIRT1 | LongevityGene::SIRT3 | LongevityGene::SIRT6 => {
                self.cellular_aging_rate *= 0.92;
                self.metabolic_efficiency *= 1.15;
            }
            LongevityGene::TERC | LongevityGene::TERT => {
                self.telomere_attrition_rate *= 0.85;
            }
            LongevityGene::APOE => {
                if variant.variant.contains("e4") {
                    self.biological_age_modifier *= 1.1;
                    self.longevity_score -= 5.0;
                } else if variant.variant.contains("e2") {
                    self.biological_age_modifier *= 0.9;
                    self.longevity_score += 5.0;
                }
            }
            _ => {}
        }

        self.variants.push(variant);
    }

    pub fn calculate_biological_age(&self, chronological_age: f64) -> f64 {
        chronological_age * self.biological_age_modifier * self.cellular_aging_rate
    }

    pub fn estimated_lifespan_years(&self) -> f64 {
        let base_lifespan = 78.0;
        let genetic_modifier = (self.longevity_score - 50.0) / 10.0;

        let variant_years: f64 = self.variants.iter().map(|v| v.lifespan_impact_years).sum();

        base_lifespan + genetic_modifier + variant_years
    }

    pub fn healthspan_percentage(&self) -> f64 {
        let base = 80.0;
        let modifier = (self.healthspan_score - 50.0) / 10.0 * 2.0;
        (base + modifier).clamp(50.0, 95.0)
    }

    pub fn telomere_health_score(&self) -> f64 {
        (1.0 / self.telomere_attrition_rate) * 100.0
    }

    pub fn longevity_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.oxidative_stress_resistance < 1.0 {
            recommendations.push("Increase antioxidant intake (vitamin C, E, polyphenols)".to_string());
        }

        if self.inflammation_tendency > 1.2 {
            recommendations.push("Anti-inflammatory diet (omega-3, reduce processed foods)".to_string());
        }

        if self.telomere_attrition_rate > 1.0 {
            recommendations.push("Stress reduction and regular exercise to preserve telomeres".to_string());
        }

        if self.metabolic_efficiency < 1.0 {
            recommendations.push("Intermittent fasting or caloric restriction may be beneficial".to_string());
        }

        if self.cellular_aging_rate > 1.1 {
            recommendations.push("Consider NAD+ precursors (NMN, NR) and resveratrol".to_string());
        }

        recommendations
    }
}

impl Default for AgingProfile {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_longevity_variants() -> Vec<LongevityVariant> {
    vec![
        LongevityVariant {
            gene: LongevityGene::FOXO3,
            rs_id: "rs2802292".to_string(),
            variant: "G allele".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 2.7,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "FOXO3 longevity variant, strongest genetic association with human lifespan".to_string(),
        },
        LongevityVariant {
            gene: LongevityGene::APOE,
            rs_id: "rs429358".to_string(),
            variant: "ε2 allele".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 1.5,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "APOE2 associated with longevity and reduced Alzheimer's risk".to_string(),
        },
        LongevityVariant {
            gene: LongevityGene::APOE,
            rs_id: "rs429358".to_string(),
            variant: "ε4 allele".to_string(),
            effect: LongevityEffect::AntiLongevity,
            lifespan_impact_years: -2.0,
            healthspan_impact: HealthspanEffect::ReducedHealthspan,
            description: "APOE4 associated with reduced lifespan and increased Alzheimer's risk".to_string(),
        },
        LongevityVariant {
            gene: LongevityGene::CETP,
            rs_id: "rs5882".to_string(),
            variant: "VV genotype".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 1.2,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "CETP variant increasing HDL cholesterol, associated with longevity".to_string(),
        },
        LongevityVariant {
            gene: LongevityGene::KLOTHO,
            rs_id: "rs9536314".to_string(),
            variant: "KL-VS heterozygous".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 1.8,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "Klotho variant associated with cognitive protection and longevity".to_string(),
        },
        LongevityVariant {
            gene: LongevityGene::IGF1R,
            rs_id: "rs2229765".to_string(),
            variant: "G allele".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 1.0,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "Reduced IGF-1 signaling associated with longevity".to_string(),
        },
        LongevityVariant {
            gene: LongevityGene::SIRT1,
            rs_id: "rs7895833".to_string(),
            variant: "A allele".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 0.8,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "Sirtuin 1 variant, involved in cellular stress response and metabolism".to_string(),
        },
        LongevityVariant {
            gene: LongevityGene::SIRT6,
            rs_id: "rs117385980".to_string(),
            variant: "C allele".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 1.5,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "SIRT6 variant associated with exceptional longevity".to_string(),
        },
        LongevityVariant {
            gene: LongevityGene::TERT,
            rs_id: "rs2736100".to_string(),
            variant: "C allele".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 0.5,
            healthspan_impact: HealthspanEffect::Normal,
            description: "Telomerase variant affecting telomere length maintenance".to_string(),
        },
        LongevityVariant {
            gene: LongevityGene::TP53,
            rs_id: "rs1042522".to_string(),
            variant: "GG genotype".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 0.7,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "P53 variant with enhanced tumor suppression".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aging_profile_creation() {
        let profile = AgingProfile::new();
        assert_eq!(profile.longevity_score, 50.0);
        assert_eq!(profile.cellular_aging_rate, 1.0);
    }

    #[test]
    fn test_foxo3_longevity_effect() {
        let mut profile = AgingProfile::new();

        profile.add_variant(LongevityVariant {
            gene: LongevityGene::FOXO3,
            rs_id: "rs2802292".to_string(),
            variant: "G allele".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 2.7,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "Test".to_string(),
        });

        assert!(profile.longevity_score > 50.0);
        assert!(profile.oxidative_stress_resistance > 1.0);
        assert!(profile.cellular_aging_rate < 1.0);
    }

    #[test]
    fn test_apoe4_negative_effect() {
        let mut profile = AgingProfile::new();

        profile.add_variant(LongevityVariant {
            gene: LongevityGene::APOE,
            rs_id: "rs429358".to_string(),
            variant: "e4 allele".to_string(),
            effect: LongevityEffect::AntiLongevity,
            lifespan_impact_years: -2.0,
            healthspan_impact: HealthspanEffect::ReducedHealthspan,
            description: "Test".to_string(),
        });

        assert!(profile.longevity_score < 50.0);
        assert!(profile.biological_age_modifier > 1.0);
    }

    #[test]
    fn test_biological_age_calculation() {
        let mut profile = AgingProfile::new();
        profile.biological_age_modifier = 0.9;
        profile.cellular_aging_rate = 0.95;

        let bio_age = profile.calculate_biological_age(50.0);
        assert!(bio_age < 50.0);
        assert!((bio_age - 42.75).abs() < 0.1);
    }

    #[test]
    fn test_estimated_lifespan() {
        let mut profile = AgingProfile::new();

        profile.add_variant(LongevityVariant {
            gene: LongevityGene::FOXO3,
            rs_id: "rs2802292".to_string(),
            variant: "G allele".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 2.7,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "Test".to_string(),
        });

        let lifespan = profile.estimated_lifespan_years();
        assert!(lifespan > 78.0);
    }

    #[test]
    fn test_telomere_effects() {
        let mut profile = AgingProfile::new();

        profile.add_variant(LongevityVariant {
            gene: LongevityGene::TERT,
            rs_id: "rs2736100".to_string(),
            variant: "C allele".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 0.5,
            healthspan_impact: HealthspanEffect::Normal,
            description: "Test".to_string(),
        });

        assert!(profile.telomere_attrition_rate < 1.0);
        let health_score = profile.telomere_health_score();
        assert!(health_score > 100.0);
    }

    #[test]
    fn test_sirtuin_metabolic_benefits() {
        let mut profile = AgingProfile::new();

        profile.add_variant(LongevityVariant {
            gene: LongevityGene::SIRT1,
            rs_id: "rs7895833".to_string(),
            variant: "A allele".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 0.8,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "Test".to_string(),
        });

        assert!(profile.metabolic_efficiency > 1.0);
        assert!(profile.cellular_aging_rate < 1.0);
    }

    #[test]
    fn test_longevity_recommendations() {
        let mut profile = AgingProfile::new();
        profile.inflammation_tendency = 1.3;
        profile.telomere_attrition_rate = 1.2;

        let recs = profile.longevity_recommendations();
        assert!(!recs.is_empty());
    }

    #[test]
    fn test_healthspan_percentage() {
        let profile = AgingProfile::new();
        let healthspan = profile.healthspan_percentage();
        assert!((50.0..=95.0).contains(&healthspan));
    }

    #[test]
    fn test_klotho_benefits() {
        let mut profile = AgingProfile::new();

        profile.add_variant(LongevityVariant {
            gene: LongevityGene::KLOTHO,
            rs_id: "rs9536314".to_string(),
            variant: "KL-VS".to_string(),
            effect: LongevityEffect::ProLongevity,
            lifespan_impact_years: 1.8,
            healthspan_impact: HealthspanEffect::ExtendedHealthspan,
            description: "Test".to_string(),
        });

        assert!(profile.metabolic_efficiency > 1.0);
        assert!(profile.biological_age_modifier < 1.0);
    }
}

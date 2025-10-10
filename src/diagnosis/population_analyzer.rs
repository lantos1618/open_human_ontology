use crate::biology::genetics::Ancestry;
use crate::human::Human;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationAnalyzer {
    pub individuals: Vec<Human>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationStatistics {
    pub total_count: usize,
    pub ancestry_distribution: Vec<(Ancestry, f64)>,
    pub average_age: f64,
    pub sex_distribution: SexDistribution,
    pub disease_prevalence: Vec<DiseasePrevalence>,
    pub genetic_risk_summary: GeneticRiskSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SexDistribution {
    pub male_count: usize,
    pub female_count: usize,
    pub male_percentage: f64,
    pub female_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseasePrevalence {
    pub condition: String,
    pub affected_count: usize,
    pub prevalence_rate: f64,
    pub risk_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticRiskSummary {
    pub high_risk_conditions: Vec<String>,
    pub carrier_frequencies: Vec<CarrierFrequency>,
    pub pharmacogenomic_insights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarrierFrequency {
    pub condition: String,
    pub carrier_count: usize,
    pub frequency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestryHealthProfile {
    pub ancestry: Ancestry,
    pub individual_count: usize,
    pub common_conditions: Vec<String>,
    pub protective_factors: Vec<String>,
    pub screening_recommendations: Vec<String>,
}

impl PopulationAnalyzer {
    pub fn new() -> Self {
        Self {
            individuals: Vec::new(),
        }
    }

    pub fn add_individual(&mut self, human: Human) {
        self.individuals.push(human);
    }

    pub fn analyze(&self) -> PopulationStatistics {
        let total = self.individuals.len();

        let mut male_count = 0;
        let mut female_count = 0;
        let mut total_age = 0.0;

        for individual in &self.individuals {
            match individual.demographics.biological_sex {
                crate::human::BiologicalSex::Male => male_count += 1,
                crate::human::BiologicalSex::Female => female_count += 1,
            }
            total_age += individual.demographics.age_years;
        }

        let ancestry_distribution = self.calculate_ancestry_distribution();
        let disease_prevalence = self.calculate_disease_prevalence();
        let genetic_risk_summary = self.calculate_genetic_risks();

        PopulationStatistics {
            total_count: total,
            ancestry_distribution,
            average_age: if total > 0 {
                total_age / total as f64
            } else {
                0.0
            },
            sex_distribution: SexDistribution {
                male_count,
                female_count,
                male_percentage: if total > 0 {
                    (male_count as f64 / total as f64) * 100.0
                } else {
                    0.0
                },
                female_percentage: if total > 0 {
                    (female_count as f64 / total as f64) * 100.0
                } else {
                    0.0
                },
            },
            disease_prevalence,
            genetic_risk_summary,
        }
    }

    fn calculate_ancestry_distribution(&self) -> Vec<(Ancestry, f64)> {
        let mut ancestry_counts: std::collections::HashMap<Ancestry, f64> =
            std::collections::HashMap::new();

        for individual in &self.individuals {
            for (ancestry, percentage) in individual.genetics.ancestry.components() {
                *ancestry_counts.entry(*ancestry).or_insert(0.0) += percentage;
            }
        }

        let total_ancestry_units: f64 = ancestry_counts.values().sum();

        let mut distribution: Vec<(Ancestry, f64)> = ancestry_counts
            .into_iter()
            .map(|(ancestry, count)| {
                (
                    ancestry,
                    if total_ancestry_units > 0.0 {
                        (count / total_ancestry_units) * 100.0
                    } else {
                        0.0
                    },
                )
            })
            .collect();

        distribution.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        distribution
    }

    fn calculate_disease_prevalence(&self) -> Vec<DiseasePrevalence> {
        let mut condition_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for individual in &self.individuals {
            for condition in &individual.health_conditions.active_conditions {
                *condition_counts.entry(condition.clone()).or_insert(0) += 1;
            }
        }

        let total = self.individuals.len();

        let mut prevalence: Vec<DiseasePrevalence> = condition_counts
            .into_iter()
            .map(|(condition, count)| DiseasePrevalence {
                condition: condition.clone(),
                affected_count: count,
                prevalence_rate: if total > 0 {
                    (count as f64 / total as f64) * 100.0
                } else {
                    0.0
                },
                risk_factors: Vec::new(),
            })
            .collect();

        prevalence.sort_by(|a, b| b.affected_count.cmp(&a.affected_count));
        prevalence
    }

    fn calculate_genetic_risks(&self) -> GeneticRiskSummary {
        let mut carrier_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for individual in &self.individuals {
            for carrier in &individual.genetics.carrier_status {
                *carrier_counts.entry(carrier.clone()).or_insert(0) += 1;
            }
        }

        let total = self.individuals.len();

        let carrier_frequencies: Vec<CarrierFrequency> = carrier_counts
            .into_iter()
            .map(|(condition, count)| CarrierFrequency {
                condition,
                carrier_count: count,
                frequency: if total > 0 {
                    (count as f64 / total as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect();

        GeneticRiskSummary {
            high_risk_conditions: Vec::new(),
            carrier_frequencies,
            pharmacogenomic_insights: Vec::new(),
        }
    }

    pub fn ancestry_specific_analysis(&self, ancestry: Ancestry) -> AncestryHealthProfile {
        let mut count = 0;
        let mut conditions: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for individual in &self.individuals {
            if individual
                .genetics
                .ancestry
                .components()
                .iter()
                .any(|(a, p)| *a == ancestry && *p > 25.0)
            {
                count += 1;

                let risks = individual.assess_genetic_disease_risks();
                for risk in risks {
                    *conditions.entry(risk.condition.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut common_conditions: Vec<(String, usize)> = conditions.into_iter().collect();
        common_conditions.sort_by(|a, b| b.1.cmp(&a.1));

        let common_conditions: Vec<String> = common_conditions
            .into_iter()
            .take(10)
            .map(|(c, _)| c)
            .collect();

        let screening_recommendations = ancestry.screening_recommendations();

        AncestryHealthProfile {
            ancestry,
            individual_count: count,
            common_conditions,
            protective_factors: Vec::new(),
            screening_recommendations,
        }
    }

    pub fn migraine_prevalence_analysis(&self) -> MigraineAnalysis {
        let mut total_at_risk = 0;
        let mut female_at_risk = 0;
        let mut male_at_risk = 0;

        for individual in &self.individuals {
            let migraine_info = individual.assess_migraine_risk();
            if migraine_info.risk_score > 2.0 {
                total_at_risk += 1;
                match individual.demographics.biological_sex {
                    crate::human::BiologicalSex::Female => female_at_risk += 1,
                    crate::human::BiologicalSex::Male => male_at_risk += 1,
                }
            }
        }

        let total = self.individuals.len();

        MigraineAnalysis {
            total_at_risk,
            prevalence_rate: if total > 0 {
                (total_at_risk as f64 / total as f64) * 100.0
            } else {
                0.0
            },
            female_risk_rate: if total > 0 {
                (female_at_risk as f64 / total as f64) * 100.0
            } else {
                0.0
            },
            male_risk_rate: if total > 0 {
                (male_at_risk as f64 / total as f64) * 100.0
            } else {
                0.0
            },
        }
    }

    pub fn pharmacogenomic_population_analysis(&self) -> PharmacogenomicPopulationReport {
        let mut warfarin_sensitive = 0;
        let mut slow_caffeine = 0;
        let mut alcohol_flush = 0;
        let mut opioid_risk = 0;

        for individual in &self.individuals {
            if individual
                .genetics
                .phenotype
                .pharmacological_traits
                .warfarin_sensitivity
                == crate::biology::genetics::WarfarinSensitivity::High
            {
                warfarin_sensitive += 1
            }

            if individual
                .genetics
                .phenotype
                .metabolic_traits
                .caffeine_metabolism
                == crate::biology::genetics::CaffeineMetabolism::Slow
            {
                slow_caffeine += 1
            }

            if individual
                .genetics
                .phenotype
                .metabolic_traits
                .alcohol_metabolism
                .alcohol_flush_reaction
            {
                alcohol_flush += 1;
            }

            match individual
                .genetics
                .phenotype
                .pharmacological_traits
                .opioid_metabolism
            {
                crate::biology::genetics::OpioidMetabolism::UltraRapid
                | crate::biology::genetics::OpioidMetabolism::Poor => {
                    opioid_risk += 1;
                }
                _ => {}
            }
        }

        let total = self.individuals.len();

        PharmacogenomicPopulationReport {
            warfarin_sensitive_percentage: if total > 0 {
                (warfarin_sensitive as f64 / total as f64) * 100.0
            } else {
                0.0
            },
            slow_caffeine_percentage: if total > 0 {
                (slow_caffeine as f64 / total as f64) * 100.0
            } else {
                0.0
            },
            alcohol_flush_percentage: if total > 0 {
                (alcohol_flush as f64 / total as f64) * 100.0
            } else {
                0.0
            },
            opioid_metabolism_risk_percentage: if total > 0 {
                (opioid_risk as f64 / total as f64) * 100.0
            } else {
                0.0
            },
        }
    }
}

impl Default for PopulationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigraineAnalysis {
    pub total_at_risk: usize,
    pub prevalence_rate: f64,
    pub female_risk_rate: f64,
    pub male_risk_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacogenomicPopulationReport {
    pub warfarin_sensitive_percentage: f64,
    pub slow_caffeine_percentage: f64,
    pub alcohol_flush_percentage: f64,
    pub opioid_metabolism_risk_percentage: f64,
}

impl Ancestry {
    pub fn screening_recommendations(&self) -> Vec<String> {
        match self {
            Ancestry::EastAsian => vec![
                "Gastric cancer screening starting age 40".to_string(),
                "Hepatitis B screening".to_string(),
                "Nasopharyngeal cancer awareness".to_string(),
                "High myopia eye exams".to_string(),
            ],
            Ancestry::Ashkenazi => vec![
                "BRCA1/2 genetic testing".to_string(),
                "Tay-Sachs carrier screening".to_string(),
                "Breast/ovarian cancer surveillance".to_string(),
                "Colorectal cancer screening age 40".to_string(),
            ],
            Ancestry::European => vec![
                "Skin cancer screening".to_string(),
                "Colorectal cancer screening age 45".to_string(),
                "Cardiovascular risk assessment".to_string(),
            ],
            Ancestry::SubSaharanAfrican => vec![
                "Sickle cell screening".to_string(),
                "Hypertension monitoring".to_string(),
                "Prostate cancer screening (males)".to_string(),
                "Vitamin D supplementation".to_string(),
            ],
            Ancestry::NorthAfrican => vec![
                "Hemoglobinopathy screening".to_string(),
                "Cardiovascular screening".to_string(),
            ],
            Ancestry::SouthAsian => vec![
                "Diabetes screening age 25".to_string(),
                "Cardiovascular risk assessment age 30".to_string(),
                "Hemoglobinopathy screening".to_string(),
            ],
            Ancestry::NativeAmerican => vec![
                "Diabetes screening".to_string(),
                "Cardiovascular screening".to_string(),
            ],
            Ancestry::Oceanian => vec![
                "Diabetes screening".to_string(),
                "Cardiovascular screening".to_string(),
            ],
            Ancestry::MiddleEastern => vec![
                "Hemoglobinopathy screening".to_string(),
                "Familial Mediterranean fever screening".to_string(),
            ],
            _ => vec!["Standard health screenings".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::human::Human;

    #[test]
    fn test_population_analyzer_creation() {
        let analyzer = PopulationAnalyzer::new();
        assert_eq!(analyzer.individuals.len(), 0);
    }

    #[test]
    fn test_add_individuals() {
        let mut analyzer = PopulationAnalyzer::new();
        let human1 = Human::new_adult_male("test1".to_string(), 30.0, 175.0, 75.0);
        let human2 = Human::new_adult_female("test2".to_string(), 28.0, 165.0, 60.0);

        analyzer.add_individual(human1);
        analyzer.add_individual(human2);

        assert_eq!(analyzer.individuals.len(), 2);
    }

    #[test]
    fn test_population_statistics() {
        let mut analyzer = PopulationAnalyzer::new();
        analyzer.add_individual(Human::new_adult_male("m1".to_string(), 30.0, 175.0, 75.0));
        analyzer.add_individual(Human::new_adult_female("f1".to_string(), 25.0, 165.0, 60.0));

        let stats = analyzer.analyze();

        assert_eq!(stats.total_count, 2);
        assert_eq!(stats.sex_distribution.male_count, 1);
        assert_eq!(stats.sex_distribution.female_count, 1);
        assert!((stats.average_age - 27.5).abs() < 0.1);
    }

    #[test]
    fn test_migraine_analysis() {
        let mut analyzer = PopulationAnalyzer::new();

        for i in 0..10 {
            analyzer.add_individual(Human::new_adult_female(
                format!("f{}", i),
                30.0,
                165.0,
                60.0,
            ));
        }

        let migraine_stats = analyzer.migraine_prevalence_analysis();
        assert!(migraine_stats.female_risk_rate > 0.0);
    }
}

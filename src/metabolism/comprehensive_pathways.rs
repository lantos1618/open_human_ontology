use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveMetabolicNetwork {
    pub carbohydrate_metabolism: CarbohydrateMetabolism,
    pub lipid_metabolism: LipidMetabolism,
    pub amino_acid_metabolism: AminoAcidMetabolism,
    pub nucleotide_metabolism: NucleotideMetabolism,
    pub one_carbon_metabolism: OneCarbonMetabolism,
    pub energy_coupling: EnergyCoupling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbohydrateMetabolism {
    pub glycolysis_rate: f64,
    pub gluconeogenesis_rate: f64,
    pub glycogen_synthesis_rate: f64,
    pub glycogen_breakdown_rate: f64,
    pub pentose_phosphate_pathway_flux: f64,
    pub fructose_metabolism: f64,
    pub galactose_metabolism: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidMetabolism {
    pub fatty_acid_oxidation_rate: f64,
    pub fatty_acid_synthesis_rate: f64,
    pub ketogenesis_rate: f64,
    pub ketolysis_rate: f64,
    pub cholesterol_synthesis_rate: f64,
    pub triglyceride_synthesis_rate: f64,
    pub phospholipid_synthesis_rate: f64,
    pub lipoprotein_metabolism: LipoproteinMetabolism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipoproteinMetabolism {
    pub vldl_production: f64,
    pub ldl_clearance: f64,
    pub hdl_reverse_transport: f64,
    pub chylomicron_metabolism: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AminoAcidMetabolism {
    pub protein_synthesis_rate: f64,
    pub protein_degradation_rate: f64,
    pub urea_cycle_flux: f64,
    pub branched_chain_aa_oxidation: f64,
    pub aromatic_aa_metabolism: f64,
    pub sulfur_aa_metabolism: f64,
    pub amino_acid_pools: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NucleotideMetabolism {
    pub purine_synthesis_rate: f64,
    pub pyrimidine_synthesis_rate: f64,
    pub purine_salvage_rate: f64,
    pub pyrimidine_salvage_rate: f64,
    pub nucleotide_degradation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneCarbonMetabolism {
    pub folate_cycle_flux: f64,
    pub methionine_cycle_flux: f64,
    pub homocysteine_level: f64,
    pub s_adenosylmethionine_level: f64,
    pub methylation_capacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyCoupling {
    pub atp_production_rate: f64,
    pub atp_consumption_rate: f64,
    pub atp_adp_ratio: f64,
    pub nadh_nad_ratio: f64,
    pub nadph_nadp_ratio: f64,
    pub energy_charge: f64,
}

impl ComprehensiveMetabolicNetwork {
    pub fn new_resting_state() -> Self {
        Self {
            carbohydrate_metabolism: CarbohydrateMetabolism {
                glycolysis_rate: 100.0,
                gluconeogenesis_rate: 20.0,
                glycogen_synthesis_rate: 30.0,
                glycogen_breakdown_rate: 10.0,
                pentose_phosphate_pathway_flux: 15.0,
                fructose_metabolism: 5.0,
                galactose_metabolism: 3.0,
            },
            lipid_metabolism: LipidMetabolism {
                fatty_acid_oxidation_rate: 50.0,
                fatty_acid_synthesis_rate: 20.0,
                ketogenesis_rate: 5.0,
                ketolysis_rate: 3.0,
                cholesterol_synthesis_rate: 10.0,
                triglyceride_synthesis_rate: 15.0,
                phospholipid_synthesis_rate: 8.0,
                lipoprotein_metabolism: LipoproteinMetabolism {
                    vldl_production: 10.0,
                    ldl_clearance: 8.0,
                    hdl_reverse_transport: 12.0,
                    chylomicron_metabolism: 5.0,
                },
            },
            amino_acid_metabolism: AminoAcidMetabolism {
                protein_synthesis_rate: 250.0,
                protein_degradation_rate: 230.0,
                urea_cycle_flux: 40.0,
                branched_chain_aa_oxidation: 15.0,
                aromatic_aa_metabolism: 10.0,
                sulfur_aa_metabolism: 8.0,
                amino_acid_pools: HashMap::new(),
            },
            nucleotide_metabolism: NucleotideMetabolism {
                purine_synthesis_rate: 20.0,
                pyrimidine_synthesis_rate: 18.0,
                purine_salvage_rate: 30.0,
                pyrimidine_salvage_rate: 25.0,
                nucleotide_degradation_rate: 15.0,
            },
            one_carbon_metabolism: OneCarbonMetabolism {
                folate_cycle_flux: 50.0,
                methionine_cycle_flux: 40.0,
                homocysteine_level: 10.0,
                s_adenosylmethionine_level: 100.0,
                methylation_capacity: 0.85,
            },
            energy_coupling: EnergyCoupling {
                atp_production_rate: 1000.0,
                atp_consumption_rate: 950.0,
                atp_adp_ratio: 10.0,
                nadh_nad_ratio: 0.1,
                nadph_nadp_ratio: 1.0,
                energy_charge: 0.9,
            },
        }
    }

    pub fn exercise_state(intensity: f64) -> Self {
        let mut state = Self::new_resting_state();

        let multiplier = 1.0 + (intensity * 4.0);

        state.carbohydrate_metabolism.glycolysis_rate *= multiplier;
        state.carbohydrate_metabolism.glycogen_breakdown_rate *= multiplier * 2.0;
        state.lipid_metabolism.fatty_acid_oxidation_rate *= multiplier * 1.5;
        state.energy_coupling.atp_consumption_rate *= multiplier;
        state.energy_coupling.atp_production_rate *= multiplier;

        state
    }

    pub fn fasting_state(hours_fasted: f64) -> Self {
        let mut state = Self::new_resting_state();

        let fasting_factor = (hours_fasted / 12.0).min(3.0);

        state.carbohydrate_metabolism.glycolysis_rate *= 0.5;
        state.carbohydrate_metabolism.gluconeogenesis_rate *= 1.0 + fasting_factor;
        state.carbohydrate_metabolism.glycogen_breakdown_rate *= 1.5 + fasting_factor;
        state.lipid_metabolism.fatty_acid_oxidation_rate *= 2.0 + fasting_factor;
        state.lipid_metabolism.ketogenesis_rate *= 3.0 + fasting_factor * 2.0;

        state
    }

    pub fn calculate_metabolic_flexibility(&self) -> MetabolicFlexibility {
        let glucose_reliance = self.carbohydrate_metabolism.glycolysis_rate
            / (self.carbohydrate_metabolism.glycolysis_rate
               + self.lipid_metabolism.fatty_acid_oxidation_rate);

        let fat_oxidation_capacity = self.lipid_metabolism.fatty_acid_oxidation_rate / 100.0;

        let substrate_switching_ability = 1.0 - (glucose_reliance - 0.5).abs() * 2.0;

        MetabolicFlexibility {
            glucose_reliance,
            fat_oxidation_capacity,
            substrate_switching_ability,
            metabolic_health_score: substrate_switching_ability * fat_oxidation_capacity,
        }
    }

    pub fn assess_metabolic_health(&self) -> MetabolicHealthAssessment {
        let energy_balance = self.energy_coupling.atp_production_rate
            - self.energy_coupling.atp_consumption_rate;

        let redox_balance = (self.energy_coupling.nadh_nad_ratio
                            + self.energy_coupling.nadph_nadp_ratio) / 2.0;

        let anabolic_catabolic_balance = self.amino_acid_metabolism.protein_synthesis_rate
            / self.amino_acid_metabolism.protein_degradation_rate;

        let methylation_status = if self.one_carbon_metabolism.homocysteine_level < 15.0
            && self.one_carbon_metabolism.methylation_capacity > 0.7
        {
            MethylationStatus::Optimal
        } else if self.one_carbon_metabolism.homocysteine_level < 20.0 {
            MethylationStatus::Suboptimal
        } else {
            MethylationStatus::Impaired
        };

        MetabolicHealthAssessment {
            energy_balance,
            redox_balance,
            anabolic_catabolic_balance,
            energy_charge: self.energy_coupling.energy_charge,
            methylation_status,
            overall_score: self.calculate_overall_metabolic_score(),
        }
    }

    fn calculate_overall_metabolic_score(&self) -> f64 {
        let energy_score = self.energy_coupling.energy_charge;
        let flexibility_score = self.calculate_metabolic_flexibility().metabolic_health_score;
        let methylation_score = self.one_carbon_metabolism.methylation_capacity;

        (energy_score + flexibility_score + methylation_score) / 3.0
    }

    pub fn simulate_nutrient_intake(&mut self, nutrient_type: NutrientType, amount: f64) {
        match nutrient_type {
            NutrientType::Carbohydrate => {
                self.carbohydrate_metabolism.glycolysis_rate += amount * 0.5;
                self.carbohydrate_metabolism.glycogen_synthesis_rate += amount * 0.3;
            }
            NutrientType::Protein => {
                self.amino_acid_metabolism.protein_synthesis_rate += amount * 0.4;
                self.amino_acid_metabolism.urea_cycle_flux += amount * 0.2;
            }
            NutrientType::Fat => {
                self.lipid_metabolism.fatty_acid_oxidation_rate += amount * 0.3;
                self.lipid_metabolism.triglyceride_synthesis_rate += amount * 0.4;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicFlexibility {
    pub glucose_reliance: f64,
    pub fat_oxidation_capacity: f64,
    pub substrate_switching_ability: f64,
    pub metabolic_health_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicHealthAssessment {
    pub energy_balance: f64,
    pub redox_balance: f64,
    pub anabolic_catabolic_balance: f64,
    pub energy_charge: f64,
    pub methylation_status: MethylationStatus,
    pub overall_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MethylationStatus {
    Optimal,
    Suboptimal,
    Impaired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NutrientType {
    Carbohydrate,
    Protein,
    Fat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resting_state_creation() {
        let network = ComprehensiveMetabolicNetwork::new_resting_state();
        assert!(network.carbohydrate_metabolism.glycolysis_rate > 0.0);
        assert!(network.energy_coupling.energy_charge > 0.8);
    }

    #[test]
    fn test_exercise_state() {
        let exercise = ComprehensiveMetabolicNetwork::exercise_state(0.7);
        let resting = ComprehensiveMetabolicNetwork::new_resting_state();

        assert!(exercise.carbohydrate_metabolism.glycolysis_rate > resting.carbohydrate_metabolism.glycolysis_rate);
        assert!(exercise.energy_coupling.atp_consumption_rate > resting.energy_coupling.atp_consumption_rate);
    }

    #[test]
    fn test_fasting_state() {
        let fasting = ComprehensiveMetabolicNetwork::fasting_state(24.0);
        let resting = ComprehensiveMetabolicNetwork::new_resting_state();

        assert!(fasting.lipid_metabolism.ketogenesis_rate > resting.lipid_metabolism.ketogenesis_rate);
        assert!(fasting.carbohydrate_metabolism.gluconeogenesis_rate > resting.carbohydrate_metabolism.gluconeogenesis_rate);
    }

    #[test]
    fn test_metabolic_flexibility() {
        let network = ComprehensiveMetabolicNetwork::new_resting_state();
        let flexibility = network.calculate_metabolic_flexibility();

        assert!(flexibility.glucose_reliance >= 0.0 && flexibility.glucose_reliance <= 1.0);
        assert!(flexibility.fat_oxidation_capacity > 0.0);
    }

    #[test]
    fn test_metabolic_health_assessment() {
        let network = ComprehensiveMetabolicNetwork::new_resting_state();
        let assessment = network.assess_metabolic_health();

        assert!(assessment.overall_score > 0.0 && assessment.overall_score <= 1.0);
        assert_eq!(assessment.methylation_status, MethylationStatus::Optimal);
    }

    #[test]
    fn test_nutrient_intake_simulation() {
        let mut network = ComprehensiveMetabolicNetwork::new_resting_state();
        let initial_glycolysis = network.carbohydrate_metabolism.glycolysis_rate;

        network.simulate_nutrient_intake(NutrientType::Carbohydrate, 100.0);

        assert!(network.carbohydrate_metabolism.glycolysis_rate > initial_glycolysis);
    }
}

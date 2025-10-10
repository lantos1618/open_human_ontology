use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PainPharmacogene {
    OPRM1,
    COMT,
    CYP2D6,
    CYP3A4,
    ABCB1,
    HTR1B,
    HTR2A,
    CACNA1A,
    KCNK18,
    SCN9A,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainMedicationProfile {
    pub genotypes: HashMap<PainPharmacogene, String>,
    pub opioid_response: OpioidResponseProfile,
    pub migraine_medication: MigraineMedicationResponse,
    pub cluster_medication: ClusterMedicationResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpioidResponseProfile {
    pub morphine_response: DrugResponseLevel,
    pub codeine_response: DrugResponseLevel,
    pub oxycodone_response: DrugResponseLevel,
    pub fentanyl_response: DrugResponseLevel,
    pub tramadol_response: DrugResponseLevel,
    pub addiction_risk: AddictionRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigraineMedicationResponse {
    pub triptan_response: DrugResponseLevel,
    pub cgrp_antagonist_response: DrugResponseLevel,
    pub ergotamine_response: DrugResponseLevel,
    pub preventive_beta_blocker: DrugResponseLevel,
    pub preventive_antiepileptic: DrugResponseLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterMedicationResponse {
    pub oxygen_efficacy: DrugResponseLevel,
    pub sumatriptan_response: DrugResponseLevel,
    pub verapamil_response: DrugResponseLevel,
    pub lithium_response: DrugResponseLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrugResponseLevel {
    Excellent,
    Good,
    Moderate,
    Poor,
    VeryPoor,
    Contraindicated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddictionRisk {
    VeryLow,
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl PainMedicationProfile {
    pub fn new() -> Self {
        Self {
            genotypes: HashMap::new(),
            opioid_response: OpioidResponseProfile::default(),
            migraine_medication: MigraineMedicationResponse::default(),
            cluster_medication: ClusterMedicationResponse::default(),
        }
    }

    pub fn add_genotype(&mut self, gene: PainPharmacogene, genotype: String) {
        self.genotypes.insert(gene, genotype);
        self.update_predictions();
    }

    pub fn update_predictions(&mut self) {
        self.predict_opioid_response();
        self.predict_migraine_response();
        self.predict_cluster_response();
    }

    fn predict_opioid_response(&mut self) {
        let mut morphine = DrugResponseLevel::Moderate;
        let mut codeine = DrugResponseLevel::Moderate;
        let mut addiction_risk = AddictionRisk::Moderate;

        if let Some(oprm1) = self.genotypes.get(&PainPharmacogene::OPRM1) {
            if oprm1.contains("118G") {
                morphine = DrugResponseLevel::Good;
                addiction_risk = AddictionRisk::High;
            } else if oprm1.contains("118A/118A") {
                morphine = DrugResponseLevel::Moderate;
                addiction_risk = AddictionRisk::Low;
            }
        }

        if let Some(comt) = self.genotypes.get(&PainPharmacogene::COMT) {
            if comt.contains("Met/Met") {
                morphine = DrugResponseLevel::Good;
                addiction_risk = match addiction_risk {
                    AddictionRisk::High => AddictionRisk::VeryHigh,
                    _ => AddictionRisk::High,
                };
            } else if comt.contains("Val/Val") {
                morphine = DrugResponseLevel::Poor;
                addiction_risk = AddictionRisk::Low;
            }
        }

        if let Some(cyp2d6) = self.genotypes.get(&PainPharmacogene::CYP2D6) {
            codeine = if cyp2d6.contains("*1/*1") || cyp2d6.contains("*2") {
                DrugResponseLevel::Good
            } else if cyp2d6.contains("*4") || cyp2d6.contains("*5") {
                DrugResponseLevel::VeryPoor
            } else {
                DrugResponseLevel::Moderate
            };
        }

        self.opioid_response = OpioidResponseProfile {
            morphine_response: morphine,
            codeine_response: codeine,
            oxycodone_response: DrugResponseLevel::Moderate,
            fentanyl_response: DrugResponseLevel::Good,
            tramadol_response: codeine,
            addiction_risk,
        };
    }

    fn predict_migraine_response(&mut self) {
        let mut triptan = DrugResponseLevel::Good;
        let mut beta_blocker = DrugResponseLevel::Good;

        if let Some(htr1b) = self.genotypes.get(&PainPharmacogene::HTR1B) {
            if htr1b.contains("G/G") {
                triptan = DrugResponseLevel::Excellent;
            } else if htr1b.contains("T/T") {
                triptan = DrugResponseLevel::Moderate;
            }
        }

        if let Some(cacna1a) = self.genotypes.get(&PainPharmacogene::CACNA1A) {
            if cacna1a.contains("FHM") {
                beta_blocker = DrugResponseLevel::Excellent;
            }
        }

        self.migraine_medication = MigraineMedicationResponse {
            triptan_response: triptan,
            cgrp_antagonist_response: DrugResponseLevel::Good,
            ergotamine_response: DrugResponseLevel::Moderate,
            preventive_beta_blocker: beta_blocker,
            preventive_antiepileptic: DrugResponseLevel::Good,
        };
    }

    fn predict_cluster_response(&mut self) {
        self.cluster_medication = ClusterMedicationResponse {
            oxygen_efficacy: DrugResponseLevel::Excellent,
            sumatriptan_response: DrugResponseLevel::Excellent,
            verapamil_response: DrugResponseLevel::Good,
            lithium_response: DrugResponseLevel::Moderate,
        };
    }

    pub fn recommend_pain_medication(&self, condition: &str) -> Vec<MedicationRecommendation> {
        let mut recommendations = Vec::new();

        match condition {
            "migraine" => {
                recommendations.push(MedicationRecommendation {
                    medication: "Sumatriptan".to_string(),
                    response_level: self.migraine_medication.triptan_response,
                    rationale: "First-line triptan for acute migraine".to_string(),
                    dosing_notes: self.get_triptan_dosing(),
                });

                recommendations.push(MedicationRecommendation {
                    medication: "Propranolol".to_string(),
                    response_level: self.migraine_medication.preventive_beta_blocker,
                    rationale: "Preventive therapy for frequent migraines".to_string(),
                    dosing_notes: "Start 40mg BID, titrate to 80-120mg BID".to_string(),
                });

                recommendations.push(MedicationRecommendation {
                    medication: "Erenumab (CGRP antagonist)".to_string(),
                    response_level: self.migraine_medication.cgrp_antagonist_response,
                    rationale: "Novel preventive with good tolerability".to_string(),
                    dosing_notes: "70mg or 140mg monthly SC injection".to_string(),
                });
            }
            "cluster_headache" => {
                recommendations.push(MedicationRecommendation {
                    medication: "100% Oxygen".to_string(),
                    response_level: self.cluster_medication.oxygen_efficacy,
                    rationale: "First-line abortive for cluster attacks".to_string(),
                    dosing_notes: "12-15 L/min for 15-20 minutes via non-rebreather mask"
                        .to_string(),
                });

                recommendations.push(MedicationRecommendation {
                    medication: "Sumatriptan SC".to_string(),
                    response_level: self.cluster_medication.sumatriptan_response,
                    rationale: "Fast-acting abortive therapy".to_string(),
                    dosing_notes: "6mg subcutaneous, may repeat once after 1 hour".to_string(),
                });

                recommendations.push(MedicationRecommendation {
                    medication: "Verapamil".to_string(),
                    response_level: self.cluster_medication.verapamil_response,
                    rationale: "First-line preventive for episodic and chronic cluster".to_string(),
                    dosing_notes:
                        "Start 80mg TID, titrate up to 360-960mg daily with EKG monitoring"
                            .to_string(),
                });
            }
            "chronic_pain" => {
                if self.opioid_response.morphine_response == DrugResponseLevel::Good {
                    recommendations.push(MedicationRecommendation {
                        medication: "Morphine".to_string(),
                        response_level: self.opioid_response.morphine_response,
                        rationale: format!(
                            "Good opioid response predicted. Addiction risk: {:?}",
                            self.opioid_response.addiction_risk
                        ),
                        dosing_notes: "Start low and titrate. Monitor for dependence".to_string(),
                    });
                } else {
                    recommendations.push(MedicationRecommendation {
                        medication: "Gabapentin or Pregabalin".to_string(),
                        response_level: DrugResponseLevel::Good,
                        rationale: "Alternative to opioids for neuropathic pain".to_string(),
                        dosing_notes: "Gabapentin 300mg TID or Pregabalin 75mg BID, titrate"
                            .to_string(),
                    });
                }
            }
            _ => {}
        }

        recommendations
    }

    fn get_triptan_dosing(&self) -> String {
        match self.migraine_medication.triptan_response {
            DrugResponseLevel::Excellent | DrugResponseLevel::Good => {
                "Sumatriptan 50-100mg PO or 6mg SC. May repeat once after 2 hours".to_string()
            }
            DrugResponseLevel::Moderate => {
                "Sumatriptan 100mg PO or consider 6mg SC for better response".to_string()
            }
            DrugResponseLevel::Poor => {
                "Consider alternative triptan (rizatriptan, eletriptan) or CGRP antagonist"
                    .to_string()
            }
            _ => "Consult specialist for alternative therapy".to_string(),
        }
    }
}

impl Default for PainMedicationProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for OpioidResponseProfile {
    fn default() -> Self {
        Self {
            morphine_response: DrugResponseLevel::Moderate,
            codeine_response: DrugResponseLevel::Moderate,
            oxycodone_response: DrugResponseLevel::Moderate,
            fentanyl_response: DrugResponseLevel::Moderate,
            tramadol_response: DrugResponseLevel::Moderate,
            addiction_risk: AddictionRisk::Moderate,
        }
    }
}

impl Default for MigraineMedicationResponse {
    fn default() -> Self {
        Self {
            triptan_response: DrugResponseLevel::Good,
            cgrp_antagonist_response: DrugResponseLevel::Good,
            ergotamine_response: DrugResponseLevel::Moderate,
            preventive_beta_blocker: DrugResponseLevel::Good,
            preventive_antiepileptic: DrugResponseLevel::Good,
        }
    }
}

impl Default for ClusterMedicationResponse {
    fn default() -> Self {
        Self {
            oxygen_efficacy: DrugResponseLevel::Excellent,
            sumatriptan_response: DrugResponseLevel::Excellent,
            verapamil_response: DrugResponseLevel::Good,
            lithium_response: DrugResponseLevel::Moderate,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRecommendation {
    pub medication: String,
    pub response_level: DrugResponseLevel,
    pub rationale: String,
    pub dosing_notes: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pain_medication_profile() {
        let mut profile = PainMedicationProfile::new();
        profile.add_genotype(PainPharmacogene::OPRM1, "118A/118G".to_string());

        assert_eq!(
            profile.opioid_response.morphine_response,
            DrugResponseLevel::Good
        );
    }

    #[test]
    fn test_migraine_recommendations() {
        let profile = PainMedicationProfile::new();
        let recs = profile.recommend_pain_medication("migraine");

        assert!(!recs.is_empty());
        assert!(recs.iter().any(|r| r.medication.contains("Sumatriptan")));
    }

    #[test]
    fn test_cluster_recommendations() {
        let profile = PainMedicationProfile::new();
        let recs = profile.recommend_pain_medication("cluster_headache");

        assert!(!recs.is_empty());
        assert!(recs.iter().any(|r| r.medication.contains("Oxygen")));
    }

    #[test]
    fn test_comt_effect() {
        let mut profile = PainMedicationProfile::new();
        profile.add_genotype(PainPharmacogene::COMT, "Met/Met".to_string());

        assert_eq!(profile.opioid_response.addiction_risk, AddictionRisk::High);
    }

    #[test]
    fn test_cyp2d6_codeine() {
        let mut profile = PainMedicationProfile::new();
        profile.add_genotype(PainPharmacogene::CYP2D6, "*4/*4".to_string());

        assert_eq!(
            profile.opioid_response.codeine_response,
            DrugResponseLevel::VeryPoor
        );
    }
}

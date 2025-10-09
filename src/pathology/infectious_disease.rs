use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfectiousDiseaseProfile {
    pub active_infections: Vec<Infection>,
    pub past_infections: Vec<PastInfection>,
    pub immunization_status: ImmunizationStatus,
    pub exposure_history: Vec<Exposure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Infection {
    pub pathogen: Pathogen,
    pub site: InfectionSite,
    pub severity: InfectionSeverity,
    pub onset_days_ago: u32,
    pub treatment_status: TreatmentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pathogen {
    pub organism_type: OrganismType,
    pub name: String,
    pub resistance_pattern: Option<ResistancePattern>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrganismType {
    Bacteria,
    Virus,
    Fungus,
    Parasite,
    Prion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfectionSite {
    Respiratory,
    Urinary,
    Gastrointestinal,
    Skin,
    Bloodstream,
    CNS,
    Bone,
    Joint,
    Cardiac,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfectionSeverity {
    Mild,
    Moderate,
    Severe,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreatmentStatus {
    Untreated,
    OnTreatment,
    Resolved,
    ChronicCarrier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResistancePattern {
    pub resistant_to: Vec<AntibioticClass>,
    pub susceptible_to: Vec<AntibioticClass>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AntibioticClass {
    Penicillins,
    Cephalosporins,
    Carbapenems,
    Fluoroquinolones,
    Macrolides,
    Tetracyclines,
    Aminoglycosides,
    Vancomycin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastInfection {
    pub pathogen_name: String,
    pub year_infected: u32,
    pub complications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationStatus {
    pub vaccines: HashMap<Vaccine, VaccineStatus>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Vaccine {
    MMR,
    Varicella,
    Hepatitis_A,
    Hepatitis_B,
    HPV,
    Influenza,
    COVID19,
    Tetanus,
    Pertussis,
    Polio,
    Pneumococcal,
    Meningococcal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaccineStatus {
    pub doses_received: u32,
    pub last_dose_years_ago: Option<f64>,
    pub immunity_status: ImmunityStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImmunityStatus {
    Immune,
    PartialImmunity,
    NotImmune,
    WaningImmunity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exposure {
    pub pathogen_name: String,
    pub exposure_type: ExposureType,
    pub days_ago: u32,
    pub prophylaxis_given: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExposureType {
    Direct,
    Respiratory,
    Bloodborne,
    Sexual,
    Foodborne,
    Vector,
}

impl InfectiousDiseaseProfile {
    pub fn new() -> Self {
        Self {
            active_infections: Vec::new(),
            past_infections: Vec::new(),
            immunization_status: ImmunizationStatus::new(),
            exposure_history: Vec::new(),
        }
    }

    pub fn add_infection(&mut self, infection: Infection) {
        self.active_infections.push(infection);
    }

    pub fn add_exposure(&mut self, exposure: Exposure) {
        self.exposure_history.push(exposure);
    }

    pub fn sepsis_risk(&self) -> SepsisRisk {
        let bloodstream_infection = self.active_infections.iter()
            .any(|i| i.site == InfectionSite::Bloodstream);

        let severe_infection = self.active_infections.iter()
            .any(|i| matches!(i.severity, InfectionSeverity::Severe | InfectionSeverity::Critical));

        match (bloodstream_infection, severe_infection) {
            (true, _) => SepsisRisk::High,
            (false, true) => SepsisRisk::Moderate,
            (false, false) => SepsisRisk::Low,
        }
    }

    pub fn requires_isolation(&self) -> bool {
        self.active_infections.iter().any(|i| {
            matches!(i.pathogen.organism_type, OrganismType::Bacteria) &&
            i.pathogen.resistance_pattern.as_ref()
                .map(|r| r.resistant_to.contains(&AntibioticClass::Vancomycin))
                .unwrap_or(false)
        }) ||
        self.active_infections.iter().any(|i| {
            i.site == InfectionSite::Respiratory &&
            matches!(i.severity, InfectionSeverity::Moderate | InfectionSeverity::Severe)
        })
    }

    pub fn vaccination_recommendations(&self) -> Vec<Vaccine> {
        let mut recommendations = Vec::new();

        for (vaccine, status) in &self.immunization_status.vaccines {
            if matches!(status.immunity_status, ImmunityStatus::NotImmune | ImmunityStatus::WaningImmunity) {
                recommendations.push(*vaccine);
            }
        }

        recommendations
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SepsisRisk {
    Low,
    Moderate,
    High,
}

impl Default for InfectiousDiseaseProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl ImmunizationStatus {
    pub fn new() -> Self {
        let mut vaccines = HashMap::new();
        vaccines.insert(Vaccine::MMR, VaccineStatus::unvaccinated());
        vaccines.insert(Vaccine::Tetanus, VaccineStatus::unvaccinated());
        vaccines.insert(Vaccine::COVID19, VaccineStatus::unvaccinated());

        Self { vaccines }
    }

    pub fn update_vaccine(&mut self, vaccine: Vaccine, status: VaccineStatus) {
        self.vaccines.insert(vaccine, status);
    }

    pub fn is_up_to_date(&self, vaccine: &Vaccine) -> bool {
        self.vaccines.get(vaccine)
            .map(|s| matches!(s.immunity_status, ImmunityStatus::Immune))
            .unwrap_or(false)
    }
}

impl VaccineStatus {
    pub fn unvaccinated() -> Self {
        Self {
            doses_received: 0,
            last_dose_years_ago: None,
            immunity_status: ImmunityStatus::NotImmune,
        }
    }

    pub fn fully_vaccinated(doses: u32, years_ago: f64) -> Self {
        let immunity = if years_ago > 10.0 {
            ImmunityStatus::WaningImmunity
        } else {
            ImmunityStatus::Immune
        };

        Self {
            doses_received: doses,
            last_dose_years_ago: Some(years_ago),
            immunity_status: immunity,
        }
    }
}

impl Vaccine {
    pub fn recommended_doses(&self) -> u32 {
        match self {
            Vaccine::MMR => 2,
            Vaccine::Varicella => 2,
            Vaccine::Hepatitis_A => 2,
            Vaccine::Hepatitis_B => 3,
            Vaccine::HPV => 3,
            Vaccine::Influenza => 1,
            Vaccine::COVID19 => 2,
            Vaccine::Tetanus => 1,
            Vaccine::Pertussis => 1,
            Vaccine::Polio => 4,
            Vaccine::Pneumococcal => 1,
            Vaccine::Meningococcal => 1,
        }
    }

    pub fn booster_interval_years(&self) -> Option<f64> {
        match self {
            Vaccine::Tetanus => Some(10.0),
            Vaccine::Influenza => Some(1.0),
            Vaccine::COVID19 => Some(1.0),
            Vaccine::Pneumococcal => Some(5.0),
            _ => None,
        }
    }
}

impl Pathogen {
    pub fn is_multidrug_resistant(&self) -> bool {
        self.resistance_pattern.as_ref()
            .map(|r| r.resistant_to.len() >= 3)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infectious_disease_profile_creation() {
        let profile = InfectiousDiseaseProfile::new();
        assert!(profile.active_infections.is_empty());
    }

    #[test]
    fn test_sepsis_risk() {
        let mut profile = InfectiousDiseaseProfile::new();

        profile.add_infection(Infection {
            pathogen: Pathogen {
                organism_type: OrganismType::Bacteria,
                name: "E. coli".to_string(),
                resistance_pattern: None,
            },
            site: InfectionSite::Bloodstream,
            severity: InfectionSeverity::Severe,
            onset_days_ago: 2,
            treatment_status: TreatmentStatus::OnTreatment,
        });

        assert_eq!(profile.sepsis_risk(), SepsisRisk::High);
    }

    #[test]
    fn test_isolation_requirement() {
        let mut profile = InfectiousDiseaseProfile::new();

        profile.add_infection(Infection {
            pathogen: Pathogen {
                organism_type: OrganismType::Bacteria,
                name: "MRSA".to_string(),
                resistance_pattern: Some(ResistancePattern {
                    resistant_to: vec![AntibioticClass::Vancomycin],
                    susceptible_to: vec![],
                }),
            },
            site: InfectionSite::Skin,
            severity: InfectionSeverity::Moderate,
            onset_days_ago: 5,
            treatment_status: TreatmentStatus::OnTreatment,
        });

        assert!(profile.requires_isolation());
    }

    #[test]
    fn test_vaccine_doses() {
        assert_eq!(Vaccine::Hepatitis_B.recommended_doses(), 3);
        assert_eq!(Vaccine::MMR.recommended_doses(), 2);
    }

    #[test]
    fn test_booster_interval() {
        assert_eq!(Vaccine::Tetanus.booster_interval_years(), Some(10.0));
        assert_eq!(Vaccine::MMR.booster_interval_years(), None);
    }

    #[test]
    fn test_immunization_status() {
        let mut status = ImmunizationStatus::new();

        status.update_vaccine(
            Vaccine::Tetanus,
            VaccineStatus::fully_vaccinated(1, 2.0)
        );

        assert!(status.is_up_to_date(&Vaccine::Tetanus));
    }

    #[test]
    fn test_multidrug_resistance() {
        let pathogen = Pathogen {
            organism_type: OrganismType::Bacteria,
            name: "XDR-TB".to_string(),
            resistance_pattern: Some(ResistancePattern {
                resistant_to: vec![
                    AntibioticClass::Penicillins,
                    AntibioticClass::Fluoroquinolones,
                    AntibioticClass::Aminoglycosides,
                ],
                susceptible_to: vec![],
            }),
        };

        assert!(pathogen.is_multidrug_resistant());
    }

    #[test]
    fn test_vaccination_recommendations() {
        let mut profile = InfectiousDiseaseProfile::new();

        let recommendations = profile.vaccination_recommendations();
        assert!(!recommendations.is_empty());
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pharmacokinetics {
    pub absorption: Absorption,
    pub distribution: Distribution,
    pub metabolism: Metabolism,
    pub excretion: Excretion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Absorption {
    pub bioavailability: f64,
    pub time_to_peak_hours: f64,
    pub food_effect: FoodEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FoodEffect {
    None,
    Increased,
    Decreased,
    MustTakeWithFood,
    MustTakeOnEmptyStomach,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Distribution {
    pub volume_of_distribution_l: f64,
    pub protein_binding_percent: f64,
    pub crosses_blood_brain_barrier: bool,
    pub placental_transfer: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metabolism {
    pub primary_pathway: MetabolicPathway,
    pub cytochrome_p450_enzymes: Vec<String>,
    pub active_metabolites: Vec<String>,
    pub half_life_hours: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolicPathway {
    Hepatic,
    Renal,
    Both,
    Minimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Excretion {
    pub primary_route: ExcretionRoute,
    pub percent_unchanged: f64,
    pub clearance_ml_per_min: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExcretionRoute {
    Renal,
    Biliary,
    Both,
    Pulmonary,
}

impl Pharmacokinetics {
    pub fn new(
        bioavailability: f64,
        half_life_hours: f64,
        volume_of_distribution_l: f64,
    ) -> Self {
        Pharmacokinetics {
            absorption: Absorption {
                bioavailability: bioavailability.clamp(0.0, 1.0),
                time_to_peak_hours: 1.0,
                food_effect: FoodEffect::None,
            },
            distribution: Distribution {
                volume_of_distribution_l,
                protein_binding_percent: 0.0,
                crosses_blood_brain_barrier: false,
                placental_transfer: false,
            },
            metabolism: Metabolism {
                primary_pathway: MetabolicPathway::Hepatic,
                cytochrome_p450_enzymes: Vec::new(),
                active_metabolites: Vec::new(),
                half_life_hours,
            },
            excretion: Excretion {
                primary_route: ExcretionRoute::Renal,
                percent_unchanged: 50.0,
                clearance_ml_per_min: 100.0,
            },
        }
    }

    pub fn time_to_steady_state_hours(&self) -> f64 {
        self.metabolism.half_life_hours * 5.0
    }

    pub fn elimination_constant(&self) -> f64 {
        0.693 / self.metabolism.half_life_hours
    }

    pub fn calculate_concentration(
        &self,
        dose_mg: f64,
        time_hours: f64,
    ) -> f64 {
        let c0 = (dose_mg * self.absorption.bioavailability) /
                 self.distribution.volume_of_distribution_l;
        let k = self.elimination_constant();
        c0 * (-k * time_hours).exp()
    }

    pub fn requires_dose_adjustment_renal(&self) -> bool {
        matches!(
            self.excretion.primary_route,
            ExcretionRoute::Renal | ExcretionRoute::Both
        ) && self.excretion.percent_unchanged > 30.0
    }

    pub fn requires_dose_adjustment_hepatic(&self) -> bool {
        matches!(
            self.metabolism.primary_pathway,
            MetabolicPathway::Hepatic | MetabolicPathway::Both
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoseAdjustment {
    pub condition: String,
    pub adjustment_factor: f64,
    pub recommendation: String,
}

impl DoseAdjustment {
    pub fn renal_impairment(creatinine_clearance: f64) -> Self {
        let (factor, recommendation) = if creatinine_clearance < 10.0 {
            (0.25, "Severe renal impairment: reduce dose to 25%")
        } else if creatinine_clearance < 30.0 {
            (0.50, "Moderate renal impairment: reduce dose to 50%")
        } else if creatinine_clearance < 60.0 {
            (0.75, "Mild renal impairment: reduce dose to 75%")
        } else {
            (1.0, "Normal renal function: no adjustment needed")
        };

        DoseAdjustment {
            condition: format!("CrCl: {:.1} mL/min", creatinine_clearance),
            adjustment_factor: factor,
            recommendation: recommendation.to_string(),
        }
    }

    pub fn hepatic_impairment(child_pugh_score: u8) -> Self {
        let (factor, recommendation) = match child_pugh_score {
            1..=6 => (1.0, "Child-Pugh A: no adjustment needed"),
            7..=9 => (0.75, "Child-Pugh B: reduce dose to 75%"),
            _ => (0.50, "Child-Pugh C: reduce dose to 50% or avoid"),
        };

        DoseAdjustment {
            condition: format!("Child-Pugh score: {}", child_pugh_score),
            adjustment_factor: factor,
            recommendation: recommendation.to_string(),
        }
    }

    pub fn adjusted_dose(&self, standard_dose: f64) -> f64 {
        standard_dose * self.adjustment_factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pharmacokinetics() {
        let pk = Pharmacokinetics::new(0.8, 6.0, 50.0);

        assert_eq!(pk.time_to_steady_state_hours(), 30.0);
        assert!(pk.elimination_constant() > 0.0);
    }

    #[test]
    fn test_concentration_calculation() {
        let pk = Pharmacokinetics::new(1.0, 2.0, 50.0);
        let c0 = pk.calculate_concentration(100.0, 0.0);
        let c2 = pk.calculate_concentration(100.0, 2.0);

        assert!(c0 > c2);
        assert!(c2 > 0.0);
    }

    #[test]
    fn test_dose_adjustment_renal() {
        let adjustment = DoseAdjustment::renal_impairment(25.0);
        assert_eq!(adjustment.adjustment_factor, 0.50);
        assert_eq!(adjustment.adjusted_dose(100.0), 50.0);
    }

    #[test]
    fn test_dose_adjustment_hepatic() {
        let adjustment = DoseAdjustment::hepatic_impairment(8);
        assert_eq!(adjustment.adjustment_factor, 0.75);
        assert_eq!(adjustment.adjusted_dose(200.0), 150.0);
    }
}

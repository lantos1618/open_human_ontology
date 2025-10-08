use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pharmacodynamics {
    pub mechanism: MechanismOfAction,
    pub receptor_binding: Option<ReceptorBinding>,
    pub dose_response: DoseResponseCurve,
    pub therapeutic_index: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MechanismOfAction {
    ReceptorAgonist {
        receptor: String,
        efficacy: f64,
    },
    ReceptorAntagonist {
        receptor: String,
        reversible: bool,
    },
    EnzymeInhibitor {
        enzyme: String,
        competitive: bool,
    },
    EnzymeActivator {
        enzyme: String,
    },
    ChannelBlocker {
        channel: String,
        voltage_dependent: bool,
    },
    ChannelOpener {
        channel: String,
    },
    DNAIntercalation,
    ImmuneModulation {
        target: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceptorBinding {
    pub receptor_type: String,
    pub affinity_nm: f64,
    pub selectivity: f64,
    pub binding_type: BindingType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BindingType {
    Reversible,
    Irreversible,
    Allosteric,
    Competitive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoseResponseCurve {
    pub ed50: f64,
    pub emax: f64,
    pub hill_coefficient: f64,
}

impl DoseResponseCurve {
    pub fn new(ed50: f64, emax: f64) -> Self {
        DoseResponseCurve {
            ed50,
            emax,
            hill_coefficient: 1.0,
        }
    }

    pub fn response_at_dose(&self, dose: f64) -> f64 {
        let numerator = self.emax * dose.powf(self.hill_coefficient);
        let denominator = self.ed50.powf(self.hill_coefficient) +
                         dose.powf(self.hill_coefficient);
        numerator / denominator
    }

    pub fn dose_for_response(&self, desired_response: f64) -> f64 {
        if desired_response >= self.emax {
            return f64::INFINITY;
        }
        let ratio = desired_response / (self.emax - desired_response);
        self.ed50 * ratio.powf(1.0 / self.hill_coefficient)
    }
}

impl Pharmacodynamics {
    pub fn new(mechanism: MechanismOfAction, ed50: f64, emax: f64) -> Self {
        Pharmacodynamics {
            mechanism,
            receptor_binding: None,
            dose_response: DoseResponseCurve::new(ed50, emax),
            therapeutic_index: 1.0,
        }
    }

    pub fn with_receptor_binding(mut self, binding: ReceptorBinding) -> Self {
        self.receptor_binding = Some(binding);
        self
    }

    pub fn with_therapeutic_index(mut self, ti: f64) -> Self {
        self.therapeutic_index = ti;
        self
    }

    pub fn is_safe(&self) -> bool {
        self.therapeutic_index > 10.0
    }

    pub fn is_narrow_therapeutic_window(&self) -> bool {
        self.therapeutic_index < 2.0
    }

    pub fn calculate_efficacy(&self, dose: f64) -> f64 {
        self.dose_response.response_at_dose(dose)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrugTarget {
    pub target_type: TargetType,
    pub name: String,
    pub pathway: String,
    pub downstream_effects: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetType {
    Receptor,
    Enzyme,
    IonChannel,
    Transporter,
    NucleicAcid,
    StructuralProtein,
}

impl DrugTarget {
    pub fn new(target_type: TargetType, name: String, pathway: String) -> Self {
        DrugTarget {
            target_type,
            name,
            pathway,
            downstream_effects: Vec::new(),
        }
    }

    pub fn add_downstream_effect(&mut self, effect: String) {
        if !self.downstream_effects.contains(&effect) {
            self.downstream_effects.push(effect);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dose_response_curve() {
        let curve = DoseResponseCurve::new(10.0, 100.0);

        let response_at_ed50 = curve.response_at_dose(10.0);
        assert!((response_at_ed50 - 50.0).abs() < 1.0);

        let dose_for_75 = curve.dose_for_response(75.0);
        assert!(dose_for_75 > 10.0);
    }

    #[test]
    fn test_pharmacodynamics() {
        let pd = Pharmacodynamics::new(
            MechanismOfAction::ReceptorAgonist {
                receptor: "Beta-2".to_string(),
                efficacy: 0.9,
            },
            5.0,
            100.0,
        ).with_therapeutic_index(15.0);

        assert!(pd.is_safe());
        assert!(!pd.is_narrow_therapeutic_window());

        let efficacy = pd.calculate_efficacy(5.0);
        assert!(efficacy > 40.0 && efficacy < 60.0);
    }

    #[test]
    fn test_drug_target() {
        let mut target = DrugTarget::new(
            TargetType::Receptor,
            "ACE".to_string(),
            "RAAS".to_string(),
        );

        target.add_downstream_effect("Decreased angiotensin II".to_string());
        target.add_downstream_effect("Decreased blood pressure".to_string());

        assert_eq!(target.downstream_effects.len(), 2);
    }
}

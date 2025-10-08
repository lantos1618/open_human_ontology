use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hormone {
    pub name: String,
    pub hormone_type: HormoneType,
    pub concentration_ng_ml: f64,
    pub half_life_minutes: f64,
    pub target_tissues: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HormoneType {
    Peptide,
    Steroid,
    Amine,
    Eicosanoid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormoneReceptor {
    pub receptor_type: ReceptorType,
    pub affinity_nm: f64,
    pub signaling_pathway: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReceptorType {
    GProteinCoupled,
    TyrosineKinase,
    Nuclear,
    IonChannel,
}

impl Hormone {
    pub fn new_insulin() -> Self {
        Self {
            name: "Insulin".to_string(),
            hormone_type: HormoneType::Peptide,
            concentration_ng_ml: 0.5,
            half_life_minutes: 5.0,
            target_tissues: vec![
                "Muscle".to_string(),
                "Liver".to_string(),
                "Adipose".to_string(),
            ],
        }
    }

    pub fn new_glucagon() -> Self {
        Self {
            name: "Glucagon".to_string(),
            hormone_type: HormoneType::Peptide,
            concentration_ng_ml: 0.05,
            half_life_minutes: 6.0,
            target_tissues: vec!["Liver".to_string()],
        }
    }

    pub fn new_cortisol() -> Self {
        Self {
            name: "Cortisol".to_string(),
            hormone_type: HormoneType::Steroid,
            concentration_ng_ml: 150.0,
            half_life_minutes: 90.0,
            target_tissues: vec![
                "Liver".to_string(),
                "Muscle".to_string(),
                "Adipose".to_string(),
                "Immune".to_string(),
            ],
        }
    }

    pub fn new_epinephrine() -> Self {
        Self {
            name: "Epinephrine".to_string(),
            hormone_type: HormoneType::Amine,
            concentration_ng_ml: 0.05,
            half_life_minutes: 2.0,
            target_tissues: vec![
                "Heart".to_string(),
                "Blood Vessels".to_string(),
                "Liver".to_string(),
                "Muscle".to_string(),
            ],
        }
    }

    pub fn new_thyroid_hormone() -> Self {
        Self {
            name: "T3".to_string(),
            hormone_type: HormoneType::Amine,
            concentration_ng_ml: 1.2,
            half_life_minutes: 10080.0,
            target_tissues: vec!["All".to_string()],
        }
    }

    pub fn concentration_after_time(&self, minutes: f64) -> f64 {
        self.concentration_ng_ml * 0.5_f64.powf(minutes / self.half_life_minutes)
    }

    pub fn is_detectable(&self) -> bool {
        self.concentration_ng_ml > 0.01
    }
}

impl HormoneReceptor {
    pub fn new_insulin_receptor() -> Self {
        Self {
            receptor_type: ReceptorType::TyrosineKinase,
            affinity_nm: 0.1,
            signaling_pathway: "PI3K-AKT".to_string(),
        }
    }

    pub fn new_glucagon_receptor() -> Self {
        Self {
            receptor_type: ReceptorType::GProteinCoupled,
            affinity_nm: 0.5,
            signaling_pathway: "cAMP-PKA".to_string(),
        }
    }

    pub fn new_cortisol_receptor() -> Self {
        Self {
            receptor_type: ReceptorType::Nuclear,
            affinity_nm: 5.0,
            signaling_pathway: "Gene Transcription".to_string(),
        }
    }

    pub fn binding_strength(&self, hormone_concentration_nm: f64) -> f64 {
        hormone_concentration_nm / (hormone_concentration_nm + self.affinity_nm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hormone_creation() {
        let insulin = Hormone::new_insulin();
        assert_eq!(insulin.hormone_type, HormoneType::Peptide);
        assert!(insulin.is_detectable());
    }

    #[test]
    fn test_hormone_half_life() {
        let insulin = Hormone::new_insulin();
        let initial = insulin.concentration_ng_ml;
        let after_half_life = insulin.concentration_after_time(insulin.half_life_minutes);

        assert!((after_half_life - initial / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_steroid_vs_peptide() {
        let cortisol = Hormone::new_cortisol();
        let insulin = Hormone::new_insulin();

        assert!(cortisol.half_life_minutes > insulin.half_life_minutes);
    }

    #[test]
    fn test_receptor_binding() {
        let receptor = HormoneReceptor::new_insulin_receptor();
        let weak_binding = receptor.binding_strength(0.05);
        let strong_binding = receptor.binding_strength(10.0);

        assert!(strong_binding > weak_binding);
        assert!(strong_binding < 1.0);
    }

    #[test]
    fn test_thyroid_hormone() {
        let t3 = Hormone::new_thyroid_hormone();
        assert!(t3.half_life_minutes > 1000.0);
    }
}

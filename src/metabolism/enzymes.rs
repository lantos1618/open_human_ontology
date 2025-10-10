use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enzyme {
    pub name: String,
    pub ec_number: String,
    pub enzyme_class: EnzymeClass,
    pub km_mm: f64,
    pub vmax_umol_min: f64,
    pub optimal_ph: f64,
    pub optimal_temp_c: f64,
    pub cofactors: Vec<Cofactor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnzymeClass {
    Oxidoreductase,
    Transferase,
    Hydrolase,
    Lyase,
    Isomerase,
    Ligase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cofactor {
    NAD,
    NADP,
    FAD,
    CoA,
    ATP,
    Metal(MetalCofactor),
    Vitamin(VitaminCofactor),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetalCofactor {
    Zinc,
    Iron,
    Copper,
    Magnesium,
    Manganese,
    Calcium,
    Molybdenum,
    Selenium,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VitaminCofactor {
    ThiaminePyrophosphate,
    Biotin,
    PyridoxalPhosphate,
    Cobalamin,
    FolicAcid,
}

impl Enzyme {
    pub fn new_hexokinase() -> Self {
        Self {
            name: "Hexokinase".to_string(),
            ec_number: "2.7.1.1".to_string(),
            enzyme_class: EnzymeClass::Transferase,
            km_mm: 0.1,
            vmax_umol_min: 100.0,
            optimal_ph: 7.4,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::ATP, Cofactor::Metal(MetalCofactor::Magnesium)],
        }
    }

    pub fn new_phosphofructokinase() -> Self {
        Self {
            name: "Phosphofructokinase".to_string(),
            ec_number: "2.7.1.11".to_string(),
            enzyme_class: EnzymeClass::Transferase,
            km_mm: 0.5,
            vmax_umol_min: 80.0,
            optimal_ph: 7.0,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::ATP, Cofactor::Metal(MetalCofactor::Magnesium)],
        }
    }

    pub fn new_pyruvate_kinase() -> Self {
        Self {
            name: "Pyruvate Kinase".to_string(),
            ec_number: "2.7.1.40".to_string(),
            enzyme_class: EnzymeClass::Transferase,
            km_mm: 0.3,
            vmax_umol_min: 120.0,
            optimal_ph: 7.5,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::Metal(MetalCofactor::Magnesium)],
        }
    }

    pub fn new_citrate_synthase() -> Self {
        Self {
            name: "Citrate Synthase".to_string(),
            ec_number: "2.3.3.1".to_string(),
            enzyme_class: EnzymeClass::Transferase,
            km_mm: 0.02,
            vmax_umol_min: 90.0,
            optimal_ph: 8.0,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::CoA],
        }
    }

    pub fn new_isocitrate_dehydrogenase() -> Self {
        Self {
            name: "Isocitrate Dehydrogenase".to_string(),
            ec_number: "1.1.1.42".to_string(),
            enzyme_class: EnzymeClass::Oxidoreductase,
            km_mm: 0.1,
            vmax_umol_min: 70.0,
            optimal_ph: 7.4,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::NAD, Cofactor::Metal(MetalCofactor::Magnesium)],
        }
    }

    pub fn new_alpha_ketoglutarate_dehydrogenase() -> Self {
        Self {
            name: "Alpha-Ketoglutarate Dehydrogenase".to_string(),
            ec_number: "1.2.4.2".to_string(),
            enzyme_class: EnzymeClass::Oxidoreductase,
            km_mm: 0.05,
            vmax_umol_min: 60.0,
            optimal_ph: 7.0,
            optimal_temp_c: 37.0,
            cofactors: vec![
                Cofactor::NAD,
                Cofactor::CoA,
                Cofactor::Vitamin(VitaminCofactor::ThiaminePyrophosphate),
            ],
        }
    }

    pub fn new_succinate_dehydrogenase() -> Self {
        Self {
            name: "Succinate Dehydrogenase".to_string(),
            ec_number: "1.3.5.1".to_string(),
            enzyme_class: EnzymeClass::Oxidoreductase,
            km_mm: 0.2,
            vmax_umol_min: 50.0,
            optimal_ph: 7.6,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::FAD, Cofactor::Metal(MetalCofactor::Iron)],
        }
    }

    pub fn new_glucose_6_phosphate_dehydrogenase() -> Self {
        Self {
            name: "Glucose-6-Phosphate Dehydrogenase".to_string(),
            ec_number: "1.1.1.49".to_string(),
            enzyme_class: EnzymeClass::Oxidoreductase,
            km_mm: 0.05,
            vmax_umol_min: 40.0,
            optimal_ph: 8.0,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::NADP],
        }
    }

    pub fn new_fatty_acid_synthase() -> Self {
        Self {
            name: "Fatty Acid Synthase".to_string(),
            ec_number: "2.3.1.85".to_string(),
            enzyme_class: EnzymeClass::Transferase,
            km_mm: 0.01,
            vmax_umol_min: 30.0,
            optimal_ph: 7.0,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::NADP, Cofactor::CoA],
        }
    }

    pub fn new_lipoprotein_lipase() -> Self {
        Self {
            name: "Lipoprotein Lipase".to_string(),
            ec_number: "3.1.1.34".to_string(),
            enzyme_class: EnzymeClass::Hydrolase,
            km_mm: 0.5,
            vmax_umol_min: 25.0,
            optimal_ph: 8.5,
            optimal_temp_c: 37.0,
            cofactors: vec![],
        }
    }

    pub fn new_acetyl_coa_carboxylase() -> Self {
        Self {
            name: "Acetyl-CoA Carboxylase".to_string(),
            ec_number: "6.4.1.2".to_string(),
            enzyme_class: EnzymeClass::Ligase,
            km_mm: 0.02,
            vmax_umol_min: 35.0,
            optimal_ph: 7.5,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::ATP, Cofactor::Vitamin(VitaminCofactor::Biotin)],
        }
    }

    pub fn new_catalase() -> Self {
        Self {
            name: "Catalase".to_string(),
            ec_number: "1.11.1.6".to_string(),
            enzyme_class: EnzymeClass::Oxidoreductase,
            km_mm: 25.0,
            vmax_umol_min: 1000.0,
            optimal_ph: 7.0,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::Metal(MetalCofactor::Iron)],
        }
    }

    pub fn new_superoxide_dismutase() -> Self {
        Self {
            name: "Superoxide Dismutase".to_string(),
            ec_number: "1.15.1.1".to_string(),
            enzyme_class: EnzymeClass::Oxidoreductase,
            km_mm: 0.001,
            vmax_umol_min: 500.0,
            optimal_ph: 7.4,
            optimal_temp_c: 37.0,
            cofactors: vec![
                Cofactor::Metal(MetalCofactor::Copper),
                Cofactor::Metal(MetalCofactor::Zinc),
            ],
        }
    }

    pub fn new_glutathione_peroxidase() -> Self {
        Self {
            name: "Glutathione Peroxidase".to_string(),
            ec_number: "1.11.1.9".to_string(),
            enzyme_class: EnzymeClass::Oxidoreductase,
            km_mm: 0.05,
            vmax_umol_min: 200.0,
            optimal_ph: 7.0,
            optimal_temp_c: 37.0,
            cofactors: vec![Cofactor::Metal(MetalCofactor::Selenium)],
        }
    }

    pub fn velocity(&self, substrate_mm: f64) -> f64 {
        (self.vmax_umol_min * substrate_mm) / (self.km_mm + substrate_mm)
    }

    pub fn is_saturated(&self, substrate_mm: f64) -> bool {
        substrate_mm > 10.0 * self.km_mm
    }

    pub fn efficiency(&self) -> f64 {
        self.vmax_umol_min / self.km_mm
    }

    pub fn requires_metal(&self) -> bool {
        self.cofactors
            .iter()
            .any(|c| matches!(c, Cofactor::Metal(_)))
    }

    pub fn requires_vitamin(&self) -> bool {
        self.cofactors
            .iter()
            .any(|c| matches!(c, Cofactor::Vitamin(_)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CytochromeP450 {
    pub isoform: P450Isoform,
    pub substrates: Vec<String>,
    pub activity_percent: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum P450Isoform {
    CYP1A2,
    CYP2C9,
    CYP2C19,
    CYP2D6,
    CYP2E1,
    CYP3A4,
    CYP3A5,
}

impl CytochromeP450 {
    pub fn new_cyp3a4() -> Self {
        Self {
            isoform: P450Isoform::CYP3A4,
            substrates: vec![
                "Simvastatin".to_string(),
                "Atorvastatin".to_string(),
                "Midazolam".to_string(),
            ],
            activity_percent: 100.0,
        }
    }

    pub fn new_cyp2d6() -> Self {
        Self {
            isoform: P450Isoform::CYP2D6,
            substrates: vec![
                "Codeine".to_string(),
                "Tramadol".to_string(),
                "Dextromethorphan".to_string(),
            ],
            activity_percent: 100.0,
        }
    }

    pub fn metabolizes(&self, drug: &str) -> bool {
        self.substrates.iter().any(|s| s == drug)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enzyme_kinetics() {
        let hexokinase = Enzyme::new_hexokinase();
        let v_half_km = hexokinase.velocity(hexokinase.km_mm);
        assert!((v_half_km - hexokinase.vmax_umol_min / 2.0).abs() < 1.0);
    }

    #[test]
    fn test_enzyme_saturation() {
        let enzyme = Enzyme::new_hexokinase();
        assert!(enzyme.is_saturated(2.0));
        assert!(!enzyme.is_saturated(0.01));
    }

    #[test]
    fn test_cofactor_requirements() {
        let sod = Enzyme::new_superoxide_dismutase();
        assert!(sod.requires_metal());

        let acc = Enzyme::new_acetyl_coa_carboxylase();
        assert!(acc.requires_vitamin());
    }

    #[test]
    fn test_cytochrome_p450() {
        let cyp3a4 = CytochromeP450::new_cyp3a4();
        assert!(cyp3a4.metabolizes("Simvastatin"));
        assert!(!cyp3a4.metabolizes("Codeine"));
    }
}

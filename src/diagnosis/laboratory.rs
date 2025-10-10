use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompleteBloodCount {
    pub white_blood_cells: BloodCellCount,
    pub red_blood_cells: BloodCellCount,
    pub hemoglobin: LabValue,
    pub hematocrit: LabValue,
    pub mean_corpuscular_volume: LabValue,
    pub mean_corpuscular_hemoglobin: LabValue,
    pub mean_corpuscular_hemoglobin_concentration: LabValue,
    pub red_cell_distribution_width: LabValue,
    pub platelet_count: BloodCellCount,
    pub mean_platelet_volume: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComprehensiveMetabolicPanel {
    pub glucose: LabValue,
    pub calcium: LabValue,
    pub sodium: LabValue,
    pub potassium: LabValue,
    pub chloride: LabValue,
    pub carbon_dioxide: LabValue,
    pub blood_urea_nitrogen: LabValue,
    pub creatinine: LabValue,
    pub albumin: LabValue,
    pub total_protein: LabValue,
    pub alkaline_phosphatase: LabValue,
    pub alanine_aminotransferase: LabValue,
    pub aspartate_aminotransferase: LabValue,
    pub bilirubin_total: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LipidPanel {
    pub total_cholesterol: LabValue,
    pub ldl_cholesterol: LabValue,
    pub hdl_cholesterol: LabValue,
    pub triglycerides: LabValue,
    pub vldl_cholesterol: LabValue,
    pub non_hdl_cholesterol: LabValue,
    pub cholesterol_hdl_ratio: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThyroidPanel {
    pub tsh: LabValue,
    pub free_t4: LabValue,
    pub free_t3: LabValue,
    pub total_t4: LabValue,
    pub total_t3: LabValue,
    pub thyroid_peroxidase_antibody: LabValue,
    pub thyroglobulin_antibody: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HormonePanel {
    pub testosterone_total: LabValue,
    pub testosterone_free: LabValue,
    pub estradiol: LabValue,
    pub progesterone: LabValue,
    pub luteinizing_hormone: LabValue,
    pub follicle_stimulating_hormone: LabValue,
    pub prolactin: LabValue,
    pub cortisol: LabValue,
    pub dhea_sulfate: LabValue,
    pub growth_hormone: LabValue,
    pub igf_1: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InflammatoryMarkers {
    pub c_reactive_protein: LabValue,
    pub erythrocyte_sedimentation_rate: LabValue,
    pub ferritin: LabValue,
    pub d_dimer: LabValue,
    pub interleukin_6: LabValue,
    pub tumor_necrosis_factor_alpha: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardiacBiomarkers {
    pub troponin_i: LabValue,
    pub troponin_t: LabValue,
    pub bnp: LabValue,
    pub nt_pro_bnp: LabValue,
    pub ck_mb: LabValue,
    pub myoglobin: LabValue,
    pub homocysteine: LabValue,
    pub lipoprotein_a: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VitaminPanel {
    pub vitamin_d_25_oh: LabValue,
    pub vitamin_b12: LabValue,
    pub folate: LabValue,
    pub vitamin_a: LabValue,
    pub vitamin_e: LabValue,
    pub vitamin_k: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IronStudies {
    pub serum_iron: LabValue,
    pub total_iron_binding_capacity: LabValue,
    pub transferrin_saturation: f64,
    pub ferritin: LabValue,
    pub transferrin: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiverFunction {
    pub alt: LabValue,
    pub ast: LabValue,
    pub alkaline_phosphatase: LabValue,
    pub ggt: LabValue,
    pub total_bilirubin: LabValue,
    pub direct_bilirubin: LabValue,
    pub indirect_bilirubin: LabValue,
    pub albumin: LabValue,
    pub total_protein: LabValue,
    pub prothrombin_time: LabValue,
    pub inr: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenalFunction {
    pub creatinine: LabValue,
    pub blood_urea_nitrogen: LabValue,
    pub bun_creatinine_ratio: f64,
    pub estimated_gfr: f64,
    pub cystatin_c: LabValue,
    pub uric_acid: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiabetesMarkers {
    pub fasting_glucose: LabValue,
    pub hba1c: LabValue,
    pub oral_glucose_tolerance_2hr: LabValue,
    pub fasting_insulin: LabValue,
    pub c_peptide: LabValue,
    pub homa_ir: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoagulationPanel {
    pub prothrombin_time: LabValue,
    pub inr: f64,
    pub activated_partial_thromboplastin_time: LabValue,
    pub fibrinogen: LabValue,
    pub d_dimer: LabValue,
    pub platelet_count: BloodCellCount,
    pub bleeding_time: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TumorMarkers {
    pub psa: LabValue,
    pub cea: LabValue,
    pub ca_125: LabValue,
    pub ca_19_9: LabValue,
    pub afp: LabValue,
    pub ca_15_3: LabValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrineAnalysis {
    pub specific_gravity: f64,
    pub ph: f64,
    pub protein: UrineProtein,
    pub glucose: UrineGlucose,
    pub ketones: UrineKetones,
    pub blood: UrineBlood,
    pub bilirubin: UrineBilirubin,
    pub urobilinogen: f64,
    pub nitrite: bool,
    pub leukocyte_esterase: bool,
    pub color: UrineColor,
    pub clarity: UrineClarity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabValue {
    pub value: f64,
    pub unit: String,
    pub reference_range: ReferenceRange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BloodCellCount {
    pub count: f64,
    pub unit: String,
    pub reference_range: ReferenceRange,
    pub differential: Option<DifferentialCount>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentialCount {
    pub neutrophils_percent: f64,
    pub lymphocytes_percent: f64,
    pub monocytes_percent: f64,
    pub eosinophils_percent: f64,
    pub basophils_percent: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceRange {
    pub low: f64,
    pub high: f64,
    pub critical_low: Option<f64>,
    pub critical_high: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UrineProtein {
    Negative,
    Trace,
    Plus1,
    Plus2,
    Plus3,
    Plus4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UrineGlucose {
    Negative,
    Trace,
    Plus1,
    Plus2,
    Plus3,
    Plus4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UrineKetones {
    Negative,
    Small,
    Moderate,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UrineBlood {
    Negative,
    Trace,
    Small,
    Moderate,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UrineBilirubin {
    Negative,
    Small,
    Moderate,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UrineColor {
    Pale,
    Yellow,
    Amber,
    Orange,
    Red,
    Brown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UrineClarity {
    Clear,
    SlightlyCloudy,
    Cloudy,
    Turbid,
}

impl LabValue {
    pub fn new(value: f64, unit: &str, low: f64, high: f64) -> Self {
        Self {
            value,
            unit: unit.to_string(),
            reference_range: ReferenceRange {
                low,
                high,
                critical_low: None,
                critical_high: None,
            },
        }
    }

    pub fn with_critical(
        value: f64,
        unit: &str,
        low: f64,
        high: f64,
        crit_low: f64,
        crit_high: f64,
    ) -> Self {
        Self {
            value,
            unit: unit.to_string(),
            reference_range: ReferenceRange {
                low,
                high,
                critical_low: Some(crit_low),
                critical_high: Some(crit_high),
            },
        }
    }

    pub fn is_normal(&self) -> bool {
        self.value >= self.reference_range.low && self.value <= self.reference_range.high
    }

    pub fn is_high(&self) -> bool {
        self.value > self.reference_range.high
    }

    pub fn is_low(&self) -> bool {
        self.value < self.reference_range.low
    }

    pub fn is_critical(&self) -> bool {
        if let Some(crit_low) = self.reference_range.critical_low {
            if self.value < crit_low {
                return true;
            }
        }
        if let Some(crit_high) = self.reference_range.critical_high {
            if self.value > crit_high {
                return true;
            }
        }
        false
    }

    pub fn deviation_percent(&self) -> f64 {
        let mid = (self.reference_range.low + self.reference_range.high) / 2.0;
        ((self.value - mid) / mid) * 100.0
    }
}

impl BloodCellCount {
    pub fn new(count: f64, unit: &str, low: f64, high: f64) -> Self {
        Self {
            count,
            unit: unit.to_string(),
            reference_range: ReferenceRange {
                low,
                high,
                critical_low: None,
                critical_high: None,
            },
            differential: None,
        }
    }

    pub fn is_normal(&self) -> bool {
        self.count >= self.reference_range.low && self.count <= self.reference_range.high
    }

    pub fn is_high(&self) -> bool {
        self.count > self.reference_range.high
    }

    pub fn is_low(&self) -> bool {
        self.count < self.reference_range.low
    }
}

impl CompleteBloodCount {
    pub fn normal() -> Self {
        Self {
            white_blood_cells: BloodCellCount::new(7.0, "K/µL", 4.5, 11.0),
            red_blood_cells: BloodCellCount::new(4.7, "M/µL", 4.2, 5.9),
            hemoglobin: LabValue::new(14.5, "g/dL", 12.0, 17.5),
            hematocrit: LabValue::new(42.0, "%", 36.0, 50.0),
            mean_corpuscular_volume: LabValue::new(90.0, "fL", 80.0, 100.0),
            mean_corpuscular_hemoglobin: LabValue::new(30.0, "pg", 27.0, 33.0),
            mean_corpuscular_hemoglobin_concentration: LabValue::new(34.0, "g/dL", 32.0, 36.0),
            red_cell_distribution_width: LabValue::new(13.0, "%", 11.5, 14.5),
            platelet_count: BloodCellCount::new(250.0, "K/µL", 150.0, 400.0),
            mean_platelet_volume: LabValue::new(10.0, "fL", 7.5, 11.5),
        }
    }

    pub fn has_anemia(&self) -> bool {
        self.hemoglobin.is_low() || self.hematocrit.is_low()
    }

    pub fn has_infection(&self) -> bool {
        self.white_blood_cells.is_high()
    }

    pub fn has_thrombocytopenia(&self) -> bool {
        self.platelet_count.is_low()
    }
}

impl ComprehensiveMetabolicPanel {
    pub fn normal() -> Self {
        Self {
            glucose: LabValue::with_critical(90.0, "mg/dL", 70.0, 100.0, 50.0, 400.0),
            calcium: LabValue::with_critical(9.5, "mg/dL", 8.5, 10.5, 7.0, 13.0),
            sodium: LabValue::with_critical(140.0, "mEq/L", 136.0, 145.0, 120.0, 160.0),
            potassium: LabValue::with_critical(4.0, "mEq/L", 3.5, 5.0, 2.5, 6.5),
            chloride: LabValue::new(100.0, "mEq/L", 96.0, 106.0),
            carbon_dioxide: LabValue::new(24.0, "mEq/L", 23.0, 29.0),
            blood_urea_nitrogen: LabValue::new(15.0, "mg/dL", 7.0, 20.0),
            creatinine: LabValue::new(1.0, "mg/dL", 0.7, 1.3),
            albumin: LabValue::new(4.2, "g/dL", 3.5, 5.5),
            total_protein: LabValue::new(7.0, "g/dL", 6.0, 8.3),
            alkaline_phosphatase: LabValue::new(70.0, "U/L", 30.0, 120.0),
            alanine_aminotransferase: LabValue::new(25.0, "U/L", 7.0, 55.0),
            aspartate_aminotransferase: LabValue::new(22.0, "U/L", 8.0, 48.0),
            bilirubin_total: LabValue::new(0.8, "mg/dL", 0.1, 1.2),
        }
    }

    pub fn has_hyperglycemia(&self) -> bool {
        self.glucose.is_high()
    }

    pub fn has_hypoglycemia(&self) -> bool {
        self.glucose.is_low()
    }

    pub fn has_renal_dysfunction(&self) -> bool {
        self.creatinine.is_high() || self.blood_urea_nitrogen.is_high()
    }

    pub fn has_liver_dysfunction(&self) -> bool {
        self.alanine_aminotransferase.is_high()
            || self.aspartate_aminotransferase.is_high()
            || self.bilirubin_total.is_high()
    }

    pub fn has_electrolyte_imbalance(&self) -> bool {
        !self.sodium.is_normal() || !self.potassium.is_normal() || !self.chloride.is_normal()
    }
}

impl LipidPanel {
    pub fn normal() -> Self {
        let total = 180.0;
        let ldl = 100.0;
        let hdl = 55.0;
        let trig = 120.0;
        let vldl = trig / 5.0;
        let ratio = total / hdl;

        Self {
            total_cholesterol: LabValue::new(total, "mg/dL", 125.0, 200.0),
            ldl_cholesterol: LabValue::new(ldl, "mg/dL", 0.0, 100.0),
            hdl_cholesterol: LabValue::new(hdl, "mg/dL", 40.0, 200.0),
            triglycerides: LabValue::new(trig, "mg/dL", 0.0, 150.0),
            vldl_cholesterol: LabValue::new(vldl, "mg/dL", 2.0, 30.0),
            non_hdl_cholesterol: LabValue::new(total - hdl, "mg/dL", 0.0, 130.0),
            cholesterol_hdl_ratio: ratio,
        }
    }

    pub fn has_high_cholesterol(&self) -> bool {
        self.total_cholesterol.is_high()
    }

    pub fn has_high_ldl(&self) -> bool {
        self.ldl_cholesterol.is_high()
    }

    pub fn has_low_hdl(&self) -> bool {
        self.hdl_cholesterol.is_low()
    }

    pub fn has_high_triglycerides(&self) -> bool {
        self.triglycerides.is_high()
    }

    pub fn cardiovascular_risk_level(&self) -> RiskLevel {
        if self.ldl_cholesterol.value > 190.0 {
            RiskLevel::VeryHigh
        } else if self.ldl_cholesterol.value > 160.0 {
            RiskLevel::High
        } else if self.ldl_cholesterol.value > 130.0 {
            RiskLevel::Moderate
        } else {
            RiskLevel::Low
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl ThyroidPanel {
    pub fn normal() -> Self {
        Self {
            tsh: LabValue::new(2.0, "mIU/L", 0.4, 4.0),
            free_t4: LabValue::new(1.2, "ng/dL", 0.8, 1.8),
            free_t3: LabValue::new(3.2, "pg/mL", 2.3, 4.2),
            total_t4: LabValue::new(8.0, "µg/dL", 4.5, 12.0),
            total_t3: LabValue::new(120.0, "ng/dL", 80.0, 200.0),
            thyroid_peroxidase_antibody: LabValue::new(10.0, "IU/mL", 0.0, 35.0),
            thyroglobulin_antibody: LabValue::new(10.0, "IU/mL", 0.0, 40.0),
        }
    }

    pub fn has_hypothyroidism(&self) -> bool {
        self.tsh.is_high() && self.free_t4.is_low()
    }

    pub fn has_hyperthyroidism(&self) -> bool {
        self.tsh.is_low() && self.free_t4.is_high()
    }

    pub fn has_subclinical_hypothyroidism(&self) -> bool {
        self.tsh.is_high() && self.free_t4.is_normal()
    }

    pub fn has_autoimmune_thyroid(&self) -> bool {
        self.thyroid_peroxidase_antibody.is_high() || self.thyroglobulin_antibody.is_high()
    }
}

impl DiabetesMarkers {
    pub fn normal() -> Self {
        let glucose = 85.0;
        let insulin = 8.0;
        let homa = (glucose * insulin) / 405.0;

        Self {
            fasting_glucose: LabValue::with_critical(glucose, "mg/dL", 70.0, 100.0, 50.0, 400.0),
            hba1c: LabValue::new(5.2, "%", 4.0, 5.6),
            oral_glucose_tolerance_2hr: LabValue::new(120.0, "mg/dL", 0.0, 140.0),
            fasting_insulin: LabValue::new(insulin, "µU/mL", 2.6, 24.9),
            c_peptide: LabValue::new(2.0, "ng/mL", 0.5, 3.0),
            homa_ir: homa,
        }
    }

    pub fn has_diabetes(&self) -> bool {
        self.fasting_glucose.value >= 126.0 || self.hba1c.value >= 6.5
    }

    pub fn has_prediabetes(&self) -> bool {
        (self.fasting_glucose.value >= 100.0 && self.fasting_glucose.value < 126.0)
            || (self.hba1c.value >= 5.7 && self.hba1c.value < 6.5)
    }

    pub fn has_insulin_resistance(&self) -> bool {
        self.homa_ir > 2.5
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteLaboratoryPanel {
    pub cbc: CompleteBloodCount,
    pub cmp: ComprehensiveMetabolicPanel,
    pub lipid_panel: LipidPanel,
    pub thyroid_panel: ThyroidPanel,
    pub hormone_panel: Option<HormonePanel>,
    pub inflammatory_markers: Option<InflammatoryMarkers>,
    pub cardiac_biomarkers: Option<CardiacBiomarkers>,
    pub vitamin_panel: Option<VitaminPanel>,
    pub iron_studies: Option<IronStudies>,
    pub liver_function: Option<LiverFunction>,
    pub renal_function: Option<RenalFunction>,
    pub diabetes_markers: Option<DiabetesMarkers>,
    pub coagulation_panel: Option<CoagulationPanel>,
    pub tumor_markers: Option<TumorMarkers>,
    pub urine_analysis: Option<UrineAnalysis>,
}

impl CompleteLaboratoryPanel {
    pub fn basic() -> Self {
        Self {
            cbc: CompleteBloodCount::normal(),
            cmp: ComprehensiveMetabolicPanel::normal(),
            lipid_panel: LipidPanel::normal(),
            thyroid_panel: ThyroidPanel::normal(),
            hormone_panel: None,
            inflammatory_markers: None,
            cardiac_biomarkers: None,
            vitamin_panel: None,
            iron_studies: None,
            liver_function: None,
            renal_function: None,
            diabetes_markers: None,
            coagulation_panel: None,
            tumor_markers: None,
            urine_analysis: None,
        }
    }

    pub fn get_abnormal_findings(&self) -> Vec<String> {
        let mut findings = Vec::new();

        if self.cbc.has_anemia() {
            findings.push("Anemia".to_string());
        }
        if self.cbc.has_infection() {
            findings.push("Elevated WBC (possible infection)".to_string());
        }
        if self.cmp.has_hyperglycemia() {
            findings.push("Hyperglycemia".to_string());
        }
        if self.cmp.has_renal_dysfunction() {
            findings.push("Renal dysfunction".to_string());
        }
        if self.cmp.has_liver_dysfunction() {
            findings.push("Liver dysfunction".to_string());
        }
        if self.lipid_panel.has_high_cholesterol() {
            findings.push("Hypercholesterolemia".to_string());
        }
        if self.thyroid_panel.has_hypothyroidism() {
            findings.push("Hypothyroidism".to_string());
        }
        if self.thyroid_panel.has_hyperthyroidism() {
            findings.push("Hyperthyroidism".to_string());
        }

        if let Some(ref diabetes) = self.diabetes_markers {
            if diabetes.has_diabetes() {
                findings.push("Diabetes mellitus".to_string());
            } else if diabetes.has_prediabetes() {
                findings.push("Prediabetes".to_string());
            }
        }

        findings
    }
}

impl HormonePanel {
    pub fn normal_male() -> Self {
        Self {
            testosterone_total: LabValue::new(500.0, "ng/dL", 300.0, 1000.0),
            testosterone_free: LabValue::new(100.0, "pg/mL", 50.0, 200.0),
            estradiol: LabValue::new(30.0, "pg/mL", 10.0, 50.0),
            progesterone: LabValue::new(0.5, "ng/mL", 0.1, 1.0),
            luteinizing_hormone: LabValue::new(5.0, "mIU/mL", 1.0, 10.0),
            follicle_stimulating_hormone: LabValue::new(5.0, "mIU/mL", 1.0, 12.0),
            prolactin: LabValue::new(10.0, "ng/mL", 2.0, 18.0),
            cortisol: LabValue::new(12.0, "µg/dL", 6.0, 23.0),
            dhea_sulfate: LabValue::new(200.0, "µg/dL", 35.0, 430.0),
            growth_hormone: LabValue::new(1.0, "ng/mL", 0.0, 10.0),
            igf_1: LabValue::new(200.0, "ng/mL", 75.0, 400.0),
        }
    }

    pub fn normal_female() -> Self {
        Self {
            testosterone_total: LabValue::new(40.0, "ng/dL", 15.0, 70.0),
            testosterone_free: LabValue::new(1.5, "pg/mL", 0.5, 3.5),
            estradiol: LabValue::new(100.0, "pg/mL", 30.0, 400.0),
            progesterone: LabValue::new(10.0, "ng/mL", 0.1, 25.0),
            luteinizing_hormone: LabValue::new(12.0, "mIU/mL", 1.0, 95.0),
            follicle_stimulating_hormone: LabValue::new(8.0, "mIU/mL", 1.0, 120.0),
            prolactin: LabValue::new(15.0, "ng/mL", 2.0, 29.0),
            cortisol: LabValue::new(12.0, "µg/dL", 6.0, 23.0),
            dhea_sulfate: LabValue::new(200.0, "µg/dL", 35.0, 430.0),
            growth_hormone: LabValue::new(1.0, "ng/mL", 0.0, 10.0),
            igf_1: LabValue::new(200.0, "ng/mL", 75.0, 400.0),
        }
    }
}

impl InflammatoryMarkers {
    pub fn normal() -> Self {
        Self {
            c_reactive_protein: LabValue::new(0.5, "mg/L", 0.0, 3.0),
            erythrocyte_sedimentation_rate: LabValue::new(8.0, "mm/hr", 0.0, 20.0),
            ferritin: LabValue::new(100.0, "ng/mL", 30.0, 400.0),
            d_dimer: LabValue::new(200.0, "ng/mL", 0.0, 500.0),
            interleukin_6: LabValue::new(2.0, "pg/mL", 0.0, 5.0),
            tumor_necrosis_factor_alpha: LabValue::new(5.0, "pg/mL", 0.0, 15.0),
        }
    }

    pub fn has_inflammation(&self) -> bool {
        self.c_reactive_protein.is_high() || self.erythrocyte_sedimentation_rate.is_high()
    }
}

impl CardiacBiomarkers {
    pub fn normal() -> Self {
        Self {
            troponin_i: LabValue::with_critical(0.01, "ng/mL", 0.0, 0.04, 0.0, 10.0),
            troponin_t: LabValue::with_critical(0.01, "ng/mL", 0.0, 0.01, 0.0, 10.0),
            bnp: LabValue::new(50.0, "pg/mL", 0.0, 100.0),
            nt_pro_bnp: LabValue::new(100.0, "pg/mL", 0.0, 125.0),
            ck_mb: LabValue::new(2.0, "ng/mL", 0.0, 5.0),
            myoglobin: LabValue::new(50.0, "ng/mL", 0.0, 107.0),
            homocysteine: LabValue::new(8.0, "µmol/L", 5.0, 15.0),
            lipoprotein_a: LabValue::new(15.0, "mg/dL", 0.0, 30.0),
        }
    }

    pub fn has_acute_mi(&self) -> bool {
        self.troponin_i.is_high() || self.troponin_t.is_high()
    }

    pub fn has_heart_failure(&self) -> bool {
        self.bnp.is_high() || self.nt_pro_bnp.is_high()
    }
}

impl VitaminPanel {
    pub fn normal() -> Self {
        Self {
            vitamin_d_25_oh: LabValue::new(40.0, "ng/mL", 30.0, 100.0),
            vitamin_b12: LabValue::new(500.0, "pg/mL", 200.0, 900.0),
            folate: LabValue::new(12.0, "ng/mL", 2.7, 17.0),
            vitamin_a: LabValue::new(50.0, "µg/dL", 30.0, 95.0),
            vitamin_e: LabValue::new(10.0, "mg/L", 5.5, 17.0),
            vitamin_k: LabValue::new(1.0, "ng/mL", 0.2, 3.2),
        }
    }

    pub fn has_deficiency(&self) -> Vec<String> {
        let mut deficiencies = Vec::new();

        if self.vitamin_d_25_oh.is_low() {
            deficiencies.push("Vitamin D".to_string());
        }
        if self.vitamin_b12.is_low() {
            deficiencies.push("Vitamin B12".to_string());
        }
        if self.folate.is_low() {
            deficiencies.push("Folate".to_string());
        }

        deficiencies
    }
}

impl IronStudies {
    pub fn normal() -> Self {
        let serum_iron = 100.0;
        let tibc = 300.0;
        let saturation = (serum_iron / tibc) * 100.0;

        Self {
            serum_iron: LabValue::new(serum_iron, "µg/dL", 60.0, 170.0),
            total_iron_binding_capacity: LabValue::new(tibc, "µg/dL", 250.0, 450.0),
            transferrin_saturation: saturation,
            ferritin: LabValue::new(100.0, "ng/mL", 30.0, 400.0),
            transferrin: LabValue::new(250.0, "mg/dL", 200.0, 360.0),
        }
    }

    pub fn has_iron_deficiency(&self) -> bool {
        self.serum_iron.is_low() && self.ferritin.is_low()
    }

    pub fn has_iron_overload(&self) -> bool {
        self.serum_iron.is_high() && self.ferritin.is_high()
    }
}

impl LiverFunction {
    pub fn normal() -> Self {
        Self {
            alt: LabValue::new(25.0, "U/L", 7.0, 55.0),
            ast: LabValue::new(22.0, "U/L", 8.0, 48.0),
            alkaline_phosphatase: LabValue::new(70.0, "U/L", 30.0, 120.0),
            ggt: LabValue::new(25.0, "U/L", 0.0, 51.0),
            total_bilirubin: LabValue::new(0.8, "mg/dL", 0.1, 1.2),
            direct_bilirubin: LabValue::new(0.2, "mg/dL", 0.0, 0.3),
            indirect_bilirubin: LabValue::new(0.6, "mg/dL", 0.1, 0.9),
            albumin: LabValue::new(4.2, "g/dL", 3.5, 5.5),
            total_protein: LabValue::new(7.0, "g/dL", 6.0, 8.3),
            prothrombin_time: LabValue::new(12.0, "seconds", 11.0, 13.5),
            inr: 1.0,
        }
    }

    pub fn has_hepatocellular_damage(&self) -> bool {
        self.alt.is_high() || self.ast.is_high()
    }

    pub fn has_cholestasis(&self) -> bool {
        self.alkaline_phosphatase.is_high() || self.ggt.is_high()
    }

    pub fn has_synthetic_dysfunction(&self) -> bool {
        self.albumin.is_low() || self.inr > 1.5
    }
}

impl RenalFunction {
    pub fn normal() -> Self {
        let creatinine = 1.0;
        let bun = 15.0;
        let ratio = bun / creatinine;
        let egfr = 90.0;

        Self {
            creatinine: LabValue::new(creatinine, "mg/dL", 0.7, 1.3),
            blood_urea_nitrogen: LabValue::new(bun, "mg/dL", 7.0, 20.0),
            bun_creatinine_ratio: ratio,
            estimated_gfr: egfr,
            cystatin_c: LabValue::new(0.9, "mg/L", 0.5, 1.3),
            uric_acid: LabValue::new(5.0, "mg/dL", 3.5, 7.2),
        }
    }

    pub fn ckd_stage(&self) -> CKDStage {
        match self.estimated_gfr as u32 {
            90.. => CKDStage::Normal,
            60..=89 => CKDStage::Mild,
            45..=59 => CKDStage::MildToModerate,
            30..=44 => CKDStage::ModerateToSevere,
            15..=29 => CKDStage::Severe,
            _ => CKDStage::KidneyFailure,
        }
    }

    pub fn has_acute_kidney_injury(&self) -> bool {
        self.creatinine.is_high() && self.bun_creatinine_ratio > 20.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CKDStage {
    Normal,
    Mild,
    MildToModerate,
    ModerateToSevere,
    Severe,
    KidneyFailure,
}

impl CoagulationPanel {
    pub fn normal() -> Self {
        Self {
            prothrombin_time: LabValue::new(12.0, "seconds", 11.0, 13.5),
            inr: 1.0,
            activated_partial_thromboplastin_time: LabValue::new(30.0, "seconds", 25.0, 35.0),
            fibrinogen: LabValue::new(300.0, "mg/dL", 200.0, 400.0),
            d_dimer: LabValue::new(200.0, "ng/mL", 0.0, 500.0),
            platelet_count: BloodCellCount::new(250.0, "K/µL", 150.0, 400.0),
            bleeding_time: LabValue::new(5.0, "minutes", 2.0, 9.0),
        }
    }

    pub fn has_hypercoagulable_state(&self) -> bool {
        self.d_dimer.is_high()
    }

    pub fn has_coagulopathy(&self) -> bool {
        self.inr > 1.5 || self.activated_partial_thromboplastin_time.is_high()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lab_value() {
        let marker = LabValue::new(95.0, "mg/dL", 70.0, 100.0);
        assert!(marker.is_normal());
        assert!(!marker.is_high());
        assert!(!marker.is_low());
    }

    #[test]
    fn test_lab_value_critical() {
        let marker = LabValue::with_critical(45.0, "mg/dL", 70.0, 100.0, 50.0, 400.0);
        assert!(!marker.is_normal());
        assert!(marker.is_low());
        assert!(marker.is_critical());
    }

    #[test]
    fn test_cbc() {
        let cbc = CompleteBloodCount::normal();
        assert!(!cbc.has_anemia());
        assert!(!cbc.has_infection());
        assert!(!cbc.has_thrombocytopenia());
    }

    #[test]
    fn test_cmp() {
        let cmp = ComprehensiveMetabolicPanel::normal();
        assert!(!cmp.has_hyperglycemia());
        assert!(!cmp.has_hypoglycemia());
        assert!(!cmp.has_renal_dysfunction());
        assert!(!cmp.has_liver_dysfunction());
        assert!(!cmp.has_electrolyte_imbalance());
    }

    #[test]
    fn test_lipid_panel() {
        let lipids = LipidPanel::normal();
        assert!(!lipids.has_high_cholesterol());
        assert!(!lipids.has_high_ldl());
        assert!(!lipids.has_low_hdl());
        assert!(!lipids.has_high_triglycerides());
        assert_eq!(lipids.cardiovascular_risk_level(), RiskLevel::Low);
    }

    #[test]
    fn test_thyroid_panel() {
        let thyroid = ThyroidPanel::normal();
        assert!(!thyroid.has_hypothyroidism());
        assert!(!thyroid.has_hyperthyroidism());
        assert!(!thyroid.has_subclinical_hypothyroidism());
        assert!(!thyroid.has_autoimmune_thyroid());
    }

    #[test]
    fn test_diabetes_markers() {
        let diabetes = DiabetesMarkers::normal();
        assert!(!diabetes.has_diabetes());
        assert!(!diabetes.has_prediabetes());
        assert!(!diabetes.has_insulin_resistance());
    }

    #[test]
    fn test_renal_function() {
        let renal = RenalFunction::normal();
        assert_eq!(renal.ckd_stage(), CKDStage::Normal);
        assert!(!renal.has_acute_kidney_injury());
    }

    #[test]
    fn test_complete_lab_panel() {
        let labs = CompleteLaboratoryPanel::basic();
        let findings = labs.get_abnormal_findings();
        assert!(findings.is_empty());
    }
}

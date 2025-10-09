use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hemoglobin {
    pub concentration_g_dl: f64,
    pub oxygen_saturation_percent: f64,
    pub p50_mmhg: f64,
    pub hill_coefficient: f64,
    pub variant: HemoglobinVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HemoglobinVariant {
    HbA,
    HbA2,
    HbF,
    HbS,
    HbC,
    HbE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxygenTransport {
    pub hemoglobin: Hemoglobin,
    pub cardiac_output_l_min: f64,
    pub arterial_po2_mmhg: f64,
    pub venous_po2_mmhg: f64,
    pub temperature_celsius: f64,
    pub ph: f64,
    pub pco2_mmhg: f64,
    pub dpg_2_3_mmol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxygenContent {
    pub dissolved_o2_ml_dl: f64,
    pub hb_bound_o2_ml_dl: f64,
    pub total_o2_ml_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TissueOxygenation {
    pub oxygen_delivery_ml_min: f64,
    pub oxygen_consumption_ml_min: f64,
    pub oxygen_extraction_ratio: f64,
    pub tissue_po2_mmhg: f64,
    pub mitochondrial_po2_mmhg: f64,
}

impl Hemoglobin {
    pub fn new_normal() -> Self {
        Self {
            concentration_g_dl: 15.0,
            oxygen_saturation_percent: 97.0,
            p50_mmhg: 27.0,
            hill_coefficient: 2.8,
            variant: HemoglobinVariant::HbA,
        }
    }

    pub fn new_with_concentration(concentration_g_dl: f64) -> Self {
        Self {
            concentration_g_dl,
            oxygen_saturation_percent: 97.0,
            p50_mmhg: 27.0,
            hill_coefficient: 2.8,
            variant: HemoglobinVariant::HbA,
        }
    }

    pub fn oxygen_binding_capacity_ml_dl(&self) -> f64 {
        self.concentration_g_dl * 1.34
    }

    pub fn calculate_saturation(&self, po2_mmhg: f64) -> f64 {
        let ratio = po2_mmhg / self.p50_mmhg;
        let numerator = ratio.powf(self.hill_coefficient);
        let denominator = 1.0 + numerator;
        100.0 * numerator / denominator
    }

    pub fn calculate_po2_from_saturation(&self, saturation_percent: f64) -> f64 {
        let y = saturation_percent / 100.0;
        self.p50_mmhg * (y / (1.0 - y)).powf(1.0 / self.hill_coefficient)
    }

    pub fn adjust_p50_for_conditions(&self, temperature_celsius: f64, ph: f64, pco2_mmhg: f64, dpg_2_3_mmol_l: f64) -> f64 {
        let mut adjusted_p50 = self.p50_mmhg;

        adjusted_p50 *= (temperature_celsius - 37.0) * 0.024 + 1.0;
        adjusted_p50 *= (7.4 - ph) * 0.48 + 1.0;
        adjusted_p50 *= (pco2_mmhg - 40.0) * 0.0013 + 1.0;
        adjusted_p50 *= (dpg_2_3_mmol_l - 5.0) * 0.1 + 1.0;

        adjusted_p50
    }

    pub fn is_anemic(&self) -> bool {
        self.concentration_g_dl < 12.0
    }

    pub fn is_polycythemic(&self) -> bool {
        self.concentration_g_dl > 18.0
    }

    pub fn has_sickling_potential(&self) -> bool {
        matches!(self.variant, HemoglobinVariant::HbS)
    }
}

impl OxygenTransport {
    pub fn new_normal() -> Self {
        Self {
            hemoglobin: Hemoglobin::new_normal(),
            cardiac_output_l_min: 5.0,
            arterial_po2_mmhg: 95.0,
            venous_po2_mmhg: 40.0,
            temperature_celsius: 37.0,
            ph: 7.4,
            pco2_mmhg: 40.0,
            dpg_2_3_mmol_l: 5.0,
        }
    }

    pub fn arterial_oxygen_content(&self) -> OxygenContent {
        let dissolved_o2 = 0.003 * self.arterial_po2_mmhg;

        let adjusted_p50 = self.hemoglobin.adjust_p50_for_conditions(
            self.temperature_celsius,
            self.ph,
            self.pco2_mmhg,
            self.dpg_2_3_mmol_l,
        );

        let mut hb = self.hemoglobin.clone();
        hb.p50_mmhg = adjusted_p50;
        let saturation = hb.calculate_saturation(self.arterial_po2_mmhg);

        let hb_bound_o2 = self.hemoglobin.oxygen_binding_capacity_ml_dl() * (saturation / 100.0);

        OxygenContent {
            dissolved_o2_ml_dl: dissolved_o2,
            hb_bound_o2_ml_dl: hb_bound_o2,
            total_o2_ml_dl: dissolved_o2 + hb_bound_o2,
        }
    }

    pub fn venous_oxygen_content(&self) -> OxygenContent {
        let dissolved_o2 = 0.003 * self.venous_po2_mmhg;

        let adjusted_p50 = self.hemoglobin.adjust_p50_for_conditions(
            self.temperature_celsius,
            self.ph + 0.03,
            self.pco2_mmhg + 6.0,
            self.dpg_2_3_mmol_l,
        );

        let mut hb = self.hemoglobin.clone();
        hb.p50_mmhg = adjusted_p50;
        let saturation = hb.calculate_saturation(self.venous_po2_mmhg);

        let hb_bound_o2 = self.hemoglobin.oxygen_binding_capacity_ml_dl() * (saturation / 100.0);

        OxygenContent {
            dissolved_o2_ml_dl: dissolved_o2,
            hb_bound_o2_ml_dl: hb_bound_o2,
            total_o2_ml_dl: dissolved_o2 + hb_bound_o2,
        }
    }

    pub fn calculate_tissue_oxygenation(&self) -> TissueOxygenation {
        let arterial_content = self.arterial_oxygen_content();
        let venous_content = self.venous_oxygen_content();

        let oxygen_delivery = arterial_content.total_o2_ml_dl * self.cardiac_output_l_min * 10.0;

        let av_difference = arterial_content.total_o2_ml_dl - venous_content.total_o2_ml_dl;
        let oxygen_consumption = av_difference * self.cardiac_output_l_min * 10.0;

        let extraction_ratio = av_difference / arterial_content.total_o2_ml_dl;

        let tissue_po2 = self.venous_po2_mmhg * 0.75;
        let mitochondrial_po2 = tissue_po2 * 0.1;

        TissueOxygenation {
            oxygen_delivery_ml_min: oxygen_delivery,
            oxygen_consumption_ml_min: oxygen_consumption,
            oxygen_extraction_ratio: extraction_ratio,
            tissue_po2_mmhg: tissue_po2,
            mitochondrial_po2_mmhg: mitochondrial_po2,
        }
    }

    pub fn oxygen_delivery_ml_min(&self) -> f64 {
        let arterial_content = self.arterial_oxygen_content();
        arterial_content.total_o2_ml_dl * self.cardiac_output_l_min * 10.0
    }

    pub fn oxygen_consumption_ml_min(&self) -> f64 {
        let arterial_content = self.arterial_oxygen_content();
        let venous_content = self.venous_oxygen_content();
        let av_difference = arterial_content.total_o2_ml_dl - venous_content.total_o2_ml_dl;
        av_difference * self.cardiac_output_l_min * 10.0
    }

    pub fn is_hypoxic(&self) -> bool {
        self.arterial_po2_mmhg < 60.0
    }

    pub fn has_tissue_hypoxia(&self) -> bool {
        let tissue_ox = self.calculate_tissue_oxygenation();
        tissue_ox.tissue_po2_mmhg < 20.0 || tissue_ox.oxygen_extraction_ratio > 0.5
    }
}

impl TissueOxygenation {
    pub fn is_adequate(&self) -> bool {
        self.oxygen_delivery_ml_min > 500.0
            && self.oxygen_extraction_ratio < 0.4
            && self.tissue_po2_mmhg > 20.0
    }

    pub fn has_supply_dependency(&self) -> bool {
        self.oxygen_extraction_ratio > 0.6
    }

    pub fn calculate_oxygen_debt(&self, baseline_vo2: f64) -> f64 {
        (baseline_vo2 - self.oxygen_consumption_ml_min).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hemoglobin_creation() {
        let hb = Hemoglobin::new_normal();
        assert_eq!(hb.concentration_g_dl, 15.0);
        assert_eq!(hb.p50_mmhg, 27.0);
    }

    #[test]
    fn test_oxygen_binding_capacity() {
        let hb = Hemoglobin::new_normal();
        let capacity = hb.oxygen_binding_capacity_ml_dl();
        assert!((capacity - 20.1).abs() < 0.1);
    }

    #[test]
    fn test_saturation_calculation() {
        let hb = Hemoglobin::new_normal();
        let sat_at_p50 = hb.calculate_saturation(27.0);
        assert!((sat_at_p50 - 50.0).abs() < 5.0);

        let sat_at_high_po2 = hb.calculate_saturation(100.0);
        assert!(sat_at_high_po2 > 95.0);
    }

    #[test]
    fn test_po2_from_saturation() {
        let hb = Hemoglobin::new_normal();
        let po2 = hb.calculate_po2_from_saturation(50.0);
        assert!((po2 - 27.0).abs() < 2.0);
    }

    #[test]
    fn test_p50_temperature_adjustment() {
        let hb = Hemoglobin::new_normal();
        let adjusted = hb.adjust_p50_for_conditions(38.0, 7.4, 40.0, 5.0);
        assert!(adjusted > hb.p50_mmhg);
    }

    #[test]
    fn test_p50_ph_adjustment() {
        let hb = Hemoglobin::new_normal();
        let adjusted = hb.adjust_p50_for_conditions(37.0, 7.2, 40.0, 5.0);
        assert!(adjusted > hb.p50_mmhg);
    }

    #[test]
    fn test_oxygen_transport() {
        let ot = OxygenTransport::new_normal();
        let arterial = ot.arterial_oxygen_content();
        assert!(arterial.total_o2_ml_dl > 19.0);
        assert!(arterial.total_o2_ml_dl < 21.0);
    }

    #[test]
    fn test_oxygen_delivery() {
        let ot = OxygenTransport::new_normal();
        let do2 = ot.oxygen_delivery_ml_min();
        assert!(do2 > 900.0);
        assert!(do2 < 1100.0);
    }

    #[test]
    fn test_oxygen_consumption() {
        let ot = OxygenTransport::new_normal();
        let vo2 = ot.oxygen_consumption_ml_min();
        assert!(vo2 > 200.0);
        assert!(vo2 < 300.0);
    }

    #[test]
    fn test_tissue_oxygenation() {
        let ot = OxygenTransport::new_normal();
        let tissue_ox = ot.calculate_tissue_oxygenation();
        assert!(tissue_ox.is_adequate());
        assert!(!tissue_ox.has_supply_dependency());
    }

    #[test]
    fn test_anemia_detection() {
        let mut hb = Hemoglobin::new_normal();
        hb.concentration_g_dl = 10.0;
        assert!(hb.is_anemic());
    }

    #[test]
    fn test_hypoxia_detection() {
        let mut ot = OxygenTransport::new_normal();
        ot.arterial_po2_mmhg = 55.0;
        assert!(ot.is_hypoxic());
    }

    #[test]
    fn test_sickle_cell_variant() {
        let mut hb = Hemoglobin::new_normal();
        hb.variant = HemoglobinVariant::HbS;
        assert!(hb.has_sickling_potential());
    }
}

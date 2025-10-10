use crate::systems::cardiovascular::{BloodVessel, Heart, VesselType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CirculatorySystem {
    pub systemic_circulation: SystemicCirculation,
    pub pulmonary_circulation: PulmonaryCirculation,
    pub coronary_circulation: CoronaryCirculation,
    pub portal_circulation: PortalCirculation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemicCirculation {
    pub aorta: BloodVessel,
    pub major_arteries: Vec<BloodVessel>,
    pub arterioles: Vec<BloodVessel>,
    pub capillary_beds: Vec<CapillaryBed>,
    pub venules: Vec<BloodVessel>,
    pub major_veins: Vec<BloodVessel>,
    pub vena_cava: VenaCava,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulmonaryCirculation {
    pub pulmonary_arteries: Vec<BloodVessel>,
    pub pulmonary_capillaries: Vec<CapillaryBed>,
    pub pulmonary_veins: Vec<BloodVessel>,
    pub total_blood_volume_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoronaryCirculation {
    pub left_coronary_artery: BloodVessel,
    pub right_coronary_artery: BloodVessel,
    pub coronary_blood_flow_ml_min: f64,
    pub coronary_perfusion_pressure_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalCirculation {
    pub portal_vein: BloodVessel,
    pub hepatic_artery: BloodVessel,
    pub hepatic_veins: Vec<BloodVessel>,
    pub portal_blood_flow_ml_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapillaryBed {
    pub organ: OrganSupplied,
    pub number_of_capillaries: usize,
    pub total_surface_area_cm2: f64,
    pub permeability_coefficient: f64,
    pub blood_flow_ml_min: f64,
    pub oxygen_extraction_ratio: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganSupplied {
    Brain,
    Heart,
    Lungs,
    Liver,
    Kidneys,
    Intestines,
    SkeletalMuscle,
    Skin,
    Bone,
    AdiposeTissue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenaCava {
    pub superior: BloodVessel,
    pub inferior: BloodVessel,
    pub total_return_ml_min: f64,
}

impl CirculatorySystem {
    pub fn new_adult() -> Self {
        Self {
            systemic_circulation: SystemicCirculation::new_adult(),
            pulmonary_circulation: PulmonaryCirculation::new_adult(),
            coronary_circulation: CoronaryCirculation::new_adult(),
            portal_circulation: PortalCirculation::new_adult(),
        }
    }

    pub fn total_blood_volume_ml(&self) -> f64 {
        let systemic = self.systemic_circulation.aorta.volume_ml()
            + self
                .systemic_circulation
                .major_arteries
                .iter()
                .map(|v| v.volume_ml())
                .sum::<f64>()
            + self
                .systemic_circulation
                .major_veins
                .iter()
                .map(|v| v.volume_ml())
                .sum::<f64>();

        let pulmonary = self.pulmonary_circulation.total_blood_volume_ml;

        systemic + pulmonary + 1500.0
    }

    pub fn cardiac_index(&self, heart: &Heart, body_surface_area_m2: f64) -> f64 {
        heart.cardiac_output_l_min() / body_surface_area_m2
    }

    pub fn total_peripheral_resistance(
        &self,
        mean_arterial_pressure: f64,
        cardiac_output: f64,
    ) -> f64 {
        if cardiac_output == 0.0 {
            return 0.0;
        }
        (mean_arterial_pressure - 5.0) / cardiac_output
    }

    pub fn organ_blood_flow_distribution(&self) -> Vec<(OrganSupplied, f64)> {
        self.systemic_circulation
            .capillary_beds
            .iter()
            .map(|bed| (bed.organ, bed.blood_flow_ml_min))
            .collect()
    }
}

impl SystemicCirculation {
    pub fn new_adult() -> Self {
        let mut major_arteries = Vec::new();
        major_arteries.push(BloodVessel::new(VesselType::Artery));
        major_arteries.push(BloodVessel::new(VesselType::Artery));

        let capillary_beds = vec![
            CapillaryBed::new_for_organ(OrganSupplied::Brain, 750.0),
            CapillaryBed::new_for_organ(OrganSupplied::Heart, 250.0),
            CapillaryBed::new_for_organ(OrganSupplied::Kidneys, 1100.0),
            CapillaryBed::new_for_organ(OrganSupplied::Liver, 1350.0),
            CapillaryBed::new_for_organ(OrganSupplied::Intestines, 1000.0),
            CapillaryBed::new_for_organ(OrganSupplied::SkeletalMuscle, 1000.0),
            CapillaryBed::new_for_organ(OrganSupplied::Skin, 300.0),
        ];

        Self {
            aorta: BloodVessel::aorta(),
            major_arteries,
            arterioles: vec![BloodVessel::new(VesselType::Arteriole); 10],
            capillary_beds,
            venules: vec![BloodVessel::new(VesselType::Venule); 10],
            major_veins: vec![BloodVessel::new(VesselType::Vein); 5],
            vena_cava: VenaCava::new_adult(),
        }
    }

    pub fn total_arteriolar_resistance(&self) -> f64 {
        self.arterioles.iter().map(|a| a.resistance_mmhg_s_ml).sum()
    }

    pub fn calculate_pressure_drop(&self) -> f64 {
        let arterial_resistance: f64 = self
            .major_arteries
            .iter()
            .map(|v| v.resistance_mmhg_s_ml)
            .sum();
        let arteriolar_resistance = self.total_arteriolar_resistance();

        arterial_resistance + arteriolar_resistance
    }
}

impl PulmonaryCirculation {
    pub fn new_adult() -> Self {
        Self {
            pulmonary_arteries: vec![BloodVessel::new(VesselType::Artery); 2],
            pulmonary_capillaries: vec![CapillaryBed::new_for_organ(OrganSupplied::Lungs, 5000.0)],
            pulmonary_veins: vec![BloodVessel::new(VesselType::Vein); 4],
            total_blood_volume_ml: 500.0,
        }
    }

    pub fn pulmonary_vascular_resistance(
        &self,
        mean_pa_pressure: f64,
        la_pressure: f64,
        cardiac_output: f64,
    ) -> f64 {
        if cardiac_output == 0.0 {
            return 0.0;
        }
        (mean_pa_pressure - la_pressure) / cardiac_output
    }

    pub fn assess_gas_exchange_surface(&self) -> f64 {
        self.pulmonary_capillaries
            .iter()
            .map(|bed| bed.total_surface_area_cm2)
            .sum()
    }
}

impl CoronaryCirculation {
    pub fn new_adult() -> Self {
        let mut left_coronary = BloodVessel::new(VesselType::Artery);
        left_coronary.diameter_mm = 4.5;

        let mut right_coronary = BloodVessel::new(VesselType::Artery);
        right_coronary.diameter_mm = 3.5;

        Self {
            left_coronary_artery: left_coronary,
            right_coronary_artery: right_coronary,
            coronary_blood_flow_ml_min: 250.0,
            coronary_perfusion_pressure_mmhg: 80.0,
        }
    }

    pub fn coronary_flow_reserve(&self, hyperemic_flow: f64) -> f64 {
        if self.coronary_blood_flow_ml_min == 0.0 {
            return 0.0;
        }
        hyperemic_flow / self.coronary_blood_flow_ml_min
    }

    pub fn assess_ischemia_risk(&self, stenosis_percentage: f64) -> bool {
        stenosis_percentage > 70.0
    }
}

impl PortalCirculation {
    pub fn new_adult() -> Self {
        let mut portal_vein = BloodVessel::new(VesselType::Vein);
        portal_vein.diameter_mm = 12.0;

        Self {
            portal_vein,
            hepatic_artery: BloodVessel::new(VesselType::Artery),
            hepatic_veins: vec![BloodVessel::new(VesselType::Vein); 3],
            portal_blood_flow_ml_min: 1050.0,
        }
    }

    pub fn portal_pressure_gradient(&self, splenic_pressure: f64, hepatic_pressure: f64) -> f64 {
        splenic_pressure - hepatic_pressure
    }

    pub fn has_portal_hypertension(&self, gradient: f64) -> bool {
        gradient > 10.0
    }
}

impl CapillaryBed {
    pub fn new_for_organ(organ: OrganSupplied, blood_flow_ml_min: f64) -> Self {
        let (capillary_count, surface_area, permeability, oxygen_extraction) = match organ {
            OrganSupplied::Brain => (100_000_000, 200.0, 0.15, 0.35),
            OrganSupplied::Heart => (50_000_000, 150.0, 0.20, 0.75),
            OrganSupplied::Lungs => (300_000_000, 700.0, 0.50, 0.25),
            OrganSupplied::Liver => (80_000_000, 500.0, 0.80, 0.35),
            OrganSupplied::Kidneys => (60_000_000, 150.0, 0.60, 0.20),
            OrganSupplied::Intestines => (100_000_000, 600.0, 0.70, 0.30),
            OrganSupplied::SkeletalMuscle => (150_000_000, 400.0, 0.10, 0.25),
            OrganSupplied::Skin => (40_000_000, 180.0, 0.12, 0.10),
            OrganSupplied::Bone => (20_000_000, 100.0, 0.08, 0.15),
            OrganSupplied::AdiposeTissue => (30_000_000, 120.0, 0.05, 0.10),
        };

        Self {
            organ,
            number_of_capillaries: capillary_count,
            total_surface_area_cm2: surface_area,
            permeability_coefficient: permeability,
            blood_flow_ml_min,
            oxygen_extraction_ratio: oxygen_extraction,
        }
    }

    pub fn oxygen_delivery_ml_min(&self, arterial_o2_content_ml_per_dl: f64) -> f64 {
        (self.blood_flow_ml_min / 100.0) * arterial_o2_content_ml_per_dl
    }

    pub fn oxygen_consumption_ml_min(&self, arterial_o2_content_ml_per_dl: f64) -> f64 {
        self.oxygen_delivery_ml_min(arterial_o2_content_ml_per_dl) * self.oxygen_extraction_ratio
    }
}

impl VenaCava {
    pub fn new_adult() -> Self {
        let mut superior = BloodVessel::new(VesselType::Vein);
        superior.diameter_mm = 20.0;

        let mut inferior = BloodVessel::new(VesselType::Vein);
        inferior.diameter_mm = 25.0;

        Self {
            superior,
            inferior,
            total_return_ml_min: 5000.0,
        }
    }

    pub fn central_venous_pressure_estimate(&self) -> f64 {
        (self.superior.pressure_mmhg + self.inferior.pressure_mmhg) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circulatory_system_creation() {
        let circulation = CirculatorySystem::new_adult();
        let volume = circulation.total_blood_volume_ml();
        assert!(volume > 2000.0, "Expected > 2000.0ml, got {}", volume);
    }

    #[test]
    fn test_organ_blood_flow() {
        let circulation = CirculatorySystem::new_adult();
        let distribution = circulation.organ_blood_flow_distribution();

        let brain_flow = distribution
            .iter()
            .find(|(organ, _)| *organ == OrganSupplied::Brain)
            .map(|(_, flow)| *flow);

        assert_eq!(brain_flow, Some(750.0));
    }

    #[test]
    fn test_capillary_oxygen_delivery() {
        let brain_bed = CapillaryBed::new_for_organ(OrganSupplied::Brain, 750.0);
        let o2_delivery = brain_bed.oxygen_delivery_ml_min(20.0);
        assert!(o2_delivery > 100.0);
    }

    #[test]
    fn test_coronary_flow_reserve() {
        let coronary = CoronaryCirculation::new_adult();
        let cfr = coronary.coronary_flow_reserve(1000.0);
        assert!(cfr > 3.0);
    }

    #[test]
    fn test_portal_hypertension() {
        let portal = PortalCirculation::new_adult();
        assert!(portal.has_portal_hypertension(15.0));
        assert!(!portal.has_portal_hypertension(8.0));
    }

    #[test]
    fn test_pulmonary_circulation() {
        let pulmonary = PulmonaryCirculation::new_adult();
        let surface_area = pulmonary.assess_gas_exchange_surface();
        assert!(surface_area > 500.0);
    }
}

use crate::biology::{BiologyError, BiologyResult};
use crate::biology::tissue::ExtracellularMatrix;
use crate::physics::FluidProperties;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VesselType {
    Artery,
    Arteriole,
    Capillary,
    Venule,
    Vein,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VesselLayer {
    pub tunica_intima_thickness_um: f64,
    pub tunica_media_thickness_um: f64,
    pub tunica_adventitia_thickness_um: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodVessel {
    pub vessel_type: VesselType,
    pub diameter_mm: f64,
    pub length_mm: f64,
    pub wall_thickness_mm: f64,
    pub layers: VesselLayer,
    pub compliance_ml_mmhg: f64,
    pub resistance_mmhg_s_ml: f64,
    pub pressure_mmhg: f64,
}

impl BloodVessel {
    pub fn new(vessel_type: VesselType) -> Self {
        let (diameter, wall_thickness, compliance, resistance, pressure) = match vessel_type {
            VesselType::Artery => (4.0, 1.0, 0.2, 0.1, 100.0),
            VesselType::Arteriole => (0.03, 0.02, 0.001, 2.0, 85.0),
            VesselType::Capillary => (0.008, 0.001, 0.0001, 5.0, 30.0),
            VesselType::Venule => (0.02, 0.002, 0.002, 1.0, 15.0),
            VesselType::Vein => (5.0, 0.5, 0.5, 0.05, 10.0),
        };

        BloodVessel {
            vessel_type,
            diameter_mm: diameter,
            length_mm: 100.0,
            wall_thickness_mm: wall_thickness,
            layers: VesselLayer::for_vessel_type(vessel_type),
            compliance_ml_mmhg: compliance,
            resistance_mmhg_s_ml: resistance,
            pressure_mmhg: pressure,
        }
    }

    pub fn aorta() -> Self {
        let mut aorta = BloodVessel::new(VesselType::Artery);
        aorta.diameter_mm = 25.0;
        aorta.wall_thickness_mm = 2.0;
        aorta.length_mm = 400.0;
        aorta.compliance_ml_mmhg = 2.0;
        aorta
    }

    pub fn cross_sectional_area_mm2(&self) -> f64 {
        std::f64::consts::PI * (self.diameter_mm / 2.0).powi(2)
    }

    pub fn volume_ml(&self) -> f64 {
        self.cross_sectional_area_mm2() * self.length_mm / 1000.0
    }

    pub fn calculate_flow(&self, pressure_gradient_mmhg: f64) -> f64 {
        if self.resistance_mmhg_s_ml == 0.0 {
            return 0.0;
        }
        pressure_gradient_mmhg / self.resistance_mmhg_s_ml
    }

    pub fn wall_stress_kdynes_cm2(&self) -> f64 {
        (self.pressure_mmhg * self.diameter_mm) / (2.0 * self.wall_thickness_mm)
    }

    pub fn is_atherosclerotic(&self, plaque_thickness_mm: f64) -> bool {
        let lumen_reduction = plaque_thickness_mm / self.diameter_mm;
        lumen_reduction > 0.5
    }

    pub fn apply_pressure(&mut self, pressure_mmhg: f64) -> BiologyResult<()> {
        if pressure_mmhg < 0.0 {
            return Err(BiologyError::InvalidValue(
                "Pressure cannot be negative".to_string()
            ));
        }

        self.pressure_mmhg = pressure_mmhg;

        let diameter_change = pressure_mmhg * self.compliance_ml_mmhg / 10.0;
        self.diameter_mm += diameter_change;

        if self.wall_stress_kdynes_cm2() > 200.0 {
            return Err(BiologyError::InvalidState(
                "Vessel wall stress exceeds rupture threshold".to_string()
            ));
        }

        Ok(())
    }
}

impl VesselLayer {
    pub fn for_vessel_type(vessel_type: VesselType) -> Self {
        match vessel_type {
            VesselType::Artery => VesselLayer {
                tunica_intima_thickness_um: 50.0,
                tunica_media_thickness_um: 800.0,
                tunica_adventitia_thickness_um: 150.0,
            },
            VesselType::Arteriole => VesselLayer {
                tunica_intima_thickness_um: 10.0,
                tunica_media_thickness_um: 15.0,
                tunica_adventitia_thickness_um: 5.0,
            },
            VesselType::Capillary => VesselLayer {
                tunica_intima_thickness_um: 1.0,
                tunica_media_thickness_um: 0.0,
                tunica_adventitia_thickness_um: 0.0,
            },
            VesselType::Venule => VesselLayer {
                tunica_intima_thickness_um: 5.0,
                tunica_media_thickness_um: 2.0,
                tunica_adventitia_thickness_um: 3.0,
            },
            VesselType::Vein => VesselLayer {
                tunica_intima_thickness_um: 30.0,
                tunica_media_thickness_um: 200.0,
                tunica_adventitia_thickness_um: 270.0,
            },
        }
    }

    pub fn total_thickness_um(&self) -> f64 {
        self.tunica_intima_thickness_um +
        self.tunica_media_thickness_um +
        self.tunica_adventitia_thickness_um
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vessel_creation() {
        let artery = BloodVessel::new(VesselType::Artery);
        assert_eq!(artery.vessel_type, VesselType::Artery);
        assert!(artery.diameter_mm > 0.0);
    }

    #[test]
    fn test_aorta() {
        let aorta = BloodVessel::aorta();
        assert!(aorta.diameter_mm > 20.0);
    }

    #[test]
    fn test_vessel_volume() {
        let artery = BloodVessel::new(VesselType::Artery);
        let volume = artery.volume_ml();
        assert!(volume > 0.0);
    }

    #[test]
    fn test_flow_calculation() {
        let artery = BloodVessel::new(VesselType::Artery);
        let flow = artery.calculate_flow(20.0);
        assert!(flow > 0.0);
    }

    #[test]
    fn test_wall_stress() {
        let artery = BloodVessel::new(VesselType::Artery);
        let stress = artery.wall_stress_kdynes_cm2();
        assert!(stress > 0.0);
    }

    #[test]
    fn test_atherosclerosis() {
        let artery = BloodVessel::new(VesselType::Artery);
        assert!(artery.is_atherosclerotic(3.0));
        assert!(!artery.is_atherosclerotic(0.5));
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoronaryArterySystem {
    pub left_main: CoronaryArtery,
    pub left_anterior_descending: CoronaryArtery,
    pub left_circumflex: CoronaryArtery,
    pub right_coronary: CoronaryArtery,
    pub coronary_flow_ml_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoronaryArtery {
    pub name: String,
    pub diameter_mm: f64,
    pub length_mm: f64,
    pub stenosis_percent: f64,
    pub flow_ml_min: f64,
    pub plaques: Vec<AtheroscleroticPlaque>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtheroscleroticPlaque {
    pub location_mm: f64,
    pub stenosis_percent: f64,
    pub composition: PlaqueComposition,
    pub stability: PlaqueStability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaqueComposition {
    pub lipid_core_percent: f64,
    pub fibrous_cap_thickness_um: f64,
    pub calcification_percent: f64,
    pub inflammatory_cells_percent: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaqueStability {
    Stable,
    Vulnerable,
    Ruptured,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyocardialPerfusion {
    pub anterior_wall: RegionalPerfusion,
    pub lateral_wall: RegionalPerfusion,
    pub inferior_wall: RegionalPerfusion,
    pub septal: RegionalPerfusion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalPerfusion {
    pub flow_ml_min_per_g: f64,
    pub is_ischemic: bool,
    pub viability_percent: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoronaryDominance {
    RightDominant,
    LeftDominant,
    Codominant,
}

impl CoronaryArterySystem {
    pub fn new() -> Self {
        CoronaryArterySystem {
            left_main: CoronaryArtery::new("Left Main".to_string(), 4.5, 10.0),
            left_anterior_descending: CoronaryArtery::new("LAD".to_string(), 3.5, 120.0),
            left_circumflex: CoronaryArtery::new("LCx".to_string(), 3.0, 80.0),
            right_coronary: CoronaryArtery::new("RCA".to_string(), 3.5, 100.0),
            coronary_flow_ml_min: 250.0,
        }
    }

    pub fn calculate_total_stenosis(&self) -> f64 {
        let stenoses = [
            self.left_main.stenosis_percent,
            self.left_anterior_descending.stenosis_percent,
            self.left_circumflex.stenosis_percent,
            self.right_coronary.stenosis_percent,
        ];

        stenoses.iter().sum::<f64>() / stenoses.len() as f64
    }

    pub fn has_critical_stenosis(&self) -> bool {
        self.left_main.stenosis_percent > 50.0
            || self.left_anterior_descending.stenosis_percent > 70.0
            || self.left_circumflex.stenosis_percent > 70.0
            || self.right_coronary.stenosis_percent > 70.0
    }

    pub fn assess_cad_severity(&self) -> CADSeverity {
        let max_stenosis = self
            .left_main
            .stenosis_percent
            .max(self.left_anterior_descending.stenosis_percent)
            .max(self.left_circumflex.stenosis_percent)
            .max(self.right_coronary.stenosis_percent);

        let num_diseased = [
            &self.left_anterior_descending,
            &self.left_circumflex,
            &self.right_coronary,
        ]
        .iter()
        .filter(|a| a.stenosis_percent > 50.0)
        .count();

        if self.left_main.stenosis_percent > 50.0 {
            CADSeverity::LeftMainDisease
        } else if num_diseased >= 3 {
            CADSeverity::ThreeVesselDisease
        } else if num_diseased == 2 {
            CADSeverity::TwoVesselDisease
        } else if num_diseased == 1 {
            CADSeverity::SingleVesselDisease
        } else if max_stenosis > 20.0 {
            CADSeverity::NonobstructiveCAD
        } else {
            CADSeverity::NoCAD
        }
    }

    pub fn calculate_syntax_score(&self) -> f64 {
        let mut score = 0.0;

        for artery in [
            &self.left_main,
            &self.left_anterior_descending,
            &self.left_circumflex,
            &self.right_coronary,
        ] {
            if artery.stenosis_percent > 50.0 {
                score += match artery.stenosis_percent {
                    s if s >= 99.0 => 5.0,
                    s if s >= 50.0 => 2.0,
                    _ => 0.0,
                };
            }
        }

        score
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CADSeverity {
    NoCAD,
    NonobstructiveCAD,
    SingleVesselDisease,
    TwoVesselDisease,
    ThreeVesselDisease,
    LeftMainDisease,
}

impl CoronaryArtery {
    pub fn new(name: String, diameter_mm: f64, length_mm: f64) -> Self {
        CoronaryArtery {
            name,
            diameter_mm,
            length_mm,
            stenosis_percent: 0.0,
            flow_ml_min: 80.0,
            plaques: Vec::new(),
        }
    }

    pub fn add_plaque(&mut self, plaque: AtheroscleroticPlaque) {
        self.stenosis_percent = self.stenosis_percent.max(plaque.stenosis_percent);
        self.plaques.push(plaque);
    }

    pub fn calculate_flow_reduction(&self) -> f64 {
        if self.stenosis_percent < 50.0 {
            1.0
        } else {
            let reduction = (self.stenosis_percent - 50.0) / 50.0;
            1.0 - reduction.min(0.9)
        }
    }

    pub fn has_vulnerable_plaque(&self) -> bool {
        self.plaques
            .iter()
            .any(|p| p.stability == PlaqueStability::Vulnerable)
    }
}

impl AtheroscleroticPlaque {
    pub fn new(location_mm: f64, stenosis_percent: f64) -> Self {
        AtheroscleroticPlaque {
            location_mm,
            stenosis_percent,
            composition: PlaqueComposition {
                lipid_core_percent: 30.0,
                fibrous_cap_thickness_um: 100.0,
                calcification_percent: 10.0,
                inflammatory_cells_percent: 5.0,
            },
            stability: PlaqueStability::Stable,
        }
    }

    pub fn assess_stability(&self) -> PlaqueStability {
        if self.composition.fibrous_cap_thickness_um < 65.0 {
            PlaqueStability::Vulnerable
        } else if self.composition.lipid_core_percent > 40.0
            && self.composition.inflammatory_cells_percent > 10.0
        {
            PlaqueStability::Vulnerable
        } else {
            PlaqueStability::Stable
        }
    }
}

impl MyocardialPerfusion {
    pub fn new() -> Self {
        MyocardialPerfusion {
            anterior_wall: RegionalPerfusion::normal(),
            lateral_wall: RegionalPerfusion::normal(),
            inferior_wall: RegionalPerfusion::normal(),
            septal: RegionalPerfusion::normal(),
        }
    }

    pub fn has_ischemia(&self) -> bool {
        self.anterior_wall.is_ischemic
            || self.lateral_wall.is_ischemic
            || self.inferior_wall.is_ischemic
            || self.septal.is_ischemic
    }
}

impl RegionalPerfusion {
    pub fn normal() -> Self {
        RegionalPerfusion {
            flow_ml_min_per_g: 1.0,
            is_ischemic: false,
            viability_percent: 100.0,
        }
    }
}

impl Default for CoronaryArterySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MyocardialPerfusion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coronary_system_creation() {
        let system = CoronaryArterySystem::new();
        assert_eq!(system.left_main.diameter_mm, 4.5);
        assert_eq!(system.coronary_flow_ml_min, 250.0);
    }

    #[test]
    fn test_no_cad() {
        let system = CoronaryArterySystem::new();
        assert_eq!(system.assess_cad_severity(), CADSeverity::NoCAD);
        assert!(!system.has_critical_stenosis());
    }

    #[test]
    fn test_single_vessel_disease() {
        let mut system = CoronaryArterySystem::new();
        system.left_anterior_descending.stenosis_percent = 80.0;
        assert_eq!(
            system.assess_cad_severity(),
            CADSeverity::SingleVesselDisease
        );
    }

    #[test]
    fn test_left_main_disease() {
        let mut system = CoronaryArterySystem::new();
        system.left_main.stenosis_percent = 60.0;
        assert_eq!(system.assess_cad_severity(), CADSeverity::LeftMainDisease);
        assert!(system.has_critical_stenosis());
    }

    #[test]
    fn test_plaque_addition() {
        let mut artery = CoronaryArtery::new("LAD".to_string(), 3.5, 120.0);
        let plaque = AtheroscleroticPlaque::new(50.0, 70.0);
        artery.add_plaque(plaque);
        assert_eq!(artery.stenosis_percent, 70.0);
    }

    #[test]
    fn test_flow_reduction() {
        let mut artery = CoronaryArtery::new("LAD".to_string(), 3.5, 120.0);
        artery.stenosis_percent = 90.0;
        let reduction = artery.calculate_flow_reduction();
        assert!(reduction < 1.0);
    }

    #[test]
    fn test_plaque_stability() {
        let mut plaque = AtheroscleroticPlaque::new(50.0, 70.0);
        plaque.composition.fibrous_cap_thickness_um = 50.0;
        assert_eq!(plaque.assess_stability(), PlaqueStability::Vulnerable);
    }

    #[test]
    fn test_syntax_score() {
        let mut system = CoronaryArterySystem::new();
        system.left_anterior_descending.stenosis_percent = 99.0;
        system.left_circumflex.stenosis_percent = 80.0;
        let score = system.calculate_syntax_score();
        assert!(score > 0.0);
    }
}

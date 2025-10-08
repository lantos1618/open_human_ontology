use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualSystem {
    pub eyes: [Eye; 2],
    pub visual_acuity: VisualAcuity,
    pub color_vision: ColorVision,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eye {
    pub side: Side,
    pub cornea: Cornea,
    pub lens: Lens,
    pub retina: Retina,
    pub optic_nerve: OpticNerve,
    pub intraocular_pressure_mmhg: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cornea {
    pub curvature_diopters: f64,
    pub thickness_microns: f64,
    pub clarity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lens {
    pub focal_length_mm: f64,
    pub accommodation_range_diopters: f64,
    pub transparency: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Retina {
    pub photoreceptors: Photoreceptors,
    pub ganglion_cells: u32,
    pub macula_health: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photoreceptors {
    pub rods: u32,
    pub cones: ConeDistribution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConeDistribution {
    pub l_cones: u32,
    pub m_cones: u32,
    pub s_cones: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpticNerve {
    pub fiber_count: u32,
    pub cup_to_disc_ratio: f64,
    pub health_status: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualAcuity {
    pub left_eye: Snellen,
    pub right_eye: Snellen,
    pub binocular: Snellen,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Snellen {
    pub numerator: u32,
    pub denominator: u32,
}

impl Snellen {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        Snellen { numerator, denominator }
    }

    pub fn decimal(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    pub fn is_normal(&self) -> bool {
        self.decimal() >= 1.0
    }

    pub fn is_legally_blind(&self) -> bool {
        self.decimal() <= 0.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorVision {
    Normal,
    Protanomaly,
    Deuteranomaly,
    Tritanomaly,
    Protanopia,
    Deuteranopia,
    Tritanopia,
    Achromatopsia,
}

impl Eye {
    pub fn new(side: Side) -> Self {
        Eye {
            side,
            cornea: Cornea {
                curvature_diopters: 43.0,
                thickness_microns: 550.0,
                clarity: 1.0,
            },
            lens: Lens {
                focal_length_mm: 17.0,
                accommodation_range_diopters: 10.0,
                transparency: 1.0,
            },
            retina: Retina {
                photoreceptors: Photoreceptors {
                    rods: 120_000_000,
                    cones: ConeDistribution {
                        l_cones: 4_000_000,
                        m_cones: 2_000_000,
                        s_cones: 500_000,
                    },
                },
                ganglion_cells: 1_200_000,
                macula_health: 1.0,
            },
            optic_nerve: OpticNerve {
                fiber_count: 1_200_000,
                cup_to_disc_ratio: 0.3,
                health_status: 1.0,
            },
            intraocular_pressure_mmhg: 15.0,
        }
    }

    pub fn has_glaucoma_risk(&self) -> bool {
        self.intraocular_pressure_mmhg > 21.0 ||
        self.optic_nerve.cup_to_disc_ratio > 0.6
    }

    pub fn has_cataract(&self) -> bool {
        self.lens.transparency < 0.8
    }

    pub fn is_presbyopic(&self) -> bool {
        self.lens.accommodation_range_diopters < 3.0
    }
}

impl VisualSystem {
    pub fn new() -> Self {
        VisualSystem {
            eyes: [Eye::new(Side::Left), Eye::new(Side::Right)],
            visual_acuity: VisualAcuity {
                left_eye: Snellen::new(20, 20),
                right_eye: Snellen::new(20, 20),
                binocular: Snellen::new(20, 15),
            },
            color_vision: ColorVision::Normal,
        }
    }

    pub fn has_vision_impairment(&self) -> bool {
        !self.visual_acuity.binocular.is_normal()
    }

    pub fn requires_correction(&self) -> bool {
        self.visual_acuity.left_eye.decimal() < 1.0 ||
        self.visual_acuity.right_eye.decimal() < 1.0
    }
}

impl Default for VisualSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snellen() {
        let vision = Snellen::new(20, 20);
        assert_eq!(vision.decimal(), 1.0);
        assert!(vision.is_normal());
        assert!(!vision.is_legally_blind());

        let poor_vision = Snellen::new(20, 200);
        assert_eq!(poor_vision.decimal(), 0.1);
        assert!(poor_vision.is_legally_blind());
    }

    #[test]
    fn test_eye() {
        let mut eye = Eye::new(Side::Right);
        assert!(!eye.has_glaucoma_risk());
        assert!(!eye.has_cataract());
        assert!(!eye.is_presbyopic());

        eye.intraocular_pressure_mmhg = 25.0;
        assert!(eye.has_glaucoma_risk());

        eye.lens.transparency = 0.7;
        assert!(eye.has_cataract());
    }

    #[test]
    fn test_visual_system() {
        let visual_system = VisualSystem::new();
        assert!(!visual_system.has_vision_impairment());
        assert!(!visual_system.requires_correction());
        assert_eq!(visual_system.color_vision, ColorVision::Normal);
    }
}

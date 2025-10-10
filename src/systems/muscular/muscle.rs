use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Muscle {
    pub name: String,
    pub muscle_type: MuscleType,
    pub fiber_composition: FiberComposition,
    pub cross_sectional_area_cm2: f64,
    pub length_cm: f64,
    pub resting_tension: f64,
    pub max_force_n: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MuscleType {
    Skeletal,
    Cardiac,
    Smooth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiberComposition {
    pub type_i_percent: f64,
    pub type_iia_percent: f64,
    pub type_iix_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleFiber {
    pub fiber_type: FiberType,
    pub diameter_um: f64,
    pub length_mm: f64,
    pub myofibrils: Vec<Myofibril>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiberType {
    TypeI,
    TypeIIA,
    TypeIIX,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Myofibril {
    pub sarcomeres: Vec<Sarcomere>,
    pub diameter_um: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sarcomere {
    pub length_um: f64,
    pub thick_filaments: usize,
    pub thin_filaments: usize,
    pub z_disc_spacing_um: f64,
}

impl Muscle {
    pub fn new_skeletal(name: &str, area_cm2: f64, length_cm: f64) -> Self {
        Self {
            name: name.to_string(),
            muscle_type: MuscleType::Skeletal,
            fiber_composition: FiberComposition::new_mixed(),
            cross_sectional_area_cm2: area_cm2,
            length_cm,
            resting_tension: 0.0,
            max_force_n: area_cm2 * 30.0,
        }
    }

    pub fn new_cardiac() -> Self {
        Self {
            name: "Cardiac".to_string(),
            muscle_type: MuscleType::Cardiac,
            fiber_composition: FiberComposition::new_cardiac(),
            cross_sectional_area_cm2: 50.0,
            length_cm: 10.0,
            resting_tension: 5.0,
            max_force_n: 1500.0,
        }
    }

    pub fn force_at_length(&self, length_cm: f64) -> f64 {
        let optimal_length = self.length_cm;
        let length_ratio = length_cm / optimal_length;

        if !(0.5..=1.5).contains(&length_ratio) {
            0.0
        } else {
            let length_factor = 1.0 - ((length_ratio - 1.0).powi(2) * 4.0);
            self.max_force_n * length_factor.max(0.0)
        }
    }

    pub fn specific_tension(&self) -> f64 {
        self.max_force_n / self.cross_sectional_area_cm2
    }

    pub fn power_output(&self, velocity_cm_per_sec: f64) -> f64 {
        let max_velocity = self.length_cm * 10.0;
        let velocity_factor = 1.0 - (velocity_cm_per_sec / max_velocity).abs();

        if velocity_factor < 0.0 {
            0.0
        } else {
            self.max_force_n * velocity_cm_per_sec * velocity_factor
        }
    }
}

impl FiberComposition {
    pub fn new_slow_twitch() -> Self {
        Self {
            type_i_percent: 80.0,
            type_iia_percent: 15.0,
            type_iix_percent: 5.0,
        }
    }

    pub fn new_fast_twitch() -> Self {
        Self {
            type_i_percent: 20.0,
            type_iia_percent: 30.0,
            type_iix_percent: 50.0,
        }
    }

    pub fn new_mixed() -> Self {
        Self {
            type_i_percent: 50.0,
            type_iia_percent: 30.0,
            type_iix_percent: 20.0,
        }
    }

    pub fn new_cardiac() -> Self {
        Self {
            type_i_percent: 100.0,
            type_iia_percent: 0.0,
            type_iix_percent: 0.0,
        }
    }

    pub fn oxidative_capacity(&self) -> f64 {
        (self.type_i_percent * 1.0 + self.type_iia_percent * 0.7 + self.type_iix_percent * 0.3)
            / 100.0
    }

    pub fn glycolytic_capacity(&self) -> f64 {
        (self.type_i_percent * 0.3 + self.type_iia_percent * 0.7 + self.type_iix_percent * 1.0)
            / 100.0
    }
}

impl MuscleFiber {
    pub fn new(fiber_type: FiberType) -> Self {
        let (diameter_um, length_mm) = match fiber_type {
            FiberType::TypeI => (50.0, 30.0),
            FiberType::TypeIIA => (60.0, 35.0),
            FiberType::TypeIIX => (70.0, 40.0),
        };

        let num_sarcomeres = (length_mm * 1000.0 / 2.5) as usize;
        let myofibrils = vec![Myofibril::new(num_sarcomeres); 100];

        Self {
            fiber_type,
            diameter_um,
            length_mm,
            myofibrils,
        }
    }

    pub fn max_contraction_velocity(&self) -> f64 {
        match self.fiber_type {
            FiberType::TypeI => 4.0,
            FiberType::TypeIIA => 8.0,
            FiberType::TypeIIX => 12.0,
        }
    }

    pub fn fatigue_resistance(&self) -> f64 {
        match self.fiber_type {
            FiberType::TypeI => 1.0,
            FiberType::TypeIIA => 0.6,
            FiberType::TypeIIX => 0.3,
        }
    }
}

impl Myofibril {
    pub fn new(num_sarcomeres: usize) -> Self {
        let sarcomeres = (0..num_sarcomeres)
            .map(|_| Sarcomere::new_relaxed())
            .collect();

        Self {
            sarcomeres,
            diameter_um: 1.0,
        }
    }

    pub fn total_length_um(&self) -> f64 {
        self.sarcomeres.iter().map(|s| s.length_um).sum()
    }
}

impl Sarcomere {
    pub fn new_relaxed() -> Self {
        Self {
            length_um: 2.5,
            thick_filaments: 300,
            thin_filaments: 600,
            z_disc_spacing_um: 2.5,
        }
    }

    pub fn new_contracted() -> Self {
        Self {
            length_um: 2.0,
            thick_filaments: 300,
            thin_filaments: 600,
            z_disc_spacing_um: 2.0,
        }
    }

    pub fn overlap_ratio(&self) -> f64 {
        let optimal_length = 2.2;
        if self.length_um > 3.6 || self.length_um < 1.3 {
            0.0
        } else {
            1.0 - ((self.length_um - optimal_length) / optimal_length).abs()
        }
    }

    pub fn force_potential(&self) -> f64 {
        self.overlap_ratio()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muscle_creation() {
        let muscle = Muscle::new_skeletal("Biceps", 10.0, 30.0);
        assert_eq!(muscle.name, "Biceps");
        assert_eq!(muscle.muscle_type, MuscleType::Skeletal);
    }

    #[test]
    fn test_length_tension() {
        let muscle = Muscle::new_skeletal("Test", 10.0, 30.0);
        let force_optimal = muscle.force_at_length(30.0);
        let force_stretched = muscle.force_at_length(40.0);
        let force_shortened = muscle.force_at_length(20.0);

        assert!(force_optimal > force_stretched);
        assert!(force_optimal > force_shortened);
    }

    #[test]
    fn test_fiber_composition() {
        let slow = FiberComposition::new_slow_twitch();
        let fast = FiberComposition::new_fast_twitch();

        assert!(slow.oxidative_capacity() > fast.oxidative_capacity());
        assert!(fast.glycolytic_capacity() > slow.glycolytic_capacity());
    }

    #[test]
    fn test_muscle_fiber() {
        let fiber = MuscleFiber::new(FiberType::TypeI);
        assert!(fiber.fatigue_resistance() > 0.9);
        assert!(fiber.max_contraction_velocity() < 5.0);
    }

    #[test]
    fn test_sarcomere_length() {
        let relaxed = Sarcomere::new_relaxed();
        let contracted = Sarcomere::new_contracted();

        assert!(relaxed.length_um > contracted.length_um);
        assert!(relaxed.overlap_ratio() > 0.0);
    }

    #[test]
    fn test_power_output() {
        let muscle = Muscle::new_skeletal("Test", 10.0, 30.0);
        let power = muscle.power_output(50.0);
        assert!(power > 0.0);
    }
}

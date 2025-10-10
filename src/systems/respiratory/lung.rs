use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lung {
    pub side: LungSide,
    pub lobes: Vec<Lobe>,
    pub total_capacity_ml: f64,
    pub residual_volume_ml: f64,
    pub tidal_volume_ml: f64,
    pub respiratory_rate_bpm: f64,
    pub compliance_ml_per_cmh2o: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LungSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lobe {
    pub name: String,
    pub volume_ml: f64,
    pub alveoli: Vec<Alveolus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alveolus {
    pub surface_area_um2: f64,
    pub wall_thickness_um: f64,
    pub surfactant_coverage: f64,
}

impl Lung {
    pub fn new_left() -> Self {
        Self {
            side: LungSide::Left,
            lobes: vec![Lobe::new("Superior", 1200.0), Lobe::new("Inferior", 1800.0)],
            total_capacity_ml: 3000.0,
            residual_volume_ml: 1200.0,
            tidal_volume_ml: 500.0,
            respiratory_rate_bpm: 12.0,
            compliance_ml_per_cmh2o: 200.0,
        }
    }

    pub fn new_right() -> Self {
        Self {
            side: LungSide::Right,
            lobes: vec![
                Lobe::new("Superior", 1200.0),
                Lobe::new("Middle", 800.0),
                Lobe::new("Inferior", 2000.0),
            ],
            total_capacity_ml: 4000.0,
            residual_volume_ml: 1200.0,
            tidal_volume_ml: 500.0,
            respiratory_rate_bpm: 12.0,
            compliance_ml_per_cmh2o: 200.0,
        }
    }

    pub fn vital_capacity(&self) -> f64 {
        self.total_capacity_ml - self.residual_volume_ml
    }

    pub fn minute_ventilation(&self) -> f64 {
        self.tidal_volume_ml * self.respiratory_rate_bpm
    }

    pub fn alveolar_ventilation(&self) -> f64 {
        let dead_space = 150.0;
        (self.tidal_volume_ml - dead_space) * self.respiratory_rate_bpm
    }

    pub fn functional_residual_capacity(&self) -> f64 {
        self.residual_volume_ml + (self.total_capacity_ml - self.vital_capacity()) / 2.0
    }
}

impl Lobe {
    pub fn new(name: &str, volume_ml: f64) -> Self {
        let num_alveoli = ((volume_ml * 1000.0) / 0.004).round() as usize;
        let alveoli = (0..num_alveoli.min(1000))
            .map(|_| Alveolus::new())
            .collect();

        Self {
            name: name.to_string(),
            volume_ml,
            alveoli,
        }
    }

    pub fn total_surface_area_m2(&self) -> f64 {
        let total_um2: f64 = self.alveoli.iter().map(|a| a.surface_area_um2).sum();
        total_um2 / 1_000_000_000_000.0
    }
}

impl Alveolus {
    pub fn new() -> Self {
        Self {
            surface_area_um2: 250_000.0,
            wall_thickness_um: 0.5,
            surfactant_coverage: 0.95,
        }
    }

    pub fn diffusion_capacity(&self) -> f64 {
        let thickness_factor = 1.0 / self.wall_thickness_um;
        let area_factor = self.surface_area_um2 / 250_000.0;
        let surfactant_factor = self.surfactant_coverage;

        thickness_factor * area_factor * surfactant_factor
    }
}

impl Default for Alveolus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lung_creation() {
        let left = Lung::new_left();
        let right = Lung::new_right();

        assert_eq!(left.side, LungSide::Left);
        assert_eq!(right.side, LungSide::Right);
        assert_eq!(left.lobes.len(), 2);
        assert_eq!(right.lobes.len(), 3);
    }

    #[test]
    fn test_vital_capacity() {
        let lung = Lung::new_left();
        let vc = lung.vital_capacity();
        assert!(vc > 0.0);
        assert_eq!(vc, lung.total_capacity_ml - lung.residual_volume_ml);
    }

    #[test]
    fn test_minute_ventilation() {
        let lung = Lung::new_left();
        let mv = lung.minute_ventilation();
        assert_eq!(mv, 500.0 * 12.0);
    }

    #[test]
    fn test_alveolar_ventilation() {
        let lung = Lung::new_left();
        let av = lung.alveolar_ventilation();
        assert!(av > 0.0);
        assert!(av < lung.minute_ventilation());
    }

    #[test]
    fn test_alveolus_diffusion() {
        let alveolus = Alveolus::new();
        let capacity = alveolus.diffusion_capacity();
        assert!(capacity > 0.0);
    }

    #[test]
    fn test_lobe_surface_area() {
        let lobe = Lobe::new("Test", 1000.0);
        let area = lobe.total_surface_area_m2();
        assert!(area > 0.0);
    }
}

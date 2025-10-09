use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spleen {
    pub mass_g: f64,
    pub length_cm: f64,
    pub width_cm: f64,
    pub thickness_cm: f64,
    pub blood_flow_ml_min: f64,
    pub white_pulp: WhitePulp,
    pub red_pulp: RedPulp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitePulp {
    pub t_cell_areas: f64,
    pub b_cell_follicles: f64,
    pub marginal_zone: f64,
    pub immune_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedPulp {
    pub sinusoids: f64,
    pub macrophage_count: f64,
    pub rbc_storage: f64,
    pub platelet_storage: f64,
}

impl Spleen {
    pub fn new() -> Self {
        Spleen {
            mass_g: 150.0,
            length_cm: 11.0,
            width_cm: 7.0,
            thickness_cm: 4.0,
            blood_flow_ml_min: 250.0,
            white_pulp: WhitePulp {
                t_cell_areas: 1e8,
                b_cell_follicles: 1e8,
                marginal_zone: 5e7,
                immune_activity: 1.0,
            },
            red_pulp: RedPulp {
                sinusoids: 1e6,
                macrophage_count: 1e9,
                rbc_storage: 5e10,
                platelet_storage: 3e11,
            },
        }
    }

    pub fn is_enlarged(&self) -> bool {
        self.length_cm > 13.0 || self.mass_g > 200.0
    }

    pub fn calculate_volume_ml(&self) -> f64 {
        self.length_cm * self.width_cm * self.thickness_cm * 0.524
    }

    pub fn filter_blood_per_day(&self) -> f64 {
        self.blood_flow_ml_min * 60.0 * 24.0
    }
}

impl Default for Spleen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spleen_creation() {
        let spleen = Spleen::new();
        assert_eq!(spleen.mass_g, 150.0);
        assert!(!spleen.is_enlarged());
    }

    #[test]
    fn test_spleen_enlargement() {
        let mut spleen = Spleen::new();
        spleen.mass_g = 250.0;
        assert!(spleen.is_enlarged());
    }

    #[test]
    fn test_blood_filtration() {
        let spleen = Spleen::new();
        let filtered = spleen.filter_blood_per_day();
        assert!(filtered > 0.0);
    }
}

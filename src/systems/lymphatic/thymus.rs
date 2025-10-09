use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thymus {
    pub mass_g: f64,
    pub cortex: ThymusCortex,
    pub medulla: ThymusMedulla,
    pub t_cell_production_per_day: f64,
    pub age_years: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThymusCortex {
    pub thymocyte_count: f64,
    pub epithelial_cells: f64,
    pub positive_selection_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThymusMedulla {
    pub mature_t_cells: f64,
    pub hassall_corpuscles: usize,
    pub negative_selection_rate: f64,
}

impl Thymus {
    pub fn new(age_years: f64) -> Self {
        let mass_g = if age_years < 20.0 {
            40.0
        } else {
            40.0 * (-0.03 * (age_years - 20.0)).exp()
        };

        let production = if age_years < 20.0 {
            1e8
        } else {
            1e8 * (-0.05 * (age_years - 20.0)).exp()
        };

        Thymus {
            mass_g,
            cortex: ThymusCortex {
                thymocyte_count: 1e9,
                epithelial_cells: 1e7,
                positive_selection_rate: 0.95,
            },
            medulla: ThymusMedulla {
                mature_t_cells: 1e8,
                hassall_corpuscles: 100,
                negative_selection_rate: 0.98,
            },
            t_cell_production_per_day: production,
            age_years,
        }
    }

    pub fn calculate_thymic_output(&self) -> f64 {
        self.t_cell_production_per_day
            * self.cortex.positive_selection_rate
            * self.medulla.negative_selection_rate
    }

    pub fn is_involuted(&self) -> bool {
        self.mass_g < 10.0 || self.age_years > 60.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thymus_young() {
        let thymus = Thymus::new(15.0);
        assert_eq!(thymus.mass_g, 40.0);
        assert!(!thymus.is_involuted());
    }

    #[test]
    fn test_thymus_old() {
        let thymus = Thymus::new(70.0);
        assert!(thymus.mass_g < 40.0);
        assert!(thymus.is_involuted());
    }

    #[test]
    fn test_thymic_output() {
        let thymus = Thymus::new(25.0);
        let output = thymus.calculate_thymic_output();
        assert!(output > 0.0);
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lymph {
    pub volume_l: f64,
    pub composition: LymphComposition,
    pub cellular_components: CellularComponents,
    pub flow_rate_ml_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphComposition {
    pub protein_g_dl: f64,
    pub albumin_g_dl: f64,
    pub globulins_g_dl: f64,
    pub glucose_mg_dl: f64,
    pub electrolytes: Electrolytes,
    pub lipids: Lipids,
    pub immunoglobulins: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Electrolytes {
    pub sodium_meq_l: f64,
    pub potassium_meq_l: f64,
    pub chloride_meq_l: f64,
    pub calcium_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lipids {
    pub chylomicrons_mg_dl: f64,
    pub triglycerides_mg_dl: f64,
    pub cholesterol_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularComponents {
    pub lymphocytes_per_ul: f64,
    pub t_cells_percent: f64,
    pub b_cells_percent: f64,
    pub nk_cells_percent: f64,
    pub macrophages_per_ul: f64,
    pub dendritic_cells_per_ul: f64,
}

impl Lymph {
    pub fn new() -> Self {
        Lymph {
            volume_l: 2.0,
            composition: LymphComposition::normal(),
            cellular_components: CellularComponents::normal(),
            flow_rate_ml_min: 2.5,
        }
    }

    pub fn calculate_protein_concentration(&self) -> f64 {
        self.composition.protein_g_dl
    }

    pub fn is_chylous(&self) -> bool {
        self.composition.lipids.chylomicrons_mg_dl > 100.0
    }
}

impl LymphComposition {
    pub fn normal() -> Self {
        LymphComposition {
            protein_g_dl: 3.0,
            albumin_g_dl: 2.0,
            globulins_g_dl: 1.0,
            glucose_mg_dl: 90.0,
            electrolytes: Electrolytes {
                sodium_meq_l: 140.0,
                potassium_meq_l: 4.0,
                chloride_meq_l: 100.0,
                calcium_mg_dl: 9.0,
            },
            lipids: Lipids {
                chylomicrons_mg_dl: 0.0,
                triglycerides_mg_dl: 50.0,
                cholesterol_mg_dl: 100.0,
            },
            immunoglobulins: HashMap::from([
                ("IgG".to_string(), 600.0),
                ("IgA".to_string(), 150.0),
                ("IgM".to_string(), 50.0),
            ]),
        }
    }
}

impl CellularComponents {
    pub fn normal() -> Self {
        CellularComponents {
            lymphocytes_per_ul: 4000.0,
            t_cells_percent: 70.0,
            b_cells_percent: 20.0,
            nk_cells_percent: 10.0,
            macrophages_per_ul: 100.0,
            dendritic_cells_per_ul: 50.0,
        }
    }
}

impl Default for Lymph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lymph_creation() {
        let lymph = Lymph::new();
        assert_eq!(lymph.volume_l, 2.0);
        assert!(lymph.flow_rate_ml_min > 0.0);
    }

    #[test]
    fn test_protein_concentration() {
        let lymph = Lymph::new();
        let protein = lymph.calculate_protein_concentration();
        assert_eq!(protein, 3.0);
    }

    #[test]
    fn test_chylous_detection() {
        let mut lymph = Lymph::new();
        assert!(!lymph.is_chylous());

        lymph.composition.lipids.chylomicrons_mg_dl = 200.0;
        assert!(lymph.is_chylous());
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientAbsorption {
    pub carbohydrate_absorption_g_per_hour: f64,
    pub protein_absorption_g_per_hour: f64,
    pub fat_absorption_g_per_hour: f64,
    pub water_absorption_ml_per_hour: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Villus {
    pub height_um: f64,
    pub width_um: f64,
    pub microvilli_count: usize,
    pub capillary_density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enterocyte {
    pub apical_transporters: Vec<Transporter>,
    pub basolateral_transporters: Vec<Transporter>,
    pub intracellular_enzymes: Vec<DigestiveEnzyme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transporter {
    pub name: String,
    pub substrate: String,
    pub transport_rate_molecules_per_sec: f64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestiveEnzyme {
    pub name: String,
    pub substrate: String,
    pub activity_units: f64,
    pub optimal_ph: f64,
}

impl NutrientAbsorption {
    pub fn new_normal() -> Self {
        Self {
            carbohydrate_absorption_g_per_hour: 60.0,
            protein_absorption_g_per_hour: 40.0,
            fat_absorption_g_per_hour: 20.0,
            water_absorption_ml_per_hour: 400.0,
        }
    }

    pub fn total_caloric_absorption_per_hour(&self) -> f64 {
        self.carbohydrate_absorption_g_per_hour * 4.0
            + self.protein_absorption_g_per_hour * 4.0
            + self.fat_absorption_g_per_hour * 9.0
    }

    pub fn absorption_efficiency(&self, intake_g: f64, absorbed_g: f64) -> f64 {
        if intake_g > 0.0 {
            (absorbed_g / intake_g) * 100.0
        } else {
            0.0
        }
    }
}

impl Villus {
    pub fn new_jejunum() -> Self {
        Self {
            height_um: 1000.0,
            width_um: 100.0,
            microvilli_count: 3000,
            capillary_density: 0.8,
        }
    }

    pub fn new_ileum() -> Self {
        Self {
            height_um: 700.0,
            width_um: 100.0,
            microvilli_count: 2500,
            capillary_density: 0.6,
        }
    }

    pub fn surface_area_um2(&self) -> f64 {
        let villus_surface = 2.0 * self.height_um * self.width_um;
        let microvillus_surface = self.microvilli_count as f64 * 0.5 * 20.0;
        villus_surface + microvillus_surface
    }

    pub fn absorption_capacity(&self) -> f64 {
        self.surface_area_um2() * self.capillary_density
    }
}

impl Enterocyte {
    pub fn new() -> Self {
        Self {
            apical_transporters: vec![
                Transporter::new_sglt1(),
                Transporter::new_pept1(),
                Transporter::new_npc1l1(),
            ],
            basolateral_transporters: vec![Transporter::new_glut2()],
            intracellular_enzymes: vec![
                DigestiveEnzyme::new_lactase(),
                DigestiveEnzyme::new_sucrase(),
                DigestiveEnzyme::new_maltase(),
            ],
        }
    }

    pub fn glucose_transport_rate(&self) -> f64 {
        self.apical_transporters
            .iter()
            .filter(|t| t.substrate == "glucose")
            .map(|t| t.transport_rate_molecules_per_sec)
            .sum()
    }

    pub fn protein_transport_rate(&self) -> f64 {
        self.apical_transporters
            .iter()
            .filter(|t| t.substrate.contains("peptide") || t.substrate.contains("amino acid"))
            .map(|t| t.transport_rate_molecules_per_sec)
            .sum()
    }
}

impl Default for Enterocyte {
    fn default() -> Self {
        Self::new()
    }
}

impl Transporter {
    pub fn new_sglt1() -> Self {
        Self {
            name: "SGLT1".to_string(),
            substrate: "glucose".to_string(),
            transport_rate_molecules_per_sec: 1000.0,
            is_active: true,
        }
    }

    pub fn new_glut2() -> Self {
        Self {
            name: "GLUT2".to_string(),
            substrate: "glucose".to_string(),
            transport_rate_molecules_per_sec: 5000.0,
            is_active: false,
        }
    }

    pub fn new_pept1() -> Self {
        Self {
            name: "PEPT1".to_string(),
            substrate: "dipeptide".to_string(),
            transport_rate_molecules_per_sec: 500.0,
            is_active: true,
        }
    }

    pub fn new_npc1l1() -> Self {
        Self {
            name: "NPC1L1".to_string(),
            substrate: "cholesterol".to_string(),
            transport_rate_molecules_per_sec: 100.0,
            is_active: true,
        }
    }
}

impl DigestiveEnzyme {
    pub fn new_lactase() -> Self {
        Self {
            name: "Lactase".to_string(),
            substrate: "lactose".to_string(),
            activity_units: 100.0,
            optimal_ph: 6.0,
        }
    }

    pub fn new_sucrase() -> Self {
        Self {
            name: "Sucrase".to_string(),
            substrate: "sucrose".to_string(),
            activity_units: 150.0,
            optimal_ph: 6.5,
        }
    }

    pub fn new_maltase() -> Self {
        Self {
            name: "Maltase".to_string(),
            substrate: "maltose".to_string(),
            activity_units: 120.0,
            optimal_ph: 6.5,
        }
    }

    pub fn activity_at_ph(&self, ph: f64) -> f64 {
        let ph_diff = (ph - self.optimal_ph).abs();
        self.activity_units * (-ph_diff * ph_diff / 2.0).exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nutrient_absorption() {
        let absorption = NutrientAbsorption::new_normal();
        let calories = absorption.total_caloric_absorption_per_hour();
        assert!(calories > 0.0);
    }

    #[test]
    fn test_absorption_efficiency() {
        let absorption = NutrientAbsorption::new_normal();
        let efficiency = absorption.absorption_efficiency(100.0, 95.0);
        assert_eq!(efficiency, 95.0);
    }

    #[test]
    fn test_villus() {
        let jejunum_villus = Villus::new_jejunum();
        let ileum_villus = Villus::new_ileum();

        assert!(jejunum_villus.surface_area_um2() > ileum_villus.surface_area_um2());
        assert!(jejunum_villus.absorption_capacity() > ileum_villus.absorption_capacity());
    }

    #[test]
    fn test_enterocyte() {
        let cell = Enterocyte::new();
        assert!(cell.glucose_transport_rate() > 0.0);
        assert!(cell.protein_transport_rate() > 0.0);
    }

    #[test]
    fn test_transporters() {
        let sglt1 = Transporter::new_sglt1();
        let glut2 = Transporter::new_glut2();

        assert!(sglt1.is_active);
        assert!(!glut2.is_active);
        assert!(glut2.transport_rate_molecules_per_sec > sglt1.transport_rate_molecules_per_sec);
    }

    #[test]
    fn test_digestive_enzymes() {
        let lactase = DigestiveEnzyme::new_lactase();
        let activity_optimal = lactase.activity_at_ph(6.0);
        let activity_suboptimal = lactase.activity_at_ph(4.0);

        assert!(activity_optimal > activity_suboptimal);
    }
}

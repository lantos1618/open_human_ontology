use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skin {
    pub epidermis: Epidermis,
    pub dermis: Dermis,
    pub hypodermis: Hypodermis,
    pub total_surface_area_m2: f64,
    pub thickness_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Epidermis {
    pub stratum_corneum: StratumCorneum,
    pub stratum_granulosum: Layer,
    pub stratum_spinosum: Layer,
    pub stratum_basale: StratumBasale,
    pub thickness_um: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratumCorneum {
    pub corneocytes: usize,
    pub barrier_function: f64,
    pub water_loss_g_per_m2_per_hour: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratumBasale {
    pub keratinocytes: usize,
    pub melanocytes: usize,
    pub cell_division_rate_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub thickness_um: f64,
    pub cell_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dermis {
    pub papillary_layer: DermisLayer,
    pub reticular_layer: DermisLayer,
    pub blood_vessels_per_cm2: f64,
    pub nerve_endings_per_cm2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DermisLayer {
    pub thickness_mm: f64,
    pub collagen_density: f64,
    pub elastin_density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypodermis {
    pub thickness_mm: f64,
    pub fat_cell_count_per_cm3: f64,
    pub insulation_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Melanin {
    pub melanin_type: MelaninType,
    pub concentration_mg_per_g: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MelaninType {
    Eumelanin,
    Pheomelanin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinType {
    pub fitzpatrick_type: u8,
    pub melanin_index: f64,
    pub uv_sensitivity: f64,
}

impl Skin {
    pub fn new_adult(surface_area_m2: f64) -> Self {
        Self {
            epidermis: Epidermis::new(),
            dermis: Dermis::new(),
            hypodermis: Hypodermis::new(),
            total_surface_area_m2: surface_area_m2,
            thickness_mm: 2.0,
        }
    }

    pub fn barrier_integrity(&self) -> f64 {
        self.epidermis.stratum_corneum.barrier_function
    }

    pub fn thermal_regulation_capacity(&self) -> f64 {
        let insulation = self.hypodermis.insulation_factor;
        let blood_flow = self.dermis.blood_vessels_per_cm2 / 100.0;
        insulation * (1.0 - blood_flow)
    }

    pub fn sensory_density(&self) -> f64 {
        self.dermis.nerve_endings_per_cm2
    }
}

impl Epidermis {
    pub fn new() -> Self {
        Self {
            stratum_corneum: StratumCorneum::new(),
            stratum_granulosum: Layer::new("Granulosum", 10.0, 5000),
            stratum_spinosum: Layer::new("Spinosum", 50.0, 10000),
            stratum_basale: StratumBasale::new(),
            thickness_um: 100.0,
        }
    }

    pub fn turnover_time_days(&self) -> f64 {
        28.0
    }

    pub fn water_barrier_effectiveness(&self) -> f64 {
        1.0 / self.stratum_corneum.water_loss_g_per_m2_per_hour
    }
}

impl Default for Epidermis {
    fn default() -> Self {
        Self::new()
    }
}

impl StratumCorneum {
    pub fn new() -> Self {
        Self {
            corneocytes: 15000,
            barrier_function: 0.95,
            water_loss_g_per_m2_per_hour: 10.0,
        }
    }
}

impl Default for StratumCorneum {
    fn default() -> Self {
        Self::new()
    }
}

impl StratumBasale {
    pub fn new() -> Self {
        Self {
            keratinocytes: 20000,
            melanocytes: 2000,
            cell_division_rate_per_day: 1000.0,
        }
    }

    pub fn melanocyte_ratio(&self) -> f64 {
        self.melanocytes as f64 / self.keratinocytes as f64
    }
}

impl Default for StratumBasale {
    fn default() -> Self {
        Self::new()
    }
}

impl Layer {
    pub fn new(name: &str, thickness_um: f64, cell_count: usize) -> Self {
        Self {
            name: name.to_string(),
            thickness_um,
            cell_count,
        }
    }

    pub fn cell_density_per_um3(&self) -> f64 {
        self.cell_count as f64 / (self.thickness_um * 1000.0)
    }
}

impl Dermis {
    pub fn new() -> Self {
        Self {
            papillary_layer: DermisLayer::new(0.4, 0.6, 0.3),
            reticular_layer: DermisLayer::new(1.6, 0.8, 0.2),
            blood_vessels_per_cm2: 50.0,
            nerve_endings_per_cm2: 200.0,
        }
    }

    pub fn total_thickness_mm(&self) -> f64 {
        self.papillary_layer.thickness_mm + self.reticular_layer.thickness_mm
    }

    pub fn structural_integrity(&self) -> f64 {
        (self.papillary_layer.collagen_density + self.reticular_layer.collagen_density) / 2.0
    }
}

impl Default for Dermis {
    fn default() -> Self {
        Self::new()
    }
}

impl DermisLayer {
    pub fn new(thickness_mm: f64, collagen_density: f64, elastin_density: f64) -> Self {
        Self {
            thickness_mm,
            collagen_density,
            elastin_density,
        }
    }

    pub fn elasticity_score(&self) -> f64 {
        self.elastin_density * (1.0 + self.collagen_density)
    }
}

impl Hypodermis {
    pub fn new() -> Self {
        Self {
            thickness_mm: 5.0,
            fat_cell_count_per_cm3: 1_000_000.0,
            insulation_factor: 0.8,
        }
    }

    pub fn energy_storage_kcal(&self, volume_cm3: f64) -> f64 {
        let total_fat_cells = volume_cm3 * self.fat_cell_count_per_cm3;
        total_fat_cells * 0.0009
    }
}

impl Default for Hypodermis {
    fn default() -> Self {
        Self::new()
    }
}

impl Melanin {
    pub fn new_eumelanin(concentration: f64) -> Self {
        Self {
            melanin_type: MelaninType::Eumelanin,
            concentration_mg_per_g: concentration,
        }
    }

    pub fn new_pheomelanin(concentration: f64) -> Self {
        Self {
            melanin_type: MelaninType::Pheomelanin,
            concentration_mg_per_g: concentration,
        }
    }

    pub fn photoprotection_factor(&self) -> f64 {
        match self.melanin_type {
            MelaninType::Eumelanin => self.concentration_mg_per_g * 2.0,
            MelaninType::Pheomelanin => self.concentration_mg_per_g * 0.5,
        }
    }
}

impl SkinType {
    pub fn new_type_i() -> Self {
        Self {
            fitzpatrick_type: 1,
            melanin_index: 0.2,
            uv_sensitivity: 1.0,
        }
    }

    pub fn new_type_iii() -> Self {
        Self {
            fitzpatrick_type: 3,
            melanin_index: 0.5,
            uv_sensitivity: 0.5,
        }
    }

    pub fn new_type_vi() -> Self {
        Self {
            fitzpatrick_type: 6,
            melanin_index: 1.0,
            uv_sensitivity: 0.1,
        }
    }

    pub fn minimal_erythema_dose_mj_per_cm2(&self) -> f64 {
        match self.fitzpatrick_type {
            1 => 20.0,
            2 => 25.0,
            3 => 30.0,
            4 => 45.0,
            5 => 60.0,
            6 => 100.0,
            _ => 30.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skin_creation() {
        let skin = Skin::new_adult(1.8);
        assert_eq!(skin.total_surface_area_m2, 1.8);
        assert!(skin.barrier_integrity() > 0.9);
    }

    #[test]
    fn test_epidermis() {
        let epidermis = Epidermis::new();
        assert_eq!(epidermis.turnover_time_days(), 28.0);
        assert!(epidermis.water_barrier_effectiveness() > 0.0);
    }

    #[test]
    fn test_stratum_basale() {
        let basale = StratumBasale::new();
        let ratio = basale.melanocyte_ratio();
        assert!(ratio > 0.0 && ratio < 0.2);
    }

    #[test]
    fn test_dermis() {
        let dermis = Dermis::new();
        assert_eq!(dermis.total_thickness_mm(), 2.0);
        assert!(dermis.structural_integrity() > 0.0);
    }

    #[test]
    fn test_hypodermis() {
        let hypodermis = Hypodermis::new();
        let energy = hypodermis.energy_storage_kcal(1000.0);
        assert!(energy > 0.0);
    }

    #[test]
    fn test_melanin() {
        let eumelanin = Melanin::new_eumelanin(1.0);
        let pheomelanin = Melanin::new_pheomelanin(1.0);

        assert!(eumelanin.photoprotection_factor() > pheomelanin.photoprotection_factor());
    }

    #[test]
    fn test_skin_types() {
        let type_i = SkinType::new_type_i();
        let type_vi = SkinType::new_type_vi();

        assert!(type_i.uv_sensitivity > type_vi.uv_sensitivity);
        assert!(type_vi.minimal_erythema_dose_mj_per_cm2() > type_i.minimal_erythema_dose_mj_per_cm2());
    }
}

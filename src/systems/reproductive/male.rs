use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaleReproductiveSystem {
    pub testes: Vec<Testis>,
    pub epididymis: Epididymis,
    pub vas_deferens: VasDeferens,
    pub prostate: Prostate,
    pub seminal_vesicles: SeminalVesicles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testis {
    pub volume_ml: f64,
    pub seminiferous_tubules: SeminiferousTubules,
    pub leydig_cells: LeydigCells,
    pub sertoli_cells: SertoliCells,
    pub temperature_c: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeminiferousTubules {
    pub total_length_m: f64,
    pub spermatogenesis_rate_per_day: u64,
    pub tubule_diameter_um: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeydigCells {
    pub cell_count: u64,
    pub testosterone_production_mg_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SertoliCells {
    pub cell_count: u64,
    pub sperm_support_capacity: f64,
    pub blood_testis_barrier_integrity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Epididymis {
    pub length_m: f64,
    pub sperm_storage_capacity: u64,
    pub maturation_time_days: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VasDeferens {
    pub length_cm: f64,
    pub diameter_mm: f64,
    pub peristalsis_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prostate {
    pub weight_g: f64,
    pub volume_ml: f64,
    pub fluid_production_ml_per_ejaculation: f64,
    pub psa_level_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeminalVesicles {
    pub volume_ml: f64,
    pub fructose_concentration_mg_ml: f64,
    pub fluid_contribution_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sperm {
    pub count_per_ml: u64,
    pub motility_percent: f64,
    pub morphology_normal_percent: f64,
    pub viability_percent: f64,
}

impl MaleReproductiveSystem {
    pub fn new_adult() -> Self {
        Self {
            testes: vec![Testis::new(), Testis::new()],
            epididymis: Epididymis::new(),
            vas_deferens: VasDeferens::new(),
            prostate: Prostate::new(),
            seminal_vesicles: SeminalVesicles::new(),
        }
    }

    pub fn daily_sperm_production(&self) -> u64 {
        self.testes
            .iter()
            .map(|t| t.seminiferous_tubules.spermatogenesis_rate_per_day)
            .sum()
    }

    pub fn testosterone_production_mg_per_day(&self) -> f64 {
        self.testes
            .iter()
            .map(|t| t.leydig_cells.testosterone_production_mg_per_day)
            .sum()
    }
}

impl Testis {
    pub fn new() -> Self {
        Self {
            volume_ml: 18.0,
            seminiferous_tubules: SeminiferousTubules::new(),
            leydig_cells: LeydigCells::new(),
            sertoli_cells: SertoliCells::new(),
            temperature_c: 35.0,
        }
    }

    pub fn is_optimal_temperature(&self) -> bool {
        self.temperature_c >= 34.0 && self.temperature_c <= 36.0
    }

    pub fn spermatogenic_efficiency(&self) -> f64 {
        let temp_factor = if self.is_optimal_temperature() {
            1.0
        } else {
            0.5
        };
        let support_factor = self.sertoli_cells.sperm_support_capacity;
        temp_factor * support_factor
    }
}

impl Default for Testis {
    fn default() -> Self {
        Self::new()
    }
}

impl SeminiferousTubules {
    pub fn new() -> Self {
        Self {
            total_length_m: 250.0,
            spermatogenesis_rate_per_day: 100_000_000,
            tubule_diameter_um: 200.0,
        }
    }

    pub fn total_volume_ml(&self) -> f64 {
        let radius_m = self.tubule_diameter_um / 2_000_000.0;
        std::f64::consts::PI * radius_m * radius_m * self.total_length_m * 1000.0
    }
}

impl Default for SeminiferousTubules {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LeydigCells {
    fn default() -> Self {
        Self::new()
    }
}

impl LeydigCells {
    pub fn new() -> Self {
        Self {
            cell_count: 200_000_000,
            testosterone_production_mg_per_day: 7.0,
        }
    }

    pub fn production_per_cell_ng(&self) -> f64 {
        (self.testosterone_production_mg_per_day * 1_000_000.0) / self.cell_count as f64
    }
}

impl Default for SertoliCells {
    fn default() -> Self {
        Self::new()
    }
}

impl SertoliCells {
    pub fn new() -> Self {
        Self {
            cell_count: 600_000_000,
            sperm_support_capacity: 1.0,
            blood_testis_barrier_integrity: 1.0,
        }
    }

    pub fn sperm_per_sertoli_cell(&self) -> f64 {
        5.0
    }
}

impl Epididymis {
    pub fn new() -> Self {
        Self {
            length_m: 6.0,
            sperm_storage_capacity: 500_000_000,
            maturation_time_days: 14.0,
        }
    }

    pub fn transit_time_hours(&self) -> f64 {
        self.maturation_time_days * 24.0
    }
}

impl Default for Epididymis {
    fn default() -> Self {
        Self::new()
    }
}

impl VasDeferens {
    pub fn new() -> Self {
        Self {
            length_cm: 30.0,
            diameter_mm: 2.5,
            peristalsis_rate: 1.0,
        }
    }
}

impl Default for VasDeferens {
    fn default() -> Self {
        Self::new()
    }
}

impl Prostate {
    pub fn new() -> Self {
        Self {
            weight_g: 20.0,
            volume_ml: 20.0,
            fluid_production_ml_per_ejaculation: 0.5,
            psa_level_ng_ml: 1.0,
        }
    }

    pub fn is_enlarged(&self) -> bool {
        self.volume_ml > 30.0
    }

    pub fn is_psa_elevated(&self) -> bool {
        self.psa_level_ng_ml > 4.0
    }
}

impl Default for Prostate {
    fn default() -> Self {
        Self::new()
    }
}

impl SeminalVesicles {
    pub fn new() -> Self {
        Self {
            volume_ml: 3.0,
            fructose_concentration_mg_ml: 2.0,
            fluid_contribution_percent: 65.0,
        }
    }

    pub fn energy_provision_for_sperm(&self) -> f64 {
        self.fructose_concentration_mg_ml * self.volume_ml
    }
}

impl Default for SeminalVesicles {
    fn default() -> Self {
        Self::new()
    }
}

impl Sperm {
    pub fn new_normal() -> Self {
        Self {
            count_per_ml: 60_000_000,
            motility_percent: 60.0,
            morphology_normal_percent: 15.0,
            viability_percent: 75.0,
        }
    }

    pub fn is_normal_by_who_standards(&self) -> bool {
        self.count_per_ml >= 15_000_000
            && self.motility_percent >= 40.0
            && self.morphology_normal_percent >= 4.0
    }

    pub fn total_motile_count(&self, volume_ml: f64) -> u64 {
        (self.count_per_ml as f64 * volume_ml * self.motility_percent / 100.0) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_male_reproductive_system() {
        let system = MaleReproductiveSystem::new_adult();
        assert_eq!(system.testes.len(), 2);
        assert!(system.daily_sperm_production() > 100_000_000);
    }

    #[test]
    fn test_testis() {
        let testis = Testis::new();
        assert!(testis.is_optimal_temperature());
        assert!(testis.spermatogenic_efficiency() > 0.9);
    }

    #[test]
    fn test_leydig_cells() {
        let leydig = LeydigCells::new();
        assert!(leydig.production_per_cell_ng() > 0.0);
    }

    #[test]
    fn test_epididymis() {
        let epididymis = Epididymis::new();
        assert_eq!(epididymis.transit_time_hours(), 14.0 * 24.0);
    }

    #[test]
    fn test_prostate() {
        let prostate = Prostate::new();
        assert!(!prostate.is_enlarged());
        assert!(!prostate.is_psa_elevated());
    }

    #[test]
    fn test_sperm() {
        let sperm = Sperm::new_normal();
        assert!(sperm.is_normal_by_who_standards());

        let total_motile = sperm.total_motile_count(3.0);
        assert!(total_motile > 100_000_000);
    }
}

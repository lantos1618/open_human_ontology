use crate::biology::{BiologyError, BiologyResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Erythrocyte {
    pub hemoglobin_g_dl: f64,
    pub oxygen_saturation: f64,
    pub mean_corpuscular_volume_fl: f64,
    pub deformability: f64,
    pub age_days: u16,
}

impl Erythrocyte {
    pub fn new() -> Self {
        Erythrocyte {
            hemoglobin_g_dl: 15.0,
            oxygen_saturation: 0.97,
            mean_corpuscular_volume_fl: 90.0,
            deformability: 1.0,
            age_days: 0,
        }
    }

    pub fn oxygen_carrying_capacity(&self) -> f64 {
        self.hemoglobin_g_dl * 1.34 * self.oxygen_saturation
    }

    pub fn is_microcytic(&self) -> bool {
        self.mean_corpuscular_volume_fl < 80.0
    }

    pub fn is_macrocytic(&self) -> bool {
        self.mean_corpuscular_volume_fl > 100.0
    }

    pub fn should_be_removed(&self) -> bool {
        self.age_days > 120 || self.deformability < 0.5
    }
}

impl Default for Erythrocyte {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leukocyte {
    pub subtype: LeukocyteType,
    pub activation_level: f64,
    pub phagocytic_capacity: f64,
    pub cytokine_production_rate: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeukocyteType {
    Neutrophil,
    Lymphocyte,
    Monocyte,
    Eosinophil,
    Basophil,
}

impl Leukocyte {
    pub fn new(subtype: LeukocyteType) -> Self {
        let (phagocytic, cytokine) = match subtype {
            LeukocyteType::Neutrophil => (0.9, 0.6),
            LeukocyteType::Monocyte => (0.8, 0.8),
            LeukocyteType::Lymphocyte => (0.1, 0.9),
            LeukocyteType::Eosinophil => (0.5, 0.5),
            LeukocyteType::Basophil => (0.2, 0.7),
        };

        Leukocyte {
            subtype,
            activation_level: 0.0,
            phagocytic_capacity: phagocytic,
            cytokine_production_rate: cytokine,
        }
    }

    pub fn activate(&mut self) {
        self.activation_level = 1.0;
        self.cytokine_production_rate *= 2.0;
    }

    pub fn is_activated(&self) -> bool {
        self.activation_level > 0.5
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platelet {
    pub diameter_um: f64,
    pub activation_state: PlateletActivation,
    pub aggregation_potential: f64,
    pub granule_content: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlateletActivation {
    Resting,
    Activated,
    Aggregated,
}

impl Platelet {
    pub fn new() -> Self {
        Platelet {
            diameter_um: 2.5,
            activation_state: PlateletActivation::Resting,
            aggregation_potential: 1.0,
            granule_content: 1.0,
        }
    }

    pub fn activate(&mut self) {
        self.activation_state = PlateletActivation::Activated;
        self.diameter_um *= 1.5;
    }

    pub fn aggregate(&mut self) {
        if self.activation_state == PlateletActivation::Activated {
            self.activation_state = PlateletActivation::Aggregated;
        }
    }

    pub fn release_granules(&mut self) -> f64 {
        let released = self.granule_content;
        self.granule_content = 0.0;
        released
    }
}

impl Default for Platelet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hepatocyte {
    pub glycogen_stores_mg: f64,
    pub lipid_content_percent: f64,
    pub albumin_synthesis_rate_mg_day: f64,
    pub detoxification_capacity: f64,
    pub bile_production_rate_ml_day: f64,
}

impl Hepatocyte {
    pub fn new() -> Self {
        Hepatocyte {
            glycogen_stores_mg: 0.05,
            lipid_content_percent: 5.0,
            albumin_synthesis_rate_mg_day: 0.00001,
            detoxification_capacity: 1.0,
            bile_production_rate_ml_day: 0.000001,
        }
    }

    pub fn is_steatotic(&self) -> bool {
        self.lipid_content_percent > 10.0
    }

    pub fn metabolic_capacity(&self) -> f64 {
        let glycogen_factor = (self.glycogen_stores_mg / 0.05).min(1.0);
        let lipid_factor = if self.is_steatotic() { 0.7 } else { 1.0 };
        let detox_factor = self.detoxification_capacity;

        (glycogen_factor + lipid_factor + detox_factor) / 3.0
    }

    pub fn synthesize_protein(&mut self, hours: f64) -> f64 {
        self.albumin_synthesis_rate_mg_day * (hours / 24.0)
    }
}

impl Default for Hepatocyte {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cardiomyocyte {
    pub length_um: f64,
    pub width_um: f64,
    pub contractility: f64,
    pub calcium_handling: f64,
    pub mitochondrial_density: f64,
    pub sarcomere_length_um: f64,
}

impl Cardiomyocyte {
    pub fn new() -> Self {
        Cardiomyocyte {
            length_um: 100.0,
            width_um: 25.0,
            contractility: 1.0,
            calcium_handling: 1.0,
            mitochondrial_density: 0.35,
            sarcomere_length_um: 2.2,
        }
    }

    pub fn volume_um3(&self) -> f64 {
        self.length_um * self.width_um * self.width_um
    }

    pub fn force_generation_potential(&self) -> f64 {
        let sarcomere_optimal =
            if self.sarcomere_length_um >= 1.8 && self.sarcomere_length_um <= 2.4 {
                1.0
            } else {
                0.7
            };

        self.contractility * self.calcium_handling * sarcomere_optimal * self.mitochondrial_density
    }

    pub fn is_hypertrophic(&self) -> bool {
        self.width_um > 30.0 || self.length_um > 150.0
    }
}

impl Default for Cardiomyocyte {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adipocyte {
    pub lipid_droplet_volume_um3: f64,
    pub cell_diameter_um: f64,
    pub adipokine_secretion_rate: f64,
    pub insulin_sensitivity: f64,
    pub lipolysis_rate: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdiposeTissueType {
    White,
    Brown,
    Beige,
}

impl Adipocyte {
    pub fn new(tissue_type: AdiposeTissueType) -> Self {
        let (diameter, sensitivity) = match tissue_type {
            AdiposeTissueType::White => (100.0, 1.0),
            AdiposeTissueType::Brown => (50.0, 1.5),
            AdiposeTissueType::Beige => (75.0, 1.3),
        };

        Adipocyte {
            lipid_droplet_volume_um3: 50000.0,
            cell_diameter_um: diameter,
            adipokine_secretion_rate: 1.0,
            insulin_sensitivity: sensitivity,
            lipolysis_rate: 0.1,
        }
    }

    pub fn lipid_content_percent(&self) -> f64 {
        let cell_volume =
            (4.0 / 3.0) * std::f64::consts::PI * (self.cell_diameter_um / 2.0).powi(3);
        (self.lipid_droplet_volume_um3 / cell_volume) * 100.0
    }

    pub fn is_hypertrophic(&self) -> bool {
        self.cell_diameter_um > 150.0
    }

    pub fn store_lipid(&mut self, amount_um3: f64) {
        self.lipid_droplet_volume_um3 += amount_um3;
        self.cell_diameter_um = (self.cell_diameter_um.powi(3) + amount_um3 / 100.0).cbrt();
    }

    pub fn release_lipid(&mut self, amount_um3: f64) -> BiologyResult<f64> {
        if amount_um3 > self.lipid_droplet_volume_um3 {
            return Err(BiologyError::InvalidValue(
                "Cannot release more lipid than stored".to_string(),
            ));
        }
        self.lipid_droplet_volume_um3 -= amount_um3;
        Ok(amount_um3)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Myocyte {
    pub fiber_type: MuscleFiberType,
    pub length_um: f64,
    pub cross_sectional_area_um2: f64,
    pub mitochondrial_density: f64,
    pub capillary_density: f64,
    pub myoglobin_concentration: f64,
    pub glycogen_content_mg: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MuscleFiberType {
    Type1,
    Type2A,
    Type2X,
}

impl Myocyte {
    pub fn new(fiber_type: MuscleFiberType) -> Self {
        let (mito_density, capillary, myoglobin) = match fiber_type {
            MuscleFiberType::Type1 => (0.25, 4.0, 0.8),
            MuscleFiberType::Type2A => (0.15, 3.0, 0.5),
            MuscleFiberType::Type2X => (0.08, 2.0, 0.3),
        };

        Myocyte {
            fiber_type,
            length_um: 10000.0,
            cross_sectional_area_um2: 5000.0,
            mitochondrial_density: mito_density,
            capillary_density: capillary,
            myoglobin_concentration: myoglobin,
            glycogen_content_mg: 0.03,
        }
    }

    pub fn oxidative_capacity(&self) -> f64 {
        self.mitochondrial_density * self.capillary_density * self.myoglobin_concentration
    }

    pub fn force_production(&self) -> f64 {
        self.cross_sectional_area_um2 / 1000.0
    }

    pub fn is_type1(&self) -> bool {
        matches!(self.fiber_type, MuscleFiberType::Type1)
    }

    pub fn is_fast_twitch(&self) -> bool {
        matches!(
            self.fiber_type,
            MuscleFiberType::Type2A | MuscleFiberType::Type2X
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Osteoblast {
    pub collagen_synthesis_rate: f64,
    pub alkaline_phosphatase_activity: f64,
    pub matrix_deposition_rate: f64,
    pub differentiation_stage: f64,
}

impl Osteoblast {
    pub fn new() -> Self {
        Osteoblast {
            collagen_synthesis_rate: 1.0,
            alkaline_phosphatase_activity: 1.0,
            matrix_deposition_rate: 1.0,
            differentiation_stage: 0.0,
        }
    }

    pub fn bone_formation_capacity(&self) -> f64 {
        (self.collagen_synthesis_rate
            + self.alkaline_phosphatase_activity
            + self.matrix_deposition_rate)
            / 3.0
    }

    pub fn differentiate(&mut self, amount: f64) {
        self.differentiation_stage = (self.differentiation_stage + amount).min(1.0);
    }

    pub fn is_mature(&self) -> bool {
        self.differentiation_stage > 0.8
    }
}

impl Default for Osteoblast {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Osteoclast {
    pub resorption_rate: f64,
    pub acid_secretion: f64,
    pub enzyme_activity: f64,
    pub nuclei_count: u8,
}

impl Osteoclast {
    pub fn new() -> Self {
        Osteoclast {
            resorption_rate: 1.0,
            acid_secretion: 1.0,
            enzyme_activity: 1.0,
            nuclei_count: 3,
        }
    }

    pub fn bone_resorption_capacity(&self) -> f64 {
        self.resorption_rate
            * self.acid_secretion
            * self.enzyme_activity
            * (self.nuclei_count as f64 / 3.0)
    }

    pub fn is_multinucleated(&self) -> bool {
        self.nuclei_count > 1
    }
}

impl Default for Osteoclast {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erythrocyte() {
        let rbc = Erythrocyte::new();
        assert!(rbc.oxygen_carrying_capacity() > 0.0);
        assert!(!rbc.is_microcytic());
        assert!(!rbc.is_macrocytic());
    }

    #[test]
    fn test_leukocyte() {
        let mut wbc = Leukocyte::new(LeukocyteType::Neutrophil);
        assert!(!wbc.is_activated());
        wbc.activate();
        assert!(wbc.is_activated());
    }

    #[test]
    fn test_platelet() {
        let mut plt = Platelet::new();
        assert_eq!(plt.activation_state, PlateletActivation::Resting);
        plt.activate();
        assert_eq!(plt.activation_state, PlateletActivation::Activated);
    }

    #[test]
    fn test_hepatocyte() {
        let hepatocyte = Hepatocyte::new();
        assert!(!hepatocyte.is_steatotic());
        assert!(hepatocyte.metabolic_capacity() > 0.0);
    }

    #[test]
    fn test_cardiomyocyte() {
        let cm = Cardiomyocyte::new();
        assert!(cm.force_generation_potential() > 0.0);
        assert!(!cm.is_hypertrophic());
    }

    #[test]
    fn test_adipocyte() {
        let mut adipocyte = Adipocyte::new(AdiposeTissueType::White);
        assert!(adipocyte.lipid_content_percent() > 0.0);
        adipocyte.store_lipid(10000.0);
        assert!(adipocyte.lipid_droplet_volume_um3 > 50000.0);
    }

    #[test]
    fn test_myocyte() {
        let type1 = Myocyte::new(MuscleFiberType::Type1);
        assert!(type1.is_type1());
        assert!(!type1.is_fast_twitch());
        assert!(type1.oxidative_capacity() > 0.0);

        let type2x = Myocyte::new(MuscleFiberType::Type2X);
        assert!(type2x.is_fast_twitch());
    }

    #[test]
    fn test_osteoblast() {
        let mut ob = Osteoblast::new();
        assert!(!ob.is_mature());
        ob.differentiate(1.0);
        assert!(ob.is_mature());
        assert!(ob.bone_formation_capacity() > 0.0);
    }

    #[test]
    fn test_osteoclast() {
        let oc = Osteoclast::new();
        assert!(oc.is_multinucleated());
        assert!(oc.bone_resorption_capacity() > 0.0);
    }
}

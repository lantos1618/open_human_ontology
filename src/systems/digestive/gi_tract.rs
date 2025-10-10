use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GITract {
    pub mouth: Mouth,
    pub esophagus: Esophagus,
    pub stomach: Stomach,
    pub small_intestine: SmallIntestine,
    pub large_intestine: LargeIntestine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mouth {
    pub saliva_production_ml_per_day: f64,
    pub salivary_amylase_activity: f64,
    pub ph: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Esophagus {
    pub length_cm: f64,
    pub peristalsis_rate_cm_per_sec: f64,
    pub lower_sphincter_pressure_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stomach {
    pub capacity_ml: f64,
    pub current_volume_ml: f64,
    pub ph: f64,
    pub gastric_acid_production_meq_per_hour: f64,
    pub pepsin_concentration_mg_per_ml: f64,
    pub emptying_rate_ml_per_hour: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmallIntestine {
    pub duodenum: IntestinalSegment,
    pub jejunum: IntestinalSegment,
    pub ileum: IntestinalSegment,
    pub total_surface_area_m2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeIntestine {
    pub cecum: IntestinalSegment,
    pub colon: IntestinalSegment,
    pub rectum: IntestinalSegment,
    pub microbiome_density_cfu_per_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntestinalSegment {
    pub name: String,
    pub length_cm: f64,
    pub diameter_cm: f64,
    pub villi_density_per_cm2: f64,
    pub transit_time_hours: f64,
}

impl GITract {
    pub fn new_adult() -> Self {
        Self {
            mouth: Mouth::new(),
            esophagus: Esophagus::new(),
            stomach: Stomach::new(),
            small_intestine: SmallIntestine::new(),
            large_intestine: LargeIntestine::new(),
        }
    }

    pub fn total_length_cm(&self) -> f64 {
        self.esophagus.length_cm
            + self.small_intestine.total_length_cm()
            + self.large_intestine.total_length_cm()
    }

    pub fn total_transit_time_hours(&self) -> f64 {
        self.small_intestine.total_transit_time() + self.large_intestine.total_transit_time()
    }
}

impl Mouth {
    pub fn new() -> Self {
        Self {
            saliva_production_ml_per_day: 1000.0,
            salivary_amylase_activity: 1.0,
            ph: 6.8,
        }
    }

    pub fn carbohydrate_digestion_rate(&self) -> f64 {
        self.salivary_amylase_activity * (7.0 - (self.ph - 7.0).abs())
    }
}

impl Default for Mouth {
    fn default() -> Self {
        Self::new()
    }
}

impl Esophagus {
    pub fn new() -> Self {
        Self {
            length_cm: 25.0,
            peristalsis_rate_cm_per_sec: 3.0,
            lower_sphincter_pressure_mmhg: 20.0,
        }
    }

    pub fn transit_time_seconds(&self) -> f64 {
        self.length_cm / self.peristalsis_rate_cm_per_sec
    }

    pub fn has_reflux_risk(&self) -> bool {
        self.lower_sphincter_pressure_mmhg < 10.0
    }
}

impl Default for Esophagus {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Stomach {
    fn default() -> Self {
        Self::new()
    }
}

impl Stomach {
    pub fn new() -> Self {
        Self {
            capacity_ml: 1500.0,
            current_volume_ml: 50.0,
            ph: 2.0,
            gastric_acid_production_meq_per_hour: 20.0,
            pepsin_concentration_mg_per_ml: 0.5,
            emptying_rate_ml_per_hour: 200.0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.current_volume_ml > self.capacity_ml * 0.8
    }

    pub fn protein_digestion_capacity(&self) -> f64 {
        let ph_factor = if self.ph < 3.0 { 1.0 } else { 0.5 };
        self.pepsin_concentration_mg_per_ml * ph_factor
    }

    pub fn add_content(&mut self, volume_ml: f64) {
        self.current_volume_ml = (self.current_volume_ml + volume_ml).min(self.capacity_ml);
        self.ph = (self.ph * self.current_volume_ml + 7.0 * volume_ml)
            / (self.current_volume_ml + volume_ml);
    }

    pub fn empty(&mut self, duration_hours: f64) -> f64 {
        let emptied = (self.emptying_rate_ml_per_hour * duration_hours).min(self.current_volume_ml);
        self.current_volume_ml -= emptied;
        emptied
    }
}

impl SmallIntestine {
    pub fn new() -> Self {
        Self {
            duodenum: IntestinalSegment::new("Duodenum", 25.0, 3.5, 40.0, 0.5),
            jejunum: IntestinalSegment::new("Jejunum", 250.0, 3.0, 35.0, 2.0),
            ileum: IntestinalSegment::new("Ileum", 350.0, 2.5, 25.0, 3.0),
            total_surface_area_m2: 250.0,
        }
    }

    pub fn total_length_cm(&self) -> f64 {
        self.duodenum.length_cm + self.jejunum.length_cm + self.ileum.length_cm
    }

    pub fn total_transit_time(&self) -> f64 {
        self.duodenum.transit_time_hours
            + self.jejunum.transit_time_hours
            + self.ileum.transit_time_hours
    }

    pub fn absorption_capacity(&self) -> f64 {
        self.total_surface_area_m2 * 10.0
    }
}

impl Default for SmallIntestine {
    fn default() -> Self {
        Self::new()
    }
}

impl LargeIntestine {
    pub fn new() -> Self {
        Self {
            cecum: IntestinalSegment::new("Cecum", 6.0, 7.5, 0.0, 4.0),
            colon: IntestinalSegment::new("Colon", 150.0, 6.0, 0.0, 16.0),
            rectum: IntestinalSegment::new("Rectum", 12.0, 4.0, 0.0, 2.0),
            microbiome_density_cfu_per_ml: 1e11,
        }
    }

    pub fn total_length_cm(&self) -> f64 {
        self.cecum.length_cm + self.colon.length_cm + self.rectum.length_cm
    }

    pub fn total_transit_time(&self) -> f64 {
        self.cecum.transit_time_hours
            + self.colon.transit_time_hours
            + self.rectum.transit_time_hours
    }

    pub fn water_absorption_capacity_ml_per_day(&self) -> f64 {
        1000.0
    }
}

impl Default for LargeIntestine {
    fn default() -> Self {
        Self::new()
    }
}

impl IntestinalSegment {
    pub fn new(
        name: &str,
        length_cm: f64,
        diameter_cm: f64,
        villi_density: f64,
        transit_hours: f64,
    ) -> Self {
        Self {
            name: name.to_string(),
            length_cm,
            diameter_cm,
            villi_density_per_cm2: villi_density,
            transit_time_hours: transit_hours,
        }
    }

    pub fn volume_ml(&self) -> f64 {
        let radius_cm = self.diameter_cm / 2.0;
        std::f64::consts::PI * radius_cm * radius_cm * self.length_cm
    }

    pub fn surface_area_cm2(&self) -> f64 {
        let base_area = std::f64::consts::PI * self.diameter_cm * self.length_cm;
        let villi_factor = 1.0 + self.villi_density_per_cm2 * 0.01;
        base_area * villi_factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gi_tract_creation() {
        let tract = GITract::new_adult();
        assert!(tract.total_length_cm() > 600.0);
    }

    #[test]
    fn test_mouth() {
        let mouth = Mouth::new();
        assert_eq!(mouth.saliva_production_ml_per_day, 1000.0);
        assert!(mouth.carbohydrate_digestion_rate() > 0.0);
    }

    #[test]
    fn test_esophagus() {
        let esophagus = Esophagus::new();
        assert!(esophagus.transit_time_seconds() < 10.0);
        assert!(!esophagus.has_reflux_risk());
    }

    #[test]
    fn test_stomach() {
        let mut stomach = Stomach::new();
        assert!(!stomach.is_full());

        stomach.add_content(500.0);
        assert!(stomach.current_volume_ml > 50.0);

        let emptied = stomach.empty(1.0);
        assert!(emptied > 0.0);
    }

    #[test]
    fn test_small_intestine() {
        let si = SmallIntestine::new();
        assert!(si.total_length_cm() > 600.0);
        assert!(si.total_transit_time() > 4.0);
        assert!(si.absorption_capacity() > 0.0);
    }

    #[test]
    fn test_large_intestine() {
        let li = LargeIntestine::new();
        assert!(li.total_transit_time() > 20.0);
        assert!(li.water_absorption_capacity_ml_per_day() > 0.0);
    }

    #[test]
    fn test_intestinal_segment() {
        let segment = IntestinalSegment::new("Test", 100.0, 3.0, 30.0, 2.0);
        assert!(segment.volume_ml() > 0.0);
        assert!(segment.surface_area_cm2() > 0.0);
    }
}

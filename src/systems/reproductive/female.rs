use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FemaleReproductiveSystem {
    pub ovaries: Vec<Ovary>,
    pub uterus: Uterus,
    pub fallopian_tubes: Vec<FallopianTube>,
    pub vagina: Vagina,
    pub menstrual_cycle: MenstrualCycle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ovary {
    pub volume_cm3: f64,
    pub follicles: Vec<Follicle>,
    pub corpus_luteum_present: bool,
    pub estrogen_production_pg_ml: f64,
    pub progesterone_production_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Follicle {
    pub follicle_type: FollicleType,
    pub diameter_mm: f64,
    pub oocyte_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FollicleType {
    Primordial,
    Primary,
    Secondary,
    Antral,
    Graafian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uterus {
    pub endometrium: Endometrium,
    pub myometrium: Myometrium,
    pub length_cm: f64,
    pub width_cm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endometrium {
    pub thickness_mm: f64,
    pub phase: EndometrialPhase,
    pub gland_density: f64,
    pub blood_vessel_density: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndometrialPhase {
    Menstrual,
    Proliferative,
    Secretory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Myometrium {
    pub thickness_mm: f64,
    pub contractility: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallopianTube {
    pub length_cm: f64,
    pub fimbriae_count: usize,
    pub ciliary_beat_frequency_hz: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vagina {
    pub length_cm: f64,
    pub ph: f64,
    pub lactobacillus_count_cfu_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenstrualCycle {
    pub cycle_length_days: f64,
    pub current_day: f64,
    pub phase: MenstrualPhase,
    pub hormone_levels: HormoneLevels,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MenstrualPhase {
    Follicular,
    Ovulation,
    Luteal,
    Menstruation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormoneLevels {
    pub fsh_miu_ml: f64,
    pub lh_miu_ml: f64,
    pub estradiol_pg_ml: f64,
    pub progesterone_ng_ml: f64,
}

impl FemaleReproductiveSystem {
    pub fn new_adult() -> Self {
        Self {
            ovaries: vec![Ovary::new(), Ovary::new()],
            uterus: Uterus::new(),
            fallopian_tubes: vec![FallopianTube::new(), FallopianTube::new()],
            vagina: Vagina::new(),
            menstrual_cycle: MenstrualCycle::new(),
        }
    }

    pub fn total_follicle_count(&self) -> usize {
        self.ovaries.iter().map(|o| o.follicles.len()).sum()
    }

    pub fn is_ovulating(&self) -> bool {
        matches!(self.menstrual_cycle.phase, MenstrualPhase::Ovulation)
    }
}

impl Ovary {
    pub fn new() -> Self {
        Self {
            volume_cm3: 10.0,
            follicles: Self::initialize_follicles(),
            corpus_luteum_present: false,
            estrogen_production_pg_ml: 50.0,
            progesterone_production_ng_ml: 1.0,
        }
    }

    fn initialize_follicles() -> Vec<Follicle> {
        let mut follicles = vec![];
        for _ in 0..100_000 {
            follicles.push(Follicle::new_primordial());
        }
        for _ in 0..10 {
            follicles.push(Follicle::new_antral());
        }
        follicles
    }

    pub fn mature_follicle(&mut self) -> Option<Follicle> {
        if let Some(pos) = self
            .follicles
            .iter()
            .position(|f| matches!(f.follicle_type, FollicleType::Graafian))
        {
            Some(self.follicles.remove(pos))
        } else {
            None
        }
    }

    pub fn form_corpus_luteum(&mut self) {
        self.corpus_luteum_present = true;
        self.progesterone_production_ng_ml = 15.0;
    }
}

impl Default for Ovary {
    fn default() -> Self {
        Self::new()
    }
}

impl Follicle {
    pub fn new_primordial() -> Self {
        Self {
            follicle_type: FollicleType::Primordial,
            diameter_mm: 0.03,
            oocyte_present: true,
        }
    }

    pub fn new_antral() -> Self {
        Self {
            follicle_type: FollicleType::Antral,
            diameter_mm: 8.0,
            oocyte_present: true,
        }
    }

    pub fn new_graafian() -> Self {
        Self {
            follicle_type: FollicleType::Graafian,
            diameter_mm: 20.0,
            oocyte_present: true,
        }
    }

    pub fn is_mature(&self) -> bool {
        matches!(self.follicle_type, FollicleType::Graafian) && self.diameter_mm >= 18.0
    }
}

impl Uterus {
    pub fn new() -> Self {
        Self {
            endometrium: Endometrium::new(),
            myometrium: Myometrium::new(),
            length_cm: 7.5,
            width_cm: 5.0,
        }
    }

    pub fn volume_cm3(&self) -> f64 {
        self.length_cm * self.width_cm * 4.0
    }

    pub fn is_receptive_for_implantation(&self) -> bool {
        matches!(self.endometrium.phase, EndometrialPhase::Secretory)
            && self.endometrium.thickness_mm >= 7.0
    }
}

impl Default for Uterus {
    fn default() -> Self {
        Self::new()
    }
}

impl Endometrium {
    pub fn new() -> Self {
        Self {
            thickness_mm: 8.0,
            phase: EndometrialPhase::Proliferative,
            gland_density: 0.7,
            blood_vessel_density: 0.8,
        }
    }

    pub fn shed(&mut self) {
        self.thickness_mm = 2.0;
        self.phase = EndometrialPhase::Menstrual;
    }

    pub fn proliferate(&mut self) {
        self.thickness_mm = 10.0;
        self.phase = EndometrialPhase::Proliferative;
        self.gland_density = 0.6;
    }

    pub fn secrete(&mut self) {
        self.thickness_mm = 12.0;
        self.phase = EndometrialPhase::Secretory;
        self.gland_density = 0.9;
    }
}

impl Default for Endometrium {
    fn default() -> Self {
        Self::new()
    }
}

impl Myometrium {
    pub fn new() -> Self {
        Self {
            thickness_mm: 15.0,
            contractility: 0.1,
        }
    }
}

impl Default for Myometrium {
    fn default() -> Self {
        Self::new()
    }
}

impl FallopianTube {
    pub fn new() -> Self {
        Self {
            length_cm: 12.0,
            fimbriae_count: 20,
            ciliary_beat_frequency_hz: 10.0,
        }
    }

    pub fn oocyte_transport_time_hours(&self) -> f64 {
        72.0
    }
}

impl Default for FallopianTube {
    fn default() -> Self {
        Self::new()
    }
}

impl Vagina {
    pub fn new() -> Self {
        Self {
            length_cm: 9.0,
            ph: 4.0,
            lactobacillus_count_cfu_ml: 1e8,
        }
    }

    pub fn is_healthy_ph(&self) -> bool {
        self.ph >= 3.8 && self.ph <= 4.5
    }
}

impl Default for Vagina {
    fn default() -> Self {
        Self::new()
    }
}

impl MenstrualCycle {
    pub fn new() -> Self {
        Self {
            cycle_length_days: 28.0,
            current_day: 1.0,
            phase: MenstrualPhase::Follicular,
            hormone_levels: HormoneLevels::new_follicular(),
        }
    }

    pub fn advance_day(&mut self) {
        self.current_day += 1.0;
        if self.current_day > self.cycle_length_days {
            self.current_day = 1.0;
        }

        self.phase = match self.current_day as u32 {
            1..=5 => MenstrualPhase::Menstruation,
            6..=13 => MenstrualPhase::Follicular,
            14..=16 => MenstrualPhase::Ovulation,
            _ => MenstrualPhase::Luteal,
        };

        self.hormone_levels = match self.phase {
            MenstrualPhase::Follicular => HormoneLevels::new_follicular(),
            MenstrualPhase::Ovulation => HormoneLevels::new_ovulation(),
            MenstrualPhase::Luteal => HormoneLevels::new_luteal(),
            MenstrualPhase::Menstruation => HormoneLevels::new_menstrual(),
        };
    }

    pub fn is_fertile_window(&self) -> bool {
        (10.0..=16.0).contains(&self.current_day)
    }
}

impl Default for MenstrualCycle {
    fn default() -> Self {
        Self::new()
    }
}

impl HormoneLevels {
    pub fn new_follicular() -> Self {
        Self {
            fsh_miu_ml: 5.0,
            lh_miu_ml: 5.0,
            estradiol_pg_ml: 100.0,
            progesterone_ng_ml: 0.5,
        }
    }

    pub fn new_ovulation() -> Self {
        Self {
            fsh_miu_ml: 10.0,
            lh_miu_ml: 50.0,
            estradiol_pg_ml: 300.0,
            progesterone_ng_ml: 1.0,
        }
    }

    pub fn new_luteal() -> Self {
        Self {
            fsh_miu_ml: 3.0,
            lh_miu_ml: 5.0,
            estradiol_pg_ml: 150.0,
            progesterone_ng_ml: 15.0,
        }
    }

    pub fn new_menstrual() -> Self {
        Self {
            fsh_miu_ml: 4.0,
            lh_miu_ml: 3.0,
            estradiol_pg_ml: 50.0,
            progesterone_ng_ml: 0.3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_female_reproductive_system() {
        let system = FemaleReproductiveSystem::new_adult();
        assert_eq!(system.ovaries.len(), 2);
        assert!(system.total_follicle_count() > 100_000);
    }

    #[test]
    fn test_ovary() {
        let ovary = Ovary::new();
        assert!(!ovary.corpus_luteum_present);
        assert!(!ovary.follicles.is_empty());
    }

    #[test]
    fn test_follicle_maturation() {
        let primordial = Follicle::new_primordial();
        let graafian = Follicle::new_graafian();

        assert!(!primordial.is_mature());
        assert!(graafian.is_mature());
    }

    #[test]
    fn test_uterus() {
        let uterus = Uterus::new();
        assert!(uterus.volume_cm3() > 0.0);
    }

    #[test]
    fn test_endometrium_phases() {
        let mut endometrium = Endometrium::new();

        endometrium.shed();
        assert!(endometrium.thickness_mm < 3.0);

        endometrium.proliferate();
        assert!(endometrium.thickness_mm > 8.0);

        endometrium.secrete();
        assert!(endometrium.thickness_mm > 10.0);
    }

    #[test]
    fn test_vagina() {
        let vagina = Vagina::new();
        assert!(vagina.is_healthy_ph());
    }

    #[test]
    fn test_menstrual_cycle() {
        let mut cycle = MenstrualCycle::new();
        assert_eq!(cycle.phase, MenstrualPhase::Follicular);

        for _ in 0..13 {
            cycle.advance_day();
        }
        assert_eq!(cycle.phase, MenstrualPhase::Ovulation);
        assert!(cycle.is_fertile_window());
    }

    #[test]
    fn test_hormone_surge() {
        let ovulation = HormoneLevels::new_ovulation();
        let follicular = HormoneLevels::new_follicular();

        assert!(ovulation.lh_miu_ml > follicular.lh_miu_ml * 5.0);
    }
}

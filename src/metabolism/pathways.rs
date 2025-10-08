use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolicPathway {
    Glycolysis,
    CitricAcidCycle,
    OxidativePhosphorylation,
    Gluconeogenesis,
    GlycogenSynthesis,
    GlycogenBreakdown,
    FattyAcidSynthesis,
    BetaOxidation,
    ProteinSynthesis,
    ProteinCatabolism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glycolysis {
    pub glucose_input_mm: f64,
    pub atp_produced: u32,
    pub nadh_produced: u32,
    pub pyruvate_output_mm: f64,
    pub rate_limiting_enzyme_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitricAcidCycle {
    pub acetyl_coa_input_mm: f64,
    pub atp_produced: u32,
    pub nadh_produced: u32,
    pub fadh2_produced: u32,
    pub co2_produced_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxidativePhosphorylation {
    pub nadh_consumed: u32,
    pub fadh2_consumed: u32,
    pub atp_produced: u32,
    pub oxygen_consumed_mm: f64,
    pub proton_gradient: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gluconeogenesis {
    pub pyruvate_input_mm: f64,
    pub glucose_output_mm: f64,
    pub atp_consumed: u32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FattyAcidOxidation {
    pub fatty_acid_input_mm: f64,
    pub acetyl_coa_output_mm: f64,
    pub atp_yield: u32,
    pub fadh2_produced: u32,
    pub nadh_produced: u32,
}

impl Glycolysis {
    pub fn new_normal() -> Self {
        Self {
            glucose_input_mm: 5.0,
            atp_produced: 2,
            nadh_produced: 2,
            pyruvate_output_mm: 10.0,
            rate_limiting_enzyme_activity: 1.0,
        }
    }

    pub fn net_atp_gain(&self) -> i32 {
        self.atp_produced as i32 - 2
    }

    pub fn is_active(&self) -> bool {
        self.rate_limiting_enzyme_activity > 0.3
    }

    pub fn flux_rate(&self) -> f64 {
        self.glucose_input_mm * self.rate_limiting_enzyme_activity
    }
}

impl CitricAcidCycle {
    pub fn new_normal() -> Self {
        Self {
            acetyl_coa_input_mm: 2.0,
            atp_produced: 1,
            nadh_produced: 3,
            fadh2_produced: 1,
            co2_produced_mm: 2.0,
        }
    }

    pub fn total_reducing_equivalents(&self) -> u32 {
        self.nadh_produced + self.fadh2_produced
    }

    pub fn atp_potential(&self) -> u32 {
        self.atp_produced + (self.nadh_produced * 3) + (self.fadh2_produced * 2)
    }
}

impl OxidativePhosphorylation {
    pub fn new_normal() -> Self {
        Self {
            nadh_consumed: 10,
            fadh2_consumed: 2,
            atp_produced: 34,
            oxygen_consumed_mm: 6.0,
            proton_gradient: 1.0,
        }
    }

    pub fn atp_per_oxygen(&self) -> f64 {
        self.atp_produced as f64 / self.oxygen_consumed_mm
    }

    pub fn efficiency(&self) -> f64 {
        let theoretical_max = (self.nadh_consumed * 3 + self.fadh2_consumed * 2) as f64;
        self.atp_produced as f64 / theoretical_max
    }

    pub fn is_functioning(&self) -> bool {
        self.proton_gradient > 0.5 && self.oxygen_consumed_mm > 0.0
    }
}

impl Gluconeogenesis {
    pub fn new_fasting() -> Self {
        Self {
            pyruvate_input_mm: 2.0,
            glucose_output_mm: 1.0,
            atp_consumed: 6,
            active: true,
        }
    }

    pub fn new_fed() -> Self {
        Self {
            pyruvate_input_mm: 0.0,
            glucose_output_mm: 0.0,
            atp_consumed: 0,
            active: false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active && self.pyruvate_input_mm > 0.0
    }

    pub fn energy_cost(&self) -> u32 {
        if self.active {
            self.atp_consumed
        } else {
            0
        }
    }
}

impl FattyAcidOxidation {
    pub fn new_palmitate() -> Self {
        Self {
            fatty_acid_input_mm: 1.0,
            acetyl_coa_output_mm: 8.0,
            atp_yield: 106,
            fadh2_produced: 7,
            nadh_produced: 7,
        }
    }

    pub fn energy_yield_per_carbon(&self) -> f64 {
        let carbons = self.acetyl_coa_output_mm * 2.0;
        self.atp_yield as f64 / carbons
    }

    pub fn total_reducing_equivalents(&self) -> u32 {
        self.fadh2_produced + self.nadh_produced
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glycolysis() {
        let glycolysis = Glycolysis::new_normal();
        assert_eq!(glycolysis.net_atp_gain(), 0);
        assert!(glycolysis.is_active());
    }

    #[test]
    fn test_citric_acid_cycle() {
        let cycle = CitricAcidCycle::new_normal();
        assert_eq!(cycle.total_reducing_equivalents(), 4);
        assert!(cycle.atp_potential() > 10);
    }

    #[test]
    fn test_oxidative_phosphorylation() {
        let oxphos = OxidativePhosphorylation::new_normal();
        assert!(oxphos.is_functioning());
        assert!(oxphos.atp_per_oxygen() > 5.0);
    }

    #[test]
    fn test_gluconeogenesis() {
        let fasting = Gluconeogenesis::new_fasting();
        let fed = Gluconeogenesis::new_fed();

        assert!(fasting.is_active());
        assert!(!fed.is_active());
        assert!(fasting.energy_cost() > 0);
    }

    #[test]
    fn test_fatty_acid_oxidation() {
        let beta_ox = FattyAcidOxidation::new_palmitate();
        assert_eq!(beta_ox.atp_yield, 106);
        assert!(beta_ox.energy_yield_per_carbon() > 6.0);
    }
}

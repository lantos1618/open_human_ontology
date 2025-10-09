use crate::biology::{BiologyError, BiologyResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedGlycolysis {
    pub glucose: f64,
    pub glucose_6_phosphate: f64,
    pub fructose_6_phosphate: f64,
    pub fructose_1_6_bisphosphate: f64,
    pub glyceraldehyde_3_phosphate: f64,
    pub dihydroxyacetone_phosphate: f64,
    pub phosphoenolpyruvate: f64,
    pub pyruvate: f64,
    pub atp_consumed: u32,
    pub atp_produced: u32,
    pub nadh_produced: u32,
}

impl DetailedGlycolysis {
    pub fn new() -> Self {
        DetailedGlycolysis {
            glucose: 1.0,
            glucose_6_phosphate: 0.0,
            fructose_6_phosphate: 0.0,
            fructose_1_6_bisphosphate: 0.0,
            glyceraldehyde_3_phosphate: 0.0,
            dihydroxyacetone_phosphate: 0.0,
            phosphoenolpyruvate: 0.0,
            pyruvate: 0.0,
            atp_consumed: 0,
            atp_produced: 0,
            nadh_produced: 0,
        }
    }

    pub fn step_1_hexokinase(&mut self) -> BiologyResult<()> {
        if self.glucose < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient glucose".to_string()));
        }
        self.glucose -= 1.0;
        self.glucose_6_phosphate += 1.0;
        self.atp_consumed += 1;
        Ok(())
    }

    pub fn step_2_phosphoglucose_isomerase(&mut self) -> BiologyResult<()> {
        if self.glucose_6_phosphate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient G6P".to_string()));
        }
        self.glucose_6_phosphate -= 1.0;
        self.fructose_6_phosphate += 1.0;
        Ok(())
    }

    pub fn step_3_phosphofructokinase(&mut self) -> BiologyResult<()> {
        if self.fructose_6_phosphate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient F6P".to_string()));
        }
        self.fructose_6_phosphate -= 1.0;
        self.fructose_1_6_bisphosphate += 1.0;
        self.atp_consumed += 1;
        Ok(())
    }

    pub fn step_4_aldolase(&mut self) -> BiologyResult<()> {
        if self.fructose_1_6_bisphosphate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient F-1,6-BP".to_string()));
        }
        self.fructose_1_6_bisphosphate -= 1.0;
        self.glyceraldehyde_3_phosphate += 1.0;
        self.dihydroxyacetone_phosphate += 1.0;
        Ok(())
    }

    pub fn complete_glycolysis(&mut self) -> BiologyResult<()> {
        self.step_1_hexokinase()?;
        self.step_2_phosphoglucose_isomerase()?;
        self.step_3_phosphofructokinase()?;
        self.step_4_aldolase()?;

        self.glyceraldehyde_3_phosphate += self.dihydroxyacetone_phosphate;
        self.dihydroxyacetone_phosphate = 0.0;

        self.phosphoenolpyruvate = self.glyceraldehyde_3_phosphate * 2.0;
        self.nadh_produced += 2;
        self.atp_produced += 4;
        self.pyruvate = self.phosphoenolpyruvate;
        self.phosphoenolpyruvate = 0.0;
        self.glyceraldehyde_3_phosphate = 0.0;

        Ok(())
    }

    pub fn net_atp(&self) -> i32 {
        self.atp_produced as i32 - self.atp_consumed as i32
    }
}

impl Default for DetailedGlycolysis {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrebsCycle {
    pub acetyl_coa: f64,
    pub citrate: f64,
    pub isocitrate: f64,
    pub alpha_ketoglutarate: f64,
    pub succinyl_coa: f64,
    pub succinate: f64,
    pub fumarate: f64,
    pub malate: f64,
    pub oxaloacetate: f64,
    pub nadh_produced: u32,
    pub fadh2_produced: u32,
    pub gtp_produced: u32,
    pub co2_released: u32,
}

impl KrebsCycle {
    pub fn new() -> Self {
        KrebsCycle {
            acetyl_coa: 2.0,
            citrate: 0.0,
            isocitrate: 0.0,
            alpha_ketoglutarate: 0.0,
            succinyl_coa: 0.0,
            succinate: 0.0,
            fumarate: 0.0,
            malate: 0.0,
            oxaloacetate: 1.0,
            nadh_produced: 0,
            fadh2_produced: 0,
            gtp_produced: 0,
            co2_released: 0,
        }
    }

    pub fn step_1_citrate_synthase(&mut self) -> BiologyResult<()> {
        if self.acetyl_coa < 0.1 || self.oxaloacetate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient substrates".to_string()));
        }
        self.acetyl_coa -= 1.0;
        self.oxaloacetate -= 1.0;
        self.citrate += 1.0;
        Ok(())
    }

    pub fn step_2_aconitase(&mut self) -> BiologyResult<()> {
        if self.citrate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient citrate".to_string()));
        }
        self.citrate -= 1.0;
        self.isocitrate += 1.0;
        Ok(())
    }

    pub fn step_3_isocitrate_dehydrogenase(&mut self) -> BiologyResult<()> {
        if self.isocitrate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient isocitrate".to_string()));
        }
        self.isocitrate -= 1.0;
        self.alpha_ketoglutarate += 1.0;
        self.nadh_produced += 1;
        self.co2_released += 1;
        Ok(())
    }

    pub fn step_4_alpha_ketoglutarate_dehydrogenase(&mut self) -> BiologyResult<()> {
        if self.alpha_ketoglutarate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient α-ketoglutarate".to_string()));
        }
        self.alpha_ketoglutarate -= 1.0;
        self.succinyl_coa += 1.0;
        self.nadh_produced += 1;
        self.co2_released += 1;
        Ok(())
    }

    pub fn step_5_succinyl_coa_synthetase(&mut self) -> BiologyResult<()> {
        if self.succinyl_coa < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient succinyl-CoA".to_string()));
        }
        self.succinyl_coa -= 1.0;
        self.succinate += 1.0;
        self.gtp_produced += 1;
        Ok(())
    }

    pub fn step_6_succinate_dehydrogenase(&mut self) -> BiologyResult<()> {
        if self.succinate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient succinate".to_string()));
        }
        self.succinate -= 1.0;
        self.fumarate += 1.0;
        self.fadh2_produced += 1;
        Ok(())
    }

    pub fn step_7_fumarase(&mut self) -> BiologyResult<()> {
        if self.fumarate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient fumarate".to_string()));
        }
        self.fumarate -= 1.0;
        self.malate += 1.0;
        Ok(())
    }

    pub fn step_8_malate_dehydrogenase(&mut self) -> BiologyResult<()> {
        if self.malate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient malate".to_string()));
        }
        self.malate -= 1.0;
        self.oxaloacetate += 1.0;
        self.nadh_produced += 1;
        Ok(())
    }

    pub fn complete_cycle(&mut self) -> BiologyResult<()> {
        self.step_1_citrate_synthase()?;
        self.step_2_aconitase()?;
        self.step_3_isocitrate_dehydrogenase()?;
        self.step_4_alpha_ketoglutarate_dehydrogenase()?;
        self.step_5_succinyl_coa_synthetase()?;
        self.step_6_succinate_dehydrogenase()?;
        self.step_7_fumarase()?;
        self.step_8_malate_dehydrogenase()?;
        Ok(())
    }

    pub fn atp_equivalent(&self) -> u32 {
        (self.nadh_produced * 3) + (self.fadh2_produced * 2) + self.gtp_produced
    }
}

impl Default for KrebsCycle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaOxidation {
    pub fatty_acid_carbons: u32,
    pub acetyl_coa_produced: u32,
    pub fadh2_produced: u32,
    pub nadh_produced: u32,
    pub cycles_completed: u32,
}

impl BetaOxidation {
    pub fn new(carbon_count: u32) -> BiologyResult<Self> {
        if carbon_count % 2 != 0 {
            return Err(BiologyError::InvalidValue(
                "Fatty acids must have even number of carbons".to_string()
            ));
        }
        if carbon_count < 4 {
            return Err(BiologyError::InvalidValue(
                "Fatty acid too short".to_string()
            ));
        }

        Ok(BetaOxidation {
            fatty_acid_carbons: carbon_count,
            acetyl_coa_produced: 0,
            fadh2_produced: 0,
            nadh_produced: 0,
            cycles_completed: 0,
        })
    }

    pub fn oxidize_one_cycle(&mut self) -> BiologyResult<()> {
        if self.fatty_acid_carbons < 4 {
            return Err(BiologyError::InvalidValue(
                "Cannot oxidize further".to_string()
            ));
        }

        self.fatty_acid_carbons -= 2;
        self.acetyl_coa_produced += 1;
        self.fadh2_produced += 1;
        self.nadh_produced += 1;
        self.cycles_completed += 1;

        Ok(())
    }

    pub fn complete_oxidation(&mut self) -> BiologyResult<()> {
        while self.fatty_acid_carbons >= 4 {
            self.oxidize_one_cycle()?;
        }

        self.acetyl_coa_produced += 1;

        Ok(())
    }

    pub fn total_atp_yield(&self) -> u32 {
        let activation_cost = 2;
        let fadh2_atp = self.fadh2_produced * 2;
        let nadh_atp = self.nadh_produced * 3;
        let acetyl_coa_atp = self.acetyl_coa_produced * 10;

        fadh2_atp + nadh_atp + acetyl_coa_atp - activation_cost
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronTransportChain {
    pub complex_i_active: bool,
    pub complex_ii_active: bool,
    pub complex_iii_active: bool,
    pub complex_iv_active: bool,
    pub atp_synthase_active: bool,
    pub nadh_input: u32,
    pub fadh2_input: u32,
    pub protons_pumped: u32,
    pub atp_produced: u32,
    pub oxygen_consumed: f64,
}

impl ElectronTransportChain {
    pub fn new() -> Self {
        ElectronTransportChain {
            complex_i_active: true,
            complex_ii_active: true,
            complex_iii_active: true,
            complex_iv_active: true,
            atp_synthase_active: true,
            nadh_input: 0,
            fadh2_input: 0,
            protons_pumped: 0,
            atp_produced: 0,
            oxygen_consumed: 0.0,
        }
    }

    pub fn process_nadh(&mut self, nadh_count: u32) -> BiologyResult<()> {
        if !self.complex_i_active {
            return Err(BiologyError::InvalidValue("Complex I inactive".to_string()));
        }

        self.nadh_input += nadh_count;
        self.protons_pumped += nadh_count * 10;
        self.atp_produced += nadh_count * 3;
        self.oxygen_consumed += nadh_count as f64 * 0.5;

        Ok(())
    }

    pub fn process_fadh2(&mut self, fadh2_count: u32) -> BiologyResult<()> {
        if !self.complex_ii_active {
            return Err(BiologyError::InvalidValue("Complex II inactive".to_string()));
        }

        self.fadh2_input += fadh2_count;
        self.protons_pumped += fadh2_count * 6;
        self.atp_produced += fadh2_count * 2;
        self.oxygen_consumed += fadh2_count as f64 * 0.5;

        Ok(())
    }

    pub fn p_o_ratio_nadh(&self) -> f64 {
        if self.nadh_input == 0 {
            return 0.0;
        }
        self.atp_produced as f64 / self.nadh_input as f64
    }

    pub fn is_coupled(&self) -> bool {
        self.protons_pumped > 0 && self.atp_synthase_active
    }
}

impl Default for ElectronTransportChain {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PentosePhosphatePathway {
    pub oxidative_phase_active: bool,
    pub non_oxidative_phase_active: bool,
    pub glucose_6_phosphate: f64,
    pub ribose_5_phosphate: f64,
    pub nadph_produced: u32,
    pub co2_produced: u32,
}

impl PentosePhosphatePathway {
    pub fn new() -> Self {
        PentosePhosphatePathway {
            oxidative_phase_active: true,
            non_oxidative_phase_active: true,
            glucose_6_phosphate: 1.0,
            ribose_5_phosphate: 0.0,
            nadph_produced: 0,
            co2_produced: 0,
        }
    }

    pub fn oxidative_phase(&mut self) -> BiologyResult<()> {
        if !self.oxidative_phase_active {
            return Err(BiologyError::InvalidValue("Oxidative phase inactive".to_string()));
        }

        if self.glucose_6_phosphate < 0.1 {
            return Err(BiologyError::InvalidValue("Insufficient G6P".to_string()));
        }

        self.glucose_6_phosphate -= 1.0;
        self.ribose_5_phosphate += 1.0;
        self.nadph_produced += 2;
        self.co2_produced += 1;

        Ok(())
    }

    pub fn nadph_nadp_ratio(&self) -> f64 {
        if self.nadph_produced == 0 {
            return 0.0;
        }
        self.nadph_produced as f64 / 100.0
    }
}

impl Default for PentosePhosphatePathway {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detailed_glycolysis() {
        let mut glycolysis = DetailedGlycolysis::new();
        glycolysis.complete_glycolysis().unwrap();
        assert_eq!(glycolysis.net_atp(), 2);
        assert_eq!(glycolysis.nadh_produced, 2);
        assert!(glycolysis.pyruvate > 0.0);
    }

    #[test]
    fn test_krebs_cycle() {
        let mut cycle = KrebsCycle::new();
        cycle.complete_cycle().unwrap();
        assert_eq!(cycle.nadh_produced, 3);
        assert_eq!(cycle.fadh2_produced, 1);
        assert_eq!(cycle.gtp_produced, 1);
        assert_eq!(cycle.co2_released, 2);
    }

    #[test]
    fn test_krebs_atp_yield() {
        let mut cycle = KrebsCycle::new();
        cycle.complete_cycle().unwrap();
        let atp_eq = cycle.atp_equivalent();
        assert_eq!(atp_eq, 12);
    }

    #[test]
    fn test_beta_oxidation_palmitate() {
        let mut beta_ox = BetaOxidation::new(16).unwrap();
        beta_ox.complete_oxidation().unwrap();
        assert_eq!(beta_ox.cycles_completed, 7);
        assert_eq!(beta_ox.acetyl_coa_produced, 8);
        assert_eq!(beta_ox.fadh2_produced, 7);
        assert_eq!(beta_ox.nadh_produced, 7);
        let expected_atp = (7 * 2) + (7 * 3) + (8 * 10) - 2;
        assert_eq!(beta_ox.total_atp_yield(), expected_atp);
    }

    #[test]
    fn test_beta_oxidation_invalid() {
        let result = BetaOxidation::new(15);
        assert!(result.is_err());

        let result = BetaOxidation::new(2);
        assert!(result.is_err());
    }

    #[test]
    fn test_electron_transport_chain() {
        let mut etc = ElectronTransportChain::new();
        etc.process_nadh(10).unwrap();
        assert_eq!(etc.atp_produced, 30);
        assert!(etc.is_coupled());
    }

    #[test]
    fn test_etc_fadh2() {
        let mut etc = ElectronTransportChain::new();
        etc.process_fadh2(4).unwrap();
        assert_eq!(etc.atp_produced, 8);
    }

    #[test]
    fn test_pentose_phosphate_pathway() {
        let mut ppp = PentosePhosphatePathway::new();
        ppp.oxidative_phase().unwrap();
        assert_eq!(ppp.nadph_produced, 2);
        assert_eq!(ppp.co2_produced, 1);
        assert!(ppp.ribose_5_phosphate > 0.0);
    }

    #[test]
    fn test_glycolysis_steps() {
        let mut glycolysis = DetailedGlycolysis::new();
        glycolysis.step_1_hexokinase().unwrap();
        assert_eq!(glycolysis.glucose_6_phosphate, 1.0);
        assert_eq!(glycolysis.atp_consumed, 1);

        glycolysis.step_2_phosphoglucose_isomerase().unwrap();
        assert_eq!(glycolysis.fructose_6_phosphate, 1.0);
    }
}

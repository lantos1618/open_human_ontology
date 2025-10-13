use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MichaelisMentenEnzyme {
    pub name: String,
    pub vmax: f64,
    pub km: f64,
    pub ki: Option<f64>,
    pub kcat: f64,
    pub enzyme_concentration: f64,
}

impl MichaelisMentenEnzyme {
    pub fn new(name: String, vmax: f64, km: f64, kcat: f64) -> Self {
        Self {
            name,
            vmax,
            km,
            ki: None,
            kcat,
            enzyme_concentration: 1.0,
        }
    }

    pub fn reaction_velocity(&self, substrate_concentration: f64) -> f64 {
        (self.vmax * substrate_concentration) / (self.km + substrate_concentration)
    }

    pub fn reaction_velocity_with_inhibitor(
        &self,
        substrate_concentration: f64,
        inhibitor_concentration: f64,
    ) -> f64 {
        if let Some(ki) = self.ki {
            let apparent_km = self.km * (1.0 + inhibitor_concentration / ki);
            (self.vmax * substrate_concentration) / (apparent_km + substrate_concentration)
        } else {
            self.reaction_velocity(substrate_concentration)
        }
    }

    pub fn catalytic_efficiency(&self) -> f64 {
        self.kcat / self.km
    }

    pub fn turnover_number(&self) -> f64 {
        self.kcat
    }

    pub fn substrate_affinity_score(&self) -> f64 {
        1.0 / self.km
    }

    pub fn percent_saturation(&self, substrate_concentration: f64) -> f64 {
        (substrate_concentration / (self.km + substrate_concentration)) * 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlycolysisWithKinetics {
    pub glucose: f64,
    pub g6p: f64,
    pub f6p: f64,
    pub f16bp: f64,
    pub g3p: f64,
    pub dhap: f64,
    pub bpg: f64,
    pub pg3: f64,
    pub pg2: f64,
    pub pep: f64,
    pub pyruvate: f64,
    pub atp: f64,
    pub adp: f64,
    pub nad: f64,
    pub nadh: f64,
    pub hexokinase: MichaelisMentenEnzyme,
    pub phosphofructokinase: MichaelisMentenEnzyme,
    pub pyruvate_kinase: MichaelisMentenEnzyme,
}

impl GlycolysisWithKinetics {
    pub fn new() -> Self {
        Self {
            glucose: 5.0,
            g6p: 0.08,
            f6p: 0.04,
            f16bp: 0.01,
            g3p: 0.03,
            dhap: 0.02,
            bpg: 0.005,
            pg3: 0.12,
            pg2: 0.02,
            pep: 0.02,
            pyruvate: 0.05,
            atp: 2.5,
            adp: 1.0,
            nad: 0.5,
            nadh: 0.05,
            hexokinase: MichaelisMentenEnzyme::new(
                "Hexokinase".to_string(),
                100.0,
                0.1,
                1000.0,
            ),
            phosphofructokinase: MichaelisMentenEnzyme::new(
                "Phosphofructokinase-1 (PFK-1)".to_string(),
                150.0,
                0.05,
                1200.0,
            ),
            pyruvate_kinase: MichaelisMentenEnzyme::new(
                "Pyruvate Kinase".to_string(),
                200.0,
                0.3,
                2000.0,
            ),
        }
    }

    pub fn step_with_kinetics(&mut self, dt: f64) {
        let v_hexokinase = self.hexokinase.reaction_velocity(self.glucose)
            * (self.atp / (self.atp + 0.1))
            * dt;

        let v_pfk = self
            .phosphofructokinase
            .reaction_velocity_with_inhibitor(self.f6p, self.atp)
            * (self.atp / (self.atp + 0.05))
            * dt;

        let v_pk = self.pyruvate_kinase.reaction_velocity(self.pep)
            * (self.adp / (self.adp + 0.1))
            * dt;

        let glycolytic_flux = v_hexokinase.min(v_pfk).min(v_pk);

        self.glucose -= glycolytic_flux * 0.1;
        self.g6p += v_hexokinase - glycolytic_flux;
        self.f6p += glycolytic_flux * 0.9 - v_pfk;
        self.f16bp += v_pfk - glycolytic_flux * 0.8;
        self.g3p += glycolytic_flux * 1.6 - glycolytic_flux * 1.5;
        self.pep += glycolytic_flux * 1.4 - v_pk;
        self.pyruvate += v_pk;

        self.atp -= (v_hexokinase + v_pfk) * 0.1;
        self.atp += v_pk * 0.2 + glycolytic_flux * 0.3;
        self.adp = 4.0 - self.atp;

        self.nad -= glycolytic_flux * 0.2;
        self.nadh += glycolytic_flux * 0.2;

        self.glucose = self.glucose.max(0.0);
        self.atp = self.atp.clamp(0.5, 5.0);
        self.nad = self.nad.clamp(0.01, 1.0);
    }

    pub fn net_atp_flux(&self) -> f64 {
        let atp_consumption = self.hexokinase.reaction_velocity(self.glucose)
            + self.phosphofructokinase.reaction_velocity(self.f6p);
        let atp_production =
            self.pyruvate_kinase.reaction_velocity(self.pep) * 2.0 + self.g3p * 2.0;

        atp_production - atp_consumption
    }

    pub fn glycolytic_flux(&self) -> f64 {
        let v1 = self.hexokinase.reaction_velocity(self.glucose);
        let v3 = self.phosphofructokinase.reaction_velocity(self.f6p);
        let v10 = self.pyruvate_kinase.reaction_velocity(self.pep);

        v1.min(v3).min(v10)
    }

    pub fn atp_adp_ratio(&self) -> f64 {
        self.atp / self.adp
    }

    pub fn nad_nadh_ratio(&self) -> f64 {
        self.nad / self.nadh
    }

    pub fn rate_limiting_enzyme(&self) -> String {
        let v_hk = self.hexokinase.reaction_velocity(self.glucose);
        let v_pfk = self.phosphofructokinase.reaction_velocity(self.f6p);
        let v_pk = self.pyruvate_kinase.reaction_velocity(self.pep);

        if v_hk < v_pfk && v_hk < v_pk {
            "Hexokinase".to_string()
        } else if v_pfk < v_hk && v_pfk < v_pk {
            "Phosphofructokinase-1".to_string()
        } else {
            "Pyruvate Kinase".to_string()
        }
    }
}

impl Default for GlycolysisWithKinetics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_michaelis_menten() {
        let enzyme = MichaelisMentenEnzyme::new("Test".to_string(), 100.0, 1.0, 1000.0);

        let v_low = enzyme.reaction_velocity(0.1);
        let v_km = enzyme.reaction_velocity(1.0);
        let v_high = enzyme.reaction_velocity(10.0);

        assert!(v_low < v_km);
        assert!(v_km < v_high);
        assert!((v_km - 50.0).abs() < 1.0);
    }

    #[test]
    fn test_competitive_inhibition() {
        let mut enzyme = MichaelisMentenEnzyme::new("Test".to_string(), 100.0, 1.0, 1000.0);
        enzyme.ki = Some(0.5);

        let v_no_inhibitor = enzyme.reaction_velocity(1.0);
        let v_with_inhibitor = enzyme.reaction_velocity_with_inhibitor(1.0, 1.0);

        assert!(v_with_inhibitor < v_no_inhibitor);
    }

    #[test]
    fn test_glycolysis_kinetics() {
        let mut pathway = GlycolysisWithKinetics::new();
        let initial_glucose = pathway.glucose;

        pathway.step_with_kinetics(0.1);

        assert!(pathway.glucose < initial_glucose);
        assert!(pathway.pyruvate > 0.0);
    }

    #[test]
    fn test_rate_limiting_step() {
        let pathway = GlycolysisWithKinetics::new();
        let limiting = pathway.rate_limiting_enzyme();

        assert!(!limiting.is_empty());
    }
}

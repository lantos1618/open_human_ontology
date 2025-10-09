use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VestibularSystem {
    pub semicircular_canals: SemicircularCanals,
    pub otolith_organs: OtolithOrgans,
    pub vestibular_nerve: VestibularNerve,
    pub balance_function: BalanceFunction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemicircularCanals {
    pub anterior_canal: Canal,
    pub posterior_canal: Canal,
    pub lateral_canal: Canal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Canal {
    pub hair_cells: u32,
    pub cupula_displacement: f64,
    pub endolymph_flow: f64,
    pub sensitivity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtolithOrgans {
    pub utricle: OtolithOrgan,
    pub saccule: OtolithOrgan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtolithOrgan {
    pub hair_cells: u32,
    pub otoconia_mass: f64,
    pub macula_area_mm2: f64,
    pub linear_acceleration_sensitivity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VestibularNerve {
    pub superior_division_fibers: u32,
    pub inferior_division_fibers: u32,
    pub conduction_velocity_ms: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceFunction {
    pub static_balance_score: f64,
    pub dynamic_balance_score: f64,
    pub vestibulo_ocular_reflex_gain: f64,
    pub vestibulospinal_reflex_latency_ms: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VestibularDisorder {
    BenignParoxysmalPositionalVertigo,
    MenieresDisease,
    VestibularNeuritis,
    Labyrinthitis,
    VestibularMigraine,
}

impl VestibularSystem {
    pub fn new_normal() -> Self {
        Self {
            semicircular_canals: SemicircularCanals::new_normal(),
            otolith_organs: OtolithOrgans::new_normal(),
            vestibular_nerve: VestibularNerve::new_normal(),
            balance_function: BalanceFunction::new_normal(),
        }
    }

    pub fn calculate_vertigo_risk(&self) -> f64 {
        let canal_dysfunction = self.semicircular_canals.average_dysfunction();
        let otolith_dysfunction = self.otolith_organs.average_dysfunction();
        let nerve_impairment = self.vestibular_nerve.impairment_score();

        (canal_dysfunction + otolith_dysfunction + nerve_impairment) / 3.0
    }

    pub fn assess_fall_risk(&self) -> FallRisk {
        let balance_score = (self.balance_function.static_balance_score +
                            self.balance_function.dynamic_balance_score) / 2.0;

        if balance_score < 0.5 {
            FallRisk::High
        } else if balance_score < 0.7 {
            FallRisk::Moderate
        } else {
            FallRisk::Low
        }
    }
}

impl SemicircularCanals {
    pub fn new_normal() -> Self {
        Self {
            anterior_canal: Canal::new_normal(),
            posterior_canal: Canal::new_normal(),
            lateral_canal: Canal::new_normal(),
        }
    }

    pub fn average_dysfunction(&self) -> f64 {
        let anterior_dysfn = 1.0 - self.anterior_canal.sensitivity;
        let posterior_dysfn = 1.0 - self.posterior_canal.sensitivity;
        let lateral_dysfn = 1.0 - self.lateral_canal.sensitivity;

        (anterior_dysfn + posterior_dysfn + lateral_dysfn) / 3.0
    }
}

impl Canal {
    pub fn new_normal() -> Self {
        Self {
            hair_cells: 7500,
            cupula_displacement: 0.0,
            endolymph_flow: 0.0,
            sensitivity: 1.0,
        }
    }
}

impl OtolithOrgans {
    pub fn new_normal() -> Self {
        Self {
            utricle: OtolithOrgan::new_utricle(),
            saccule: OtolithOrgan::new_saccule(),
        }
    }

    pub fn average_dysfunction(&self) -> f64 {
        let utricle_dysfn = 1.0 - self.utricle.linear_acceleration_sensitivity;
        let saccule_dysfn = 1.0 - self.saccule.linear_acceleration_sensitivity;

        (utricle_dysfn + saccule_dysfn) / 2.0
    }
}

impl OtolithOrgan {
    pub fn new_utricle() -> Self {
        Self {
            hair_cells: 33000,
            otoconia_mass: 1.0,
            macula_area_mm2: 3.0,
            linear_acceleration_sensitivity: 1.0,
        }
    }

    pub fn new_saccule() -> Self {
        Self {
            hair_cells: 19000,
            otoconia_mass: 1.0,
            macula_area_mm2: 2.0,
            linear_acceleration_sensitivity: 1.0,
        }
    }
}

impl VestibularNerve {
    pub fn new_normal() -> Self {
        Self {
            superior_division_fibers: 10000,
            inferior_division_fibers: 10000,
            conduction_velocity_ms: 15.0,
        }
    }

    pub fn impairment_score(&self) -> f64 {
        let total_fibers = self.superior_division_fibers + self.inferior_division_fibers;
        let normal_total = 20000.0;
        let fiber_loss = 1.0 - (total_fibers as f64 / normal_total);

        let velocity_impairment = if self.conduction_velocity_ms < 15.0 {
            1.0 - (self.conduction_velocity_ms / 15.0)
        } else {
            0.0
        };

        (fiber_loss + velocity_impairment) / 2.0
    }
}

impl BalanceFunction {
    pub fn new_normal() -> Self {
        Self {
            static_balance_score: 1.0,
            dynamic_balance_score: 1.0,
            vestibulo_ocular_reflex_gain: 1.0,
            vestibulospinal_reflex_latency_ms: 12.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FallRisk {
    Low,
    Moderate,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_vestibular_system() {
        let system = VestibularSystem::new_normal();
        assert_eq!(system.semicircular_canals.anterior_canal.hair_cells, 7500);
        assert_eq!(system.otolith_organs.utricle.hair_cells, 33000);
    }

    #[test]
    fn test_vertigo_risk_calculation() {
        let system = VestibularSystem::new_normal();
        let risk = system.calculate_vertigo_risk();
        assert!(risk < 0.1);
    }

    #[test]
    fn test_fall_risk_assessment() {
        let system = VestibularSystem::new_normal();
        let risk = system.assess_fall_risk();
        assert_eq!(risk, FallRisk::Low);
    }
}

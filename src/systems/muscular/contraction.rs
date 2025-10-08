use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractionMechanism {
    pub calcium_concentration_um: f64,
    pub atp_concentration_mm: f64,
    pub crossbridge_cycle_rate: f64,
    pub troponin_binding: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractionType {
    Isometric,
    Isotonic,
    Eccentric,
    Concentric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crossbridge {
    pub state: CrossbridgeState,
    pub force_pn: f64,
    pub attachment_duration_ms: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrossbridgeState {
    Detached,
    Attached,
    PowerStroke,
    Rigor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExcitationContraction {
    pub action_potential_mv: f64,
    pub t_tubule_depolarization: bool,
    pub sr_calcium_release: f64,
    pub calcium_binding_troponin: f64,
}

impl ContractionMechanism {
    pub fn new_resting() -> Self {
        Self {
            calcium_concentration_um: 0.1,
            atp_concentration_mm: 5.0,
            crossbridge_cycle_rate: 0.0,
            troponin_binding: 0.0,
        }
    }

    pub fn new_activated() -> Self {
        Self {
            calcium_concentration_um: 10.0,
            atp_concentration_mm: 5.0,
            crossbridge_cycle_rate: 50.0,
            troponin_binding: 0.9,
        }
    }

    pub fn activation_level(&self) -> f64 {
        let ca_threshold = 0.5;
        let ca_max = 10.0;

        if self.calcium_concentration_um < ca_threshold {
            0.0
        } else {
            let ca_factor = (self.calcium_concentration_um - ca_threshold) / (ca_max - ca_threshold);
            ca_factor.min(1.0)
        }
    }

    pub fn force_generation(&self) -> f64 {
        let activation = self.activation_level();
        let atp_factor = (self.atp_concentration_mm / 5.0).min(1.0);
        let crossbridge_factor = (self.crossbridge_cycle_rate / 100.0).min(1.0);

        activation * atp_factor * crossbridge_factor
    }

    pub fn energy_consumption_rate(&self) -> f64 {
        self.crossbridge_cycle_rate * 0.05
    }

    pub fn is_fatigued(&self) -> bool {
        self.atp_concentration_mm < 1.0
    }
}

impl Crossbridge {
    pub fn new_detached() -> Self {
        Self {
            state: CrossbridgeState::Detached,
            force_pn: 0.0,
            attachment_duration_ms: 0.0,
        }
    }

    pub fn attach(&mut self, calcium_present: bool, atp_available: bool) -> bool {
        if calcium_present && atp_available && self.state == CrossbridgeState::Detached {
            self.state = CrossbridgeState::Attached;
            self.force_pn = 0.0;
            true
        } else {
            false
        }
    }

    pub fn power_stroke(&mut self) -> f64 {
        if self.state == CrossbridgeState::Attached {
            self.state = CrossbridgeState::PowerStroke;
            self.force_pn = 6.0;
            self.force_pn
        } else {
            0.0
        }
    }

    pub fn detach(&mut self, atp_available: bool) -> bool {
        if atp_available && self.state == CrossbridgeState::PowerStroke {
            self.state = CrossbridgeState::Detached;
            self.force_pn = 0.0;
            self.attachment_duration_ms = 0.0;
            true
        } else if !atp_available {
            self.state = CrossbridgeState::Rigor;
            false
        } else {
            false
        }
    }

    pub fn cycle_rate(&self, atp_mm: f64, calcium_um: f64) -> f64 {
        let atp_factor = (atp_mm / 5.0).min(1.0);
        let calcium_factor = (calcium_um / 10.0).min(1.0);

        50.0 * atp_factor * calcium_factor
    }
}

impl ExcitationContraction {
    pub fn new_resting() -> Self {
        Self {
            action_potential_mv: -90.0,
            t_tubule_depolarization: false,
            sr_calcium_release: 0.0,
            calcium_binding_troponin: 0.0,
        }
    }

    pub fn initiate_action_potential(&mut self) {
        self.action_potential_mv = 30.0;
        self.t_tubule_depolarization = true;
    }

    pub fn propagate_to_sr(&mut self) {
        if self.t_tubule_depolarization {
            self.sr_calcium_release = 10.0;
        }
    }

    pub fn bind_troponin(&mut self) {
        if self.sr_calcium_release > 1.0 {
            self.calcium_binding_troponin = (self.sr_calcium_release / 10.0).min(1.0);
        }
    }

    pub fn relax(&mut self) {
        self.action_potential_mv = -90.0;
        self.t_tubule_depolarization = false;
        self.sr_calcium_release = 0.0;
        self.calcium_binding_troponin = 0.0;
    }

    pub fn coupling_strength(&self) -> f64 {
        if self.action_potential_mv > 0.0 && self.t_tubule_depolarization {
            (self.action_potential_mv / 120.0).min(1.0)
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contraction_mechanism() {
        let resting = ContractionMechanism::new_resting();
        let activated = ContractionMechanism::new_activated();

        assert!(resting.activation_level() < 0.1);
        assert!(activated.activation_level() > 0.8);
    }

    #[test]
    fn test_force_generation() {
        let mechanism = ContractionMechanism::new_activated();
        let force = mechanism.force_generation();

        assert!(force > 0.0);
        assert!(force <= 1.0);
    }

    #[test]
    fn test_crossbridge_cycle() {
        let mut cb = Crossbridge::new_detached();

        assert!(cb.attach(true, true));
        assert_eq!(cb.state, CrossbridgeState::Attached);

        let force = cb.power_stroke();
        assert!(force > 0.0);
        assert_eq!(cb.state, CrossbridgeState::PowerStroke);

        assert!(cb.detach(true));
        assert_eq!(cb.state, CrossbridgeState::Detached);
    }

    #[test]
    fn test_rigor_mortis() {
        let mut cb = Crossbridge::new_detached();
        cb.attach(true, true);
        cb.power_stroke();

        assert!(!cb.detach(false));
        assert_eq!(cb.state, CrossbridgeState::Rigor);
    }

    #[test]
    fn test_excitation_contraction() {
        let mut ec = ExcitationContraction::new_resting();

        ec.initiate_action_potential();
        assert_eq!(ec.action_potential_mv, 30.0);

        ec.propagate_to_sr();
        assert!(ec.sr_calcium_release > 0.0);

        ec.bind_troponin();
        assert!(ec.calcium_binding_troponin > 0.0);

        ec.relax();
        assert_eq!(ec.action_potential_mv, -90.0);
    }

    #[test]
    fn test_energy_consumption() {
        let mechanism = ContractionMechanism::new_activated();
        let energy = mechanism.energy_consumption_rate();
        assert!(energy > 0.0);
    }

    #[test]
    fn test_fatigue() {
        let mut mechanism = ContractionMechanism::new_activated();
        assert!(!mechanism.is_fatigued());

        mechanism.atp_concentration_mm = 0.5;
        assert!(mechanism.is_fatigued());
    }
}

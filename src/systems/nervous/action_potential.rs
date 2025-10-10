use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPotentialDynamics {
    pub membrane_potential_mv: f64,
    pub resting_potential_mv: f64,
    pub threshold_mv: f64,
    pub sodium_channels: IonChannelPopulation,
    pub potassium_channels: IonChannelPopulation,
    pub calcium_channels: IonChannelPopulation,
    pub leak_conductance_ms_cm2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonChannelPopulation {
    pub total_channels: usize,
    pub open_fraction: f64,
    pub max_conductance_ms_cm2: f64,
    pub reversal_potential_mv: f64,
    pub activation_gate_m: f64,
    pub inactivation_gate_h: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HodgkinHuxleyModel {
    pub v_membrane_mv: f64,
    pub m_activation: f64,
    pub h_inactivation: f64,
    pub n_potassium: f64,
    pub membrane_capacitance_uf_cm2: f64,
    pub time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapticTransmission {
    pub presynaptic_vm_mv: f64,
    pub postsynaptic_vm_mv: f64,
    pub neurotransmitter_concentration_um: f64,
    pub receptor_bound_fraction: f64,
    pub synaptic_conductance_ns: f64,
    pub synaptic_current_pa: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuronType {
    Pyramidal,
    Interneuron,
    Purkinje,
    Motor,
    Sensory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeurotransmitterType {
    Glutamate,
    GABA,
    Acetylcholine,
    Dopamine,
    Serotonin,
    Norepinephrine,
}

impl ActionPotentialDynamics {
    pub fn new_resting() -> Self {
        Self {
            membrane_potential_mv: -70.0,
            resting_potential_mv: -70.0,
            threshold_mv: -55.0,
            sodium_channels: IonChannelPopulation::new_sodium(),
            potassium_channels: IonChannelPopulation::new_potassium(),
            calcium_channels: IonChannelPopulation::new_calcium(),
            leak_conductance_ms_cm2: 0.3,
        }
    }

    pub fn stimulate(&mut self, current_ua_cm2: f64, duration_ms: f64, dt_ms: f64) {
        let steps = (duration_ms / dt_ms) as usize;

        for _ in 0..steps {
            self.update(dt_ms, current_ua_cm2);
        }
    }

    pub fn update(&mut self, dt_ms: f64, stimulus_current_ua_cm2: f64) {
        let capacitance = 1.0;

        let i_na = self.sodium_current();
        let i_k = self.potassium_current();
        let i_ca = self.calcium_current();
        let i_leak = self.leak_current();

        let total_current = stimulus_current_ua_cm2 - (i_na + i_k + i_ca + i_leak);

        let dv_dt = total_current / capacitance;
        self.membrane_potential_mv += dv_dt * dt_ms;

        self.update_channel_states(dt_ms);
    }

    fn sodium_current(&self) -> f64 {
        let g_na = self.sodium_channels.max_conductance_ms_cm2
            * self.sodium_channels.activation_gate_m.powi(3)
            * self.sodium_channels.inactivation_gate_h;

        g_na * (self.membrane_potential_mv - self.sodium_channels.reversal_potential_mv)
    }

    fn potassium_current(&self) -> f64 {
        let g_k = self.potassium_channels.max_conductance_ms_cm2
            * self.potassium_channels.activation_gate_m.powi(4);

        g_k * (self.membrane_potential_mv - self.potassium_channels.reversal_potential_mv)
    }

    fn calcium_current(&self) -> f64 {
        let g_ca = self.calcium_channels.max_conductance_ms_cm2
            * self.calcium_channels.activation_gate_m.powi(2);

        g_ca * (self.membrane_potential_mv - self.calcium_channels.reversal_potential_mv)
    }

    fn leak_current(&self) -> f64 {
        self.leak_conductance_ms_cm2 * (self.membrane_potential_mv - self.resting_potential_mv)
    }

    fn update_channel_states(&mut self, dt_ms: f64) {
        let v = self.membrane_potential_mv;

        let alpha_m = 0.1 * (v + 40.0) / (1.0 - (-0.1 * (v + 40.0)).exp());
        let beta_m = 4.0 * ((-0.0556 * (v + 65.0)).exp());
        let tau_m = 1.0 / (alpha_m + beta_m);
        let m_inf = alpha_m / (alpha_m + beta_m);

        self.sodium_channels.activation_gate_m +=
            (m_inf - self.sodium_channels.activation_gate_m) * dt_ms / tau_m;

        let alpha_h = 0.07 * ((-0.05 * (v + 65.0)).exp());
        let beta_h = 1.0 / (1.0 + ((-0.1 * (v + 35.0)).exp()));
        let tau_h = 1.0 / (alpha_h + beta_h);
        let h_inf = alpha_h / (alpha_h + beta_h);

        self.sodium_channels.inactivation_gate_h +=
            (h_inf - self.sodium_channels.inactivation_gate_h) * dt_ms / tau_h;

        let alpha_n = 0.01 * (v + 55.0) / (1.0 - (-0.1 * (v + 55.0)).exp());
        let beta_n = 0.125 * ((-0.0125 * (v + 65.0)).exp());
        let tau_n = 1.0 / (alpha_n + beta_n);
        let n_inf = alpha_n / (alpha_n + beta_n);

        self.potassium_channels.activation_gate_m +=
            (n_inf - self.potassium_channels.activation_gate_m) * dt_ms / tau_n;

        self.sodium_channels.open_fraction = self.sodium_channels.activation_gate_m.powi(3)
            * self.sodium_channels.inactivation_gate_h;
        self.potassium_channels.open_fraction = self.potassium_channels.activation_gate_m.powi(4);
    }

    pub fn is_at_rest(&self) -> bool {
        (self.membrane_potential_mv - self.resting_potential_mv).abs() < 1.0
    }

    pub fn is_firing(&self) -> bool {
        self.membrane_potential_mv > self.threshold_mv
    }

    pub fn is_refractory(&self) -> bool {
        self.sodium_channels.inactivation_gate_h < 0.3
    }
}

impl IonChannelPopulation {
    pub fn new_sodium() -> Self {
        Self {
            total_channels: 60_000,
            open_fraction: 0.0,
            max_conductance_ms_cm2: 120.0,
            reversal_potential_mv: 50.0,
            activation_gate_m: 0.05,
            inactivation_gate_h: 0.6,
        }
    }

    pub fn new_potassium() -> Self {
        Self {
            total_channels: 18_000,
            open_fraction: 0.0,
            max_conductance_ms_cm2: 36.0,
            reversal_potential_mv: -77.0,
            activation_gate_m: 0.32,
            inactivation_gate_h: 1.0,
        }
    }

    pub fn new_calcium() -> Self {
        Self {
            total_channels: 5_000,
            open_fraction: 0.0,
            max_conductance_ms_cm2: 10.0,
            reversal_potential_mv: 120.0,
            activation_gate_m: 0.0,
            inactivation_gate_h: 0.8,
        }
    }

    pub fn open_channels(&self) -> usize {
        (self.total_channels as f64 * self.open_fraction) as usize
    }

    pub fn current_conductance(&self) -> f64 {
        self.max_conductance_ms_cm2 * self.open_fraction
    }
}

impl HodgkinHuxleyModel {
    pub fn new() -> Self {
        Self {
            v_membrane_mv: -65.0,
            m_activation: 0.05,
            h_inactivation: 0.6,
            n_potassium: 0.32,
            membrane_capacitance_uf_cm2: 1.0,
            time_ms: 0.0,
        }
    }

    pub fn step(&mut self, dt_ms: f64, i_stimulus_ua_cm2: f64) {
        let v = self.v_membrane_mv;

        let g_na = 120.0;
        let g_k = 36.0;
        let g_l = 0.3;

        let e_na = 50.0;
        let e_k = -77.0;
        let e_l = -54.387;

        let i_na = g_na * self.m_activation.powi(3) * self.h_inactivation * (v - e_na);
        let i_k = g_k * self.n_potassium.powi(4) * (v - e_k);
        let i_l = g_l * (v - e_l);

        let dv_dt = (i_stimulus_ua_cm2 - i_na - i_k - i_l) / self.membrane_capacitance_uf_cm2;
        self.v_membrane_mv += dv_dt * dt_ms;

        let alpha_m = 0.1 * (v + 40.0) / (1.0 - (-(v + 40.0) / 10.0).exp());
        let beta_m = 4.0 * (-(v + 65.0) / 18.0).exp();
        let dm_dt = alpha_m * (1.0 - self.m_activation) - beta_m * self.m_activation;
        self.m_activation += dm_dt * dt_ms;

        let alpha_h = 0.07 * (-(v + 65.0) / 20.0).exp();
        let beta_h = 1.0 / (1.0 + (-(v + 35.0) / 10.0).exp());
        let dh_dt = alpha_h * (1.0 - self.h_inactivation) - beta_h * self.h_inactivation;
        self.h_inactivation += dh_dt * dt_ms;

        let alpha_n = 0.01 * (v + 55.0) / (1.0 - (-(v + 55.0) / 10.0).exp());
        let beta_n = 0.125 * (-(v + 65.0) / 80.0).exp();
        let dn_dt = alpha_n * (1.0 - self.n_potassium) - beta_n * self.n_potassium;
        self.n_potassium += dn_dt * dt_ms;

        self.time_ms += dt_ms;
    }

    pub fn simulate_spike(&mut self, duration_ms: f64, stimulus_ua_cm2: f64) -> Vec<(f64, f64)> {
        let dt = 0.01;
        let steps = (duration_ms / dt) as usize;
        let mut trace = Vec::with_capacity(steps);

        for _ in 0..steps {
            trace.push((self.time_ms, self.v_membrane_mv));
            self.step(dt, stimulus_ua_cm2);
        }

        trace
    }
}

impl Default for HodgkinHuxleyModel {
    fn default() -> Self {
        Self::new()
    }
}

impl SynapticTransmission {
    pub fn new() -> Self {
        Self {
            presynaptic_vm_mv: -70.0,
            postsynaptic_vm_mv: -70.0,
            neurotransmitter_concentration_um: 0.0,
            receptor_bound_fraction: 0.0,
            synaptic_conductance_ns: 0.0,
            synaptic_current_pa: 0.0,
        }
    }

    pub fn update(&mut self, dt_ms: f64, neurotransmitter_type: NeurotransmitterType) {
        if self.presynaptic_vm_mv > -20.0 {
            let release_rate = 1000.0 * ((self.presynaptic_vm_mv + 20.0) / 80.0).min(1.0);
            self.neurotransmitter_concentration_um += release_rate * dt_ms / 1000.0;
        }

        let decay_tau_ms = 2.0;
        self.neurotransmitter_concentration_um *= (-dt_ms / decay_tau_ms).exp();

        let kd_um = match neurotransmitter_type {
            NeurotransmitterType::Glutamate => 1.0,
            NeurotransmitterType::GABA => 5.0,
            NeurotransmitterType::Acetylcholine => 10.0,
            _ => 2.0,
        };

        self.receptor_bound_fraction = self.neurotransmitter_concentration_um
            / (self.neurotransmitter_concentration_um + kd_um);

        let max_conductance_ns = match neurotransmitter_type {
            NeurotransmitterType::Glutamate => 1.0,
            NeurotransmitterType::GABA => 2.0,
            NeurotransmitterType::Acetylcholine => 0.5,
            _ => 1.0,
        };

        self.synaptic_conductance_ns = max_conductance_ns * self.receptor_bound_fraction;

        let reversal_potential_mv = match neurotransmitter_type {
            NeurotransmitterType::Glutamate => 0.0,
            NeurotransmitterType::GABA => -70.0,
            NeurotransmitterType::Acetylcholine => -5.0,
            _ => 0.0,
        };

        self.synaptic_current_pa =
            self.synaptic_conductance_ns * (self.postsynaptic_vm_mv - reversal_potential_mv);

        let tau_membrane_ms = 20.0;
        let capacitance_pf = 100.0;
        let dv_dt = -self.synaptic_current_pa / capacitance_pf;
        self.postsynaptic_vm_mv += dv_dt * dt_ms;

        let leak_rate = (self.postsynaptic_vm_mv + 70.0) / tau_membrane_ms;
        self.postsynaptic_vm_mv -= leak_rate * dt_ms;
    }

    pub fn is_excitatory(&self, neurotransmitter: NeurotransmitterType) -> bool {
        matches!(
            neurotransmitter,
            NeurotransmitterType::Glutamate | NeurotransmitterType::Acetylcholine
        )
    }

    pub fn is_inhibitory(&self, neurotransmitter: NeurotransmitterType) -> bool {
        matches!(neurotransmitter, NeurotransmitterType::GABA)
    }

    pub fn epsp_amplitude_mv(&self) -> f64 {
        (self.postsynaptic_vm_mv + 70.0).max(0.0)
    }

    pub fn ipsp_amplitude_mv(&self) -> f64 {
        (-70.0 - self.postsynaptic_vm_mv).max(0.0)
    }
}

impl Default for SynapticTransmission {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_potential_resting() {
        let ap = ActionPotentialDynamics::new_resting();
        assert!(ap.is_at_rest());
        assert!(!ap.is_firing());
    }

    #[test]
    fn test_action_potential_stimulation() {
        let mut ap = ActionPotentialDynamics::new_resting();
        ap.stimulate(10.0, 5.0, 0.1);
        assert!(ap.membrane_potential_mv != -70.0);
    }

    #[test]
    fn test_ion_channels() {
        let na_channels = IonChannelPopulation::new_sodium();
        let k_channels = IonChannelPopulation::new_potassium();

        assert!(na_channels.reversal_potential_mv > 0.0);
        assert!(k_channels.reversal_potential_mv < 0.0);
        assert!(na_channels.total_channels > k_channels.total_channels);
    }

    #[test]
    fn test_hodgkin_huxley_step() {
        let mut hh = HodgkinHuxleyModel::new();
        let initial_v = hh.v_membrane_mv;

        hh.step(0.1, 10.0);

        assert_ne!(hh.v_membrane_mv, initial_v);
        assert_eq!(hh.time_ms, 0.1);
    }

    #[test]
    fn test_hodgkin_huxley_spike() {
        let mut hh = HodgkinHuxleyModel::new();
        let trace = hh.simulate_spike(50.0, 10.0);

        assert!(!trace.is_empty());

        let max_v = trace
            .iter()
            .map(|(_, v)| v)
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        assert!(max_v > 0.0);
    }

    #[test]
    fn test_synaptic_transmission() {
        let mut synapse = SynapticTransmission::new();
        synapse.presynaptic_vm_mv = 30.0;

        synapse.update(1.0, NeurotransmitterType::Glutamate);

        assert!(synapse.neurotransmitter_concentration_um > 0.0);
        assert!(synapse.receptor_bound_fraction > 0.0);
    }

    #[test]
    fn test_excitatory_inhibitory() {
        let synapse = SynapticTransmission::new();

        assert!(synapse.is_excitatory(NeurotransmitterType::Glutamate));
        assert!(synapse.is_inhibitory(NeurotransmitterType::GABA));
        assert!(!synapse.is_inhibitory(NeurotransmitterType::Glutamate));
    }
}

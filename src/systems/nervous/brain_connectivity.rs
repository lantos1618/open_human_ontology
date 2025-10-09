use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainNetwork {
    pub regions: Vec<BrainRegion>,
    pub connections: Vec<NeuralConnection>,
    pub network_metrics: NetworkMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainRegion {
    pub name: String,
    pub region_type: BrainRegionType,
    pub coordinates_mni: [f64; 3],
    pub volume_mm3: f64,
    pub neuron_count: usize,
    pub activity_level: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrainRegionType {
    PrefrontalCortex,
    MotorCortex,
    SomatosensoryCortex,
    VisualCortex,
    AuditoryCortex,
    Hippocampus,
    Amygdala,
    Thalamus,
    Hypothalamus,
    BasalGanglia,
    Cerebellum,
    BrainstemNucleus,
    CingulateCortex,
    Insula,
    TemporalLobe,
    ParietalLobe,
    OccipitalLobe,
    FrontalLobe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralConnection {
    pub from_region: String,
    pub to_region: String,
    pub connection_type: ConnectionType,
    pub strength: f64,
    pub fiber_count: usize,
    pub latency_ms: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectionType {
    Excitatory,
    Inhibitory,
    Modulatory,
    Bidirectional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub global_efficiency: f64,
    pub local_efficiency: f64,
    pub modularity: f64,
    pub clustering_coefficient: f64,
    pub characteristic_path_length: f64,
    pub small_worldness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteMatterTract {
    pub name: String,
    pub tract_type: TractType,
    pub origin_region: String,
    pub target_region: String,
    pub fractional_anisotropy: f64,
    pub mean_diffusivity: f64,
    pub axon_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TractType {
    AssociationFiber,
    CommissuralFiber,
    ProjectionFiber,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalConnectivity {
    pub roi_pair: (String, String),
    pub correlation: f64,
    pub coherence: f64,
    pub phase_lag: f64,
    pub frequency_band: FrequencyBand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FrequencyBand {
    Delta,
    Theta,
    Alpha,
    Beta,
    Gamma,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveDomain {
    pub domain: DomainType,
    pub associated_regions: Vec<String>,
    pub network_strength: f64,
    pub performance_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DomainType {
    ExecutiveFunction,
    Memory,
    Attention,
    Language,
    VisuospatialProcessing,
    SocialCognition,
    EmotionalProcessing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralOscillation {
    pub frequency_hz: f64,
    pub amplitude_uv: f64,
    pub phase_radians: f64,
    pub location: String,
    pub oscillation_type: OscillationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OscillationType {
    SpontaneousActivity,
    EvokedResponse,
    SteadyState,
    PhaseAmplitudeCoupling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapticPlasticity {
    pub plasticity_type: PlasticityType,
    pub induction_threshold_hz: f64,
    pub time_constant_minutes: f64,
    pub magnitude_fold_change: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlasticityType {
    LTP,
    LTD,
    STDP,
    Metaplasticity,
    HomeostasicPlasticity,
}

impl BrainNetwork {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            connections: Vec::new(),
            network_metrics: NetworkMetrics::default(),
        }
    }

    pub fn total_connections(&self) -> usize {
        self.connections.len()
    }

    pub fn average_connection_strength(&self) -> f64 {
        if self.connections.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.connections.iter().map(|c| c.strength).sum();
        sum / self.connections.len() as f64
    }

    pub fn region_connectivity(&self, region_name: &str) -> usize {
        self.connections.iter()
            .filter(|c| c.from_region == region_name || c.to_region == region_name)
            .count()
    }

    pub fn hub_regions(&self, threshold: usize) -> Vec<&BrainRegion> {
        self.regions.iter()
            .filter(|r| self.region_connectivity(&r.name) >= threshold)
            .collect()
    }
}

impl Default for BrainNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkMetrics {
    pub fn default() -> Self {
        Self {
            global_efficiency: 0.5,
            local_efficiency: 0.6,
            modularity: 0.4,
            clustering_coefficient: 0.3,
            characteristic_path_length: 2.5,
            small_worldness: 1.5,
        }
    }

    pub fn is_small_world(&self) -> bool {
        self.small_worldness > 1.0
    }

    pub fn is_highly_modular(&self) -> bool {
        self.modularity >= 0.4
    }

    pub fn network_efficiency(&self) -> f64 {
        (self.global_efficiency + self.local_efficiency) / 2.0
    }
}

impl BrainRegion {
    pub fn new(name: String, region_type: BrainRegionType) -> Self {
        Self {
            name,
            region_type,
            coordinates_mni: [0.0, 0.0, 0.0],
            volume_mm3: 1000.0,
            neuron_count: 1_000_000,
            activity_level: 0.5,
        }
    }

    pub fn is_active(&self) -> bool {
        self.activity_level > 0.6
    }

    pub fn distance_to(&self, other: &BrainRegion) -> f64 {
        let dx = self.coordinates_mni[0] - other.coordinates_mni[0];
        let dy = self.coordinates_mni[1] - other.coordinates_mni[1];
        let dz = self.coordinates_mni[2] - other.coordinates_mni[2];

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn neuron_density(&self) -> f64 {
        self.neuron_count as f64 / self.volume_mm3
    }
}

impl NeuralConnection {
    pub fn new(from: String, to: String, connection_type: ConnectionType) -> Self {
        Self {
            from_region: from,
            to_region: to,
            connection_type,
            strength: 0.5,
            fiber_count: 10000,
            latency_ms: 10.0,
        }
    }

    pub fn is_strong(&self) -> bool {
        self.strength > 0.7
    }

    pub fn conduction_velocity_m_per_s(&self) -> f64 {
        50.0
    }
}

impl WhiteMatterTract {
    pub fn new(name: String, tract_type: TractType) -> Self {
        Self {
            name,
            tract_type,
            origin_region: String::new(),
            target_region: String::new(),
            fractional_anisotropy: 0.5,
            mean_diffusivity: 0.0007,
            axon_count: 100000,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.fractional_anisotropy > 0.4 && self.mean_diffusivity < 0.001
    }

    pub fn has_white_matter_damage(&self) -> bool {
        self.fractional_anisotropy < 0.3 || self.mean_diffusivity > 0.0012
    }
}

impl FunctionalConnectivity {
    pub fn new(roi1: String, roi2: String, frequency_band: FrequencyBand) -> Self {
        Self {
            roi_pair: (roi1, roi2),
            correlation: 0.0,
            coherence: 0.0,
            phase_lag: 0.0,
            frequency_band,
        }
    }

    pub fn is_strongly_connected(&self) -> bool {
        self.correlation > 0.7 && self.coherence > 0.6
    }

    pub fn is_synchronized(&self) -> bool {
        self.phase_lag.abs() < 0.1
    }
}

impl NeuralOscillation {
    pub fn new(frequency_hz: f64, location: String) -> Self {
        let oscillation_type = match frequency_hz {
            f if f < 4.0 => OscillationType::SpontaneousActivity,
            f if f < 8.0 => OscillationType::SpontaneousActivity,
            f if f < 13.0 => OscillationType::SpontaneousActivity,
            f if f < 30.0 => OscillationType::SpontaneousActivity,
            _ => OscillationType::SpontaneousActivity,
        };

        Self {
            frequency_hz,
            amplitude_uv: 10.0,
            phase_radians: 0.0,
            location,
            oscillation_type,
        }
    }

    pub fn frequency_band(&self) -> FrequencyBand {
        match self.frequency_hz {
            f if f < 4.0 => FrequencyBand::Delta,
            f if f < 8.0 => FrequencyBand::Theta,
            f if f < 13.0 => FrequencyBand::Alpha,
            f if f < 30.0 => FrequencyBand::Beta,
            _ => FrequencyBand::Gamma,
        }
    }

    pub fn power(&self) -> f64 {
        self.amplitude_uv * self.amplitude_uv
    }
}

impl SynapticPlasticity {
    pub fn ltp() -> Self {
        Self {
            plasticity_type: PlasticityType::LTP,
            induction_threshold_hz: 100.0,
            time_constant_minutes: 60.0,
            magnitude_fold_change: 2.0,
        }
    }

    pub fn ltd() -> Self {
        Self {
            plasticity_type: PlasticityType::LTD,
            induction_threshold_hz: 1.0,
            time_constant_minutes: 30.0,
            magnitude_fold_change: 0.5,
        }
    }

    pub fn is_potentiation(&self) -> bool {
        self.magnitude_fold_change > 1.0
    }

    pub fn is_depression(&self) -> bool {
        self.magnitude_fold_change < 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brain_network() {
        let mut network = BrainNetwork::new();

        let region1 = BrainRegion::new("V1".to_string(), BrainRegionType::VisualCortex);
        let region2 = BrainRegion::new("PFC".to_string(), BrainRegionType::PrefrontalCortex);

        network.regions.push(region1);
        network.regions.push(region2);

        network.connections.push(NeuralConnection::new(
            "V1".to_string(),
            "PFC".to_string(),
            ConnectionType::Excitatory,
        ));

        assert_eq!(network.total_connections(), 1);
        assert_eq!(network.region_connectivity("V1"), 1);
    }

    #[test]
    fn test_network_metrics() {
        let metrics = NetworkMetrics::default();
        assert!(metrics.is_small_world());
        assert!(metrics.is_highly_modular());
    }

    #[test]
    fn test_brain_region() {
        let region = BrainRegion::new("Hippocampus".to_string(), BrainRegionType::Hippocampus);
        assert!(region.neuron_density() > 0.0);
    }

    #[test]
    fn test_white_matter_tract() {
        let tract = WhiteMatterTract::new(
            "Corpus Callosum".to_string(),
            TractType::CommissuralFiber,
        );

        assert!(tract.is_healthy());
        assert!(!tract.has_white_matter_damage());
    }

    #[test]
    fn test_synaptic_plasticity() {
        let ltp = SynapticPlasticity::ltp();
        assert!(ltp.is_potentiation());
        assert!(!ltp.is_depression());

        let ltd = SynapticPlasticity::ltd();
        assert!(!ltd.is_potentiation());
        assert!(ltd.is_depression());
    }

    #[test]
    fn test_neural_oscillation() {
        let alpha = NeuralOscillation::new(10.0, "Occipital".to_string());
        assert_eq!(alpha.frequency_band(), FrequencyBand::Alpha);

        let gamma = NeuralOscillation::new(40.0, "PFC".to_string());
        assert_eq!(gamma.frequency_band(), FrequencyBand::Gamma);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuroplasticity {
    pub synaptic_plasticity: SynapticPlasticity,
    pub structural_plasticity: StructuralPlasticity,
    pub homeostatic_plasticity: HomeostaticPlasticity,
    pub metaplasticity: Metaplasticity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapticPlasticity {
    pub long_term_potentiation: LongTermPotentiation,
    pub long_term_depression: LongTermDepression,
    pub short_term_plasticity: ShortTermPlasticity,
    pub spike_timing_dependent: SpikeTimingDependentPlasticity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermPotentiation {
    pub nmda_receptor_activation: f64,
    pub ampa_receptor_insertion: f64,
    pub calcium_influx_umol: f64,
    pub camkii_activation: f64,
    pub protein_synthesis_rate: f64,
    pub early_ltp_magnitude: f64,
    pub late_ltp_magnitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermDepression {
    pub low_frequency_stimulation: bool,
    pub ampa_receptor_endocytosis: f64,
    pub phosphatase_activation: f64,
    pub depression_magnitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortTermPlasticity {
    pub facilitation: f64,
    pub depression: f64,
    pub paired_pulse_ratio: f64,
    pub residual_calcium_nmol: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikeTimingDependentPlasticity {
    pub pre_post_timing_ms: f64,
    pub potentiation_window_ms: f64,
    pub depression_window_ms: f64,
    pub weight_change: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralPlasticity {
    pub dendritic_spine_density: f64,
    pub spine_morphology: SpineMorphology,
    pub axonal_sprouting_rate: f64,
    pub synaptogenesis_rate: f64,
    pub synaptic_pruning_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpineMorphology {
    pub mushroom_spines_percent: f64,
    pub thin_spines_percent: f64,
    pub stubby_spines_percent: f64,
    pub filopodia_percent: f64,
    pub spine_head_volume_um3: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeostaticPlasticity {
    pub synaptic_scaling: f64,
    pub intrinsic_excitability: f64,
    pub homeostatic_set_point: f64,
    pub bcm_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metaplasticity {
    pub modification_threshold: f64,
    pub prior_activity_history: Vec<f64>,
    pub priming_effect: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synaptogenesis {
    pub new_synapses_per_day: f64,
    pub bdnf_concentration_ng_ml: f64,
    pub ngf_concentration_ng_ml: f64,
    pub activity_dependent_formation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurotrophicFactors {
    pub bdnf: f64,
    pub ngf: f64,
    pub nt3: f64,
    pub nt4: f64,
    pub gdnf: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningAndMemory {
    pub working_memory_capacity: usize,
    pub long_term_memory_consolidation: f64,
    pub memory_reconsolidation_active: bool,
    pub hippocampal_neurogenesis_rate: f64,
}

impl Neuroplasticity {
    pub fn new_normal() -> Self {
        Self {
            synaptic_plasticity: SynapticPlasticity::new_normal(),
            structural_plasticity: StructuralPlasticity::new_normal(),
            homeostatic_plasticity: HomeostaticPlasticity::new_normal(),
            metaplasticity: Metaplasticity::new_normal(),
        }
    }

    pub fn plasticity_index(&self) -> f64 {
        let synaptic = self.synaptic_plasticity.ltp_ltd_ratio();
        let structural = self.structural_plasticity.net_growth_rate();
        let homeostatic = self.homeostatic_plasticity.stability_score();

        (synaptic * 0.4 + structural * 0.3 + homeostatic * 0.3).min(1.0)
    }

    pub fn enhance_learning(&mut self) {
        self.synaptic_plasticity.long_term_potentiation.induce_early_ltp();
        self.structural_plasticity.synaptogenesis_rate *= 1.5;
    }

    pub fn aging_effect(&mut self, age_years: u32) {
        if age_years > 60 {
            let factor = 1.0 - ((age_years as f64 - 60.0) / 100.0).min(0.4);
            self.synaptic_plasticity.long_term_potentiation.late_ltp_magnitude *= factor;
            self.structural_plasticity.synaptogenesis_rate *= factor;
        }
    }

    pub fn stress_effect(&mut self, cortisol_ug_dl: f64) {
        if cortisol_ug_dl > 20.0 {
            let inhibition = ((cortisol_ug_dl - 20.0) / 30.0).min(0.5);
            self.synaptic_plasticity.long_term_potentiation.protein_synthesis_rate *= 1.0 - inhibition;
            self.structural_plasticity.synaptic_pruning_rate *= 1.0 + inhibition;
        }
    }
}

impl SynapticPlasticity {
    pub fn new_normal() -> Self {
        Self {
            long_term_potentiation: LongTermPotentiation::new_basal(),
            long_term_depression: LongTermDepression::new_basal(),
            short_term_plasticity: ShortTermPlasticity::new_basal(),
            spike_timing_dependent: SpikeTimingDependentPlasticity::new_basal(),
        }
    }

    pub fn ltp_ltd_ratio(&self) -> f64 {
        let ltp = self.long_term_potentiation.early_ltp_magnitude;
        let ltd = self.long_term_depression.depression_magnitude.max(0.1);
        ltp / ltd
    }

    pub fn bidirectional_modification_range(&self) -> f64 {
        self.long_term_potentiation.late_ltp_magnitude +
        self.long_term_depression.depression_magnitude
    }
}

impl LongTermPotentiation {
    pub fn new_basal() -> Self {
        Self {
            nmda_receptor_activation: 0.3,
            ampa_receptor_insertion: 0.0,
            calcium_influx_umol: 0.1,
            camkii_activation: 0.2,
            protein_synthesis_rate: 1.0,
            early_ltp_magnitude: 1.3,
            late_ltp_magnitude: 2.0,
        }
    }

    pub fn induce_early_ltp(&mut self) {
        self.nmda_receptor_activation = 1.0;
        self.calcium_influx_umol = 2.0;
        self.camkii_activation = 0.9;
        self.ampa_receptor_insertion = 0.5;
        self.early_ltp_magnitude = 1.5;
    }

    pub fn induce_late_ltp(&mut self) {
        self.induce_early_ltp();
        self.protein_synthesis_rate = 3.0;
        self.late_ltp_magnitude = 2.5;
    }

    pub fn requires_protein_synthesis(&self) -> bool {
        self.late_ltp_magnitude > 2.0
    }

    pub fn calcium_dependent_kinases_active(&self) -> bool {
        self.calcium_influx_umol > 1.0 && self.camkii_activation > 0.5
    }
}

impl LongTermDepression {
    pub fn new_basal() -> Self {
        Self {
            low_frequency_stimulation: false,
            ampa_receptor_endocytosis: 0.0,
            phosphatase_activation: 0.2,
            depression_magnitude: 0.7,
        }
    }

    pub fn induce_ltd(&mut self) {
        self.low_frequency_stimulation = true;
        self.ampa_receptor_endocytosis = 0.4;
        self.phosphatase_activation = 0.8;
        self.depression_magnitude = 0.5;
    }

    pub fn synapse_weakening_rate(&self) -> f64 {
        self.ampa_receptor_endocytosis * self.phosphatase_activation
    }
}

impl ShortTermPlasticity {
    pub fn new_basal() -> Self {
        Self {
            facilitation: 1.0,
            depression: 1.0,
            paired_pulse_ratio: 1.0,
            residual_calcium_nmol: 100.0,
        }
    }

    pub fn apply_facilitation(&mut self, stimulation_frequency_hz: f64) {
        if stimulation_frequency_hz > 10.0 {
            self.facilitation = 1.0 + (stimulation_frequency_hz / 50.0);
            self.residual_calcium_nmol *= 1.5;
            self.paired_pulse_ratio = 1.5;
        }
    }

    pub fn apply_depression(&mut self, high_frequency: bool) {
        if high_frequency {
            self.depression = 0.7;
            self.paired_pulse_ratio = 0.8;
        }
    }
}

impl SpikeTimingDependentPlasticity {
    pub fn new_basal() -> Self {
        Self {
            pre_post_timing_ms: 0.0,
            potentiation_window_ms: 20.0,
            depression_window_ms: -20.0,
            weight_change: 0.0,
        }
    }

    pub fn calculate_weight_change(&mut self, pre_post_interval_ms: f64) {
        self.pre_post_timing_ms = pre_post_interval_ms;

        if pre_post_interval_ms > 0.0 && pre_post_interval_ms < 20.0 {
            self.weight_change = (-pre_post_interval_ms / 20.0).exp() * 0.5;
        } else if pre_post_interval_ms < 0.0 && pre_post_interval_ms > -20.0 {
            self.weight_change = (pre_post_interval_ms / 20.0).exp() * -0.3;
        } else {
            self.weight_change = 0.0;
        }
    }

    pub fn is_potentiating(&self) -> bool {
        self.weight_change > 0.0
    }

    pub fn is_depressing(&self) -> bool {
        self.weight_change < 0.0
    }
}

impl StructuralPlasticity {
    pub fn new_normal() -> Self {
        Self {
            dendritic_spine_density: 10.0,
            spine_morphology: SpineMorphology::new_normal(),
            axonal_sprouting_rate: 0.5,
            synaptogenesis_rate: 1.0,
            synaptic_pruning_rate: 0.8,
        }
    }

    pub fn net_growth_rate(&self) -> f64 {
        self.synaptogenesis_rate - self.synaptic_pruning_rate
    }

    pub fn spine_stability_index(&self) -> f64 {
        self.spine_morphology.mushroom_spines_percent / 100.0
    }

    pub fn developmental_pruning(&mut self) {
        self.synaptic_pruning_rate = 2.0;
        self.synaptogenesis_rate = 0.5;
    }

    pub fn activity_dependent_growth(&mut self) {
        self.synaptogenesis_rate = 2.0;
        self.axonal_sprouting_rate = 1.5;
        self.spine_morphology.mushroom_spines_percent += 5.0;
    }
}

impl SpineMorphology {
    pub fn new_normal() -> Self {
        Self {
            mushroom_spines_percent: 60.0,
            thin_spines_percent: 25.0,
            stubby_spines_percent: 10.0,
            filopodia_percent: 5.0,
            spine_head_volume_um3: 0.05,
        }
    }

    pub fn mature_spine_fraction(&self) -> f64 {
        (self.mushroom_spines_percent + self.stubby_spines_percent) / 100.0
    }

    pub fn learning_induced_changes(&mut self) {
        self.thin_spines_percent -= 10.0;
        self.mushroom_spines_percent += 10.0;
        self.spine_head_volume_um3 *= 1.3;
    }
}

impl HomeostaticPlasticity {
    pub fn new_normal() -> Self {
        Self {
            synaptic_scaling: 1.0,
            intrinsic_excitability: 1.0,
            homeostatic_set_point: 5.0,
            bcm_threshold: 10.0,
        }
    }

    pub fn scale_up_synapses(&mut self) {
        self.synaptic_scaling = 1.5;
        self.intrinsic_excitability = 1.2;
    }

    pub fn scale_down_synapses(&mut self) {
        self.synaptic_scaling = 0.7;
        self.intrinsic_excitability = 0.8;
    }

    pub fn stability_score(&self) -> f64 {
        let scaling_deviation = (self.synaptic_scaling - 1.0).abs();
        1.0 - scaling_deviation.min(0.5)
    }

    pub fn adjust_bcm_threshold(&mut self, average_activity: f64) {
        self.bcm_threshold = average_activity * 2.0;
    }
}

impl Metaplasticity {
    pub fn new_normal() -> Self {
        Self {
            modification_threshold: 1.0,
            prior_activity_history: Vec::new(),
            priming_effect: 0.0,
        }
    }

    pub fn prime_for_ltp(&mut self) {
        self.modification_threshold *= 0.7;
        self.priming_effect = 0.5;
    }

    pub fn update_history(&mut self, activity: f64) {
        self.prior_activity_history.push(activity);
        if self.prior_activity_history.len() > 100 {
            self.prior_activity_history.remove(0);
        }

        let avg_activity: f64 = self.prior_activity_history.iter().sum::<f64>()
            / self.prior_activity_history.len() as f64;

        if avg_activity > 5.0 {
            self.modification_threshold = 1.5;
        } else {
            self.modification_threshold = 0.8;
        }
    }
}

impl Synaptogenesis {
    pub fn new_normal() -> Self {
        Self {
            new_synapses_per_day: 100.0,
            bdnf_concentration_ng_ml: 20.0,
            ngf_concentration_ng_ml: 10.0,
            activity_dependent_formation: true,
        }
    }

    pub fn enhance_with_bdnf(&mut self, bdnf_ng_ml: f64) {
        self.bdnf_concentration_ng_ml = bdnf_ng_ml;
        self.new_synapses_per_day = 100.0 * (bdnf_ng_ml / 20.0);
    }

    pub fn developmental_synaptogenesis(&mut self) {
        self.new_synapses_per_day = 1000.0;
        self.activity_dependent_formation = false;
    }

    pub fn neurotrophin_sufficiency(&self) -> bool {
        self.bdnf_concentration_ng_ml > 10.0 && self.ngf_concentration_ng_ml > 5.0
    }
}

impl NeurotrophicFactors {
    pub fn new_normal() -> Self {
        Self {
            bdnf: 20.0,
            ngf: 10.0,
            nt3: 5.0,
            nt4: 3.0,
            gdnf: 8.0,
        }
    }

    pub fn exercise_induced_increase(&mut self) {
        self.bdnf *= 2.0;
        self.nt3 *= 1.5;
    }

    pub fn depression_associated_decrease(&mut self) {
        self.bdnf *= 0.5;
        self.ngf *= 0.7;
    }

    pub fn total_neurotrophin_support(&self) -> f64 {
        self.bdnf + self.ngf + self.nt3 + self.nt4 + self.gdnf
    }
}

impl LearningAndMemory {
    pub fn new_normal() -> Self {
        Self {
            working_memory_capacity: 7,
            long_term_memory_consolidation: 0.8,
            memory_reconsolidation_active: false,
            hippocampal_neurogenesis_rate: 700.0,
        }
    }

    pub fn enhance_consolidation(&mut self) {
        self.long_term_memory_consolidation = 0.95;
        self.hippocampal_neurogenesis_rate *= 1.5;
    }

    pub fn sleep_dependent_consolidation(&mut self, sleep_hours: f64) {
        if sleep_hours >= 7.0 {
            self.long_term_memory_consolidation = 0.9;
        } else {
            self.long_term_memory_consolidation = 0.5 + (sleep_hours / 14.0);
        }
    }

    pub fn aging_decline(&mut self, age_years: u32) {
        if age_years > 60 {
            let factor = 1.0 - ((age_years as f64 - 60.0) / 80.0).min(0.5);
            self.working_memory_capacity = (7.0 * factor) as usize;
            self.hippocampal_neurogenesis_rate *= factor;
        }
    }

    pub fn memory_formation_potential(&self) -> f64 {
        (self.long_term_memory_consolidation * 0.6) +
        ((self.hippocampal_neurogenesis_rate / 1000.0) * 0.4).min(0.4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuroplasticity() {
        let plasticity = Neuroplasticity::new_normal();
        assert!(plasticity.plasticity_index() > 0.5);
    }

    #[test]
    fn test_ltp_induction() {
        let mut ltp = LongTermPotentiation::new_basal();
        ltp.induce_early_ltp();

        assert_eq!(ltp.nmda_receptor_activation, 1.0);
        assert!(ltp.early_ltp_magnitude > 1.0);
    }

    #[test]
    fn test_late_ltp() {
        let mut ltp = LongTermPotentiation::new_basal();
        ltp.induce_late_ltp();

        assert!(ltp.requires_protein_synthesis());
        assert!(ltp.late_ltp_magnitude > 2.0);
    }

    #[test]
    fn test_ltd_induction() {
        let mut ltd = LongTermDepression::new_basal();
        ltd.induce_ltd();

        assert!(ltd.low_frequency_stimulation);
        assert!(ltd.synapse_weakening_rate() > 0.0);
    }

    #[test]
    fn test_short_term_facilitation() {
        let mut stp = ShortTermPlasticity::new_basal();
        stp.apply_facilitation(20.0);

        assert!(stp.facilitation > 1.0);
        assert!(stp.paired_pulse_ratio > 1.0);
    }

    #[test]
    fn test_stdp() {
        let mut stdp = SpikeTimingDependentPlasticity::new_basal();
        stdp.calculate_weight_change(10.0);

        assert!(stdp.is_potentiating());
        assert!(stdp.weight_change > 0.0);

        stdp.calculate_weight_change(-10.0);
        assert!(stdp.is_depressing());
    }

    #[test]
    fn test_structural_plasticity() {
        let plasticity = StructuralPlasticity::new_normal();
        assert!(plasticity.net_growth_rate() > 0.0);
        assert!(plasticity.spine_stability_index() > 0.5);
    }

    #[test]
    fn test_spine_morphology() {
        let mut morphology = SpineMorphology::new_normal();
        let baseline_mushroom = morphology.mushroom_spines_percent;

        morphology.learning_induced_changes();
        assert!(morphology.mushroom_spines_percent > baseline_mushroom);
    }

    #[test]
    fn test_homeostatic_plasticity() {
        let mut homeostatic = HomeostaticPlasticity::new_normal();
        homeostatic.scale_up_synapses();

        assert!(homeostatic.synaptic_scaling > 1.0);
        assert!(homeostatic.stability_score() < 1.0);
    }

    #[test]
    fn test_metaplasticity() {
        let mut meta = Metaplasticity::new_normal();
        meta.prime_for_ltp();

        assert!(meta.modification_threshold < 1.0);
        assert_eq!(meta.priming_effect, 0.5);
    }

    #[test]
    fn test_synaptogenesis() {
        let mut syn = Synaptogenesis::new_normal();
        syn.enhance_with_bdnf(40.0);

        assert!(syn.new_synapses_per_day > 100.0);
    }

    #[test]
    fn test_neurotrophic_factors() {
        let mut factors = NeurotrophicFactors::new_normal();
        let baseline_bdnf = factors.bdnf;

        factors.exercise_induced_increase();
        assert!(factors.bdnf > baseline_bdnf);
    }

    #[test]
    fn test_learning_and_memory() {
        let mut memory = LearningAndMemory::new_normal();
        assert_eq!(memory.working_memory_capacity, 7);

        memory.enhance_consolidation();
        assert!(memory.long_term_memory_consolidation > 0.9);
    }

    #[test]
    fn test_sleep_consolidation() {
        let mut memory = LearningAndMemory::new_normal();
        memory.sleep_dependent_consolidation(8.0);

        assert!(memory.long_term_memory_consolidation > 0.8);
    }

    #[test]
    fn test_aging_neuroplasticity() {
        let mut plasticity = Neuroplasticity::new_normal();
        plasticity.aging_effect(75);

        assert!(plasticity.synaptic_plasticity.long_term_potentiation.late_ltp_magnitude < 2.0);
    }

    #[test]
    fn test_stress_effect() {
        let mut plasticity = Neuroplasticity::new_normal();
        plasticity.stress_effect(30.0);

        assert!(plasticity.structural_plasticity.synaptic_pruning_rate > 0.8);
    }
}

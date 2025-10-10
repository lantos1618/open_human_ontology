use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Neurotransmitter {
    Dopamine,
    Serotonin,
    Norepinephrine,
    Epinephrine,
    Acetylcholine,
    GABA,
    Glutamate,
    Glycine,
    Histamine,
    Endorphins,
    Enkephalins,
    Orexin,
    Oxytocin,
    Vasopressin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DopaminePathway {
    Nigrostriatal,
    Mesolimbic,
    Mesocortical,
    Tuberoinfundibular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerotoninPathway {
    RapheNuclei,
    ProjectionsToCortex,
    ProjectionsToLimbic,
    ProjectionsToHypothalamus,
    ProjectionsToSpinalCord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DopamineSystem {
    pub nigrostriatal_activity: f64,
    pub mesolimbic_activity: f64,
    pub mesocortical_activity: f64,
    pub tuberoinfundibular_activity: f64,
    pub d1_receptor_density: f64,
    pub d2_receptor_density: f64,
    pub dopamine_synthesis_rate: f64,
    pub dopamine_reuptake_rate: f64,
    pub mao_activity: f64,
    pub comt_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerotoninSystem {
    pub raphe_nuclei_activity: f64,
    pub cortical_projection_activity: f64,
    pub limbic_projection_activity: f64,
    pub serotonin_synthesis_rate: f64,
    pub serotonin_reuptake_rate: f64,
    pub receptor_5ht1a_density: f64,
    pub receptor_5ht2a_density: f64,
    pub receptor_5ht3_density: f64,
    pub tryptophan_availability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NorepinephrineSystem {
    pub locus_coeruleus_activity: f64,
    pub sympathetic_outflow: f64,
    pub synthesis_rate: f64,
    pub reuptake_rate: f64,
    pub alpha1_receptor_density: f64,
    pub alpha2_receptor_density: f64,
    pub beta1_receptor_density: f64,
    pub beta2_receptor_density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GABASystem {
    pub total_gaba_concentration: f64,
    pub gaba_synthesis_rate: f64,
    pub gaba_degradation_rate: f64,
    pub gabaa_receptor_density: f64,
    pub gabab_receptor_density: f64,
    pub benzodiazepine_binding_sites: f64,
    pub inhibitory_tone: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlutamateSystem {
    pub glutamate_concentration: f64,
    pub nmda_receptor_density: f64,
    pub ampa_receptor_density: f64,
    pub kainate_receptor_density: f64,
    pub mglu_receptor_density: f64,
    pub excitatory_tone: f64,
    pub excitotoxicity_risk: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcetylcholineSystem {
    pub synthesis_rate: f64,
    pub muscarinic_m1_density: f64,
    pub muscarinic_m2_density: f64,
    pub nicotinic_receptor_density: f64,
    pub acetylcholinesterase_activity: f64,
    pub basal_forebrain_activity: f64,
    pub brainstem_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndogenousOpioidSystem {
    pub beta_endorphin_level: f64,
    pub met_enkephalin_level: f64,
    pub leu_enkephalin_level: f64,
    pub dynorphin_level: f64,
    pub mu_receptor_density: f64,
    pub delta_receptor_density: f64,
    pub kappa_receptor_density: f64,
}

impl DopamineSystem {
    pub fn new_normal() -> Self {
        Self {
            nigrostriatal_activity: 1.0,
            mesolimbic_activity: 1.0,
            mesocortical_activity: 1.0,
            tuberoinfundibular_activity: 1.0,
            d1_receptor_density: 1.0,
            d2_receptor_density: 1.0,
            dopamine_synthesis_rate: 1.0,
            dopamine_reuptake_rate: 1.0,
            mao_activity: 1.0,
            comt_activity: 1.0,
        }
    }

    pub fn calculate_dopamine_tone(&self) -> f64 {
        let synthesis = self.dopamine_synthesis_rate;
        let degradation = (self.mao_activity + self.comt_activity) / 2.0;
        let reuptake = self.dopamine_reuptake_rate;

        synthesis / (degradation + reuptake)
    }

    pub fn calculate_pathway_balance(&self) -> f64 {
        (self.nigrostriatal_activity + self.mesolimbic_activity + self.mesocortical_activity) / 3.0
    }

    pub fn has_parkinsonian_pattern(&self) -> bool {
        self.nigrostriatal_activity < 0.5
    }

    pub fn has_schizophrenia_like_pattern(&self) -> bool {
        self.mesolimbic_activity > 1.5 && self.mesocortical_activity < 0.8
    }

    pub fn has_adhd_like_pattern(&self) -> bool {
        self.mesocortical_activity < 0.7 && self.dopamine_reuptake_rate > 1.3
    }

    pub fn receptor_balance(&self) -> f64 {
        self.d1_receptor_density / self.d2_receptor_density
    }
}

impl SerotoninSystem {
    pub fn new_normal() -> Self {
        Self {
            raphe_nuclei_activity: 1.0,
            cortical_projection_activity: 1.0,
            limbic_projection_activity: 1.0,
            serotonin_synthesis_rate: 1.0,
            serotonin_reuptake_rate: 1.0,
            receptor_5ht1a_density: 1.0,
            receptor_5ht2a_density: 1.0,
            receptor_5ht3_density: 1.0,
            tryptophan_availability: 1.0,
        }
    }

    pub fn calculate_serotonin_tone(&self) -> f64 {
        (self.serotonin_synthesis_rate * self.tryptophan_availability)
            / self.serotonin_reuptake_rate
    }

    pub fn has_depression_like_pattern(&self) -> bool {
        self.calculate_serotonin_tone() < 0.7
    }

    pub fn has_anxiety_like_pattern(&self) -> bool {
        self.receptor_5ht2a_density > 1.3 || self.limbic_projection_activity > 1.4
    }

    pub fn receptor_ratio_1a_2a(&self) -> f64 {
        self.receptor_5ht1a_density / self.receptor_5ht2a_density
    }
}

impl NorepinephrineSystem {
    pub fn new_normal() -> Self {
        Self {
            locus_coeruleus_activity: 1.0,
            sympathetic_outflow: 1.0,
            synthesis_rate: 1.0,
            reuptake_rate: 1.0,
            alpha1_receptor_density: 1.0,
            alpha2_receptor_density: 1.0,
            beta1_receptor_density: 1.0,
            beta2_receptor_density: 1.0,
        }
    }

    pub fn calculate_noradrenergic_tone(&self) -> f64 {
        (self.locus_coeruleus_activity * self.synthesis_rate) / self.reuptake_rate
    }

    pub fn has_hyperarousal_pattern(&self) -> bool {
        self.locus_coeruleus_activity > 1.5 || self.sympathetic_outflow > 1.5
    }

    pub fn has_hypoarousal_pattern(&self) -> bool {
        self.locus_coeruleus_activity < 0.6
    }

    pub fn alpha_beta_balance(&self) -> f64 {
        (self.alpha1_receptor_density + self.alpha2_receptor_density)
            / (self.beta1_receptor_density + self.beta2_receptor_density)
    }
}

impl GABASystem {
    pub fn new_normal() -> Self {
        Self {
            total_gaba_concentration: 1.0,
            gaba_synthesis_rate: 1.0,
            gaba_degradation_rate: 1.0,
            gabaa_receptor_density: 1.0,
            gabab_receptor_density: 1.0,
            benzodiazepine_binding_sites: 1.0,
            inhibitory_tone: 1.0,
        }
    }

    pub fn calculate_gaba_tone(&self) -> f64 {
        (self.gaba_synthesis_rate * self.gabaa_receptor_density) / self.gaba_degradation_rate
    }

    pub fn has_hyperexcitability(&self) -> bool {
        self.inhibitory_tone < 0.7 || self.gabaa_receptor_density < 0.7
    }

    pub fn has_excess_inhibition(&self) -> bool {
        self.inhibitory_tone > 1.5
    }
}

impl GlutamateSystem {
    pub fn new_normal() -> Self {
        Self {
            glutamate_concentration: 1.0,
            nmda_receptor_density: 1.0,
            ampa_receptor_density: 1.0,
            kainate_receptor_density: 1.0,
            mglu_receptor_density: 1.0,
            excitatory_tone: 1.0,
            excitotoxicity_risk: 0.1,
        }
    }

    pub fn calculate_excitatory_tone(&self) -> f64 {
        self.glutamate_concentration * (self.nmda_receptor_density + self.ampa_receptor_density)
            / 2.0
    }

    pub fn has_excitotoxicity_risk(&self) -> bool {
        self.excitotoxicity_risk > 0.5 || self.glutamate_concentration > 1.5
    }

    pub fn nmda_ampa_ratio(&self) -> f64 {
        self.nmda_receptor_density / self.ampa_receptor_density
    }
}

impl AcetylcholineSystem {
    pub fn new_normal() -> Self {
        Self {
            synthesis_rate: 1.0,
            muscarinic_m1_density: 1.0,
            muscarinic_m2_density: 1.0,
            nicotinic_receptor_density: 1.0,
            acetylcholinesterase_activity: 1.0,
            basal_forebrain_activity: 1.0,
            brainstem_activity: 1.0,
        }
    }

    pub fn calculate_cholinergic_tone(&self) -> f64 {
        (self.synthesis_rate * self.basal_forebrain_activity) / self.acetylcholinesterase_activity
    }

    pub fn has_cholinergic_deficit(&self) -> bool {
        self.calculate_cholinergic_tone() < 0.6
    }

    pub fn muscarinic_nicotinic_balance(&self) -> f64 {
        (self.muscarinic_m1_density + self.muscarinic_m2_density)
            / (2.0 * self.nicotinic_receptor_density)
    }
}

impl EndogenousOpioidSystem {
    pub fn new_normal() -> Self {
        Self {
            beta_endorphin_level: 1.0,
            met_enkephalin_level: 1.0,
            leu_enkephalin_level: 1.0,
            dynorphin_level: 1.0,
            mu_receptor_density: 1.0,
            delta_receptor_density: 1.0,
            kappa_receptor_density: 1.0,
        }
    }

    pub fn calculate_opioid_tone(&self) -> f64 {
        (self.beta_endorphin_level * self.mu_receptor_density
            + self.met_enkephalin_level * self.delta_receptor_density
            + self.dynorphin_level * self.kappa_receptor_density)
            / 3.0
    }

    pub fn has_reduced_analgesia(&self) -> bool {
        self.calculate_opioid_tone() < 0.6
    }

    pub fn mu_delta_ratio(&self) -> f64 {
        (self.beta_endorphin_level * self.mu_receptor_density)
            / (self.met_enkephalin_level * self.delta_receptor_density)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurotransmitterProfile {
    pub dopamine: DopamineSystem,
    pub serotonin: SerotoninSystem,
    pub norepinephrine: NorepinephrineSystem,
    pub gaba: GABASystem,
    pub glutamate: GlutamateSystem,
    pub acetylcholine: AcetylcholineSystem,
    pub opioids: EndogenousOpioidSystem,
}

impl NeurotransmitterProfile {
    pub fn new_normal() -> Self {
        Self {
            dopamine: DopamineSystem::new_normal(),
            serotonin: SerotoninSystem::new_normal(),
            norepinephrine: NorepinephrineSystem::new_normal(),
            gaba: GABASystem::new_normal(),
            glutamate: GlutamateSystem::new_normal(),
            acetylcholine: AcetylcholineSystem::new_normal(),
            opioids: EndogenousOpioidSystem::new_normal(),
        }
    }

    pub fn excitatory_inhibitory_balance(&self) -> f64 {
        self.glutamate.calculate_excitatory_tone() / self.gaba.calculate_gaba_tone()
    }

    pub fn monoamine_balance(&self) -> (f64, f64, f64) {
        (
            self.dopamine.calculate_dopamine_tone(),
            self.serotonin.calculate_serotonin_tone(),
            self.norepinephrine.calculate_noradrenergic_tone(),
        )
    }

    pub fn has_imbalance(&self) -> bool {
        let ei_balance = self.excitatory_inhibitory_balance();
        !(0.7..=1.5).contains(&ei_balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dopamine_system() {
        let da = DopamineSystem::new_normal();
        assert_eq!(da.calculate_dopamine_tone(), 0.5);
        assert!(!da.has_parkinsonian_pattern());
    }

    #[test]
    fn test_serotonin_system() {
        let sert = SerotoninSystem::new_normal();
        assert_eq!(sert.calculate_serotonin_tone(), 1.0);
        assert!(!sert.has_depression_like_pattern());
    }

    #[test]
    fn test_norepinephrine_system() {
        let ne = NorepinephrineSystem::new_normal();
        assert_eq!(ne.calculate_noradrenergic_tone(), 1.0);
        assert!(!ne.has_hyperarousal_pattern());
    }

    #[test]
    fn test_gaba_system() {
        let gaba = GABASystem::new_normal();
        assert_eq!(gaba.calculate_gaba_tone(), 1.0);
        assert!(!gaba.has_hyperexcitability());
    }

    #[test]
    fn test_glutamate_system() {
        let glu = GlutamateSystem::new_normal();
        assert_eq!(glu.calculate_excitatory_tone(), 1.0);
        assert!(!glu.has_excitotoxicity_risk());
    }

    #[test]
    fn test_neurotransmitter_profile() {
        let profile = NeurotransmitterProfile::new_normal();
        assert_eq!(profile.excitatory_inhibitory_balance(), 1.0);
        assert!(!profile.has_imbalance());
    }
}

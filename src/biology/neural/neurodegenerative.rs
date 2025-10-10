use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurodegenerativeDisease {
    pub disease_type: DiseaseType,
    pub protein_misfolding: ProteinMisfolding,
    pub neuronal_loss: NeuronalLoss,
    pub cognitive_decline: CognitiveDecline,
    pub disease_progression: DiseaseProgression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiseaseType {
    Alzheimers,
    Parkinsons,
    HuntingtonsDisease,
    ALS,
    FrontotemporalDementia,
    LewyBodyDementia,
    MultipleSystemAtrophy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinMisfolding {
    pub amyloid_beta: AmyloidBeta,
    pub tau_protein: TauProtein,
    pub alpha_synuclein: AlphaSynuclein,
    pub tdp43: TDP43,
    pub huntingtin: Huntingtin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmyloidBeta {
    pub ab40_pg_ml: f64,
    pub ab42_pg_ml: f64,
    pub ab42_ab40_ratio: f64,
    pub oligomers_present: bool,
    pub plaque_burden: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauProtein {
    pub total_tau_pg_ml: f64,
    pub phospho_tau_pg_ml: f64,
    pub neurofibrillary_tangles: f64,
    pub braak_stage: u8,
    pub hyperphosphorylation_sites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlphaSynuclein {
    pub concentration_ng_ml: f64,
    pub oligomeric_form_percent: f64,
    pub lewy_bodies_present: bool,
    pub lewy_neurites_present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDP43 {
    pub cytoplasmic_aggregates: bool,
    pub nuclear_clearance: f64,
    pub phosphorylated_tdp43: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Huntingtin {
    pub cag_repeat_length: u32,
    pub mutant_huntingtin_aggregates: f64,
    pub striatal_degeneration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuronalLoss {
    pub hippocampal_volume_percent: f64,
    pub cortical_thickness_mm: f64,
    pub substantia_nigra_neurons_percent: f64,
    pub striatal_volume_percent: f64,
    pub motor_neuron_count_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveDecline {
    pub mmse_score: u8,
    pub moca_score: u8,
    pub memory_impairment: f64,
    pub executive_function: f64,
    pub language_function: f64,
    pub visuospatial_function: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseProgression {
    pub disease_duration_years: f64,
    pub progression_rate: f64,
    pub clinical_stage: ClinicalStage,
    pub biomarker_changes: BiomarkerChanges,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClinicalStage {
    Preclinical,
    MildCognitiveImpairment,
    MildDementia,
    ModerateDementia,
    SevereDementia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomarkerChanges {
    pub csf_abeta42_pg_ml: f64,
    pub csf_tau_pg_ml: f64,
    pub csf_ptau_pg_ml: f64,
    pub plasma_nfl_pg_ml: f64,
    pub fdg_pet_hypometabolism: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialDysfunction {
    pub atp_production_percent: f64,
    pub reactive_oxygen_species: f64,
    pub mitochondrial_membrane_potential_mv: f64,
    pub mitophagy_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuroinflammation {
    pub microglial_activation: f64,
    pub astrogliosis: f64,
    pub pro_inflammatory_cytokines: f64,
    pub blood_brain_barrier_integrity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapticDysfunction {
    pub synaptic_density_percent: f64,
    pub neurotransmitter_release: f64,
    pub receptor_expression: f64,
    pub synaptic_pruning_excessive: bool,
}

impl NeurodegenerativeDisease {
    pub fn new_alzheimers() -> Self {
        Self {
            disease_type: DiseaseType::Alzheimers,
            protein_misfolding: ProteinMisfolding::alzheimers_pattern(),
            neuronal_loss: NeuronalLoss::hippocampal_atrophy(),
            cognitive_decline: CognitiveDecline::memory_dominant(),
            disease_progression: DiseaseProgression::new_preclinical(),
        }
    }

    pub fn new_parkinsons() -> Self {
        Self {
            disease_type: DiseaseType::Parkinsons,
            protein_misfolding: ProteinMisfolding::parkinsons_pattern(),
            neuronal_loss: NeuronalLoss::substantia_nigra_loss(),
            cognitive_decline: CognitiveDecline::executive_dominant(),
            disease_progression: DiseaseProgression::new_preclinical(),
        }
    }

    pub fn new_huntingtons(cag_repeats: u32) -> Self {
        Self {
            disease_type: DiseaseType::HuntingtonsDisease,
            protein_misfolding: ProteinMisfolding::huntingtons_pattern(cag_repeats),
            neuronal_loss: NeuronalLoss::striatal_atrophy(),
            cognitive_decline: CognitiveDecline::executive_dominant(),
            disease_progression: DiseaseProgression::new_preclinical(),
        }
    }

    pub fn disease_severity_score(&self) -> f64 {
        let protein_score = self.protein_misfolding.pathology_score();
        let neuronal_score = 1.0 - self.neuronal_loss.overall_preservation();
        let cognitive_score = self.cognitive_decline.impairment_score();

        (protein_score * 0.3 + neuronal_score * 0.4 + cognitive_score * 0.3).min(1.0)
    }

    pub fn advance_disease(&mut self, years: f64) {
        self.disease_progression.disease_duration_years += years;

        match self.disease_type {
            DiseaseType::Alzheimers => {
                self.protein_misfolding.amyloid_beta.plaque_burden += 0.1 * years;
                self.protein_misfolding.tau_protein.braak_stage =
                    (self.protein_misfolding.tau_protein.braak_stage as f64 + 0.5 * years).min(6.0)
                        as u8;
                self.neuronal_loss.hippocampal_volume_percent -= 2.0 * years;
            }
            DiseaseType::Parkinsons => {
                self.neuronal_loss.substantia_nigra_neurons_percent -= 5.0 * years;
                self.protein_misfolding.alpha_synuclein.lewy_bodies_present = true;
            }
            DiseaseType::HuntingtonsDisease => {
                self.neuronal_loss.striatal_volume_percent -= 3.0 * years;
            }
            _ => {}
        }

        self.update_clinical_stage();
    }

    fn update_clinical_stage(&mut self) {
        let severity = self.disease_severity_score();

        self.disease_progression.clinical_stage = if severity < 0.2 {
            ClinicalStage::Preclinical
        } else if severity < 0.4 {
            ClinicalStage::MildCognitiveImpairment
        } else if severity < 0.6 {
            ClinicalStage::MildDementia
        } else if severity < 0.8 {
            ClinicalStage::ModerateDementia
        } else {
            ClinicalStage::SevereDementia
        };
    }

    pub fn therapeutic_target_identification(&self) -> Vec<String> {
        let mut targets = Vec::new();

        match self.disease_type {
            DiseaseType::Alzheimers => {
                if self.protein_misfolding.amyloid_beta.plaque_burden > 0.3 {
                    targets.push("Anti-amyloid therapy (aducanumab, lecanemab)".to_string());
                }
                if self.protein_misfolding.tau_protein.phospho_tau_pg_ml > 80.0 {
                    targets.push("Anti-tau therapy".to_string());
                }
                targets.push("Cholinesterase inhibitors".to_string());
                targets.push("NMDA antagonists (memantine)".to_string());
            }
            DiseaseType::Parkinsons => {
                targets.push("Levodopa/carbidopa".to_string());
                targets.push("Dopamine agonists".to_string());
                targets.push("MAO-B inhibitors".to_string());
                if self
                    .protein_misfolding
                    .alpha_synuclein
                    .oligomeric_form_percent
                    > 30.0
                {
                    targets.push("Anti-synuclein antibodies".to_string());
                }
            }
            DiseaseType::HuntingtonsDisease => {
                targets
                    .push("Huntingtin lowering therapy (antisense oligonucleotides)".to_string());
                targets.push("VMAT2 inhibitors (tetrabenazine)".to_string());
            }
            _ => {}
        }

        targets
    }
}

impl ProteinMisfolding {
    pub fn new_normal() -> Self {
        Self {
            amyloid_beta: AmyloidBeta::new_normal(),
            tau_protein: TauProtein::new_normal(),
            alpha_synuclein: AlphaSynuclein::new_normal(),
            tdp43: TDP43::new_normal(),
            huntingtin: Huntingtin::new_normal(),
        }
    }

    pub fn alzheimers_pattern() -> Self {
        let mut misfolding = Self::new_normal();
        misfolding.amyloid_beta.ab42_pg_ml = 400.0;
        misfolding.amyloid_beta.ab42_ab40_ratio = 0.05;
        misfolding.amyloid_beta.oligomers_present = true;
        misfolding.tau_protein.phospho_tau_pg_ml = 80.0;
        misfolding.tau_protein.braak_stage = 3;
        misfolding
    }

    pub fn parkinsons_pattern() -> Self {
        let mut misfolding = Self::new_normal();
        misfolding.alpha_synuclein.concentration_ng_ml = 150.0;
        misfolding.alpha_synuclein.oligomeric_form_percent = 40.0;
        misfolding.alpha_synuclein.lewy_bodies_present = true;
        misfolding
    }

    pub fn huntingtons_pattern(cag_repeats: u32) -> Self {
        let mut misfolding = Self::new_normal();
        misfolding.huntingtin.cag_repeat_length = cag_repeats;
        if cag_repeats > 40 {
            misfolding.huntingtin.mutant_huntingtin_aggregates = 0.7;
        }
        misfolding
    }

    pub fn pathology_score(&self) -> f64 {
        let amyloid_score = if self.amyloid_beta.plaque_burden > 0.0 {
            self.amyloid_beta.plaque_burden
        } else {
            0.0
        };

        let tau_score = (self.tau_protein.braak_stage as f64) / 6.0;
        let synuclein_score = if self.alpha_synuclein.lewy_bodies_present {
            0.5
        } else {
            0.0
        };

        (amyloid_score * 0.4 + tau_score * 0.4 + synuclein_score * 0.2).min(1.0)
    }
}

impl AmyloidBeta {
    pub fn new_normal() -> Self {
        Self {
            ab40_pg_ml: 10000.0,
            ab42_pg_ml: 1000.0,
            ab42_ab40_ratio: 0.10,
            oligomers_present: false,
            plaque_burden: 0.0,
        }
    }

    pub fn is_pathological(&self) -> bool {
        self.ab42_ab40_ratio < 0.067 || self.ab42_pg_ml < 600.0
    }

    pub fn amyloid_positive(&self) -> bool {
        self.plaque_burden > 0.2 || self.ab42_pg_ml < 600.0
    }
}

impl TauProtein {
    pub fn new_normal() -> Self {
        Self {
            total_tau_pg_ml: 300.0,
            phospho_tau_pg_ml: 50.0,
            neurofibrillary_tangles: 0.0,
            braak_stage: 0,
            hyperphosphorylation_sites: Vec::new(),
        }
    }

    pub fn is_pathological(&self) -> bool {
        self.phospho_tau_pg_ml > 60.0 || self.braak_stage > 2
    }

    pub fn tau_positive(&self) -> bool {
        self.phospho_tau_pg_ml > 80.0
    }
}

impl AlphaSynuclein {
    pub fn new_normal() -> Self {
        Self {
            concentration_ng_ml: 50.0,
            oligomeric_form_percent: 5.0,
            lewy_bodies_present: false,
            lewy_neurites_present: false,
        }
    }

    pub fn is_pathological(&self) -> bool {
        self.oligomeric_form_percent > 20.0 || self.lewy_bodies_present
    }
}

impl TDP43 {
    pub fn new_normal() -> Self {
        Self {
            cytoplasmic_aggregates: false,
            nuclear_clearance: 1.0,
            phosphorylated_tdp43: 0.1,
        }
    }

    pub fn is_pathological(&self) -> bool {
        self.cytoplasmic_aggregates || self.nuclear_clearance < 0.5
    }
}

impl Huntingtin {
    pub fn new_normal() -> Self {
        Self {
            cag_repeat_length: 18,
            mutant_huntingtin_aggregates: 0.0,
            striatal_degeneration: 0.0,
        }
    }

    pub fn is_pathological(&self) -> bool {
        self.cag_repeat_length >= 40
    }

    pub fn age_of_onset_prediction(&self) -> Option<u32> {
        if self.cag_repeat_length >= 40 {
            Some(80 - (self.cag_repeat_length - 40) * 2)
        } else {
            None
        }
    }
}

impl NeuronalLoss {
    pub fn new_normal() -> Self {
        Self {
            hippocampal_volume_percent: 100.0,
            cortical_thickness_mm: 2.5,
            substantia_nigra_neurons_percent: 100.0,
            striatal_volume_percent: 100.0,
            motor_neuron_count_percent: 100.0,
        }
    }

    pub fn hippocampal_atrophy() -> Self {
        Self {
            hippocampal_volume_percent: 80.0,
            cortical_thickness_mm: 2.2,
            substantia_nigra_neurons_percent: 100.0,
            striatal_volume_percent: 100.0,
            motor_neuron_count_percent: 100.0,
        }
    }

    pub fn substantia_nigra_loss() -> Self {
        Self {
            hippocampal_volume_percent: 100.0,
            cortical_thickness_mm: 2.5,
            substantia_nigra_neurons_percent: 60.0,
            striatal_volume_percent: 95.0,
            motor_neuron_count_percent: 100.0,
        }
    }

    pub fn striatal_atrophy() -> Self {
        Self {
            hippocampal_volume_percent: 95.0,
            cortical_thickness_mm: 2.3,
            substantia_nigra_neurons_percent: 100.0,
            striatal_volume_percent: 70.0,
            motor_neuron_count_percent: 100.0,
        }
    }

    pub fn overall_preservation(&self) -> f64 {
        (self.hippocampal_volume_percent
            + self.substantia_nigra_neurons_percent
            + self.striatal_volume_percent
            + self.motor_neuron_count_percent)
            / 400.0
    }
}

impl CognitiveDecline {
    pub fn new_normal() -> Self {
        Self {
            mmse_score: 30,
            moca_score: 28,
            memory_impairment: 0.0,
            executive_function: 1.0,
            language_function: 1.0,
            visuospatial_function: 1.0,
        }
    }

    pub fn memory_dominant() -> Self {
        Self {
            mmse_score: 22,
            moca_score: 20,
            memory_impairment: 0.6,
            executive_function: 0.8,
            language_function: 0.9,
            visuospatial_function: 0.8,
        }
    }

    pub fn executive_dominant() -> Self {
        Self {
            mmse_score: 24,
            moca_score: 21,
            memory_impairment: 0.3,
            executive_function: 0.5,
            language_function: 0.8,
            visuospatial_function: 0.7,
        }
    }

    pub fn impairment_score(&self) -> f64 {
        (self.memory_impairment * 0.3
            + (1.0 - self.executive_function) * 0.3
            + (1.0 - self.language_function) * 0.2
            + (1.0 - self.visuospatial_function) * 0.2)
            .min(1.0)
    }

    pub fn dementia_severity(&self) -> ClinicalStage {
        if self.mmse_score >= 24 {
            ClinicalStage::MildCognitiveImpairment
        } else if self.mmse_score >= 18 {
            ClinicalStage::MildDementia
        } else if self.mmse_score >= 10 {
            ClinicalStage::ModerateDementia
        } else {
            ClinicalStage::SevereDementia
        }
    }
}

impl DiseaseProgression {
    pub fn new_preclinical() -> Self {
        Self {
            disease_duration_years: 0.0,
            progression_rate: 1.0,
            clinical_stage: ClinicalStage::Preclinical,
            biomarker_changes: BiomarkerChanges::new_normal(),
        }
    }

    pub fn estimate_time_to_dementia(&self) -> Option<f64> {
        match self.clinical_stage {
            ClinicalStage::Preclinical => Some(10.0 / self.progression_rate),
            ClinicalStage::MildCognitiveImpairment => Some(3.0 / self.progression_rate),
            _ => None,
        }
    }
}

impl BiomarkerChanges {
    pub fn new_normal() -> Self {
        Self {
            csf_abeta42_pg_ml: 1000.0,
            csf_tau_pg_ml: 300.0,
            csf_ptau_pg_ml: 50.0,
            plasma_nfl_pg_ml: 15.0,
            fdg_pet_hypometabolism: 0.0,
        }
    }

    pub fn alzheimers_signature(&self) -> bool {
        self.csf_abeta42_pg_ml < 600.0
            && self.csf_ptau_pg_ml > 60.0
            && self.fdg_pet_hypometabolism > 0.2
    }
}

impl MitochondrialDysfunction {
    pub fn new_normal() -> Self {
        Self {
            atp_production_percent: 100.0,
            reactive_oxygen_species: 1.0,
            mitochondrial_membrane_potential_mv: -140.0,
            mitophagy_rate: 1.0,
        }
    }

    pub fn neurodegenerative_dysfunction(&mut self) {
        self.atp_production_percent = 60.0;
        self.reactive_oxygen_species = 3.0;
        self.mitochondrial_membrane_potential_mv = -100.0;
        self.mitophagy_rate = 0.5;
    }

    pub fn bioenergetic_capacity(&self) -> f64 {
        (self.atp_production_percent / 100.0) * (self.mitochondrial_membrane_potential_mv / -140.0)
    }
}

impl Neuroinflammation {
    pub fn new_normal() -> Self {
        Self {
            microglial_activation: 0.1,
            astrogliosis: 0.1,
            pro_inflammatory_cytokines: 0.1,
            blood_brain_barrier_integrity: 1.0,
        }
    }

    pub fn chronic_activation(&mut self) {
        self.microglial_activation = 0.8;
        self.astrogliosis = 0.7;
        self.pro_inflammatory_cytokines = 3.0;
        self.blood_brain_barrier_integrity = 0.6;
    }

    pub fn inflammation_score(&self) -> f64 {
        (self.microglial_activation * 0.4
            + self.astrogliosis * 0.3
            + (self.pro_inflammatory_cytokines / 5.0).min(0.3))
        .min(1.0)
    }
}

impl SynapticDysfunction {
    pub fn new_normal() -> Self {
        Self {
            synaptic_density_percent: 100.0,
            neurotransmitter_release: 1.0,
            receptor_expression: 1.0,
            synaptic_pruning_excessive: false,
        }
    }

    pub fn neurodegenerative_synapse_loss(&mut self) {
        self.synaptic_density_percent = 60.0;
        self.neurotransmitter_release = 0.5;
        self.receptor_expression = 0.7;
        self.synaptic_pruning_excessive = true;
    }

    pub fn synaptic_function_score(&self) -> f64 {
        (self.synaptic_density_percent / 100.0)
            * self.neurotransmitter_release
            * self.receptor_expression
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alzheimers_disease() {
        let ad = NeurodegenerativeDisease::new_alzheimers();
        assert_eq!(ad.disease_type, DiseaseType::Alzheimers);
        assert!(ad.protein_misfolding.amyloid_beta.oligomers_present);
    }

    #[test]
    fn test_parkinsons_disease() {
        let pd = NeurodegenerativeDisease::new_parkinsons();
        assert_eq!(pd.disease_type, DiseaseType::Parkinsons);
        assert!(pd.protein_misfolding.alpha_synuclein.lewy_bodies_present);
    }

    #[test]
    fn test_huntingtons_disease() {
        let hd = NeurodegenerativeDisease::new_huntingtons(45);
        assert_eq!(hd.disease_type, DiseaseType::HuntingtonsDisease);
        assert_eq!(hd.protein_misfolding.huntingtin.cag_repeat_length, 45);
    }

    #[test]
    fn test_disease_severity() {
        let ad = NeurodegenerativeDisease::new_alzheimers();
        let severity = ad.disease_severity_score();
        assert!(severity > 0.0 && severity < 1.0);
    }

    #[test]
    fn test_disease_progression() {
        let mut ad = NeurodegenerativeDisease::new_alzheimers();
        let initial_duration = ad.disease_progression.disease_duration_years;

        ad.advance_disease(2.0);
        assert!(ad.disease_progression.disease_duration_years > initial_duration);
    }

    #[test]
    fn test_amyloid_pathology() {
        let amyloid = AmyloidBeta::new_normal();
        assert!(!amyloid.is_pathological());
        assert!(!amyloid.amyloid_positive());
    }

    #[test]
    fn test_tau_pathology() {
        let mut tau = TauProtein::new_normal();
        assert!(!tau.is_pathological());

        tau.phospho_tau_pg_ml = 90.0;
        assert!(tau.is_pathological());
        assert!(tau.tau_positive());
    }

    #[test]
    fn test_alpha_synuclein() {
        let synuclein = AlphaSynuclein::new_normal();
        assert!(!synuclein.is_pathological());
    }

    #[test]
    fn test_huntingtin_cag_repeats() {
        let htt = Huntingtin::new_normal();
        assert!(!htt.is_pathological());
        assert!(htt.age_of_onset_prediction().is_none());

        let mut htt_mutant = Huntingtin::new_normal();
        htt_mutant.cag_repeat_length = 45;
        assert!(htt_mutant.is_pathological());
        assert!(htt_mutant.age_of_onset_prediction().is_some());
    }

    #[test]
    fn test_neuronal_loss() {
        let loss = NeuronalLoss::new_normal();
        assert!(loss.overall_preservation() > 0.95);

        let atrophy = NeuronalLoss::hippocampal_atrophy();
        assert!(atrophy.overall_preservation() < 1.0);
        assert!(atrophy.hippocampal_volume_percent < 100.0);
    }

    #[test]
    fn test_cognitive_decline() {
        let normal = CognitiveDecline::new_normal();
        assert_eq!(normal.mmse_score, 30);
        assert!(normal.impairment_score() < 0.1);

        let decline = CognitiveDecline::memory_dominant();
        assert!(decline.memory_impairment > 0.5);
    }

    #[test]
    fn test_biomarker_signature() {
        let mut biomarkers = BiomarkerChanges::new_normal();
        assert!(!biomarkers.alzheimers_signature());

        biomarkers.csf_abeta42_pg_ml = 500.0;
        biomarkers.csf_ptau_pg_ml = 70.0;
        biomarkers.fdg_pet_hypometabolism = 0.3;
        assert!(biomarkers.alzheimers_signature());
    }

    #[test]
    fn test_mitochondrial_dysfunction() {
        let mut mito = MitochondrialDysfunction::new_normal();
        let baseline = mito.bioenergetic_capacity();

        mito.neurodegenerative_dysfunction();
        assert!(mito.bioenergetic_capacity() < baseline);
    }

    #[test]
    fn test_neuroinflammation() {
        let mut inflam = Neuroinflammation::new_normal();
        let baseline = inflam.inflammation_score();

        inflam.chronic_activation();
        assert!(inflam.inflammation_score() > baseline);
    }

    #[test]
    fn test_synaptic_dysfunction() {
        let mut synapse = SynapticDysfunction::new_normal();
        let baseline = synapse.synaptic_function_score();

        synapse.neurodegenerative_synapse_loss();
        assert!(synapse.synaptic_function_score() < baseline);
    }

    #[test]
    fn test_therapeutic_targets() {
        let ad = NeurodegenerativeDisease::new_alzheimers();
        let targets = ad.therapeutic_target_identification();
        assert!(!targets.is_empty());
    }

    #[test]
    fn test_clinical_staging() {
        let decline = CognitiveDecline::new_normal();
        assert_eq!(
            decline.dementia_severity(),
            ClinicalStage::MildCognitiveImpairment
        );
    }
}

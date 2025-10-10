use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neurotransmitter {
    pub name: String,
    pub neurotransmitter_type: NeurotransmitterType,
    pub concentration_nm: f64,
    pub synthesis_location: BrainRegion,
    pub primary_receptors: Vec<ReceptorSubtype>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeurotransmitterType {
    Monoamine,
    AminoAcid,
    Peptide,
    Purine,
    Gaseous,
    Lipid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrainRegion {
    Substantia,
    VentralTegmentalArea,
    RapheNuclei,
    LocusCoeruleus,
    BasalForebrain,
    Hypothalamus,
    Hippocampus,
    Cortex,
    Striatum,
    Cerebellum,
    SpinalCord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReceptorSubtype {
    Dopamine(DopamineReceptor),
    Serotonin(SerotoninReceptor),
    Norepinephrine(AdrenergicReceptor),
    Acetylcholine(CholinergicReceptor),
    GABA(GABAReceptor),
    Glutamate(GlutamateReceptor),
    Opioid(OpioidReceptor),
    Cannabinoid(CannabinoidReceptor),
    Histamine(HistamineReceptor),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DopamineReceptor {
    D1,
    D2,
    D3,
    D4,
    D5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerotoninReceptor {
    R5HT1A,
    R5HT1B,
    R5HT1D,
    R5HT2A,
    R5HT2B,
    R5HT2C,
    R5HT3,
    R5HT4,
    R5HT5,
    R5HT6,
    R5HT7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdrenergicReceptor {
    Alpha1,
    Alpha2,
    Beta1,
    Beta2,
    Beta3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CholinergicReceptor {
    Nicotinic,
    M1,
    M2,
    M3,
    M4,
    M5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GABAReceptor {
    GABAA,
    GABAB,
    GABAC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GlutamateReceptor {
    NMDA,
    AMPA,
    Kainate,
    MGluR1,
    MGluR2,
    MGluR3,
    MGluR4,
    MGluR5,
    MGluR6,
    MGluR7,
    MGluR8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpioidReceptor {
    Mu,
    Delta,
    Kappa,
    Nociceptin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CannabinoidReceptor {
    CB1,
    CB2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistamineReceptor {
    H1,
    H2,
    H3,
    H4,
}

impl Neurotransmitter {
    pub fn new_dopamine() -> Self {
        Self {
            name: "Dopamine".to_string(),
            neurotransmitter_type: NeurotransmitterType::Monoamine,
            concentration_nm: 10.0,
            synthesis_location: BrainRegion::Substantia,
            primary_receptors: vec![
                ReceptorSubtype::Dopamine(DopamineReceptor::D1),
                ReceptorSubtype::Dopamine(DopamineReceptor::D2),
            ],
        }
    }

    pub fn new_serotonin() -> Self {
        Self {
            name: "Serotonin".to_string(),
            neurotransmitter_type: NeurotransmitterType::Monoamine,
            concentration_nm: 5.0,
            synthesis_location: BrainRegion::RapheNuclei,
            primary_receptors: vec![
                ReceptorSubtype::Serotonin(SerotoninReceptor::R5HT1A),
                ReceptorSubtype::Serotonin(SerotoninReceptor::R5HT2A),
            ],
        }
    }

    pub fn new_norepinephrine() -> Self {
        Self {
            name: "Norepinephrine".to_string(),
            neurotransmitter_type: NeurotransmitterType::Monoamine,
            concentration_nm: 8.0,
            synthesis_location: BrainRegion::LocusCoeruleus,
            primary_receptors: vec![
                ReceptorSubtype::Norepinephrine(AdrenergicReceptor::Alpha1),
                ReceptorSubtype::Norepinephrine(AdrenergicReceptor::Beta1),
            ],
        }
    }

    pub fn new_acetylcholine() -> Self {
        Self {
            name: "Acetylcholine".to_string(),
            neurotransmitter_type: NeurotransmitterType::AminoAcid,
            concentration_nm: 50.0,
            synthesis_location: BrainRegion::BasalForebrain,
            primary_receptors: vec![
                ReceptorSubtype::Acetylcholine(CholinergicReceptor::Nicotinic),
                ReceptorSubtype::Acetylcholine(CholinergicReceptor::M1),
            ],
        }
    }

    pub fn new_gaba() -> Self {
        Self {
            name: "GABA".to_string(),
            neurotransmitter_type: NeurotransmitterType::AminoAcid,
            concentration_nm: 100.0,
            synthesis_location: BrainRegion::Cortex,
            primary_receptors: vec![
                ReceptorSubtype::GABA(GABAReceptor::GABAA),
                ReceptorSubtype::GABA(GABAReceptor::GABAB),
            ],
        }
    }

    pub fn new_glutamate() -> Self {
        Self {
            name: "Glutamate".to_string(),
            neurotransmitter_type: NeurotransmitterType::AminoAcid,
            concentration_nm: 200.0,
            synthesis_location: BrainRegion::Cortex,
            primary_receptors: vec![
                ReceptorSubtype::Glutamate(GlutamateReceptor::NMDA),
                ReceptorSubtype::Glutamate(GlutamateReceptor::AMPA),
            ],
        }
    }

    pub fn new_endorphin() -> Self {
        Self {
            name: "Beta-Endorphin".to_string(),
            neurotransmitter_type: NeurotransmitterType::Peptide,
            concentration_nm: 0.5,
            synthesis_location: BrainRegion::Hypothalamus,
            primary_receptors: vec![
                ReceptorSubtype::Opioid(OpioidReceptor::Mu),
                ReceptorSubtype::Opioid(OpioidReceptor::Delta),
            ],
        }
    }

    pub fn new_enkephalin() -> Self {
        Self {
            name: "Met-Enkephalin".to_string(),
            neurotransmitter_type: NeurotransmitterType::Peptide,
            concentration_nm: 1.0,
            synthesis_location: BrainRegion::Striatum,
            primary_receptors: vec![ReceptorSubtype::Opioid(OpioidReceptor::Delta)],
        }
    }

    pub fn new_substance_p() -> Self {
        Self {
            name: "Substance P".to_string(),
            neurotransmitter_type: NeurotransmitterType::Peptide,
            concentration_nm: 2.0,
            synthesis_location: BrainRegion::SpinalCord,
            primary_receptors: vec![],
        }
    }

    pub fn new_histamine() -> Self {
        Self {
            name: "Histamine".to_string(),
            neurotransmitter_type: NeurotransmitterType::Monoamine,
            concentration_nm: 3.0,
            synthesis_location: BrainRegion::Hypothalamus,
            primary_receptors: vec![
                ReceptorSubtype::Histamine(HistamineReceptor::H1),
                ReceptorSubtype::Histamine(HistamineReceptor::H3),
            ],
        }
    }

    pub fn new_anandamide() -> Self {
        Self {
            name: "Anandamide".to_string(),
            neurotransmitter_type: NeurotransmitterType::Lipid,
            concentration_nm: 0.1,
            synthesis_location: BrainRegion::Hippocampus,
            primary_receptors: vec![ReceptorSubtype::Cannabinoid(CannabinoidReceptor::CB1)],
        }
    }

    pub fn new_nitric_oxide() -> Self {
        Self {
            name: "Nitric Oxide".to_string(),
            neurotransmitter_type: NeurotransmitterType::Gaseous,
            concentration_nm: 0.05,
            synthesis_location: BrainRegion::Cortex,
            primary_receptors: vec![],
        }
    }

    pub fn is_excitatory(&self) -> bool {
        matches!(
            self.name.as_str(),
            "Glutamate" | "Acetylcholine" | "Norepinephrine"
        )
    }

    pub fn is_inhibitory(&self) -> bool {
        matches!(self.name.as_str(), "GABA")
    }

    pub fn is_modulatory(&self) -> bool {
        matches!(
            self.name.as_str(),
            "Dopamine" | "Serotonin" | "Histamine" | "Anandamide"
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuropeptide {
    pub name: String,
    pub sequence_length: usize,
    pub function: NeuropeptideFunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuropeptideFunction {
    PainModulation,
    AppetiteRegulation,
    StressResponse,
    SocialBonding,
    Reward,
    Memory,
}

impl Neuropeptide {
    pub fn new_oxytocin() -> Self {
        Self {
            name: "Oxytocin".to_string(),
            sequence_length: 9,
            function: NeuropeptideFunction::SocialBonding,
        }
    }

    pub fn new_vasopressin() -> Self {
        Self {
            name: "Vasopressin".to_string(),
            sequence_length: 9,
            function: NeuropeptideFunction::StressResponse,
        }
    }

    pub fn new_neuropeptide_y() -> Self {
        Self {
            name: "Neuropeptide Y".to_string(),
            sequence_length: 36,
            function: NeuropeptideFunction::AppetiteRegulation,
        }
    }

    pub fn new_orexin() -> Self {
        Self {
            name: "Orexin A".to_string(),
            sequence_length: 33,
            function: NeuropeptideFunction::AppetiteRegulation,
        }
    }

    pub fn new_dynorphin() -> Self {
        Self {
            name: "Dynorphin".to_string(),
            sequence_length: 17,
            function: NeuropeptideFunction::PainModulation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neurotransmitter_creation() {
        let dopamine = Neurotransmitter::new_dopamine();
        assert_eq!(dopamine.name, "Dopamine");
        assert!(dopamine.is_modulatory());
    }

    #[test]
    fn test_excitatory_inhibitory() {
        let glutamate = Neurotransmitter::new_glutamate();
        let gaba = Neurotransmitter::new_gaba();

        assert!(glutamate.is_excitatory());
        assert!(gaba.is_inhibitory());
    }

    #[test]
    fn test_neuropeptides() {
        let oxytocin = Neuropeptide::new_oxytocin();
        assert_eq!(oxytocin.sequence_length, 9);
        assert_eq!(oxytocin.function, NeuropeptideFunction::SocialBonding);
    }
}

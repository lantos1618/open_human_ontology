use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalingCascade {
    pub pathway: SignalingPathwayType,
    pub receptors: Vec<Receptor>,
    pub second_messengers: Vec<SecondMessenger>,
    pub kinases: Vec<Kinase>,
    pub phosphatases: Vec<Phosphatase>,
    pub transcription_factors: Vec<TranscriptionFactor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignalingPathwayType {
    GProteinCoupled,
    ReceptorTyrosineKinase,
    TGFBeta,
    Wnt,
    Hedgehog,
    Notch,
    JAKStat,
    MAPK,
    PI3KAkt,
    NFKB,
    CalciumCalmodulin,
    CyclicNucleotide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receptor {
    pub name: String,
    pub receptor_type: ReceptorType,
    pub location: CellularLocation,
    pub ligands: Vec<String>,
    pub activated: bool,
    pub sensitivity_fold: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReceptorType {
    GPCR,
    RTK,
    NuclearReceptor,
    LigandGatedIonChannel,
    IntegrinReceptor,
    CytokineReceptor,
    TollLikeReceptor,
    NotchReceptor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellularLocation {
    PlasmaMembrane,
    Cytoplasm,
    Nucleus,
    Mitochondria,
    EndoplasmicReticulum,
    GolgiApparatus,
    Lysosome,
    Peroxisome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondMessenger {
    pub messenger_type: SecondMessengerType,
    pub concentration_um: f64,
    pub half_life_seconds: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecondMessengerType {
    CyclicAMP,
    CyclicGMP,
    CalciumIon,
    InositolTriphosphate,
    Diacylglycerol,
    NitricOxide,
    CarbonMonoxide,
    Ceramide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kinase {
    pub name: String,
    pub kinase_class: KinaseClass,
    pub substrates: Vec<String>,
    pub activity_level: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KinaseClass {
    SerThrKinase,
    TyrKinase,
    DualSpecificityKinase,
    HistidineKinase,
    LipidKinase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phosphatase {
    pub name: String,
    pub phosphatase_type: PhosphataseType,
    pub targets: Vec<String>,
    pub activity_level: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhosphataseType {
    ProteinPhosphatase1,
    ProteinPhosphatase2A,
    ProteinPhosphatase2B,
    DualSpecificityPhosphatase,
    ProteinTyrosinePhosphatase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionFactor {
    pub name: String,
    pub family: TranscriptionFactorFamily,
    pub dna_binding_domains: Vec<String>,
    pub target_genes: Vec<String>,
    pub activation_state: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TranscriptionFactorFamily {
    BasicHelixLoopHelix,
    BasicLeucineZipper,
    ZincFinger,
    HelixTurnHelix,
    Homeobox,
    NuclearReceptor,
    STAT,
    NFKappaB,
    AP1,
    CREB,
    MYC,
    P53,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellCellCommunication {
    pub communication_type: CommunicationType,
    pub molecules: Vec<SignalingMolecule>,
    pub distance_um: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommunicationType {
    Autocrine,
    Paracrine,
    Endocrine,
    Juxtacrine,
    GapJunction,
    Synaptic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalingMolecule {
    pub name: String,
    pub molecule_type: SignalingMoleculeType,
    pub molecular_weight_da: f64,
    pub concentration_nm: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignalingMoleculeType {
    Hormone,
    Cytokine,
    GrowthFactor,
    Chemokine,
    Neurotransmitter,
    Neuropeptide,
    Lipid,
    GaseousSignal,
}

impl SignalingCascade {
    pub fn new(pathway: SignalingPathwayType) -> Self {
        Self {
            pathway,
            receptors: Vec::new(),
            second_messengers: Vec::new(),
            kinases: Vec::new(),
            phosphatases: Vec::new(),
            transcription_factors: Vec::new(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.receptors.iter().any(|r| r.activated)
    }

    pub fn signal_strength(&self) -> f64 {
        let kinase_activity: f64 = self.kinases.iter().map(|k| k.activity_level).sum();
        let phosphatase_activity: f64 = self.phosphatases.iter().map(|p| p.activity_level).sum();

        if phosphatase_activity > 0.0 {
            kinase_activity / phosphatase_activity
        } else {
            kinase_activity
        }
    }

    pub fn activated_transcription_factors(&self) -> Vec<&TranscriptionFactor> {
        self.transcription_factors.iter()
            .filter(|tf| tf.activation_state)
            .collect()
    }
}

impl Receptor {
    pub fn new(name: String, receptor_type: ReceptorType, location: CellularLocation) -> Self {
        Self {
            name,
            receptor_type,
            location,
            ligands: Vec::new(),
            activated: false,
            sensitivity_fold: 1.0,
        }
    }

    pub fn bind_ligand(&mut self, ligand: String) {
        self.ligands.push(ligand);
        self.activated = true;
    }

    pub fn unbind_ligands(&mut self) {
        self.ligands.clear();
        self.activated = false;
    }

    pub fn effective_sensitivity(&self) -> f64 {
        if self.activated {
            1.0 / self.sensitivity_fold
        } else {
            1.0
        }
    }
}

impl SecondMessenger {
    pub fn new(messenger_type: SecondMessengerType, concentration_um: f64) -> Self {
        let half_life_seconds = match messenger_type {
            SecondMessengerType::CyclicAMP => 1.0,
            SecondMessengerType::CyclicGMP => 2.0,
            SecondMessengerType::CalciumIon => 0.5,
            SecondMessengerType::InositolTriphosphate => 0.3,
            SecondMessengerType::Diacylglycerol => 5.0,
            SecondMessengerType::NitricOxide => 0.1,
            SecondMessengerType::CarbonMonoxide => 10.0,
            SecondMessengerType::Ceramide => 60.0,
        };

        Self {
            messenger_type,
            concentration_um,
            half_life_seconds,
        }
    }

    pub fn concentration_after_time(&self, time_seconds: f64) -> f64 {
        self.concentration_um * 0.5_f64.powf(time_seconds / self.half_life_seconds)
    }

    pub fn is_elevated(&self) -> bool {
        let basal_level = match self.messenger_type {
            SecondMessengerType::CyclicAMP => 0.1,
            SecondMessengerType::CyclicGMP => 0.05,
            SecondMessengerType::CalciumIon => 0.0001,
            SecondMessengerType::InositolTriphosphate => 0.01,
            SecondMessengerType::Diacylglycerol => 0.1,
            SecondMessengerType::NitricOxide => 0.0001,
            SecondMessengerType::CarbonMonoxide => 0.0001,
            SecondMessengerType::Ceramide => 0.01,
        };

        self.concentration_um > basal_level * 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signaling_cascade() {
        let cascade = SignalingCascade::new(SignalingPathwayType::MAPK);
        assert_eq!(cascade.pathway, SignalingPathwayType::MAPK);
        assert!(!cascade.is_active());
    }

    #[test]
    fn test_receptor_activation() {
        let mut receptor = Receptor::new(
            "EGFR".to_string(),
            ReceptorType::RTK,
            CellularLocation::PlasmaMembrane,
        );

        assert!(!receptor.activated);

        receptor.bind_ligand("EGF".to_string());
        assert!(receptor.activated);
        assert_eq!(receptor.ligands.len(), 1);

        receptor.unbind_ligands();
        assert!(!receptor.activated);
        assert_eq!(receptor.ligands.len(), 0);
    }

    #[test]
    fn test_second_messenger_decay() {
        let camp = SecondMessenger::new(SecondMessengerType::CyclicAMP, 1.0);

        let concentration_after_1s = camp.concentration_after_time(1.0);
        assert!((concentration_after_1s - 0.5).abs() < 0.01);

        let concentration_after_2s = camp.concentration_after_time(2.0);
        assert!((concentration_after_2s - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_second_messenger_elevation() {
        let normal_camp = SecondMessenger::new(SecondMessengerType::CyclicAMP, 0.1);
        assert!(!normal_camp.is_elevated());

        let elevated_camp = SecondMessenger::new(SecondMessengerType::CyclicAMP, 0.5);
        assert!(elevated_camp.is_elevated());
    }

    #[test]
    fn test_signal_strength() {
        let mut cascade = SignalingCascade::new(SignalingPathwayType::PI3KAkt);

        cascade.kinases.push(Kinase {
            name: "AKT1".to_string(),
            kinase_class: KinaseClass::SerThrKinase,
            substrates: vec!["GSK3B".to_string()],
            activity_level: 2.0,
        });

        cascade.phosphatases.push(Phosphatase {
            name: "PTEN".to_string(),
            phosphatase_type: PhosphataseType::ProteinTyrosinePhosphatase,
            targets: vec!["PIP3".to_string()],
            activity_level: 1.0,
        });

        let strength = cascade.signal_strength();
        assert_eq!(strength, 2.0);
    }
}

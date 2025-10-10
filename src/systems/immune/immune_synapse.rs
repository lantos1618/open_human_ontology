use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneSynapse {
    pub synapse_type: SynapseType,
    pub effector_cell: EffectorCell,
    pub target_cell: TargetCell,
    pub contact_area_um2: f64,
    pub formation_time_seconds: f64,
    pub signaling_molecules: Vec<SignalingMolecule>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SynapseType {
    CTLTarget,
    ThelpB,
    ThelpMacrophage,
    NKTarget,
    BcellAPC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectorCell {
    pub cell_type: EffectorCellType,
    pub activation_state: ActivationState,
    pub tcr_specificity: Option<String>,
    pub bcr_specificity: Option<String>,
    pub mhc_restriction: Option<MHCRestriction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectorCellType {
    Cd8Tcell,
    Cd4Tcell,
    Bcell,
    NKCell,
    Macrophage,
    DendriticCell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActivationState {
    Naive,
    Primed,
    Activated,
    Effector,
    Memory,
    Exhausted,
    Anergic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MHCRestriction {
    MhcI,
    MhcIi,
    NonRestricted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetCell {
    pub cell_type: TargetCellType,
    pub antigen_presentation: AntigenPresentation,
    pub stressed: bool,
    pub infected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetCellType {
    InfectedCell,
    TumorCell,
    Bcell,
    Macrophage,
    DendriticCell,
    NormalCell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntigenPresentation {
    pub mhc_class: MHCClass,
    pub peptide_sequence: String,
    pub costimulatory_molecules: Vec<CostimulatoryMolecule>,
    pub expression_level: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MHCClass {
    ClassI,
    ClassII,
    NonClassical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CostimulatoryMolecule {
    B71Cd80,
    B72Cd86,
    CD40,
    ICOSL,
    OX40L,
    CD70,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalingMolecule {
    pub molecule: ImmuneSignal,
    pub concentration_ng_per_ml: f64,
    pub effect: SignalEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImmuneSignal {
    IL2,
    IL4,
    IL6,
    IL10,
    IL12,
    IFNGamma,
    TNFAlpha,
    Perforin,
    Granzyme,
    FasLigand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignalEffect {
    Activation,
    Proliferation,
    Differentiation,
    Cytotoxicity,
    Immunosuppression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcellActivation {
    pub signal1_tcr_engagement: bool,
    pub signal2_costimulation: bool,
    pub signal3_cytokines: Vec<String>,
    pub activation_threshold_met: bool,
    pub calcium_flux: CalciumFlux,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalciumFlux {
    pub basal_concentration_nm: f64,
    pub peak_concentration_nm: f64,
    pub oscillation_frequency_hz: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CytotoxicGranule {
    pub granule_type: GranuleType,
    pub contents: Vec<CytotoxicMolecule>,
    pub released: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GranuleType {
    Lytic,
    Perforin,
    Granzyme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CytotoxicMolecule {
    pub molecule_type: CytotoxicMoleculeType,
    pub concentration_ug_per_ml: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CytotoxicMoleculeType {
    PerforinProtein,
    GranzymeA,
    GranzymeB,
    Granulysin,
    FasLigand,
    TRAIL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointMolecule {
    pub molecule: CheckpointType,
    pub expression_level: f64,
    pub effect: CheckpointEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckpointType {
    PD1,
    PDL1,
    CTLA4,
    LAG3,
    TIM3,
    TIGIT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckpointEffect {
    Inhibitory,
    Exhaustion,
    Tolerance,
}

impl ImmuneSynapse {
    pub fn new(synapse_type: SynapseType) -> Self {
        Self {
            synapse_type,
            effector_cell: EffectorCell::default(),
            target_cell: TargetCell::default(),
            contact_area_um2: 100.0,
            formation_time_seconds: 60.0,
            signaling_molecules: Vec::new(),
        }
    }

    pub fn is_mature(&self) -> bool {
        self.contact_area_um2 > 50.0
    }

    pub fn is_cytotoxic(&self) -> bool {
        matches!(
            self.synapse_type,
            SynapseType::CTLTarget | SynapseType::NKTarget
        )
    }

    pub fn has_costimulation(&self) -> bool {
        !self
            .target_cell
            .antigen_presentation
            .costimulatory_molecules
            .is_empty()
    }

    pub fn signal_strength(&self) -> f64 {
        let area_factor = self.contact_area_um2 / 100.0;
        let molecular_signal: f64 = self
            .signaling_molecules
            .iter()
            .map(|s| s.concentration_ng_per_ml)
            .sum();

        area_factor * molecular_signal
    }
}

impl EffectorCell {
    pub fn new(cell_type: EffectorCellType) -> Self {
        Self {
            cell_type,
            activation_state: ActivationState::Naive,
            tcr_specificity: None,
            bcr_specificity: None,
            mhc_restriction: None,
        }
    }

    pub fn is_activated(&self) -> bool {
        matches!(
            self.activation_state,
            ActivationState::Activated | ActivationState::Effector
        )
    }

    pub fn can_kill_target(&self) -> bool {
        matches!(
            self.cell_type,
            EffectorCellType::Cd8Tcell | EffectorCellType::NKCell
        ) && self.is_activated()
    }

    pub fn is_functional(&self) -> bool {
        !matches!(
            self.activation_state,
            ActivationState::Exhausted | ActivationState::Anergic
        )
    }
}

impl Default for EffectorCell {
    fn default() -> Self {
        Self::new(EffectorCellType::Cd8Tcell)
    }
}

impl TargetCell {
    pub fn new(cell_type: TargetCellType) -> Self {
        Self {
            cell_type,
            antigen_presentation: AntigenPresentation::default(),
            stressed: false,
            infected: false,
        }
    }

    pub fn is_immunogenic(&self) -> bool {
        self.infected
            || self.stressed
            || matches!(self.cell_type, TargetCellType::TumorCell)
            || self.antigen_presentation.expression_level > 0.5
    }

    pub fn should_be_killed(&self) -> bool {
        matches!(
            self.cell_type,
            TargetCellType::InfectedCell | TargetCellType::TumorCell
        )
    }
}

impl Default for TargetCell {
    fn default() -> Self {
        Self::new(TargetCellType::NormalCell)
    }
}

impl AntigenPresentation {
    pub fn new(mhc_class: MHCClass, peptide: String) -> Self {
        Self {
            mhc_class,
            peptide_sequence: peptide,
            costimulatory_molecules: Vec::new(),
            expression_level: 0.5,
        }
    }

    pub fn has_costimulation(&self) -> bool {
        !self.costimulatory_molecules.is_empty()
    }

    pub fn is_strong_signal(&self) -> bool {
        self.expression_level > 0.7 && self.has_costimulation()
    }
}

impl Default for AntigenPresentation {
    fn default() -> Self {
        Self::new(MHCClass::ClassI, String::new())
    }
}

impl TcellActivation {
    pub fn new() -> Self {
        Self {
            signal1_tcr_engagement: false,
            signal2_costimulation: false,
            signal3_cytokines: Vec::new(),
            activation_threshold_met: false,
            calcium_flux: CalciumFlux::default(),
        }
    }

    pub fn check_activation(&mut self) {
        self.activation_threshold_met = self.signal1_tcr_engagement
            && self.signal2_costimulation
            && !self.signal3_cytokines.is_empty();
    }

    pub fn is_fully_activated(&self) -> bool {
        self.activation_threshold_met
    }

    pub fn has_minimal_signal(&self) -> bool {
        self.signal1_tcr_engagement
    }
}

impl Default for TcellActivation {
    fn default() -> Self {
        Self::new()
    }
}

impl CalciumFlux {
    pub fn default() -> Self {
        Self {
            basal_concentration_nm: 100.0,
            peak_concentration_nm: 100.0,
            oscillation_frequency_hz: 0.0,
        }
    }

    pub fn is_elevated(&self) -> bool {
        self.peak_concentration_nm > self.basal_concentration_nm * 5.0
    }

    pub fn is_oscillating(&self) -> bool {
        self.oscillation_frequency_hz > 0.1
    }

    pub fn fold_increase(&self) -> f64 {
        self.peak_concentration_nm / self.basal_concentration_nm
    }
}

impl CytotoxicGranule {
    pub fn new(granule_type: GranuleType) -> Self {
        Self {
            granule_type,
            contents: Vec::new(),
            released: false,
        }
    }

    pub fn release(&mut self) {
        self.released = true;
    }

    pub fn is_lytic(&self) -> bool {
        matches!(self.granule_type, GranuleType::Lytic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immune_synapse() {
        let synapse = ImmuneSynapse::new(SynapseType::CTLTarget);
        assert!(synapse.is_cytotoxic());
        assert!(synapse.is_mature());
    }

    #[test]
    fn test_effector_cell() {
        let mut cell = EffectorCell::new(EffectorCellType::Cd8Tcell);
        assert!(!cell.is_activated());

        cell.activation_state = ActivationState::Activated;
        assert!(cell.is_activated());
        assert!(cell.can_kill_target());
    }

    #[test]
    fn test_target_cell() {
        let mut target = TargetCell::new(TargetCellType::InfectedCell);
        target.infected = true;

        assert!(target.is_immunogenic());
        assert!(target.should_be_killed());
    }

    #[test]
    fn test_tcell_activation() {
        let mut activation = TcellActivation::new();
        assert!(!activation.is_fully_activated());

        activation.signal1_tcr_engagement = true;
        activation.signal2_costimulation = true;
        activation.signal3_cytokines.push("IL-2".to_string());
        activation.check_activation();

        assert!(activation.is_fully_activated());
    }

    #[test]
    fn test_calcium_flux() {
        let mut flux = CalciumFlux::default();
        flux.peak_concentration_nm = 1000.0;

        assert!(flux.is_elevated());
        assert_eq!(flux.fold_increase(), 10.0);
    }

    #[test]
    fn test_cytotoxic_granule() {
        let mut granule = CytotoxicGranule::new(GranuleType::Lytic);
        assert!(granule.is_lytic());
        assert!(!granule.released);

        granule.release();
        assert!(granule.released);
    }
}

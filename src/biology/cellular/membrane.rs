use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellMembrane {
    pub lipid_bilayer: LipidBilayer,
    pub membrane_proteins: Vec<MembraneProtein>,
    pub cholesterol_content: f64,
    pub fluidity: f64,
    pub membrane_potential_mv: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidBilayer {
    pub phospholipids: HashMap<PhospholipidType, f64>,
    pub glycolipids: f64,
    pub sphingolipids: f64,
    pub lipid_rafts: Vec<LipidRaft>,
    pub asymmetry_index: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhospholipidType {
    Phosphatidylcholine,
    Phosphatidylethanolamine,
    Phosphatidylserine,
    Phosphatidylinositol,
    Phosphatidylinositol_4_5_bisphosphate,
    Cardiolipin,
    Sphingomyelin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidRaft {
    pub name: String,
    pub area_nm2: f64,
    pub cholesterol_enriched: bool,
    pub associated_proteins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembraneProtein {
    pub name: String,
    pub protein_type: MembraneProteinType,
    pub topology: MembraneTopology,
    pub function: ProteinFunction,
    pub copy_number: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MembraneProteinType {
    IntegralMonotopic,
    IntegralBitopic,
    IntegralPolytopic,
    PeripheralIntracellular,
    PeripheralExtracellular,
    LipidAnchored,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembraneTopology {
    pub transmembrane_domains: usize,
    pub extracellular_loops: usize,
    pub intracellular_loops: usize,
    pub n_terminus_location: TerminusLocation,
    pub c_terminus_location: TerminusLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerminusLocation {
    Extracellular,
    Intracellular,
    Lumen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProteinFunction {
    Transporter,
    IonChannel,
    Receptor,
    Enzyme,
    Adhesion,
    Structural,
    Recognition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembraneTransport {
    pub transport_type: TransportType,
    pub substrate: String,
    pub rate_molecules_per_second: f64,
    pub km_um: Option<f64>,
    pub vmax_molecules_per_second: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportType {
    PassiveDiffusion,
    FacilitatedDiffusion,
    PrimaryActiveTransport,
    SecondaryActiveTransport,
    GroupTranslocation,
    Endocytosis,
    Exocytosis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonChannel {
    pub name: String,
    pub ion_selectivity: IonSelectivity,
    pub gating_mechanism: GatingMechanism,
    pub conductance_ps: f64,
    pub open_probability: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IonSelectivity {
    Sodium,
    Potassium,
    Calcium,
    Chloride,
    Nonselective,
    SelectiveMonovalent,
    SelectiveDivalent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GatingMechanism {
    VoltageGated,
    LigandGated,
    MechanicallyGated,
    TemperatureGated,
    LightGated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembraneReceptor {
    pub name: String,
    pub receptor_class: ReceptorClass,
    pub ligand_binding_site: BindingSite,
    pub downstream_signaling: Vec<String>,
    pub desensitization_rate: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReceptorClass {
    IonotropicReceptor,
    MetabotropicReceptor,
    EnzymeLinkedReceptor,
    IntegrinReceptor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingSite {
    pub kd_nm: f64,
    pub cooperativity_factor: f64,
    pub allosteric_sites: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembraneRaft {
    pub size_nm: f64,
    pub lipid_composition: HashMap<String, f64>,
    pub protein_clusters: Vec<ProteinCluster>,
    pub stability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinCluster {
    pub proteins: Vec<String>,
    pub cluster_size: usize,
    pub diffusion_coefficient_cm2_s: f64,
}

impl CellMembrane {
    pub fn new() -> Self {
        Self {
            lipid_bilayer: LipidBilayer::new(),
            membrane_proteins: Vec::new(),
            cholesterol_content: 0.3,
            fluidity: 1.0,
            membrane_potential_mv: -70.0,
        }
    }

    pub fn is_polarized(&self) -> bool {
        self.membrane_potential_mv.abs() > 10.0
    }

    pub fn is_depolarized(&self) -> bool {
        self.membrane_potential_mv > -50.0
    }

    pub fn is_hyperpolarized(&self) -> bool {
        self.membrane_potential_mv < -80.0
    }

    pub fn total_protein_count(&self) -> usize {
        self.membrane_proteins.iter()
            .map(|p| p.copy_number)
            .sum()
    }

    pub fn protein_density(&self, membrane_area_um2: f64) -> f64 {
        self.total_protein_count() as f64 / membrane_area_um2
    }

    pub fn fluidity_factor(&self) -> f64 {
        let cholesterol_factor = 1.0 - (self.cholesterol_content - 0.3).abs() * 0.5;
        let temperature_factor = 1.0;

        self.fluidity * cholesterol_factor * temperature_factor
    }
}

impl Default for CellMembrane {
    fn default() -> Self {
        Self::new()
    }
}

impl LipidBilayer {
    pub fn new() -> Self {
        let mut phospholipids = HashMap::new();
        phospholipids.insert(PhospholipidType::Phosphatidylcholine, 0.5);
        phospholipids.insert(PhospholipidType::Phosphatidylethanolamine, 0.3);
        phospholipids.insert(PhospholipidType::Phosphatidylserine, 0.1);
        phospholipids.insert(PhospholipidType::Phosphatidylinositol, 0.1);

        Self {
            phospholipids,
            glycolipids: 0.05,
            sphingolipids: 0.15,
            lipid_rafts: Vec::new(),
            asymmetry_index: 0.8,
        }
    }

    pub fn total_lipid_content(&self) -> f64 {
        let phospholipid_total: f64 = self.phospholipids.values().sum();
        phospholipid_total + self.glycolipids + self.sphingolipids
    }

    pub fn is_asymmetric(&self) -> bool {
        self.asymmetry_index > 0.5
    }
}

impl Default for LipidBilayer {
    fn default() -> Self {
        Self::new()
    }
}

impl MembraneProtein {
    pub fn new(name: String, protein_type: MembraneProteinType) -> Self {
        Self {
            name,
            protein_type,
            topology: MembraneTopology {
                transmembrane_domains: 1,
                extracellular_loops: 0,
                intracellular_loops: 0,
                n_terminus_location: TerminusLocation::Extracellular,
                c_terminus_location: TerminusLocation::Intracellular,
            },
            function: ProteinFunction::Structural,
            copy_number: 1,
        }
    }

    pub fn is_polytopic(&self) -> bool {
        matches!(self.protein_type, MembraneProteinType::IntegralPolytopic)
    }

    pub fn is_integral(&self) -> bool {
        matches!(self.protein_type,
            MembraneProteinType::IntegralMonotopic |
            MembraneProteinType::IntegralBitopic |
            MembraneProteinType::IntegralPolytopic
        )
    }
}

impl IonChannel {
    pub fn new(name: String, ion_selectivity: IonSelectivity) -> Self {
        Self {
            name,
            ion_selectivity,
            gating_mechanism: GatingMechanism::VoltageGated,
            conductance_ps: 10.0,
            open_probability: 0.1,
        }
    }

    pub fn is_open(&self) -> bool {
        rand::random::<f64>() < self.open_probability
    }

    pub fn current_at_voltage(&self, voltage_mv: f64, reversal_potential_mv: f64) -> f64 {
        if self.is_open() {
            self.conductance_ps * (voltage_mv - reversal_potential_mv) * 1e-12
        } else {
            0.0
        }
    }

    pub fn mean_current(&self, voltage_mv: f64, reversal_potential_mv: f64) -> f64 {
        self.conductance_ps * self.open_probability * (voltage_mv - reversal_potential_mv) * 1e-12
    }
}

impl MembraneTransport {
    pub fn new(transport_type: TransportType, substrate: String) -> Self {
        Self {
            transport_type,
            substrate,
            rate_molecules_per_second: 100.0,
            km_um: None,
            vmax_molecules_per_second: None,
        }
    }

    pub fn is_active_transport(&self) -> bool {
        matches!(self.transport_type,
            TransportType::PrimaryActiveTransport |
            TransportType::SecondaryActiveTransport
        )
    }

    pub fn michaelis_menten_rate(&self, substrate_concentration_um: f64) -> Option<f64> {
        if let (Some(km), Some(vmax)) = (self.km_um, self.vmax_molecules_per_second) {
            Some((vmax * substrate_concentration_um) / (km + substrate_concentration_um))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_membrane() {
        let membrane = CellMembrane::new();
        assert!(membrane.is_polarized());
        assert!(!membrane.is_depolarized());
    }

    #[test]
    fn test_depolarization() {
        let mut membrane = CellMembrane::new();
        membrane.membrane_potential_mv = -40.0;
        assert!(membrane.is_depolarized());
    }

    #[test]
    fn test_hyperpolarization() {
        let mut membrane = CellMembrane::new();
        membrane.membrane_potential_mv = -90.0;
        assert!(membrane.is_hyperpolarized());
    }

    #[test]
    fn test_lipid_bilayer() {
        let bilayer = LipidBilayer::new();
        assert!(bilayer.total_lipid_content() > 0.0);
        assert!(bilayer.is_asymmetric());
    }

    #[test]
    fn test_membrane_protein() {
        let protein = MembraneProtein::new(
            "Test Protein".to_string(),
            MembraneProteinType::IntegralPolytopic,
        );

        assert!(protein.is_polytopic());
        assert!(protein.is_integral());
    }

    #[test]
    fn test_ion_channel() {
        let channel = IonChannel::new(
            "Sodium Channel".to_string(),
            IonSelectivity::Sodium,
        );

        assert_eq!(channel.ion_selectivity, IonSelectivity::Sodium);
        assert_eq!(channel.gating_mechanism, GatingMechanism::VoltageGated);
    }

    #[test]
    fn test_membrane_transport() {
        let transport = MembraneTransport::new(
            TransportType::PrimaryActiveTransport,
            "ATP".to_string(),
        );

        assert!(transport.is_active_transport());
    }

    #[test]
    fn test_passive_transport() {
        let transport = MembraneTransport::new(
            TransportType::PassiveDiffusion,
            "Oxygen".to_string(),
        );

        assert!(!transport.is_active_transport());
    }
}

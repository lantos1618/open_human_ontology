use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinSynthesis {
    pub transcription: Transcription,
    pub translation: Translation,
    pub post_translational_modifications: Vec<PostTranslationalModification>,
    pub protein_folding: ProteinFolding,
    pub degradation: ProteinDegradation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcription {
    pub gene_name: String,
    pub promoter_active: bool,
    pub rna_polymerase_rate: f64,
    pub enhancers: Vec<Enhancer>,
    pub silencers: Vec<Silencer>,
    pub chromatin_state: ChromatinState,
    pub mrna_produced: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChromatinState {
    Euchromatin,
    Heterochromatin,
    FaculativeHeterochromatin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enhancer {
    pub name: String,
    pub transcription_factors: Vec<String>,
    pub distance_from_promoter_bp: usize,
    pub fold_activation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Silencer {
    pub name: String,
    pub repressor_proteins: Vec<String>,
    pub fold_repression: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub mrna_id: String,
    pub ribosome_count: usize,
    pub translation_rate: f64,
    pub kozak_sequence_strength: f64,
    pub ires_present: bool,
    pub utr5_length: usize,
    pub utr3_regulatory_elements: Vec<UTRElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTRElement {
    pub element_type: UTRElementType,
    pub position: usize,
    pub effect: RegulatoryEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UTRElementType {
    IronResponseElement,
    MicroRNABindingSite,
    AURichElement,
    UpstreamORF,
    InternalRibosomalEntrysite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegulatoryEffect {
    Activation,
    Repression,
    Stabilization,
    Destabilization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTranslationalModification {
    pub modification_type: PTMType,
    pub site: AminoAcidSite,
    pub enzyme: String,
    pub reversible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PTMType {
    Phosphorylation,
    Acetylation,
    Methylation,
    Ubiquitination,
    SUMOylation,
    Glycosylation,
    Palmitoylation,
    Myristoylation,
    Hydroxylation,
    Nitrosylation,
    AdpRibosylation,
    Neddylation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AminoAcidSite {
    pub residue: char,
    pub position: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinFolding {
    pub chaperones: Vec<Chaperone>,
    pub folding_state: FoldingState,
    pub disulfide_bonds: usize,
    pub quaternary_structure: Option<QuaternaryStructure>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoldingState {
    Unfolded,
    PartiallyFolded,
    Folded,
    Misfolded,
    Aggregated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chaperone {
    pub name: String,
    pub chaperone_type: ChaperoneType,
    pub atp_dependent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChaperoneType {
    HSP70,
    HSP90,
    HSP60,
    HSP40,
    SmallHSP,
    BiP,
    Calnexin,
    Calreticulin,
    PDI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuaternaryStructure {
    pub subunit_count: usize,
    pub stoichiometry: String,
    pub interface_area_a2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinDegradation {
    pub pathway: DegradationPathway,
    pub half_life_hours: f64,
    pub degradation_signals: Vec<DegradationSignal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DegradationPathway {
    UbiquitinProteasome,
    AutophagyLysosome,
    CalpainMediated,
    CaspaseMediated,
    UnfoldedProteinResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationSignal {
    pub signal_type: DegradationSignalType,
    pub sequence_motif: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DegradationSignalType {
    DESTBox,
    PESTSequence,
    NEndRule,
    UbiquitinChain,
    OxidativeDamage,
}

impl Transcription {
    pub fn new(gene_name: String) -> Self {
        Self {
            gene_name,
            promoter_active: false,
            rna_polymerase_rate: 20.0,
            enhancers: Vec::new(),
            silencers: Vec::new(),
            chromatin_state: ChromatinState::Euchromatin,
            mrna_produced: 0,
        }
    }

    pub fn transcription_rate(&self) -> f64 {
        if !self.promoter_active {
            return 0.0;
        }

        let chromatin_factor = match self.chromatin_state {
            ChromatinState::Euchromatin => 1.0,
            ChromatinState::FaculativeHeterochromatin => 0.3,
            ChromatinState::Heterochromatin => 0.01,
        };

        let enhancer_factor: f64 = self.enhancers.iter()
            .map(|e| e.fold_activation)
            .product();

        let silencer_factor: f64 = self.silencers.iter()
            .map(|s| 1.0 / s.fold_repression)
            .product();

        self.rna_polymerase_rate * chromatin_factor * enhancer_factor * silencer_factor
    }

    pub fn is_actively_transcribed(&self) -> bool {
        self.transcription_rate() > 1.0
    }
}

impl Translation {
    pub fn new(mrna_id: String) -> Self {
        Self {
            mrna_id,
            ribosome_count: 0,
            translation_rate: 5.0,
            kozak_sequence_strength: 0.8,
            ires_present: false,
            utr5_length: 100,
            utr3_regulatory_elements: Vec::new(),
        }
    }

    pub fn effective_translation_rate(&self) -> f64 {
        let kozak_factor = self.kozak_sequence_strength;
        let ribosome_factor = self.ribosome_count as f64;

        let utr_factor: f64 = self.utr3_regulatory_elements.iter()
            .map(|elem| match elem.effect {
                RegulatoryEffect::Activation => 1.5,
                RegulatoryEffect::Repression => 0.3,
                RegulatoryEffect::Stabilization => 1.2,
                RegulatoryEffect::Destabilization => 0.7,
            })
            .product();

        self.translation_rate * kozak_factor * ribosome_factor * utr_factor
    }

    pub fn proteins_per_minute(&self) -> f64 {
        self.effective_translation_rate() * self.ribosome_count as f64
    }
}

impl ProteinFolding {
    pub fn new() -> Self {
        Self {
            chaperones: Vec::new(),
            folding_state: FoldingState::Unfolded,
            disulfide_bonds: 0,
            quaternary_structure: None,
        }
    }

    pub fn is_functional(&self) -> bool {
        matches!(self.folding_state, FoldingState::Folded)
    }

    pub fn requires_chaperone_assistance(&self) -> bool {
        matches!(self.folding_state,
            FoldingState::Misfolded |
            FoldingState::PartiallyFolded
        )
    }

    pub fn folding_efficiency(&self) -> f64 {
        let chaperone_count = self.chaperones.len() as f64;
        let state_efficiency = match self.folding_state {
            FoldingState::Folded => 1.0,
            FoldingState::PartiallyFolded => 0.5,
            FoldingState::Unfolded => 0.1,
            FoldingState::Misfolded => 0.0,
            FoldingState::Aggregated => 0.0,
        };

        state_efficiency * (1.0 + chaperone_count * 0.1)
    }
}

impl Default for ProteinFolding {
    fn default() -> Self {
        Self::new()
    }
}

impl ProteinDegradation {
    pub fn new(pathway: DegradationPathway, half_life_hours: f64) -> Self {
        Self {
            pathway,
            half_life_hours,
            degradation_signals: Vec::new(),
        }
    }

    pub fn degradation_rate_per_hour(&self) -> f64 {
        0.693 / self.half_life_hours
    }

    pub fn fraction_remaining_after_hours(&self, hours: f64) -> f64 {
        0.5_f64.powf(hours / self.half_life_hours)
    }

    pub fn is_rapidly_degraded(&self) -> bool {
        self.half_life_hours < 2.0
    }

    pub fn is_stable(&self) -> bool {
        self.half_life_hours > 24.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcription_rate() {
        let mut transcription = Transcription::new("TEST_GENE".to_string());
        transcription.promoter_active = true;
        transcription.rna_polymerase_rate = 20.0;

        assert!(transcription.transcription_rate() > 0.0);
        assert!(transcription.is_actively_transcribed());
    }

    #[test]
    fn test_chromatin_effect() {
        let mut transcription = Transcription::new("TEST_GENE".to_string());
        transcription.promoter_active = true;

        transcription.chromatin_state = ChromatinState::Euchromatin;
        let euchromatin_rate = transcription.transcription_rate();

        transcription.chromatin_state = ChromatinState::Heterochromatin;
        let heterochromatin_rate = transcription.transcription_rate();

        assert!(euchromatin_rate > heterochromatin_rate);
    }

    #[test]
    fn test_translation_rate() {
        let mut translation = Translation::new("TEST_MRNA".to_string());
        translation.ribosome_count = 5;
        translation.kozak_sequence_strength = 0.9;

        let rate = translation.effective_translation_rate();
        assert!(rate > 0.0);
    }

    #[test]
    fn test_protein_folding() {
        let folding = ProteinFolding::new();
        assert_eq!(folding.folding_state, FoldingState::Unfolded);
        assert!(!folding.is_functional());

        let mut functional_folding = ProteinFolding::new();
        functional_folding.folding_state = FoldingState::Folded;
        assert!(functional_folding.is_functional());
    }

    #[test]
    fn test_protein_degradation() {
        let degradation = ProteinDegradation::new(
            DegradationPathway::UbiquitinProteasome,
            12.0,
        );

        assert!(!degradation.is_rapidly_degraded());
        assert!(!degradation.is_stable());

        let remaining = degradation.fraction_remaining_after_hours(12.0);
        assert!((remaining - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_ptm() {
        let ptm = PostTranslationalModification {
            modification_type: PTMType::Phosphorylation,
            site: AminoAcidSite {
                residue: 'S',
                position: 473,
            },
            enzyme: "AKT1".to_string(),
            reversible: true,
        };

        assert_eq!(ptm.modification_type, PTMType::Phosphorylation);
        assert!(ptm.reversible);
    }
}

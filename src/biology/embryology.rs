use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbryonicDevelopment {
    pub developmental_stage: DevelopmentalStage,
    pub gestational_age_weeks: f64,
    pub carnegie_stage: Option<u8>,
    pub germ_layers: GermLayers,
    pub organ_systems: Vec<OrganSystemDevelopment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DevelopmentalStage {
    Zygote,
    Cleavage,
    Morula,
    Blastocyst,
    Gastrulation,
    Neurulation,
    Organogenesis,
    FetalPeriod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GermLayers {
    pub ectoderm: EctodermDerivatives,
    pub mesoderm: MesodermDerivatives,
    pub endoderm: EndodermDerivatives,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EctodermDerivatives {
    pub neural_tube: NeuralTubeDevelopment,
    pub neural_crest: NeuralCrestCells,
    pub surface_ectoderm: SurfaceEctodermDerivatives,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralTubeDevelopment {
    pub closure_complete: bool,
    pub brain_vesicles: BrainVesicles,
    pub spinal_cord_segments: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrainVesicles {
    ThreeVesicle,
    FiveVesicle,
    MatureStructures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralCrestCells {
    pub migration_complete: bool,
    pub derivatives: Vec<NeuralCrestDerivative>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NeuralCrestDerivative {
    PeripheralNervousSystem,
    MelanocytesAdrenalMedulla,
    CranialCartilage,
    DentalPapilla,
    EntericNervousSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceEctodermDerivatives {
    pub epidermis_formed: bool,
    pub lens_placode: bool,
    pub otic_placode: bool,
    pub anterior_pituitary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MesodermDerivatives {
    pub axial_mesoderm: AxialMesoderm,
    pub paraxial_mesoderm: ParaxialMesoderm,
    pub intermediate_mesoderm: IntermediateMesoderm,
    pub lateral_plate_mesoderm: LateralPlateMesoderm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxialMesoderm {
    pub notochord_formed: bool,
    pub vertebral_bodies_forming: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParaxialMesoderm {
    pub somite_count: usize,
    pub sclerotome_differentiated: bool,
    pub myotome_differentiated: bool,
    pub dermatome_differentiated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntermediateMesoderm {
    pub pronephros: bool,
    pub mesonephros: bool,
    pub metanephros: bool,
    pub gonadal_ridge_formed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LateralPlateMesoderm {
    pub somatic_layer: SomaticMesoderm,
    pub splanchnic_layer: SplanchnicMesoderm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomaticMesoderm {
    pub body_wall_formed: bool,
    pub limb_buds_formed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplanchnicMesoderm {
    pub heart_tube_formed: bool,
    pub blood_islands_formed: bool,
    pub gut_wall_formed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndodermDerivatives {
    pub foregut: ForegutDerivatives,
    pub midgut: MidgutDerivatives,
    pub hindgut: HindgutDerivatives,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForegutDerivatives {
    pub pharynx: PharyngealArches,
    pub thyroid: bool,
    pub lung_buds: bool,
    pub liver_bud: bool,
    pub pancreas_buds: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharyngealArches {
    pub arch_count: u8,
    pub derivatives_formed: Vec<PharyngealDerivative>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PharyngealDerivative {
    Mandible,
    MiddleEarOssicles,
    Hyoid,
    Thyroid,
    Cricoid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidgutDerivatives {
    pub small_intestine_formed: bool,
    pub cecum_formed: bool,
    pub physiological_herniation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HindgutDerivatives {
    pub colon_formed: bool,
    pub cloaca_divided: bool,
    pub urogenital_sinus: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganSystemDevelopment {
    pub system: OrganSystemType,
    pub developmental_progress: f64,
    pub critical_period: bool,
    pub teratogen_sensitivity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrganSystemType {
    NervousSystem,
    CardiovascularSystem,
    RespiratorySystem,
    DigestiveSystem,
    UrinarySystem,
    ReproductiveSystem,
    MusculoskeletalSystem,
    IntegumentarySystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphogenSignaling {
    pub morphogen_type: MorphogenType,
    pub concentration_gradient: Vec<f64>,
    pub source_location: String,
    pub target_genes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MorphogenType {
    SonicHedgehog,
    Wnt,
    BMP,
    FGF,
    RetinoicAcid,
    Nodal,
    TGFBeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellFate {
    pub cell_type: String,
    pub specification: SpecificationType,
    pub determination_factors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecificationType {
    Autonomous,
    Conditional,
    Syncytial,
}

impl EmbryonicDevelopment {
    pub fn new(gestational_age_weeks: f64) -> Self {
        let stage = if gestational_age_weeks < 1.0 {
            DevelopmentalStage::Zygote
        } else if gestational_age_weeks < 2.0 {
            DevelopmentalStage::Cleavage
        } else if gestational_age_weeks < 3.0 {
            DevelopmentalStage::Gastrulation
        } else if gestational_age_weeks < 4.0 {
            DevelopmentalStage::Neurulation
        } else if gestational_age_weeks < 8.0 {
            DevelopmentalStage::Organogenesis
        } else {
            DevelopmentalStage::FetalPeriod
        };

        Self {
            developmental_stage: stage,
            gestational_age_weeks,
            carnegie_stage: Self::calculate_carnegie_stage(gestational_age_weeks),
            germ_layers: GermLayers::default(),
            organ_systems: Vec::new(),
        }
    }

    fn calculate_carnegie_stage(gestational_age_weeks: f64) -> Option<u8> {
        let days = gestational_age_weeks * 7.0;
        if days < 15.0 {
            None
        } else if days < 21.0 {
            Some(1)
        } else if days < 56.0 {
            Some(((days - 21.0) / 2.0) as u8 + 9)
        } else {
            Some(23)
        }
    }

    pub fn is_embryonic_period(&self) -> bool {
        self.gestational_age_weeks < 8.0
    }

    pub fn is_fetal_period(&self) -> bool {
        self.gestational_age_weeks >= 8.0
    }

    pub fn is_critical_period(&self) -> bool {
        self.gestational_age_weeks >= 3.0 && self.gestational_age_weeks <= 8.0
    }

    pub fn teratogen_vulnerability(&self) -> f64 {
        if self.is_critical_period() {
            1.0
        } else if self.gestational_age_weeks < 2.0 {
            0.3
        } else {
            0.5
        }
    }
}

impl GermLayers {
    pub fn default() -> Self {
        Self {
            ectoderm: EctodermDerivatives::default(),
            mesoderm: MesodermDerivatives::default(),
            endoderm: EndodermDerivatives::default(),
        }
    }
}

impl EctodermDerivatives {
    pub fn default() -> Self {
        Self {
            neural_tube: NeuralTubeDevelopment::default(),
            neural_crest: NeuralCrestCells::default(),
            surface_ectoderm: SurfaceEctodermDerivatives::default(),
        }
    }
}

impl NeuralTubeDevelopment {
    pub fn default() -> Self {
        Self {
            closure_complete: false,
            brain_vesicles: BrainVesicles::ThreeVesicle,
            spinal_cord_segments: 0,
        }
    }
}

impl NeuralCrestCells {
    pub fn default() -> Self {
        Self {
            migration_complete: false,
            derivatives: Vec::new(),
        }
    }
}

impl SurfaceEctodermDerivatives {
    pub fn default() -> Self {
        Self {
            epidermis_formed: false,
            lens_placode: false,
            otic_placode: false,
            anterior_pituitary: false,
        }
    }
}

impl MesodermDerivatives {
    pub fn default() -> Self {
        Self {
            axial_mesoderm: AxialMesoderm::default(),
            paraxial_mesoderm: ParaxialMesoderm::default(),
            intermediate_mesoderm: IntermediateMesoderm::default(),
            lateral_plate_mesoderm: LateralPlateMesoderm::default(),
        }
    }
}

impl AxialMesoderm {
    pub fn default() -> Self {
        Self {
            notochord_formed: false,
            vertebral_bodies_forming: false,
        }
    }
}

impl ParaxialMesoderm {
    pub fn default() -> Self {
        Self {
            somite_count: 0,
            sclerotome_differentiated: false,
            myotome_differentiated: false,
            dermatome_differentiated: false,
        }
    }
}

impl IntermediateMesoderm {
    pub fn default() -> Self {
        Self {
            pronephros: false,
            mesonephros: false,
            metanephros: false,
            gonadal_ridge_formed: false,
        }
    }
}

impl LateralPlateMesoderm {
    pub fn default() -> Self {
        Self {
            somatic_layer: SomaticMesoderm::default(),
            splanchnic_layer: SplanchnicMesoderm::default(),
        }
    }
}

impl SomaticMesoderm {
    pub fn default() -> Self {
        Self {
            body_wall_formed: false,
            limb_buds_formed: false,
        }
    }
}

impl SplanchnicMesoderm {
    pub fn default() -> Self {
        Self {
            heart_tube_formed: false,
            blood_islands_formed: false,
            gut_wall_formed: false,
        }
    }
}

impl EndodermDerivatives {
    pub fn default() -> Self {
        Self {
            foregut: ForegutDerivatives::default(),
            midgut: MidgutDerivatives::default(),
            hindgut: HindgutDerivatives::default(),
        }
    }
}

impl ForegutDerivatives {
    pub fn default() -> Self {
        Self {
            pharynx: PharyngealArches::default(),
            thyroid: false,
            lung_buds: false,
            liver_bud: false,
            pancreas_buds: false,
        }
    }
}

impl PharyngealArches {
    pub fn default() -> Self {
        Self {
            arch_count: 0,
            derivatives_formed: Vec::new(),
        }
    }
}

impl MidgutDerivatives {
    pub fn default() -> Self {
        Self {
            small_intestine_formed: false,
            cecum_formed: false,
            physiological_herniation: false,
        }
    }
}

impl HindgutDerivatives {
    pub fn default() -> Self {
        Self {
            colon_formed: false,
            cloaca_divided: false,
            urogenital_sinus: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embryonic_development_stages() {
        let zygote = EmbryonicDevelopment::new(0.5);
        assert_eq!(zygote.developmental_stage, DevelopmentalStage::Zygote);

        let organogenesis = EmbryonicDevelopment::new(5.0);
        assert_eq!(
            organogenesis.developmental_stage,
            DevelopmentalStage::Organogenesis
        );
        assert!(organogenesis.is_critical_period());

        let fetal = EmbryonicDevelopment::new(12.0);
        assert_eq!(fetal.developmental_stage, DevelopmentalStage::FetalPeriod);
        assert!(fetal.is_fetal_period());
    }

    #[test]
    fn test_teratogen_vulnerability() {
        let early = EmbryonicDevelopment::new(1.0);
        assert!(early.teratogen_vulnerability() < 0.5);

        let critical = EmbryonicDevelopment::new(5.0);
        assert_eq!(critical.teratogen_vulnerability(), 1.0);

        let fetal = EmbryonicDevelopment::new(10.0);
        assert!(fetal.teratogen_vulnerability() < 1.0);
    }

    #[test]
    fn test_carnegie_stages() {
        let dev = EmbryonicDevelopment::new(4.0);
        assert!(dev.carnegie_stage.is_some());
    }
}

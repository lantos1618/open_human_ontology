use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RareDisease {
    LysosomalStorage(LysosomalStorageDisorder),
    Mitochondrial(MitochondrialDisorder),
    Peroxisomal(PeroxisomalDisorder),
    ConnectiveTissue(ConnectiveTissueDisorder),
    MetabolicDefect(MetabolicDefect),
    PrimaryImmunodeficiency(Immunodeficiency),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LysosomalStorageDisorder {
    GaucherDisease,
    TaySachsDisease,
    NiemannPickDisease,
    FabryDisease,
    PompeDisease,
    MucopolysaccharidosisTypeI,
    MucopolysaccharidosisTypeII,
    Krabbe,
    MetachromaticLeukodystrophy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MitochondrialDisorder {
    MELAS,
    MERRF,
    LeighSyndrome,
    KEARNSSayreSyndrome,
    LHON,
    MitochondrialMyopathy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PeroxisomalDisorder {
    ZellwegerSyndrome,
    AdrenoleukodystrophyXLinked,
    RefsumsDisease,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectiveTissueDisorder {
    EhlersDanlosSyndrome,
    MarfanSyndrome,
    LoeysDietzSyndrome,
    OsteogenesisImperfecta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetabolicDefect {
    Phenylketonuria,
    MapleSyrupUrineDisease,
    HomocystinuriaClassical,
    Tyrosinemia,
    GlycogenStorageDiseaseTypeI,
    GlycogenStorageDiseaseTypeII,
    GalactosemiaClassical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Immunodeficiency {
    SevereCombinedImmunodeficiency,
    CommonVariableImmunodeficiency,
    ChronicGranulomatousDisease,
    WiskottAldrichSyndrome,
    DiGeorgeSyndrome,
    SelectiveIgADeficiency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RareDiseaseProfile {
    pub confirmed_diagnoses: Vec<RareDisease>,
    pub suspected_diagnoses: Vec<RareDisease>,
    pub genetic_testing: HashMap<String, GeneticTestResult>,
    pub enzyme_assays: HashMap<String, EnzymeActivity>,
    pub biomarkers: HashMap<String, f64>,
}

impl RareDiseaseProfile {
    pub fn new() -> Self {
        Self {
            confirmed_diagnoses: Vec::new(),
            suspected_diagnoses: Vec::new(),
            genetic_testing: HashMap::new(),
            enzyme_assays: HashMap::new(),
            biomarkers: HashMap::new(),
        }
    }

    pub fn add_confirmed_diagnosis(&mut self, disease: RareDisease) {
        self.confirmed_diagnoses.push(disease);
    }

    pub fn has_treatable_condition(&self) -> bool {
        self.confirmed_diagnoses.iter().any(|d| match d {
            RareDisease::LysosomalStorage(LysosomalStorageDisorder::GaucherDisease) => true,
            RareDisease::LysosomalStorage(LysosomalStorageDisorder::FabryDisease) => true,
            RareDisease::LysosomalStorage(LysosomalStorageDisorder::PompeDisease) => true,
            RareDisease::MetabolicDefect(MetabolicDefect::Phenylketonuria) => true,
            RareDisease::Peroxisomal(PeroxisomalDisorder::AdrenoleukodystrophyXLinked) => true,
            _ => false,
        })
    }
}

impl Default for RareDiseaseProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneticTestResult {
    pub gene: String,
    pub variant: String,
    pub zygosity: Zygosity,
    pub pathogenicity: Pathogenicity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Zygosity {
    Heterozygous,
    Homozygous,
    CompoundHeterozygous,
    Hemizygous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pathogenicity {
    Benign,
    LikelyBenign,
    UncertainSignificance,
    LikelyPathogenic,
    Pathogenic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnzymeActivity {
    pub enzyme_name: String,
    pub activity_percent_of_normal: f64,
    pub reference_range: (f64, f64),
}

impl EnzymeActivity {
    pub fn is_deficient(&self) -> bool {
        self.activity_percent_of_normal < self.reference_range.0
    }

    pub fn severity(&self) -> DeficiencySeverity {
        match self.activity_percent_of_normal {
            a if a < 1.0 => DeficiencySeverity::Complete,
            a if a < 10.0 => DeficiencySeverity::Severe,
            a if a < 30.0 => DeficiencySeverity::Moderate,
            a if a < 50.0 => DeficiencySeverity::Mild,
            _ => DeficiencySeverity::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeficiencySeverity {
    Normal,
    Mild,
    Moderate,
    Severe,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewbornScreeningPanel {
    pub phenylalanine_mg_dl: f64,
    pub galactose_mg_dl: f64,
    pub leucine_umol_l: f64,
    pub biotinidase_activity: f64,
    pub acylcarnitine_profile: HashMap<String, f64>,
}

impl NewbornScreeningPanel {
    pub fn new() -> Self {
        Self {
            phenylalanine_mg_dl: 0.0,
            galactose_mg_dl: 0.0,
            leucine_umol_l: 0.0,
            biotinidase_activity: 100.0,
            acylcarnitine_profile: HashMap::new(),
        }
    }

    pub fn screen_results(&self) -> Vec<String> {
        let mut alerts = Vec::new();

        if self.phenylalanine_mg_dl > 2.0 {
            alerts.push("Elevated phenylalanine - consider PKU".to_string());
        }

        if self.galactose_mg_dl > 7.0 {
            alerts.push("Elevated galactose - consider galactosemia".to_string());
        }

        if self.leucine_umol_l > 300.0 {
            alerts.push("Elevated leucine - consider MSUD".to_string());
        }

        if self.biotinidase_activity < 30.0 {
            alerts.push("Low biotinidase activity".to_string());
        }

        alerts
    }
}

impl Default for NewbornScreeningPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanDrugTherapy {
    pub drug_name: String,
    pub indication: RareDisease,
    pub mechanism: String,
    pub availability: DrugAvailability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DrugAvailability {
    FDAApproved,
    EMAApproved,
    CompassionateUse,
    ClinicalTrial,
    NotAvailable,
}

impl OrphanDrugTherapy {
    pub fn new(drug_name: String, indication: RareDisease) -> Self {
        Self {
            drug_name,
            indication,
            mechanism: String::new(),
            availability: DrugAvailability::NotAvailable,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalTrial {
    pub nct_id: String,
    pub title: String,
    pub conditions: Vec<RareDisease>,
    pub phase: ClinicalPhase,
    pub status: TrialStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClinicalPhase {
    EarlyPhase1,
    Phase1,
    Phase2,
    Phase3,
    Phase4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrialStatus {
    Recruiting,
    Active,
    Completed,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneTherapy {
    pub target_gene: String,
    pub vector_type: VectorType,
    pub delivery_route: DeliveryRoute,
    pub indicated_conditions: Vec<RareDisease>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VectorType {
    AAV,
    Lentiviral,
    Retroviral,
    Adenoviral,
    CRISPR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeliveryRoute {
    Intravenous,
    Intrathecal,
    Intraocular,
    Intramuscular,
    Direct,
}

impl GeneTherapy {
    pub fn new(target_gene: String, vector_type: VectorType) -> Self {
        Self {
            target_gene,
            vector_type,
            delivery_route: DeliveryRoute::Intravenous,
            indicated_conditions: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rare_disease_profile() {
        let mut profile = RareDiseaseProfile::new();
        profile.add_confirmed_diagnosis(RareDisease::LysosomalStorage(
            LysosomalStorageDisorder::GaucherDisease
        ));

        assert!(profile.has_treatable_condition());
    }

    #[test]
    fn test_enzyme_activity() {
        let enzyme = EnzymeActivity {
            enzyme_name: "Glucocerebrosidase".to_string(),
            activity_percent_of_normal: 5.0,
            reference_range: (50.0, 150.0),
        };

        assert!(enzyme.is_deficient());
        assert_eq!(enzyme.severity(), DeficiencySeverity::Severe);
    }

    #[test]
    fn test_newborn_screening() {
        let mut screen = NewbornScreeningPanel::new();
        screen.phenylalanine_mg_dl = 4.0;
        screen.galactose_mg_dl = 10.0;

        let results = screen.screen_results();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_genetic_test_result() {
        let result = GeneticTestResult {
            gene: "GBA".to_string(),
            variant: "N370S".to_string(),
            zygosity: Zygosity::Heterozygous,
            pathogenicity: Pathogenicity::Pathogenic,
        };

        assert_eq!(result.pathogenicity, Pathogenicity::Pathogenic);
    }

    #[test]
    fn test_orphan_drug() {
        let drug = OrphanDrugTherapy::new(
            "Imiglucerase".to_string(),
            RareDisease::LysosomalStorage(LysosomalStorageDisorder::GaucherDisease)
        );

        assert_eq!(drug.drug_name, "Imiglucerase");
    }

    #[test]
    fn test_gene_therapy() {
        let mut therapy = GeneTherapy::new("RPE65".to_string(), VectorType::AAV);
        therapy.delivery_route = DeliveryRoute::Intraocular;

        assert_eq!(therapy.vector_type, VectorType::AAV);
        assert_eq!(therapy.delivery_route, DeliveryRoute::Intraocular);
    }

    #[test]
    fn test_pathogenicity_classification() {
        let variant = GeneticTestResult {
            gene: "CFTR".to_string(),
            variant: "F508del".to_string(),
            zygosity: Zygosity::Homozygous,
            pathogenicity: Pathogenicity::Pathogenic,
        };

        assert!(matches!(variant.pathogenicity, Pathogenicity::Pathogenic | Pathogenicity::LikelyPathogenic));
    }
}

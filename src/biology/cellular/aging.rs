use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CellularAge {
    pub chronological_age_years: f64,
    pub biological_age_years: f64,
    pub telomere_length_kb: f64,
    pub senescent_cell_burden_percent: f64,
    pub mitochondrial_function_percent: f64,
    pub dna_methylation_age: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SenescenceMarkers {
    pub p16_expression_level: f64,
    pub p21_expression_level: f64,
    pub beta_galactosidase_activity: f64,
    pub sasp_factor_secretion: SASPProfile,
    pub cell_cycle_arrest: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SASPProfile {
    pub il_6_level: f64,
    pub il_8_level: f64,
    pub mmp_3_level: f64,
    pub mmp_9_level: f64,
    pub tgf_beta_level: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelomereStatus {
    pub average_length_kb: f64,
    pub shortest_telomeres_kb: f64,
    pub telomerase_activity: f64,
    pub critically_short_telomeres_percent: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OxidativeStress {
    pub reactive_oxygen_species_level: f64,
    pub superoxide_dismutase_activity: f64,
    pub catalase_activity: f64,
    pub glutathione_peroxidase_activity: f64,
    pub glutathione_ratio: f64,
    pub lipid_peroxidation_level: f64,
    pub protein_carbonylation_level: f64,
    pub dna_oxidation_8_ohdg: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MitochondrialHealth {
    pub mitochondrial_count: u32,
    pub membrane_potential: f64,
    pub atp_production_rate: f64,
    pub oxidative_phosphorylation_efficiency: f64,
    pub mitochondrial_dna_integrity: f64,
    pub mitophagy_rate: f64,
    pub fusion_fission_balance: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutophagyActivity {
    pub basal_autophagy_rate: f64,
    pub lc3_ii_lc3_i_ratio: f64,
    pub p62_degradation_rate: f64,
    pub lysosomal_activity: f64,
    pub mtor_activity: f64,
    pub ampk_activity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DNADamageResponse {
    pub double_strand_breaks: u32,
    pub single_strand_breaks: u32,
    pub base_excision_repair_activity: f64,
    pub nucleotide_excision_repair_activity: f64,
    pub mismatch_repair_activity: f64,
    pub homologous_recombination_activity: f64,
    pub non_homologous_end_joining_activity: f64,
    pub gamma_h2ax_foci: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpigeneticAge {
    pub horvath_clock_age: f64,
    pub hannum_clock_age: f64,
    pub phenoage: f64,
    pub grimage: f64,
    pub dunedin_pace: f64,
    pub global_dna_methylation_percent: f64,
    pub histone_modification_index: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProteostasisCapacity {
    pub protein_folding_efficiency: f64,
    pub chaperone_activity: f64,
    pub proteasome_activity: f64,
    pub unfolded_protein_response_activation: f64,
    pub aggregated_protein_burden: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgingHallmark {
    GenomicInstability,
    TelomereAttrition,
    EpigeneticAlterations,
    LossOfProteostasis,
    DisabledMacroautophagy,
    DeregulatedNutrientSensing,
    MitochondrialDysfunction,
    CellularSenescence,
    StemCellExhaustion,
    AlteredIntercellularCommunication,
    ChronicInflammation,
    Dysbiosis,
}

impl CellularAge {
    pub fn new(chronological_age_years: f64) -> Self {
        let base_telomere = 10.0 - (chronological_age_years * 0.05);

        Self {
            chronological_age_years,
            biological_age_years: chronological_age_years,
            telomere_length_kb: base_telomere.max(3.0),
            senescent_cell_burden_percent: (chronological_age_years * 0.1).min(15.0),
            mitochondrial_function_percent: 100.0 - (chronological_age_years * 0.5),
            dna_methylation_age: chronological_age_years,
        }
    }

    pub fn is_accelerated_aging(&self) -> bool {
        self.biological_age_years > self.chronological_age_years + 5.0
    }

    pub fn is_decelerated_aging(&self) -> bool {
        self.biological_age_years < self.chronological_age_years - 5.0
    }

    pub fn aging_rate(&self) -> f64 {
        self.biological_age_years / self.chronological_age_years
    }

    pub fn cellular_health_score(&self) -> f64 {
        let telomere_score = (self.telomere_length_kb / 10.0) * 100.0;
        let senescence_score = (15.0 - self.senescent_cell_burden_percent) / 15.0 * 100.0;
        let mitochondrial_score = self.mitochondrial_function_percent;

        (telomere_score + senescence_score + mitochondrial_score) / 3.0
    }
}

impl TelomereStatus {
    pub fn new(age_years: f64) -> Self {
        let avg_length = 10.0 - (age_years * 0.05);

        Self {
            average_length_kb: avg_length.max(3.0),
            shortest_telomeres_kb: (avg_length * 0.6).max(2.0),
            telomerase_activity: 0.1,
            critically_short_telomeres_percent: (age_years * 0.2).min(30.0),
        }
    }

    pub fn is_critically_short(&self) -> bool {
        self.average_length_kb < 4.0 || self.critically_short_telomeres_percent > 15.0
    }

    pub fn replicative_capacity_remaining(&self) -> u32 {
        let hayflick_limit = 50.0;
        let divisions_used = (10.0 - self.average_length_kb) / 0.05;
        ((hayflick_limit - divisions_used).max(0.0)) as u32
    }
}

impl SenescenceMarkers {
    pub fn normal() -> Self {
        Self {
            p16_expression_level: 1.0,
            p21_expression_level: 1.0,
            beta_galactosidase_activity: 1.0,
            sasp_factor_secretion: SASPProfile::normal(),
            cell_cycle_arrest: false,
        }
    }

    pub fn senescent() -> Self {
        Self {
            p16_expression_level: 10.0,
            p21_expression_level: 8.0,
            beta_galactosidase_activity: 15.0,
            sasp_factor_secretion: SASPProfile::elevated(),
            cell_cycle_arrest: true,
        }
    }

    pub fn is_senescent(&self) -> bool {
        self.p16_expression_level > 5.0 &&
        self.beta_galactosidase_activity > 5.0 &&
        self.cell_cycle_arrest
    }
}

impl SASPProfile {
    pub fn normal() -> Self {
        Self {
            il_6_level: 1.0,
            il_8_level: 1.0,
            mmp_3_level: 1.0,
            mmp_9_level: 1.0,
            tgf_beta_level: 1.0,
        }
    }

    pub fn elevated() -> Self {
        Self {
            il_6_level: 10.0,
            il_8_level: 12.0,
            mmp_3_level: 8.0,
            mmp_9_level: 9.0,
            tgf_beta_level: 7.0,
        }
    }

    pub fn inflammatory_burden(&self) -> f64 {
        (self.il_6_level + self.il_8_level + self.tgf_beta_level) / 3.0
    }
}

impl OxidativeStress {
    pub fn normal() -> Self {
        Self {
            reactive_oxygen_species_level: 1.0,
            superoxide_dismutase_activity: 100.0,
            catalase_activity: 100.0,
            glutathione_peroxidase_activity: 100.0,
            glutathione_ratio: 100.0,
            lipid_peroxidation_level: 1.0,
            protein_carbonylation_level: 1.0,
            dna_oxidation_8_ohdg: 1.0,
        }
    }

    pub fn oxidative_balance(&self) -> f64 {
        let antioxidant_capacity = (
            self.superoxide_dismutase_activity +
            self.catalase_activity +
            self.glutathione_peroxidase_activity +
            self.glutathione_ratio
        ) / 4.0;

        let oxidative_damage = (
            self.reactive_oxygen_species_level +
            self.lipid_peroxidation_level +
            self.protein_carbonylation_level +
            self.dna_oxidation_8_ohdg
        ) / 4.0;

        antioxidant_capacity / oxidative_damage
    }

    pub fn has_high_oxidative_stress(&self) -> bool {
        self.oxidative_balance() < 20.0
    }
}

impl MitochondrialHealth {
    pub fn normal() -> Self {
        Self {
            mitochondrial_count: 1000,
            membrane_potential: 180.0,
            atp_production_rate: 100.0,
            oxidative_phosphorylation_efficiency: 95.0,
            mitochondrial_dna_integrity: 99.0,
            mitophagy_rate: 5.0,
            fusion_fission_balance: 1.0,
        }
    }

    pub fn health_score(&self) -> f64 {
        let potential_score = (self.membrane_potential / 180.0) * 100.0;
        let atp_score = self.atp_production_rate;
        let efficiency_score = self.oxidative_phosphorylation_efficiency;
        let dna_score = self.mitochondrial_dna_integrity;

        (potential_score + atp_score + efficiency_score + dna_score) / 4.0
    }

    pub fn is_dysfunctional(&self) -> bool {
        self.health_score() < 60.0
    }
}

impl AutophagyActivity {
    pub fn normal() -> Self {
        Self {
            basal_autophagy_rate: 100.0,
            lc3_ii_lc3_i_ratio: 0.5,
            p62_degradation_rate: 100.0,
            lysosomal_activity: 100.0,
            mtor_activity: 50.0,
            ampk_activity: 50.0,
        }
    }

    pub fn is_impaired(&self) -> bool {
        self.basal_autophagy_rate < 50.0 || self.lysosomal_activity < 50.0
    }

    pub fn autophagy_flux(&self) -> f64 {
        (self.basal_autophagy_rate + self.p62_degradation_rate + self.lysosomal_activity) / 3.0
    }
}

impl DNADamageResponse {
    pub fn normal() -> Self {
        Self {
            double_strand_breaks: 0,
            single_strand_breaks: 5,
            base_excision_repair_activity: 100.0,
            nucleotide_excision_repair_activity: 100.0,
            mismatch_repair_activity: 100.0,
            homologous_recombination_activity: 100.0,
            non_homologous_end_joining_activity: 100.0,
            gamma_h2ax_foci: 0,
        }
    }

    pub fn total_dna_breaks(&self) -> u32 {
        self.double_strand_breaks + self.single_strand_breaks
    }

    pub fn repair_capacity(&self) -> f64 {
        (
            self.base_excision_repair_activity +
            self.nucleotide_excision_repair_activity +
            self.mismatch_repair_activity +
            self.homologous_recombination_activity +
            self.non_homologous_end_joining_activity
        ) / 5.0
    }

    pub fn has_excessive_damage(&self) -> bool {
        self.double_strand_breaks > 10 || self.single_strand_breaks > 100
    }
}

impl EpigeneticAge {
    pub fn new(chronological_age: f64) -> Self {
        Self {
            horvath_clock_age: chronological_age,
            hannum_clock_age: chronological_age,
            phenoage: chronological_age,
            grimage: chronological_age,
            dunedin_pace: 1.0,
            global_dna_methylation_percent: 70.0 - (chronological_age * 0.1),
            histone_modification_index: 100.0 - (chronological_age * 0.5),
        }
    }

    pub fn average_epigenetic_age(&self) -> f64 {
        (self.horvath_clock_age + self.hannum_clock_age + self.phenoage + self.grimage) / 4.0
    }

    pub fn is_epigenetically_older(&self, chronological_age: f64) -> bool {
        self.average_epigenetic_age() > chronological_age + 5.0
    }
}

impl ProteostasisCapacity {
    pub fn normal() -> Self {
        Self {
            protein_folding_efficiency: 95.0,
            chaperone_activity: 100.0,
            proteasome_activity: 100.0,
            unfolded_protein_response_activation: 0.0,
            aggregated_protein_burden: 0.0,
        }
    }

    pub fn is_impaired(&self) -> bool {
        self.protein_folding_efficiency < 70.0 ||
        self.proteasome_activity < 70.0 ||
        self.aggregated_protein_burden > 20.0
    }

    pub fn proteostasis_score(&self) -> f64 {
        let capacity = (self.protein_folding_efficiency + self.chaperone_activity + self.proteasome_activity) / 3.0;
        let burden = 100.0 - self.aggregated_protein_burden;

        (capacity + burden) / 2.0
    }
}

impl AgingHallmark {
    pub fn all_hallmarks() -> Vec<AgingHallmark> {
        vec![
            AgingHallmark::GenomicInstability,
            AgingHallmark::TelomereAttrition,
            AgingHallmark::EpigeneticAlterations,
            AgingHallmark::LossOfProteostasis,
            AgingHallmark::DisabledMacroautophagy,
            AgingHallmark::DeregulatedNutrientSensing,
            AgingHallmark::MitochondrialDysfunction,
            AgingHallmark::CellularSenescence,
            AgingHallmark::StemCellExhaustion,
            AgingHallmark::AlteredIntercellularCommunication,
            AgingHallmark::ChronicInflammation,
            AgingHallmark::Dysbiosis,
        ]
    }

    pub fn description(&self) -> &str {
        match self {
            AgingHallmark::GenomicInstability => "Accumulation of DNA damage and mutations",
            AgingHallmark::TelomereAttrition => "Progressive shortening of telomeres",
            AgingHallmark::EpigeneticAlterations => "Changes in DNA methylation and histone modifications",
            AgingHallmark::LossOfProteostasis => "Impaired protein quality control",
            AgingHallmark::DisabledMacroautophagy => "Reduced cellular cleanup and recycling",
            AgingHallmark::DeregulatedNutrientSensing => "Dysregulation of metabolic pathways",
            AgingHallmark::MitochondrialDysfunction => "Impaired energy production and increased ROS",
            AgingHallmark::CellularSenescence => "Accumulation of senescent cells",
            AgingHallmark::StemCellExhaustion => "Depletion of stem cell reserves",
            AgingHallmark::AlteredIntercellularCommunication => "Changes in cell-cell signaling",
            AgingHallmark::ChronicInflammation => "Persistent low-grade inflammation (inflammaging)",
            AgingHallmark::Dysbiosis => "Imbalance in microbiome composition",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cellular_age() {
        let age = CellularAge::new(30.0);
        assert_eq!(age.chronological_age_years, 30.0);
        assert!(age.telomere_length_kb > 0.0);
        assert!(!age.is_accelerated_aging());
    }

    #[test]
    fn test_telomere_status() {
        let telomeres = TelomereStatus::new(50.0);
        assert!(telomeres.average_length_kb > 0.0);
        let capacity = telomeres.replicative_capacity_remaining();
        assert!(capacity <= 50);
    }

    #[test]
    fn test_senescence_markers() {
        let normal = SenescenceMarkers::normal();
        assert!(!normal.is_senescent());

        let senescent = SenescenceMarkers::senescent();
        assert!(senescent.is_senescent());
    }

    #[test]
    fn test_oxidative_stress() {
        let stress = OxidativeStress::normal();
        assert!(!stress.has_high_oxidative_stress());
        assert!(stress.oxidative_balance() > 20.0);
    }

    #[test]
    fn test_mitochondrial_health() {
        let mito = MitochondrialHealth::normal();
        assert!(!mito.is_dysfunctional());
        assert!(mito.health_score() > 90.0);
    }

    #[test]
    fn test_autophagy() {
        let autophagy = AutophagyActivity::normal();
        assert!(!autophagy.is_impaired());
        assert!(autophagy.autophagy_flux() > 50.0);
    }

    #[test]
    fn test_dna_damage() {
        let dna = DNADamageResponse::normal();
        assert!(!dna.has_excessive_damage());
        assert!(dna.repair_capacity() > 90.0);
    }

    #[test]
    fn test_epigenetic_age() {
        let epi_age = EpigeneticAge::new(40.0);
        assert_eq!(epi_age.average_epigenetic_age(), 40.0);
        assert!(!epi_age.is_epigenetically_older(40.0));
    }

    #[test]
    fn test_proteostasis() {
        let proteostasis = ProteostasisCapacity::normal();
        assert!(!proteostasis.is_impaired());
        assert!(proteostasis.proteostasis_score() > 90.0);
    }

    #[test]
    fn test_aging_hallmarks() {
        let hallmarks = AgingHallmark::all_hallmarks();
        assert_eq!(hallmarks.len(), 12);

        for hallmark in hallmarks {
            assert!(!hallmark.description().is_empty());
        }
    }
}

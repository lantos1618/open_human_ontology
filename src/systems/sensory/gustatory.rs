use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GustatorySensorySystem {
    pub taste_buds: TasteBuds,
    pub taste_receptors: TasteReceptors,
    pub gustatory_pathways: GustatoryPathways,
    pub taste_perception: TastePerception,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteBuds {
    pub total_count: u32,
    pub distribution: TasteBudDistribution,
    pub taste_cells: TasteCells,
    pub lifespan_days: f64,
    pub turnover_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteBudDistribution {
    pub tongue_anterior: u32,
    pub tongue_posterior: u32,
    pub soft_palate: u32,
    pub pharynx: u32,
    pub epiglottis: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteCells {
    pub type_i_cells: TypeICells,
    pub type_ii_cells: TypeIICells,
    pub type_iii_cells: TypeIIICells,
    pub type_iv_cells: TypeIVCells,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeICells {
    pub cell_count: f64,
    pub function: String,
    pub support_role: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeIICells {
    pub cell_count: f64,
    pub receptor_types: Vec<TasteReceptorType>,
    pub signal_transduction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeIIICells {
    pub cell_count: f64,
    pub receptor_types: Vec<TasteReceptorType>,
    pub synaptic_connections: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeIVCells {
    pub cell_count: f64,
    pub basal_stem_cells: bool,
    pub proliferation_rate: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TasteReceptorType {
    Sweet,
    Umami,
    Bitter,
    Sour,
    Salty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteReceptors {
    pub sweet_receptors: SweetReceptors,
    pub umami_receptors: UmamiReceptors,
    pub bitter_receptors: BitterReceptors,
    pub sour_receptors: SourReceptors,
    pub salty_receptors: SaltyReceptors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweetReceptors {
    pub t1r2_t1r3: T1R2T1R3Heterodimer,
    pub sensitivity_mm: f64,
    pub ligands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct T1R2T1R3Heterodimer {
    pub expression_level: f64,
    pub g_protein_coupling: GPCRCoupling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UmamiReceptors {
    pub t1r1_t1r3: T1R1T1R3Heterodimer,
    pub sensitivity_mm: f64,
    pub glutamate_binding: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct T1R1T1R3Heterodimer {
    pub expression_level: f64,
    pub g_protein_coupling: GPCRCoupling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitterReceptors {
    pub t2r_family: Vec<T2RReceptor>,
    pub total_types: u32,
    pub sensitivity_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct T2RReceptor {
    pub receptor_subtype: String,
    pub expression_level: f64,
    pub ligand_specificity: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourReceptors {
    pub pkd2l1_pkd1l3: PKDChannels,
    pub otop1_channels: OTOP1Channels,
    pub proton_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PKDChannels {
    pub expression_level: f64,
    pub channel_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OTOP1Channels {
    pub expression_level: f64,
    pub proton_conductance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaltyReceptors {
    pub enac_channels: ENaCChannels,
    pub sodium_sensitivity_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ENaCChannels {
    pub expression_level: f64,
    pub sodium_conductance: f64,
    pub amiloride_sensitivity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPCRCoupling {
    pub gustducin_level: f64,
    pub plc_beta2_activity: f64,
    pub ip3_production: f64,
    pub calcium_release: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GustatoryPathways {
    pub cranial_nerves: CranialNerveGustatory,
    pub brainstem_nuclei: BrainstemGustatoryNuclei,
    pub thalamic_relay: ThalamusGustatory,
    pub cortical_processing: GustatoCortex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CranialNerveGustatory {
    pub facial_nerve_cn7: FacialNerveGustatory,
    pub glossopharyngeal_cn9: GlossopharyngealNerve,
    pub vagus_cn10: VagusNerveGustatory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacialNerveGustatory {
    pub chorda_tympani: ChordaTympani,
    pub greater_petrosal_nerve: GreaterPetrosalNerve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChordaTympani {
    pub innervation_region: String,
    pub taste_bud_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreaterPetrosalNerve {
    pub innervation_region: String,
    pub taste_bud_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossopharyngealNerve {
    pub innervation_region: String,
    pub taste_bud_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VagusNerveGustatory {
    pub innervation_region: String,
    pub taste_bud_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstemGustatoryNuclei {
    pub nucleus_of_solitary_tract: NucleusSolitaryTract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NucleusSolitaryTract {
    pub rostral_portion: RostralNST,
    pub convergence_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RostralNST {
    pub taste_processing: f64,
    pub visceral_integration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThalamusGustatory {
    pub vpm_nucleus: VPMNucleus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPMNucleus {
    pub location: String,
    pub relay_function: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GustatoCortex {
    pub primary_gustatory_cortex: PrimaryGustatoryCortex,
    pub secondary_areas: SecondaryGustatoryAreas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryGustatoryCortex {
    pub location: String,
    pub taste_quality_coding: f64,
    pub topographic_organization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryGustatoryAreas {
    pub orbitofrontal_cortex: OrbitofrontalCortexGustatory,
    pub insular_cortex: InsularCortex,
    pub amygdala: AmygdalaGustatory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitofrontalCortexGustatory {
    pub hedonic_evaluation: f64,
    pub flavor_integration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsularCortex {
    pub taste_intensity: f64,
    pub visceral_integration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmygdalaGustatory {
    pub emotional_valence: f64,
    pub learned_preferences: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TastePerception {
    pub basic_tastes: BasicTastes,
    pub taste_thresholds: TasteThresholds,
    pub adaptation: TasteAdaptation,
    pub integration: TasteIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicTastes {
    pub sweet_perception: f64,
    pub umami_perception: f64,
    pub bitter_perception: f64,
    pub sour_perception: f64,
    pub salty_perception: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteThresholds {
    pub sweet_threshold_mm: f64,
    pub umami_threshold_mm: f64,
    pub bitter_threshold_mm: f64,
    pub sour_threshold_ph: f64,
    pub salty_threshold_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteAdaptation {
    pub adaptation_rate: f64,
    pub cross_adaptation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteIntegration {
    pub olfactory_integration: f64,
    pub trigeminal_integration: f64,
    pub flavor_perception: f64,
}

impl GustatorySensorySystem {
    pub fn new_healthy() -> Self {
        Self {
            taste_buds: TasteBuds {
                total_count: 10000,
                distribution: TasteBudDistribution {
                    tongue_anterior: 1600,
                    tongue_posterior: 2400,
                    soft_palate: 2500,
                    pharynx: 2000,
                    epiglottis: 1500,
                },
                taste_cells: TasteCells {
                    type_i_cells: TypeICells {
                        cell_count: 30000.0,
                        function: "Support cells".to_string(),
                        support_role: 1.0,
                    },
                    type_ii_cells: TypeIICells {
                        cell_count: 20000.0,
                        receptor_types: vec![
                            TasteReceptorType::Sweet,
                            TasteReceptorType::Umami,
                            TasteReceptorType::Bitter,
                        ],
                        signal_transduction: "GPCR-PLC-IP3".to_string(),
                    },
                    type_iii_cells: TypeIIICells {
                        cell_count: 10000.0,
                        receptor_types: vec![TasteReceptorType::Sour],
                        synaptic_connections: true,
                    },
                    type_iv_cells: TypeIVCells {
                        cell_count: 5000.0,
                        basal_stem_cells: true,
                        proliferation_rate: 0.1,
                    },
                },
                lifespan_days: 10.0,
                turnover_rate: 0.1,
            },
            taste_receptors: TasteReceptors {
                sweet_receptors: SweetReceptors {
                    t1r2_t1r3: T1R2T1R3Heterodimer {
                        expression_level: 1.0,
                        g_protein_coupling: GPCRCoupling {
                            gustducin_level: 1.0,
                            plc_beta2_activity: 1.0,
                            ip3_production: 1.0,
                            calcium_release: 1.0,
                        },
                    },
                    sensitivity_mm: 10.0,
                    ligands: vec![
                        "Sucrose".to_string(),
                        "Glucose".to_string(),
                        "Fructose".to_string(),
                        "Artificial sweeteners".to_string(),
                    ],
                },
                umami_receptors: UmamiReceptors {
                    t1r1_t1r3: T1R1T1R3Heterodimer {
                        expression_level: 1.0,
                        g_protein_coupling: GPCRCoupling {
                            gustducin_level: 1.0,
                            plc_beta2_activity: 1.0,
                            ip3_production: 1.0,
                            calcium_release: 1.0,
                        },
                    },
                    sensitivity_mm: 0.3,
                    glutamate_binding: 1.0,
                },
                bitter_receptors: BitterReceptors {
                    t2r_family: vec![],
                    total_types: 25,
                    sensitivity_mm: 0.001,
                },
                sour_receptors: SourReceptors {
                    pkd2l1_pkd1l3: PKDChannels {
                        expression_level: 1.0,
                        channel_activity: 1.0,
                    },
                    otop1_channels: OTOP1Channels {
                        expression_level: 1.0,
                        proton_conductance: 1.0,
                    },
                    proton_sensitivity: 1.0,
                },
                salty_receptors: SaltyReceptors {
                    enac_channels: ENaCChannels {
                        expression_level: 1.0,
                        sodium_conductance: 1.0,
                        amiloride_sensitivity: true,
                    },
                    sodium_sensitivity_mm: 10.0,
                },
            },
            gustatory_pathways: GustatoryPathways {
                cranial_nerves: CranialNerveGustatory {
                    facial_nerve_cn7: FacialNerveGustatory {
                        chorda_tympani: ChordaTympani {
                            innervation_region: "Anterior 2/3 tongue".to_string(),
                            taste_bud_count: 1600,
                        },
                        greater_petrosal_nerve: GreaterPetrosalNerve {
                            innervation_region: "Soft palate".to_string(),
                            taste_bud_count: 2500,
                        },
                    },
                    glossopharyngeal_cn9: GlossopharyngealNerve {
                        innervation_region: "Posterior 1/3 tongue".to_string(),
                        taste_bud_count: 2400,
                    },
                    vagus_cn10: VagusNerveGustatory {
                        innervation_region: "Pharynx and epiglottis".to_string(),
                        taste_bud_count: 3500,
                    },
                },
                brainstem_nuclei: BrainstemGustatoryNuclei {
                    nucleus_of_solitary_tract: NucleusSolitaryTract {
                        rostral_portion: RostralNST {
                            taste_processing: 1.0,
                            visceral_integration: 1.0,
                        },
                        convergence_ratio: 1.0,
                    },
                },
                thalamic_relay: ThalamusGustatory {
                    vpm_nucleus: VPMNucleus {
                        location: "Ventral posteromedial nucleus".to_string(),
                        relay_function: 1.0,
                    },
                },
                cortical_processing: GustatoCortex {
                    primary_gustatory_cortex: PrimaryGustatoryCortex {
                        location: "Anterior insula and frontal operculum".to_string(),
                        taste_quality_coding: 1.0,
                        topographic_organization: true,
                    },
                    secondary_areas: SecondaryGustatoryAreas {
                        orbitofrontal_cortex: OrbitofrontalCortexGustatory {
                            hedonic_evaluation: 1.0,
                            flavor_integration: 1.0,
                        },
                        insular_cortex: InsularCortex {
                            taste_intensity: 1.0,
                            visceral_integration: 1.0,
                        },
                        amygdala: AmygdalaGustatory {
                            emotional_valence: 1.0,
                            learned_preferences: 1.0,
                        },
                    },
                },
            },
            taste_perception: TastePerception {
                basic_tastes: BasicTastes {
                    sweet_perception: 1.0,
                    umami_perception: 1.0,
                    bitter_perception: 1.0,
                    sour_perception: 1.0,
                    salty_perception: 1.0,
                },
                taste_thresholds: TasteThresholds {
                    sweet_threshold_mm: 10.0,
                    umami_threshold_mm: 0.3,
                    bitter_threshold_mm: 0.001,
                    sour_threshold_ph: 3.5,
                    salty_threshold_mm: 10.0,
                },
                adaptation: TasteAdaptation {
                    adaptation_rate: 1.0,
                    cross_adaptation: 0.5,
                },
                integration: TasteIntegration {
                    olfactory_integration: 1.0,
                    trigeminal_integration: 1.0,
                    flavor_perception: 1.0,
                },
            },
        }
    }

    pub fn calculate_taste_sensitivity(&self) -> f64 {
        let bitter_sensitivity = 1.0 / self.taste_receptors.bitter_receptors.sensitivity_mm;
        let sweet_sensitivity = 1.0 / self.taste_receptors.sweet_receptors.sensitivity_mm;

        (bitter_sensitivity + sweet_sensitivity) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gustatory_system_creation() {
        let system = GustatorySensorySystem::new_healthy();
        assert_eq!(system.taste_buds.total_count, 10000);
    }

    #[test]
    fn test_taste_bud_distribution() {
        let system = GustatorySensorySystem::new_healthy();
        let total = system.taste_buds.distribution.tongue_anterior
            + system.taste_buds.distribution.tongue_posterior
            + system.taste_buds.distribution.soft_palate
            + system.taste_buds.distribution.pharynx
            + system.taste_buds.distribution.epiglottis;

        assert_eq!(total, 10000);
    }

    #[test]
    fn test_taste_sensitivity() {
        let system = GustatorySensorySystem::new_healthy();
        let sensitivity = system.calculate_taste_sensitivity();
        assert!(sensitivity > 0.0);
    }

    #[test]
    fn test_bitter_receptors() {
        let system = GustatorySensorySystem::new_healthy();
        assert_eq!(system.taste_receptors.bitter_receptors.total_types, 25);
    }
}

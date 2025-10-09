use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactorySensorySystem {
    pub olfactory_epithelium: OlfactoryEpithelium,
    pub olfactory_receptors: OlfactoryReceptors,
    pub olfactory_bulb: OlfactoryBulb,
    pub olfactory_pathways: OlfactoryPathways,
    pub smell_perception: SmellPerception,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryEpithelium {
    pub location: String,
    pub surface_area_cm2: f64,
    pub thickness_um: f64,
    pub cell_types: OlfactoryEpithelialCells,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryEpithelialCells {
    pub olfactory_sensory_neurons: OlfactorySensoryNeurons,
    pub supporting_cells: SupportingCells,
    pub basal_cells: BasalCells,
    pub bowmans_glands: BowmansGlands,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactorySensoryNeurons {
    pub total_count: f64,
    pub lifespan_days: f64,
    pub turnover_rate: f64,
    pub cilia_count_per_neuron: f64,
    pub receptor_expression: Vec<ReceptorExpression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceptorExpression {
    pub receptor_type: String,
    pub expression_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportingCells {
    pub cell_count: f64,
    pub support_function: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasalCells {
    pub stem_cell_count: f64,
    pub proliferation_rate: f64,
    pub differentiation_capacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowmansGlands {
    pub gland_count: f64,
    pub mucus_production_ml_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryReceptors {
    pub total_receptor_types: u32,
    pub receptor_families: Vec<OlfactoryReceptorFamily>,
    pub binding_specificity: f64,
    pub signal_transduction: OlfactoryTransduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryReceptorFamily {
    pub family_name: String,
    pub receptor_count: u32,
    pub ligand_specificity: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryTransduction {
    pub g_protein_coupling: GProteinCoupling,
    pub camp_cascade: cAMPCascade,
    pub ion_channels: OlfactoryIonChannels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GProteinCoupling {
    pub golf_protein_level: f64,
    pub coupling_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cAMPCascade {
    pub adenylyl_cyclase_activity: f64,
    pub camp_production_rate: f64,
    pub pde_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryIonChannels {
    pub cng_channels: f64,
    pub calcium_activated_chloride_channels: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryBulb {
    pub glomeruli: Glomeruli,
    pub mitral_cells: MitralCells,
    pub tufted_cells: TuftedCells,
    pub periglomerular_cells: PeriglomerularCells,
    pub granule_cells: GranuleCells,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glomeruli {
    pub total_count: u32,
    pub diameter_um: f64,
    pub convergence_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitralCells {
    pub cell_count: f64,
    pub axon_projection: String,
    pub firing_rate_hz: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuftedCells {
    pub cell_count: f64,
    pub axon_projection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriglomerularCells {
    pub cell_count: f64,
    pub inhibitory_function: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GranuleCells {
    pub cell_count: f64,
    pub lateral_inhibition: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryPathways {
    pub olfactory_tract: OlfactoryTract,
    pub primary_olfactory_cortex: PrimaryOlfactoryCortex,
    pub secondary_projections: SecondaryOlfactoryProjections,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryTract {
    pub lateral_tract: LateralOlfactoryTract,
    pub medial_tract: MedialOlfactoryTract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LateralOlfactoryTract {
    pub projection_strength: f64,
    pub target_regions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedialOlfactoryTract {
    pub projection_strength: f64,
    pub target_regions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryOlfactoryCortex {
    pub piriform_cortex: PiriformCortex,
    pub anterior_olfactory_nucleus: AnteriorOlfactoryNucleus,
    pub olfactory_tubercle: OlfactoryTubercle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiriformCortex {
    pub location: String,
    pub volume_mm3: f64,
    pub pattern_recognition: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnteriorOlfactoryNucleus {
    pub location: String,
    pub contralateral_connections: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlfactoryTubercle {
    pub location: String,
    pub reward_integration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryOlfactoryProjections {
    pub orbitofrontal_cortex: OrbitofrontalCortexOlfactory,
    pub amygdala: AmygdalaOlfactory,
    pub hippocampus: HippocampusOlfactory,
    pub hypothalamus: HypothalamusOlfactory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitofrontalCortexOlfactory {
    pub conscious_perception: f64,
    pub odor_identification: f64,
    pub hedonic_evaluation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmygdalaOlfactory {
    pub emotional_processing: f64,
    pub learned_associations: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HippocampusOlfactory {
    pub olfactory_memory: f64,
    pub spatial_context: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypothalamusOlfactory {
    pub autonomic_responses: f64,
    pub feeding_behavior: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmellPerception {
    pub detection_threshold: DetectionThreshold,
    pub discrimination_ability: f64,
    pub adaptation_rate: f64,
    pub odor_memory: OdorMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionThreshold {
    pub threshold_ppm: f64,
    pub sensitivity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdorMemory {
    pub capacity: f64,
    pub recall_accuracy: f64,
    pub emotional_association_strength: f64,
}

impl OlfactorySensorySystem {
    pub fn new_healthy() -> Self {
        Self {
            olfactory_epithelium: OlfactoryEpithelium {
                location: "Superior nasal cavity".to_string(),
                surface_area_cm2: 10.0,
                thickness_um: 60.0,
                cell_types: OlfactoryEpithelialCells {
                    olfactory_sensory_neurons: OlfactorySensoryNeurons {
                        total_count: 1e7,
                        lifespan_days: 60.0,
                        turnover_rate: 0.016,
                        cilia_count_per_neuron: 10.0,
                        receptor_expression: vec![],
                    },
                    supporting_cells: SupportingCells {
                        cell_count: 1e7,
                        support_function: 1.0,
                    },
                    basal_cells: BasalCells {
                        stem_cell_count: 1e6,
                        proliferation_rate: 1.0,
                        differentiation_capacity: 1.0,
                    },
                    bowmans_glands: BowmansGlands {
                        gland_count: 1000.0,
                        mucus_production_ml_per_day: 1.0,
                    },
                },
            },
            olfactory_receptors: OlfactoryReceptors {
                total_receptor_types: 400,
                receptor_families: vec![],
                binding_specificity: 1.0,
                signal_transduction: OlfactoryTransduction {
                    g_protein_coupling: GProteinCoupling {
                        golf_protein_level: 1.0,
                        coupling_efficiency: 1.0,
                    },
                    camp_cascade: cAMPCascade {
                        adenylyl_cyclase_activity: 1.0,
                        camp_production_rate: 1.0,
                        pde_activity: 1.0,
                    },
                    ion_channels: OlfactoryIonChannels {
                        cng_channels: 1.0,
                        calcium_activated_chloride_channels: 1.0,
                    },
                },
            },
            olfactory_bulb: OlfactoryBulb {
                glomeruli: Glomeruli {
                    total_count: 2000,
                    diameter_um: 100.0,
                    convergence_ratio: 5000.0,
                },
                mitral_cells: MitralCells {
                    cell_count: 50000.0,
                    axon_projection: "Olfactory tract".to_string(),
                    firing_rate_hz: 20.0,
                },
                tufted_cells: TuftedCells {
                    cell_count: 50000.0,
                    axon_projection: "Olfactory tract".to_string(),
                },
                periglomerular_cells: PeriglomerularCells {
                    cell_count: 100000.0,
                    inhibitory_function: 1.0,
                },
                granule_cells: GranuleCells {
                    cell_count: 500000.0,
                    lateral_inhibition: 1.0,
                },
            },
            olfactory_pathways: OlfactoryPathways {
                olfactory_tract: OlfactoryTract {
                    lateral_tract: LateralOlfactoryTract {
                        projection_strength: 1.0,
                        target_regions: vec![
                            "Piriform cortex".to_string(),
                            "Amygdala".to_string(),
                            "Entorhinal cortex".to_string(),
                        ],
                    },
                    medial_tract: MedialOlfactoryTract {
                        projection_strength: 1.0,
                        target_regions: vec![
                            "Anterior olfactory nucleus".to_string(),
                            "Olfactory tubercle".to_string(),
                        ],
                    },
                },
                primary_olfactory_cortex: PrimaryOlfactoryCortex {
                    piriform_cortex: PiriformCortex {
                        location: "Temporal lobe".to_string(),
                        volume_mm3: 1000.0,
                        pattern_recognition: 1.0,
                    },
                    anterior_olfactory_nucleus: AnteriorOlfactoryNucleus {
                        location: "Olfactory peduncle".to_string(),
                        contralateral_connections: true,
                    },
                    olfactory_tubercle: OlfactoryTubercle {
                        location: "Ventral striatum".to_string(),
                        reward_integration: 1.0,
                    },
                },
                secondary_projections: SecondaryOlfactoryProjections {
                    orbitofrontal_cortex: OrbitofrontalCortexOlfactory {
                        conscious_perception: 1.0,
                        odor_identification: 1.0,
                        hedonic_evaluation: 1.0,
                    },
                    amygdala: AmygdalaOlfactory {
                        emotional_processing: 1.0,
                        learned_associations: 1.0,
                    },
                    hippocampus: HippocampusOlfactory {
                        olfactory_memory: 1.0,
                        spatial_context: 1.0,
                    },
                    hypothalamus: HypothalamusOlfactory {
                        autonomic_responses: 1.0,
                        feeding_behavior: 1.0,
                    },
                },
            },
            smell_perception: SmellPerception {
                detection_threshold: DetectionThreshold {
                    threshold_ppm: 0.001,
                    sensitivity_score: 1.0,
                },
                discrimination_ability: 1.0,
                adaptation_rate: 1.0,
                odor_memory: OdorMemory {
                    capacity: 10000.0,
                    recall_accuracy: 0.65,
                    emotional_association_strength: 1.0,
                },
            },
        }
    }

    pub fn estimate_odor_detection_capacity(&self) -> f64 {
        self.olfactory_receptors.total_receptor_types as f64
            * self.smell_perception.detection_threshold.sensitivity_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_olfactory_system_creation() {
        let system = OlfactorySensorySystem::new_healthy();
        assert_eq!(system.olfactory_receptors.total_receptor_types, 400);
    }

    #[test]
    fn test_odor_detection() {
        let system = OlfactorySensorySystem::new_healthy();
        let capacity = system.estimate_odor_detection_capacity();
        assert!(capacity > 0.0);
    }

    #[test]
    fn test_olfactory_neurons() {
        let system = OlfactorySensorySystem::new_healthy();
        assert_eq!(system.olfactory_epithelium.cell_types.olfactory_sensory_neurons.total_count, 1e7);
    }
}

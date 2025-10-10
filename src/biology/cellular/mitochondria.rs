use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mitochondrion {
    pub structure: MitochondrialStructure,
    pub energy_production: EnergyProduction,
    pub electron_transport_chain: ElectronTransportChain,
    pub oxidative_phosphorylation: OxidativePhosphorylation,
    pub metabolic_pathways: MitochondrialMetabolism,
    pub dynamics: MitochondrialDynamics,
    pub quality_control: MitochondrialQualityControl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialStructure {
    pub outer_membrane: Membrane,
    pub inner_membrane: Membrane,
    pub intermembrane_space_volume_fl: f64,
    pub matrix_volume_fl: f64,
    pub cristae_density: f64,
    pub membrane_potential_mv: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membrane {
    pub surface_area_um2: f64,
    pub lipid_composition: LipidComposition,
    pub protein_density_per_um2: f64,
    pub permeability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidComposition {
    pub cardiolipin_percent: f64,
    pub phosphatidylcholine_percent: f64,
    pub phosphatidylethanolamine_percent: f64,
    pub phosphatidylinositol_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProduction {
    pub atp_production_rate_pmol_s: f64,
    pub atp_pool_size_pmol: f64,
    pub adp_concentration_mm: f64,
    pub atp_adp_ratio: f64,
    pub nadh_concentration_mm: f64,
    pub nad_plus_concentration_mm: f64,
    pub nadh_nad_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronTransportChain {
    pub complex_i: ETCComplex,
    pub complex_ii: ETCComplex,
    pub complex_iii: ETCComplex,
    pub complex_iv: ETCComplex,
    pub complex_v: ATPSynthase,
    pub coenzyme_q_pool: f64,
    pub cytochrome_c_pool: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ETCComplex {
    pub activity_level: f64,
    pub protein_amount: f64,
    pub electron_transfer_rate: f64,
    pub proton_pumping_efficiency: f64,
    pub ros_production_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATPSynthase {
    pub f0_subunit: f64,
    pub f1_subunit: f64,
    pub rotation_rate_per_second: f64,
    pub atp_synthesis_rate_pmol_s: f64,
    pub coupling_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxidativePhosphorylation {
    pub oxygen_consumption_rate_pmol_s: f64,
    pub proton_motive_force_mv: f64,
    pub respiratory_control_ratio: f64,
    pub p_o_ratio: f64,
    pub uncoupling_protein_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialMetabolism {
    pub tca_cycle: TCACycle,
    pub beta_oxidation: BetaOxidation,
    pub amino_acid_metabolism: AminoAcidMetabolism,
    pub ketone_body_production: KetoneProduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCACycle {
    pub citrate_synthase_activity: f64,
    pub isocitrate_dehydrogenase_activity: f64,
    pub alpha_ketoglutarate_dehydrogenase_activity: f64,
    pub succinate_dehydrogenase_activity: f64,
    pub malate_dehydrogenase_activity: f64,
    pub cycle_flux_rate: f64,
    pub nadh_production_rate: f64,
    pub fadh2_production_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaOxidation {
    pub fatty_acid_uptake_rate: f64,
    pub cpt1_activity: f64,
    pub acyl_coa_dehydrogenase_activity: f64,
    pub acetyl_coa_production_rate: f64,
    pub metabolic_flexibility: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AminoAcidMetabolism {
    pub glutamate_dehydrogenase_activity: f64,
    pub branched_chain_amino_acid_oxidation: f64,
    pub urea_cycle_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KetoneProduction {
    pub acetoacetate_production_rate: f64,
    pub beta_hydroxybutyrate_production_rate: f64,
    pub hmg_coa_synthase_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialDynamics {
    pub fusion_rate: f64,
    pub fission_rate: f64,
    pub mitofusin1_expression: f64,
    pub mitofusin2_expression: f64,
    pub opa1_expression: f64,
    pub drp1_expression: f64,
    pub network_connectivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialQualityControl {
    pub mitophagy_rate: f64,
    pub pink1_parkin_pathway: f64,
    pub mitochondrial_proteases: Proteases,
    pub unfolded_protein_response: f64,
    pub antioxidant_defenses: AntioxidantDefenses,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proteases {
    pub yme1l_activity: f64,
    pub oma1_activity: f64,
    pub lonp1_activity: f64,
    pub clpp_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntioxidantDefenses {
    pub superoxide_dismutase2_activity: f64,
    pub catalase_activity: f64,
    pub glutathione_peroxidase_activity: f64,
    pub glutathione_pool_mm: f64,
    pub peroxiredoxin_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialDNA {
    pub copy_number: u32,
    pub mutation_rate: f64,
    pub deletion_count: u32,
    pub heteroplasmy_level: f64,
    pub transcription_rate: f64,
    pub mtdna_encoded_proteins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactiveOxygenSpecies {
    pub superoxide_production_pmol_s: f64,
    pub hydrogen_peroxide_concentration_um: f64,
    pub hydroxyl_radical_level: f64,
    pub total_ros_burden: f64,
    pub oxidative_damage_markers: OxidativeDamage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxidativeDamage {
    pub protein_carbonyl_content: f64,
    pub lipid_peroxidation_tbars: f64,
    pub eight_oxo_dg_level: f64,
    pub nitrotyrosine_level: f64,
}

impl Mitochondrion {
    pub fn new_healthy() -> Self {
        Self {
            structure: MitochondrialStructure::new_normal(),
            energy_production: EnergyProduction::new_optimal(),
            electron_transport_chain: ElectronTransportChain::new_normal(),
            oxidative_phosphorylation: OxidativePhosphorylation::new_efficient(),
            metabolic_pathways: MitochondrialMetabolism::new_active(),
            dynamics: MitochondrialDynamics::new_balanced(),
            quality_control: MitochondrialQualityControl::new_robust(),
        }
    }

    pub fn calculate_respiratory_capacity(&self) -> f64 {
        let etc_efficiency =
            (self.electron_transport_chain.complex_i.activity_level +
             self.electron_transport_chain.complex_iii.activity_level +
             self.electron_transport_chain.complex_iv.activity_level) / 3.0;

        let coupling = self.oxidative_phosphorylation.respiratory_control_ratio / 10.0;

        (etc_efficiency * coupling).clamp(0.0, 1.0)
    }

    pub fn calculate_bioenergetic_health_index(&self) -> f64 {
        let atp_production_score = (self.energy_production.atp_production_rate_pmol_s / 100.0).min(1.0);
        let membrane_potential_score = ((self.structure.membrane_potential_mv.abs() - 140.0) / 40.0).clamp(0.0, 1.0);
        let respiratory_capacity = self.calculate_respiratory_capacity();
        let quality_control_score = self.quality_control.mitophagy_rate;

        ((atp_production_score + membrane_potential_score + respiratory_capacity + quality_control_score) / 4.0 * 100.0)
            .clamp(0.0, 100.0)
    }

    pub fn assess_dysfunction_level(&self) -> MitochondrialDysfunction {
        let bhi = self.calculate_bioenergetic_health_index();

        if bhi > 80.0 {
            MitochondrialDysfunction::Healthy
        } else if bhi > 60.0 {
            MitochondrialDysfunction::Mild
        } else if bhi > 40.0 {
            MitochondrialDysfunction::Moderate
        } else {
            MitochondrialDysfunction::Severe
        }
    }

    pub fn simulate_atp_production(&mut self, substrate_availability: f64, oxygen_level: f64) {
        let substrate_factor = substrate_availability.clamp(0.0, 1.0);
        let oxygen_factor = oxygen_level.clamp(0.0, 1.0);

        let etc_activity = (substrate_factor + oxygen_factor) / 2.0;

        self.electron_transport_chain.complex_i.activity_level = etc_activity;
        self.electron_transport_chain.complex_iii.activity_level = etc_activity * 0.95;
        self.electron_transport_chain.complex_iv.activity_level = etc_activity * oxygen_factor;

        let protons_pumped = etc_activity * 10.0;
        self.oxidative_phosphorylation.proton_motive_force_mv = protons_pumped * 18.0;

        self.energy_production.atp_production_rate_pmol_s =
            self.electron_transport_chain.complex_v.atp_synthesis_rate_pmol_s * etc_activity;

        self.energy_production.atp_adp_ratio = self.energy_production.atp_production_rate_pmol_s / 20.0;
    }

    pub fn induce_oxidative_stress(&mut self, stressor_intensity: f64) {
        let intensity = stressor_intensity.clamp(0.0, 10.0);

        self.electron_transport_chain.complex_i.ros_production_rate += intensity * 0.5;
        self.electron_transport_chain.complex_iii.ros_production_rate += intensity * 0.3;

        self.structure.membrane_potential_mv *= 1.0 - (intensity * 0.02);

        self.quality_control.mitophagy_rate += intensity * 0.1;
    }

    pub fn activate_mitochondrial_biogenesis(&mut self, pgc1_alpha_activity: f64) {
        let activity = pgc1_alpha_activity.clamp(0.0, 1.0);

        self.structure.cristae_density += activity * 0.1;
        self.electron_transport_chain.complex_i.protein_amount += activity * 0.15;
        self.electron_transport_chain.complex_iii.protein_amount += activity * 0.15;
        self.electron_transport_chain.complex_iv.protein_amount += activity * 0.15;

        self.metabolic_pathways.tca_cycle.citrate_synthase_activity += activity * 0.2;
    }
}

impl MitochondrialStructure {
    pub fn new_normal() -> Self {
        Self {
            outer_membrane: Membrane {
                surface_area_um2: 10.0,
                lipid_composition: LipidComposition {
                    cardiolipin_percent: 20.0,
                    phosphatidylcholine_percent: 40.0,
                    phosphatidylethanolamine_percent: 30.0,
                    phosphatidylinositol_percent: 10.0,
                },
                protein_density_per_um2: 10000.0,
                permeability: 0.8,
            },
            inner_membrane: Membrane {
                surface_area_um2: 50.0,
                lipid_composition: LipidComposition {
                    cardiolipin_percent: 20.0,
                    phosphatidylcholine_percent: 40.0,
                    phosphatidylethanolamine_percent: 30.0,
                    phosphatidylinositol_percent: 10.0,
                },
                protein_density_per_um2: 30000.0,
                permeability: 0.1,
            },
            intermembrane_space_volume_fl: 0.5,
            matrix_volume_fl: 2.0,
            cristae_density: 1.0,
            membrane_potential_mv: -180.0,
        }
    }
}

impl EnergyProduction {
    pub fn new_optimal() -> Self {
        Self {
            atp_production_rate_pmol_s: 100.0,
            atp_pool_size_pmol: 5000.0,
            adp_concentration_mm: 0.5,
            atp_adp_ratio: 5.0,
            nadh_concentration_mm: 0.3,
            nad_plus_concentration_mm: 1.5,
            nadh_nad_ratio: 0.2,
        }
    }
}

impl ElectronTransportChain {
    pub fn new_normal() -> Self {
        Self {
            complex_i: ETCComplex {
                activity_level: 1.0,
                protein_amount: 1.0,
                electron_transfer_rate: 100.0,
                proton_pumping_efficiency: 0.9,
                ros_production_rate: 0.1,
            },
            complex_ii: ETCComplex {
                activity_level: 1.0,
                protein_amount: 1.0,
                electron_transfer_rate: 80.0,
                proton_pumping_efficiency: 0.0,
                ros_production_rate: 0.05,
            },
            complex_iii: ETCComplex {
                activity_level: 1.0,
                protein_amount: 1.0,
                electron_transfer_rate: 95.0,
                proton_pumping_efficiency: 0.85,
                ros_production_rate: 0.15,
            },
            complex_iv: ETCComplex {
                activity_level: 1.0,
                protein_amount: 1.0,
                electron_transfer_rate: 90.0,
                proton_pumping_efficiency: 0.8,
                ros_production_rate: 0.02,
            },
            complex_v: ATPSynthase {
                f0_subunit: 1.0,
                f1_subunit: 1.0,
                rotation_rate_per_second: 150.0,
                atp_synthesis_rate_pmol_s: 100.0,
                coupling_efficiency: 0.9,
            },
            coenzyme_q_pool: 1.0,
            cytochrome_c_pool: 1.0,
        }
    }
}

impl OxidativePhosphorylation {
    pub fn new_efficient() -> Self {
        Self {
            oxygen_consumption_rate_pmol_s: 50.0,
            proton_motive_force_mv: 180.0,
            respiratory_control_ratio: 8.0,
            p_o_ratio: 2.5,
            uncoupling_protein_activity: 0.1,
        }
    }
}

impl MitochondrialMetabolism {
    pub fn new_active() -> Self {
        Self {
            tca_cycle: TCACycle {
                citrate_synthase_activity: 1.0,
                isocitrate_dehydrogenase_activity: 1.0,
                alpha_ketoglutarate_dehydrogenase_activity: 1.0,
                succinate_dehydrogenase_activity: 1.0,
                malate_dehydrogenase_activity: 1.0,
                cycle_flux_rate: 1.0,
                nadh_production_rate: 3.0,
                fadh2_production_rate: 1.0,
            },
            beta_oxidation: BetaOxidation {
                fatty_acid_uptake_rate: 1.0,
                cpt1_activity: 1.0,
                acyl_coa_dehydrogenase_activity: 1.0,
                acetyl_coa_production_rate: 1.0,
                metabolic_flexibility: 1.0,
            },
            amino_acid_metabolism: AminoAcidMetabolism {
                glutamate_dehydrogenase_activity: 1.0,
                branched_chain_amino_acid_oxidation: 1.0,
                urea_cycle_activity: 1.0,
            },
            ketone_body_production: KetoneProduction {
                acetoacetate_production_rate: 0.0,
                beta_hydroxybutyrate_production_rate: 0.0,
                hmg_coa_synthase_activity: 0.1,
            },
        }
    }
}

impl MitochondrialDynamics {
    pub fn new_balanced() -> Self {
        Self {
            fusion_rate: 0.5,
            fission_rate: 0.5,
            mitofusin1_expression: 1.0,
            mitofusin2_expression: 1.0,
            opa1_expression: 1.0,
            drp1_expression: 1.0,
            network_connectivity: 0.8,
        }
    }
}

impl MitochondrialQualityControl {
    pub fn new_robust() -> Self {
        Self {
            mitophagy_rate: 0.1,
            pink1_parkin_pathway: 1.0,
            mitochondrial_proteases: Proteases {
                yme1l_activity: 1.0,
                oma1_activity: 1.0,
                lonp1_activity: 1.0,
                clpp_activity: 1.0,
            },
            unfolded_protein_response: 0.0,
            antioxidant_defenses: AntioxidantDefenses {
                superoxide_dismutase2_activity: 1.0,
                catalase_activity: 1.0,
                glutathione_peroxidase_activity: 1.0,
                glutathione_pool_mm: 5.0,
                peroxiredoxin_activity: 1.0,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MitochondrialDysfunction {
    Healthy,
    Mild,
    Moderate,
    Severe,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mitochondrion_creation() {
        let mito = Mitochondrion::new_healthy();
        assert_eq!(mito.structure.membrane_potential_mv, -180.0);
    }

    #[test]
    fn test_respiratory_capacity() {
        let mito = Mitochondrion::new_healthy();
        let capacity = mito.calculate_respiratory_capacity();
        assert!(capacity > 0.7);
    }

    #[test]
    fn test_bioenergetic_health_index() {
        let mito = Mitochondrion::new_healthy();
        let bhi = mito.calculate_bioenergetic_health_index();
        assert!(bhi > 70.0);
    }

    #[test]
    fn test_atp_production_simulation() {
        let mut mito = Mitochondrion::new_healthy();
        mito.simulate_atp_production(1.0, 1.0);
        assert!(mito.energy_production.atp_production_rate_pmol_s > 50.0);
    }

    #[test]
    fn test_oxidative_stress() {
        let mut mito = Mitochondrion::new_healthy();
        let baseline_ros = mito.electron_transport_chain.complex_i.ros_production_rate;

        mito.induce_oxidative_stress(5.0);

        assert!(mito.electron_transport_chain.complex_i.ros_production_rate > baseline_ros);
    }

    #[test]
    fn test_mitochondrial_biogenesis() {
        let mut mito = Mitochondrion::new_healthy();
        let baseline_cristae = mito.structure.cristae_density;

        mito.activate_mitochondrial_biogenesis(0.8);

        assert!(mito.structure.cristae_density > baseline_cristae);
    }

    #[test]
    fn test_dysfunction_assessment() {
        let mito = Mitochondrion::new_healthy();
        let dysfunction = mito.assess_dysfunction_level();
        assert!(matches!(dysfunction, MitochondrialDysfunction::Healthy | MitochondrialDysfunction::Mild));
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nephron {
    pub nephron_type: NephronType,
    pub renal_corpuscle: RenalCorpuscle,
    pub proximal_tubule: ProximalTubule,
    pub loop_of_henle: LoopOfHenle,
    pub distal_tubule: DistalTubule,
    pub collecting_duct: CollectingDuct,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NephronType {
    Cortical,
    Juxtamedullary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalCorpuscle {
    pub glomerulus: Glomerulus,
    pub bowmans_capsule: BowmansCapsule,
    pub juxtaglomerular_apparatus: JuxtaglomerularApparatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glomerulus {
    pub capillary_surface_area_cm2: f64,
    pub capillary_pressure_mmhg: f64,
    pub filtration_rate_ml_per_min: f64,
    pub podocytes: Vec<Podocyte>,
    pub mesangial_cells: Vec<MesangialCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Podocyte {
    pub foot_processes: usize,
    pub slit_diaphragm_width_nm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MesangialCell {
    pub contractile: bool,
    pub phagocytic: bool,
    pub matrix_production: f64,
}

impl Glomerulus {
    pub fn new() -> Self {
        Self {
            capillary_surface_area_cm2: 1.5,
            capillary_pressure_mmhg: 55.0,
            filtration_rate_ml_per_min: 125.0,
            podocytes: vec![Podocyte {
                foot_processes: 1000,
                slit_diaphragm_width_nm: 40.0,
            }],
            mesangial_cells: vec![MesangialCell {
                contractile: true,
                phagocytic: true,
                matrix_production: 1.0,
            }],
        }
    }

    pub fn filtration_fraction(&self, renal_plasma_flow: f64) -> f64 {
        self.filtration_rate_ml_per_min / renal_plasma_flow
    }
}

impl Default for Glomerulus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowmansCapsule {
    pub capsular_space_volume_ul: f64,
    pub capsular_pressure_mmhg: f64,
}

impl BowmansCapsule {
    pub fn new() -> Self {
        Self {
            capsular_space_volume_ul: 5.0,
            capsular_pressure_mmhg: 15.0,
        }
    }
}

impl Default for BowmansCapsule {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JuxtaglomerularApparatus {
    pub juxtaglomerular_cells: JuxtaglomerularCells,
    pub macula_densa: MaculaDensa,
    pub extraglomerular_mesangial_cells: Vec<MesangialCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JuxtaglomerularCells {
    pub renin_production: f64,
    pub renin_secretion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaculaDensa {
    pub sodium_sensing: f64,
    pub chloride_sensing: f64,
    pub tubuloglomerular_feedback_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximalTubule {
    pub length_mm: f64,
    pub diameter_um: f64,
    pub segments: ProximalTubuleSegments,
    pub reabsorption: ProximalReabsorption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximalTubuleSegments {
    pub s1_pars_convoluta: bool,
    pub s2_pars_convoluta: bool,
    pub s3_pars_recta: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximalReabsorption {
    pub sodium_percent: f64,
    pub water_percent: f64,
    pub glucose_percent: f64,
    pub amino_acids_percent: f64,
    pub bicarbonate_percent: f64,
    pub phosphate_percent: f64,
}

impl ProximalReabsorption {
    pub fn new() -> Self {
        Self {
            sodium_percent: 65.0,
            water_percent: 65.0,
            glucose_percent: 100.0,
            amino_acids_percent: 100.0,
            bicarbonate_percent: 80.0,
            phosphate_percent: 80.0,
        }
    }
}

impl Default for ProximalReabsorption {
    fn default() -> Self {
        Self::new()
    }
}

impl ProximalTubule {
    pub fn new() -> Self {
        Self {
            length_mm: 14.0,
            diameter_um: 60.0,
            segments: ProximalTubuleSegments {
                s1_pars_convoluta: true,
                s2_pars_convoluta: true,
                s3_pars_recta: true,
            },
            reabsorption: ProximalReabsorption::new(),
        }
    }
}

impl Default for ProximalTubule {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopOfHenle {
    pub descending_limb: DescendingLimb,
    pub thin_ascending_limb: ThinAscendingLimb,
    pub thick_ascending_limb: ThickAscendingLimb,
    pub countercurrent_multiplier_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescendingLimb {
    pub length_mm: f64,
    pub water_permeable: bool,
    pub sodium_permeable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinAscendingLimb {
    pub length_mm: f64,
    pub water_permeable: bool,
    pub sodium_permeable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThickAscendingLimb {
    pub length_mm: f64,
    pub nkcc2_transporters: usize,
    pub sodium_reabsorption_percent: f64,
    pub potassium_recycling: bool,
}

impl LoopOfHenle {
    pub fn new(nephron_type: NephronType) -> Self {
        let descending_length = match nephron_type {
            NephronType::Cortical => 2.0,
            NephronType::Juxtamedullary => 14.0,
        };

        Self {
            descending_limb: DescendingLimb {
                length_mm: descending_length,
                water_permeable: true,
                sodium_permeable: false,
            },
            thin_ascending_limb: ThinAscendingLimb {
                length_mm: descending_length * 0.5,
                water_permeable: false,
                sodium_permeable: true,
            },
            thick_ascending_limb: ThickAscendingLimb {
                length_mm: 9.0,
                nkcc2_transporters: 10000,
                sodium_reabsorption_percent: 25.0,
                potassium_recycling: true,
            },
            countercurrent_multiplier_active: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistalTubule {
    pub length_mm: f64,
    pub early_dct: EarlyDistalTubule,
    pub late_dct: LateDistalTubule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyDistalTubule {
    pub ncc_transporters: usize,
    pub sodium_reabsorption_percent: f64,
    pub calcium_reabsorption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LateDistalTubule {
    pub principal_cells: Vec<PrincipalCell>,
    pub intercalated_cells: Vec<IntercalatedCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrincipalCell {
    pub enac_channels: usize,
    pub romk_channels: usize,
    pub aldosterone_sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntercalatedCell {
    pub cell_type: IntercalatedCellType,
    pub h_atpase_pumps: usize,
    pub hco3_transporters: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IntercalatedCellType {
    TypeA,
    TypeB,
}

impl DistalTubule {
    pub fn new() -> Self {
        Self {
            length_mm: 5.0,
            early_dct: EarlyDistalTubule {
                ncc_transporters: 5000,
                sodium_reabsorption_percent: 5.0,
                calcium_reabsorption: true,
            },
            late_dct: LateDistalTubule {
                principal_cells: vec![PrincipalCell {
                    enac_channels: 1000,
                    romk_channels: 500,
                    aldosterone_sensitive: true,
                }],
                intercalated_cells: vec![IntercalatedCell {
                    cell_type: IntercalatedCellType::TypeA,
                    h_atpase_pumps: 500,
                    hco3_transporters: 300,
                }],
            },
        }
    }
}

impl Default for DistalTubule {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectingDuct {
    pub length_mm: f64,
    pub cortical_segment: CorticalCollectingDuct,
    pub medullary_segment: MedullaryCollectingDuct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorticalCollectingDuct {
    pub principal_cells: Vec<PrincipalCell>,
    pub intercalated_cells: Vec<IntercalatedCell>,
    pub vasopressin_sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedullaryCollectingDuct {
    pub principal_cells: Vec<PrincipalCell>,
    pub aquaporin2_channels: usize,
    pub urea_permeability: f64,
}

impl CollectingDuct {
    pub fn new() -> Self {
        Self {
            length_mm: 20.0,
            cortical_segment: CorticalCollectingDuct {
                principal_cells: vec![PrincipalCell {
                    enac_channels: 1000,
                    romk_channels: 500,
                    aldosterone_sensitive: true,
                }],
                intercalated_cells: vec![IntercalatedCell {
                    cell_type: IntercalatedCellType::TypeA,
                    h_atpase_pumps: 500,
                    hco3_transporters: 300,
                }],
                vasopressin_sensitive: true,
            },
            medullary_segment: MedullaryCollectingDuct {
                principal_cells: vec![PrincipalCell {
                    enac_channels: 1000,
                    romk_channels: 500,
                    aldosterone_sensitive: true,
                }],
                aquaporin2_channels: 10000,
                urea_permeability: 0.5,
            },
        }
    }
}

impl Default for CollectingDuct {
    fn default() -> Self {
        Self::new()
    }
}

impl Nephron {
    pub fn new_cortical() -> Self {
        Self {
            nephron_type: NephronType::Cortical,
            renal_corpuscle: RenalCorpuscle {
                glomerulus: Glomerulus::new(),
                bowmans_capsule: BowmansCapsule::new(),
                juxtaglomerular_apparatus: JuxtaglomerularApparatus {
                    juxtaglomerular_cells: JuxtaglomerularCells {
                        renin_production: 1.0,
                        renin_secretion_rate: 1.0,
                    },
                    macula_densa: MaculaDensa {
                        sodium_sensing: 1.0,
                        chloride_sensing: 1.0,
                        tubuloglomerular_feedback_active: true,
                    },
                    extraglomerular_mesangial_cells: Vec::new(),
                },
            },
            proximal_tubule: ProximalTubule::new(),
            loop_of_henle: LoopOfHenle::new(NephronType::Cortical),
            distal_tubule: DistalTubule::new(),
            collecting_duct: CollectingDuct::new(),
        }
    }

    pub fn new_juxtamedullary() -> Self {
        Self {
            nephron_type: NephronType::Juxtamedullary,
            renal_corpuscle: RenalCorpuscle {
                glomerulus: Glomerulus::new(),
                bowmans_capsule: BowmansCapsule::new(),
                juxtaglomerular_apparatus: JuxtaglomerularApparatus {
                    juxtaglomerular_cells: JuxtaglomerularCells {
                        renin_production: 1.0,
                        renin_secretion_rate: 1.0,
                    },
                    macula_densa: MaculaDensa {
                        sodium_sensing: 1.0,
                        chloride_sensing: 1.0,
                        tubuloglomerular_feedback_active: true,
                    },
                    extraglomerular_mesangial_cells: Vec::new(),
                },
            },
            proximal_tubule: ProximalTubule::new(),
            loop_of_henle: LoopOfHenle::new(NephronType::Juxtamedullary),
            distal_tubule: DistalTubule::new(),
            collecting_duct: CollectingDuct::new(),
        }
    }

    pub fn total_length_mm(&self) -> f64 {
        self.proximal_tubule.length_mm
            + self.loop_of_henle.descending_limb.length_mm
            + self.loop_of_henle.thin_ascending_limb.length_mm
            + self.loop_of_henle.thick_ascending_limb.length_mm
            + self.distal_tubule.length_mm
            + self.collecting_duct.length_mm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cortical_nephron() {
        let nephron = Nephron::new_cortical();
        assert!(matches!(nephron.nephron_type, NephronType::Cortical));
        assert!(nephron.total_length_mm() > 0.0);
    }

    #[test]
    fn test_juxtamedullary_nephron() {
        let nephron = Nephron::new_juxtamedullary();
        assert!(matches!(nephron.nephron_type, NephronType::Juxtamedullary));
        assert!(nephron.total_length_mm() > nephron.proximal_tubule.length_mm);
    }

    #[test]
    fn test_glomerular_filtration() {
        let glom = Glomerulus::new();
        let ff = glom.filtration_fraction(600.0);
        assert!((ff - 0.208).abs() < 0.01);
    }

    #[test]
    fn test_proximal_reabsorption() {
        let reabs = ProximalReabsorption::new();
        assert_eq!(reabs.glucose_percent, 100.0);
        assert!(reabs.sodium_percent > 60.0);
    }

    #[test]
    fn test_loop_length_varies_by_type() {
        let cortical = LoopOfHenle::new(NephronType::Cortical);
        let juxta = LoopOfHenle::new(NephronType::Juxtamedullary);

        assert!(juxta.descending_limb.length_mm > cortical.descending_limb.length_mm);
    }
}

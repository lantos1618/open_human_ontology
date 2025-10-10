use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotilityPattern {
    Peristalsis,
    Segmentation,
    MassMovement,
    HaustalChurning,
    MigrationMotorComplex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GIRegion {
    Esophagus,
    Stomach,
    Duodenum,
    Jejunum,
    Ileum,
    Cecum,
    Colon,
    Rectum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GutMotility {
    pub region: GIRegion,
    pub contraction_frequency_per_min: f64,
    pub contraction_amplitude_mmhg: f64,
    pub propagation_velocity_cm_s: f64,
    pub pattern: MotilityPattern,
    pub transit_time_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntericNervousSystem {
    pub myenteric_plexus_neurons: u64,
    pub submucosal_plexus_neurons: u64,
    pub neurotransmitters_nm: Vec<EntericNeurotransmitter>,
    pub intrinsic_reflexes: Vec<EntericReflex>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntericNeurotransmitter {
    Acetylcholine,
    Serotonin,
    DopamineGI,
    SubstanceP,
    VIP,
    NO,
    GABA,
    ATP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntericReflex {
    Peristaltic,
    Receptive,
    GastroColonic,
    GastroIleal,
    IntestinalIntestinal,
    Defecation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmoothMuscleLayers {
    pub longitudinal_layer_thickness_mm: f64,
    pub circular_layer_thickness_mm: f64,
    pub muscularis_mucosa_thickness_mm: f64,
    pub resting_tension_g: f64,
    pub maximal_contraction_g: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GastricEmptying {
    pub half_emptying_time_min: f64,
    pub pyloric_resistance_mmhg_s_ml: f64,
    pub antral_contraction_frequency: f64,
    pub meal_volume_ml: f64,
    pub meal_composition: MealComposition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealComposition {
    pub carbohydrate_percent: f64,
    pub protein_percent: f64,
    pub fat_percent: f64,
    pub fiber_g: f64,
    pub osmolality_mosm_kg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColonTransit {
    pub total_transit_time_hours: f64,
    pub ascending_colon_hours: f64,
    pub transverse_colon_hours: f64,
    pub descending_colon_hours: f64,
    pub rectosigmoid_hours: f64,
    pub stool_consistency: BristolStoolScale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BristolStoolScale {
    Type1SeparateHardLumps,
    Type2SausageShapedButLumpy,
    Type3LikeSausageWithCracks,
    Type4SmoothAndSoft,
    Type5SoftBlobsClearCut,
    Type6FluffyPiecesRagged,
    Type7WateryNoSolidPieces,
}

impl GutMotility {
    pub fn new_esophageal() -> Self {
        Self {
            region: GIRegion::Esophagus,
            contraction_frequency_per_min: 0.0,
            contraction_amplitude_mmhg: 60.0,
            propagation_velocity_cm_s: 4.0,
            pattern: MotilityPattern::Peristalsis,
            transit_time_hours: 0.015,
        }
    }

    pub fn new_gastric() -> Self {
        Self {
            region: GIRegion::Stomach,
            contraction_frequency_per_min: 3.0,
            contraction_amplitude_mmhg: 80.0,
            propagation_velocity_cm_s: 1.0,
            pattern: MotilityPattern::Peristalsis,
            transit_time_hours: 3.0,
        }
    }

    pub fn new_small_intestine() -> Self {
        Self {
            region: GIRegion::Jejunum,
            contraction_frequency_per_min: 12.0,
            contraction_amplitude_mmhg: 40.0,
            propagation_velocity_cm_s: 2.0,
            pattern: MotilityPattern::Segmentation,
            transit_time_hours: 4.0,
        }
    }

    pub fn new_colon() -> Self {
        Self {
            region: GIRegion::Colon,
            contraction_frequency_per_min: 6.0,
            contraction_amplitude_mmhg: 50.0,
            propagation_velocity_cm_s: 0.5,
            pattern: MotilityPattern::HaustalChurning,
            transit_time_hours: 24.0,
        }
    }

    pub fn calculate_motility_index(&self) -> f64 {
        self.contraction_frequency_per_min * self.contraction_amplitude_mmhg
    }

    pub fn is_hypomotile(&self) -> bool {
        let baseline = match self.region {
            GIRegion::Stomach => 3.0,
            GIRegion::Jejunum | GIRegion::Ileum => 12.0,
            GIRegion::Duodenum => 12.0,
            GIRegion::Colon => 6.0,
            _ => 1.0,
        };
        self.contraction_frequency_per_min < baseline * 0.5
    }

    pub fn is_hypermotile(&self) -> bool {
        let baseline = match self.region {
            GIRegion::Stomach => 3.0,
            GIRegion::Jejunum | GIRegion::Ileum => 12.0,
            GIRegion::Duodenum => 12.0,
            GIRegion::Colon => 6.0,
            _ => 1.0,
        };
        self.contraction_frequency_per_min > baseline * 2.0
    }
}

impl EntericNervousSystem {
    pub fn new_normal() -> Self {
        Self {
            myenteric_plexus_neurons: 100_000_000,
            submucosal_plexus_neurons: 50_000_000,
            neurotransmitters_nm: vec![
                EntericNeurotransmitter::Acetylcholine,
                EntericNeurotransmitter::Serotonin,
                EntericNeurotransmitter::VIP,
                EntericNeurotransmitter::NO,
                EntericNeurotransmitter::SubstanceP,
            ],
            intrinsic_reflexes: vec![
                EntericReflex::Peristaltic,
                EntericReflex::Receptive,
                EntericReflex::GastroColonic,
            ],
        }
    }

    pub fn total_neurons(&self) -> u64 {
        self.myenteric_plexus_neurons + self.submucosal_plexus_neurons
    }

    pub fn has_neurotransmitter(&self, nt: EntericNeurotransmitter) -> bool {
        self.neurotransmitters_nm.contains(&nt)
    }

    pub fn has_reflex(&self, reflex: EntericReflex) -> bool {
        self.intrinsic_reflexes.contains(&reflex)
    }

    pub fn is_functional(&self) -> bool {
        self.total_neurons() > 50_000_000 && self.neurotransmitters_nm.len() >= 3
    }
}

impl GastricEmptying {
    pub fn new_normal_solid() -> Self {
        Self {
            half_emptying_time_min: 90.0,
            pyloric_resistance_mmhg_s_ml: 0.1,
            antral_contraction_frequency: 3.0,
            meal_volume_ml: 500.0,
            meal_composition: MealComposition {
                carbohydrate_percent: 50.0,
                protein_percent: 30.0,
                fat_percent: 20.0,
                fiber_g: 10.0,
                osmolality_mosm_kg: 300.0,
            },
        }
    }

    pub fn new_normal_liquid() -> Self {
        Self {
            half_emptying_time_min: 20.0,
            pyloric_resistance_mmhg_s_ml: 0.05,
            antral_contraction_frequency: 3.0,
            meal_volume_ml: 300.0,
            meal_composition: MealComposition {
                carbohydrate_percent: 10.0,
                protein_percent: 5.0,
                fat_percent: 5.0,
                fiber_g: 0.0,
                osmolality_mosm_kg: 280.0,
            },
        }
    }

    pub fn calculate_emptying_rate(&self) -> f64 {
        self.meal_volume_ml / self.half_emptying_time_min
    }

    pub fn is_delayed(&self) -> bool {
        self.half_emptying_time_min > 120.0
    }

    pub fn is_rapid(&self) -> bool {
        self.half_emptying_time_min < 30.0
    }

    pub fn caloric_load(&self) -> f64 {
        let carb_kcal = self.meal_composition.carbohydrate_percent * 4.0;
        let protein_kcal = self.meal_composition.protein_percent * 4.0;
        let fat_kcal = self.meal_composition.fat_percent * 9.0;
        (carb_kcal + protein_kcal + fat_kcal) * self.meal_volume_ml / 100.0
    }
}

impl ColonTransit {
    pub fn new_normal() -> Self {
        Self {
            total_transit_time_hours: 30.0,
            ascending_colon_hours: 8.0,
            transverse_colon_hours: 10.0,
            descending_colon_hours: 8.0,
            rectosigmoid_hours: 4.0,
            stool_consistency: BristolStoolScale::Type4SmoothAndSoft,
        }
    }

    pub fn is_constipated(&self) -> bool {
        self.total_transit_time_hours > 72.0
            || matches!(
                self.stool_consistency,
                BristolStoolScale::Type1SeparateHardLumps
                    | BristolStoolScale::Type2SausageShapedButLumpy
            )
    }

    pub fn has_diarrhea(&self) -> bool {
        self.total_transit_time_hours < 12.0
            || matches!(
                self.stool_consistency,
                BristolStoolScale::Type6FluffyPiecesRagged
                    | BristolStoolScale::Type7WateryNoSolidPieces
            )
    }

    pub fn is_normal(&self) -> bool {
        (24.0..=48.0).contains(&self.total_transit_time_hours)
            && matches!(
                self.stool_consistency,
                BristolStoolScale::Type3LikeSausageWithCracks
                    | BristolStoolScale::Type4SmoothAndSoft
                    | BristolStoolScale::Type5SoftBlobsClearCut
            )
    }
}

impl SmoothMuscleLayers {
    pub fn new_normal() -> Self {
        Self {
            longitudinal_layer_thickness_mm: 0.5,
            circular_layer_thickness_mm: 1.0,
            muscularis_mucosa_thickness_mm: 0.05,
            resting_tension_g: 5.0,
            maximal_contraction_g: 50.0,
        }
    }

    pub fn contractile_capacity(&self) -> f64 {
        self.maximal_contraction_g / self.resting_tension_g
    }

    pub fn total_muscle_thickness(&self) -> f64 {
        self.longitudinal_layer_thickness_mm
            + self.circular_layer_thickness_mm
            + self.muscularis_mucosa_thickness_mm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gastric_motility() {
        let motility = GutMotility::new_gastric();
        assert_eq!(motility.region, GIRegion::Stomach);
        assert_eq!(motility.contraction_frequency_per_min, 3.0);
    }

    #[test]
    fn test_motility_index() {
        let motility = GutMotility::new_gastric();
        let index = motility.calculate_motility_index();
        assert_eq!(index, 240.0);
    }

    #[test]
    fn test_enteric_nervous_system() {
        let ens = EntericNervousSystem::new_normal();
        assert_eq!(ens.total_neurons(), 150_000_000);
        assert!(ens.is_functional());
    }

    #[test]
    fn test_gastric_emptying() {
        let ge = GastricEmptying::new_normal_solid();
        assert_eq!(ge.half_emptying_time_min, 90.0);
        assert!(!ge.is_delayed());
        assert!(!ge.is_rapid());
    }

    #[test]
    fn test_colon_transit() {
        let transit = ColonTransit::new_normal();
        assert!(transit.is_normal());
        assert!(!transit.is_constipated());
        assert!(!transit.has_diarrhea());
    }

    #[test]
    fn test_constipation() {
        let mut transit = ColonTransit::new_normal();
        transit.total_transit_time_hours = 80.0;
        transit.stool_consistency = BristolStoolScale::Type1SeparateHardLumps;
        assert!(transit.is_constipated());
    }

    #[test]
    fn test_diarrhea() {
        let mut transit = ColonTransit::new_normal();
        transit.total_transit_time_hours = 8.0;
        transit.stool_consistency = BristolStoolScale::Type7WateryNoSolidPieces;
        assert!(transit.has_diarrhea());
    }

    #[test]
    fn test_smooth_muscle() {
        let muscle = SmoothMuscleLayers::new_normal();
        let capacity = muscle.contractile_capacity();
        assert_eq!(capacity, 10.0);
    }

    #[test]
    fn test_caloric_load() {
        let ge = GastricEmptying::new_normal_solid();
        let kcal = ge.caloric_load();
        assert!(kcal > 1000.0);
    }
}

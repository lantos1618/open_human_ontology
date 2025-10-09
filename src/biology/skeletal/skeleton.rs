use super::bone::{Bone, BoneType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skeleton {
    pub axial: AxialSkeleton,
    pub appendicular: AppendicularSkeleton,
    pub total_bone_mass_kg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxialSkeleton {
    pub skull: SkullBones,
    pub vertebral_column: VertebralColumn,
    pub rib_cage: RibCage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkullBones {
    pub cranium: Vec<Bone>,
    pub facial_bones: Vec<Bone>,
    pub auditory_ossicles: Vec<Bone>,
    pub hyoid: Bone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertebralColumn {
    pub cervical: Vec<Bone>,
    pub thoracic: Vec<Bone>,
    pub lumbar: Vec<Bone>,
    pub sacrum: Bone,
    pub coccyx: Bone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibCage {
    pub ribs: Vec<Bone>,
    pub sternum: Bone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendicularSkeleton {
    pub upper_limbs: UpperLimbs,
    pub lower_limbs: LowerLimbs,
    pub pectoral_girdles: PectoralGirdles,
    pub pelvic_girdle: PelvicGirdle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpperLimbs {
    pub left: UpperLimb,
    pub right: UpperLimb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpperLimb {
    pub humerus: Bone,
    pub radius: Bone,
    pub ulna: Bone,
    pub carpals: Vec<Bone>,
    pub metacarpals: Vec<Bone>,
    pub phalanges: Vec<Bone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowerLimbs {
    pub left: LowerLimb,
    pub right: LowerLimb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowerLimb {
    pub femur: Bone,
    pub tibia: Bone,
    pub fibula: Bone,
    pub patella: Bone,
    pub tarsals: Vec<Bone>,
    pub metatarsals: Vec<Bone>,
    pub phalanges: Vec<Bone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PectoralGirdles {
    pub left_clavicle: Bone,
    pub right_clavicle: Bone,
    pub left_scapula: Bone,
    pub right_scapula: Bone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PelvicGirdle {
    pub left_hip_bone: Bone,
    pub right_hip_bone: Bone,
}

impl Skeleton {
    pub fn new_adult() -> Self {
        let axial = AxialSkeleton::new_adult();
        let appendicular = AppendicularSkeleton::new_adult();

        let total_mass = axial.total_mass_kg() + appendicular.total_mass_kg();

        Skeleton {
            axial,
            appendicular,
            total_bone_mass_kg: total_mass,
        }
    }

    pub fn bone_count(&self) -> usize {
        self.axial.bone_count() + self.appendicular.bone_count()
    }

    pub fn update_total_mass(&mut self) {
        self.total_bone_mass_kg = self.axial.total_mass_kg() + self.appendicular.total_mass_kg();
    }

    pub fn average_bone_mineral_density(&self) -> f64 {
        let all_bones = self.all_bones();
        if all_bones.is_empty() {
            return 0.0;
        }

        let total_density: f64 = all_bones.iter()
            .map(|b| b.mineral_density_g_cm3)
            .sum();

        total_density / all_bones.len() as f64
    }

    pub fn all_bones(&self) -> Vec<&Bone> {
        let mut bones = Vec::new();
        bones.extend(self.axial.all_bones());
        bones.extend(self.appendicular.all_bones());
        bones
    }

    pub fn osteoporotic_bone_count(&self) -> usize {
        self.all_bones().iter()
            .filter(|b| b.is_osteoporotic())
            .count()
    }

    pub fn overall_skeletal_health_score(&self) -> f64 {
        let bones = self.all_bones();
        if bones.is_empty() {
            return 0.0;
        }

        let total_strength: f64 = bones.iter()
            .map(|b| b.assess_strength())
            .sum();

        total_strength / bones.len() as f64
    }
}

impl AxialSkeleton {
    pub fn new_adult() -> Self {
        AxialSkeleton {
            skull: SkullBones::new_adult(),
            vertebral_column: VertebralColumn::new_adult(),
            rib_cage: RibCage::new_adult(),
        }
    }

    pub fn bone_count(&self) -> usize {
        self.skull.bone_count()
            + self.vertebral_column.bone_count()
            + self.rib_cage.bone_count()
    }

    pub fn total_mass_kg(&self) -> f64 {
        (self.skull.total_mass_g()
            + self.vertebral_column.total_mass_g()
            + self.rib_cage.total_mass_g()) / 1000.0
    }

    pub fn all_bones(&self) -> Vec<&Bone> {
        let mut bones = Vec::new();
        bones.extend(&self.skull.cranium);
        bones.extend(&self.skull.facial_bones);
        bones.extend(&self.skull.auditory_ossicles);
        bones.push(&self.skull.hyoid);
        bones.extend(&self.vertebral_column.cervical);
        bones.extend(&self.vertebral_column.thoracic);
        bones.extend(&self.vertebral_column.lumbar);
        bones.push(&self.vertebral_column.sacrum);
        bones.push(&self.vertebral_column.coccyx);
        bones.extend(&self.rib_cage.ribs);
        bones.push(&self.rib_cage.sternum);
        bones
    }
}

impl SkullBones {
    pub fn new_adult() -> Self {
        let cranium = vec![
            Bone::new("Frontal".to_string(), BoneType::Flat),
            Bone::new("Parietal (L)".to_string(), BoneType::Flat),
            Bone::new("Parietal (R)".to_string(), BoneType::Flat),
            Bone::new("Temporal (L)".to_string(), BoneType::Flat),
            Bone::new("Temporal (R)".to_string(), BoneType::Flat),
            Bone::new("Occipital".to_string(), BoneType::Flat),
            Bone::new("Sphenoid".to_string(), BoneType::Irregular),
            Bone::new("Ethmoid".to_string(), BoneType::Irregular),
        ];

        let facial_bones = vec![
            Bone::new("Maxilla (L)".to_string(), BoneType::Irregular),
            Bone::new("Maxilla (R)".to_string(), BoneType::Irregular),
            Bone::new("Mandible".to_string(), BoneType::Irregular),
            Bone::new("Zygomatic (L)".to_string(), BoneType::Irregular),
            Bone::new("Zygomatic (R)".to_string(), BoneType::Irregular),
            Bone::new("Nasal (L)".to_string(), BoneType::Irregular),
            Bone::new("Nasal (R)".to_string(), BoneType::Irregular),
            Bone::new("Lacrimal (L)".to_string(), BoneType::Irregular),
            Bone::new("Lacrimal (R)".to_string(), BoneType::Irregular),
            Bone::new("Palatine (L)".to_string(), BoneType::Irregular),
            Bone::new("Palatine (R)".to_string(), BoneType::Irregular),
            Bone::new("Inferior Nasal Concha (L)".to_string(), BoneType::Irregular),
            Bone::new("Inferior Nasal Concha (R)".to_string(), BoneType::Irregular),
            Bone::new("Vomer".to_string(), BoneType::Irregular),
        ];

        let auditory_ossicles = vec![
            Bone::new("Malleus (L)".to_string(), BoneType::Irregular),
            Bone::new("Malleus (R)".to_string(), BoneType::Irregular),
            Bone::new("Incus (L)".to_string(), BoneType::Irregular),
            Bone::new("Incus (R)".to_string(), BoneType::Irregular),
            Bone::new("Stapes (L)".to_string(), BoneType::Irregular),
            Bone::new("Stapes (R)".to_string(), BoneType::Irregular),
        ];

        let mut hyoid = Bone::new("Hyoid".to_string(), BoneType::Irregular);
        hyoid.mass_g = 3.0;

        SkullBones {
            cranium,
            facial_bones,
            auditory_ossicles,
            hyoid,
        }
    }

    pub fn bone_count(&self) -> usize {
        self.cranium.len() + self.facial_bones.len() + self.auditory_ossicles.len() + 1
    }

    pub fn total_mass_g(&self) -> f64 {
        self.cranium.iter().map(|b| b.mass_g).sum::<f64>()
            + self.facial_bones.iter().map(|b| b.mass_g).sum::<f64>()
            + self.auditory_ossicles.iter().map(|b| b.mass_g).sum::<f64>()
            + self.hyoid.mass_g
    }
}

impl VertebralColumn {
    pub fn new_adult() -> Self {
        let cervical: Vec<Bone> = (1..=7)
            .map(|i| {
                let mut v = Bone::vertebra();
                v.name = format!("C{}", i);
                v.mass_g = 15.0;
                v
            })
            .collect();

        let thoracic: Vec<Bone> = (1..=12)
            .map(|i| {
                let mut v = Bone::vertebra();
                v.name = format!("T{}", i);
                v.mass_g = 25.0;
                v
            })
            .collect();

        let lumbar: Vec<Bone> = (1..=5)
            .map(|i| {
                let mut v = Bone::vertebra();
                v.name = format!("L{}", i);
                v.mass_g = 40.0;
                v
            })
            .collect();

        let mut sacrum = Bone::new("Sacrum".to_string(), BoneType::Irregular);
        sacrum.mass_g = 150.0;

        let mut coccyx = Bone::new("Coccyx".to_string(), BoneType::Irregular);
        coccyx.mass_g = 10.0;

        VertebralColumn {
            cervical,
            thoracic,
            lumbar,
            sacrum,
            coccyx,
        }
    }

    pub fn bone_count(&self) -> usize {
        self.cervical.len() + self.thoracic.len() + self.lumbar.len() + 2
    }

    pub fn total_mass_g(&self) -> f64 {
        self.cervical.iter().map(|b| b.mass_g).sum::<f64>()
            + self.thoracic.iter().map(|b| b.mass_g).sum::<f64>()
            + self.lumbar.iter().map(|b| b.mass_g).sum::<f64>()
            + self.sacrum.mass_g
            + self.coccyx.mass_g
    }

    pub fn total_vertebrae(&self) -> usize {
        self.cervical.len() + self.thoracic.len() + self.lumbar.len()
    }
}

impl RibCage {
    pub fn new_adult() -> Self {
        let ribs: Vec<Bone> = (1..=24)
            .map(|i| {
                let mut rib = Bone::new(format!("Rib {}", i), BoneType::Flat);
                rib.length_mm = if i <= 7 { 200.0 } else { 150.0 };
                rib.mass_g = 25.0;
                rib
            })
            .collect();

        let mut sternum = Bone::new("Sternum".to_string(), BoneType::Flat);
        sternum.length_mm = 170.0;
        sternum.mass_g = 80.0;

        RibCage { ribs, sternum }
    }

    pub fn bone_count(&self) -> usize {
        self.ribs.len() + 1
    }

    pub fn total_mass_g(&self) -> f64 {
        self.ribs.iter().map(|b| b.mass_g).sum::<f64>() + self.sternum.mass_g
    }
}

impl AppendicularSkeleton {
    pub fn new_adult() -> Self {
        AppendicularSkeleton {
            upper_limbs: UpperLimbs::new_adult(),
            lower_limbs: LowerLimbs::new_adult(),
            pectoral_girdles: PectoralGirdles::new_adult(),
            pelvic_girdle: PelvicGirdle::new_adult(),
        }
    }

    pub fn bone_count(&self) -> usize {
        self.upper_limbs.bone_count()
            + self.lower_limbs.bone_count()
            + 4 + 2
    }

    pub fn total_mass_kg(&self) -> f64 {
        (self.upper_limbs.total_mass_g()
            + self.lower_limbs.total_mass_g()
            + self.pectoral_girdles.total_mass_g()
            + self.pelvic_girdle.total_mass_g()) / 1000.0
    }

    pub fn all_bones(&self) -> Vec<&Bone> {
        let mut bones = Vec::new();
        bones.push(&self.upper_limbs.left.humerus);
        bones.push(&self.upper_limbs.left.radius);
        bones.push(&self.upper_limbs.left.ulna);
        bones.extend(&self.upper_limbs.left.carpals);
        bones.extend(&self.upper_limbs.left.metacarpals);
        bones.extend(&self.upper_limbs.left.phalanges);
        bones.push(&self.upper_limbs.right.humerus);
        bones.push(&self.upper_limbs.right.radius);
        bones.push(&self.upper_limbs.right.ulna);
        bones.extend(&self.upper_limbs.right.carpals);
        bones.extend(&self.upper_limbs.right.metacarpals);
        bones.extend(&self.upper_limbs.right.phalanges);
        bones.push(&self.lower_limbs.left.femur);
        bones.push(&self.lower_limbs.left.tibia);
        bones.push(&self.lower_limbs.left.fibula);
        bones.push(&self.lower_limbs.left.patella);
        bones.extend(&self.lower_limbs.left.tarsals);
        bones.extend(&self.lower_limbs.left.metatarsals);
        bones.extend(&self.lower_limbs.left.phalanges);
        bones.push(&self.lower_limbs.right.femur);
        bones.push(&self.lower_limbs.right.tibia);
        bones.push(&self.lower_limbs.right.fibula);
        bones.push(&self.lower_limbs.right.patella);
        bones.extend(&self.lower_limbs.right.tarsals);
        bones.extend(&self.lower_limbs.right.metatarsals);
        bones.extend(&self.lower_limbs.right.phalanges);
        bones.push(&self.pectoral_girdles.left_clavicle);
        bones.push(&self.pectoral_girdles.right_clavicle);
        bones.push(&self.pectoral_girdles.left_scapula);
        bones.push(&self.pectoral_girdles.right_scapula);
        bones.push(&self.pelvic_girdle.left_hip_bone);
        bones.push(&self.pelvic_girdle.right_hip_bone);
        bones
    }
}

impl UpperLimbs {
    pub fn new_adult() -> Self {
        UpperLimbs {
            left: UpperLimb::new_adult(),
            right: UpperLimb::new_adult(),
        }
    }

    pub fn bone_count(&self) -> usize {
        self.left.bone_count() + self.right.bone_count()
    }

    pub fn total_mass_g(&self) -> f64 {
        self.left.total_mass_g() + self.right.total_mass_g()
    }
}

impl UpperLimb {
    pub fn new_adult() -> Self {
        let carpals: Vec<Bone> = ["Scaphoid", "Lunate", "Triquetrum", "Pisiform",
            "Trapezium", "Trapezoid", "Capitate", "Hamate"].iter().map(|name| {
            let mut bone = Bone::new(name.to_string(), BoneType::Short);
            bone.mass_g = 5.0;
            bone
        }).collect();

        let metacarpals: Vec<Bone> = (1..=5).map(|i| {
            let mut bone = Bone::new(format!("Metacarpal {}", i), BoneType::Long);
            bone.length_mm = 65.0;
            bone.mass_g = 8.0;
            bone
        }).collect();

        let phalanges: Vec<Bone> = (1..=14).map(|i| {
            let mut bone = Bone::new(format!("Phalanx {}", i), BoneType::Long);
            bone.length_mm = 30.0;
            bone.mass_g = 3.0;
            bone
        }).collect();

        let mut radius = Bone::new("Radius".to_string(), BoneType::Long);
        radius.length_mm = 240.0;
        radius.mass_g = 120.0;

        let mut ulna = Bone::new("Ulna".to_string(), BoneType::Long);
        ulna.length_mm = 260.0;
        ulna.mass_g = 130.0;

        UpperLimb {
            humerus: Bone::humerus(),
            radius,
            ulna,
            carpals,
            metacarpals,
            phalanges,
        }
    }

    pub fn bone_count(&self) -> usize {
        3 + self.carpals.len() + self.metacarpals.len() + self.phalanges.len()
    }

    pub fn total_mass_g(&self) -> f64 {
        self.humerus.mass_g
            + self.radius.mass_g
            + self.ulna.mass_g
            + self.carpals.iter().map(|b| b.mass_g).sum::<f64>()
            + self.metacarpals.iter().map(|b| b.mass_g).sum::<f64>()
            + self.phalanges.iter().map(|b| b.mass_g).sum::<f64>()
    }
}

impl LowerLimbs {
    pub fn new_adult() -> Self {
        LowerLimbs {
            left: LowerLimb::new_adult(),
            right: LowerLimb::new_adult(),
        }
    }

    pub fn bone_count(&self) -> usize {
        self.left.bone_count() + self.right.bone_count()
    }

    pub fn total_mass_g(&self) -> f64 {
        self.left.total_mass_g() + self.right.total_mass_g()
    }
}

impl LowerLimb {
    pub fn new_adult() -> Self {
        let tarsals: Vec<Bone> = ["Calcaneus", "Talus", "Navicular", "Cuboid",
            "Medial Cuneiform", "Intermediate Cuneiform", "Lateral Cuneiform"].iter().map(|name| {
            let mut bone = Bone::new(name.to_string(), BoneType::Short);
            bone.mass_g = 15.0;
            bone
        }).collect();

        let metatarsals: Vec<Bone> = (1..=5).map(|i| {
            let mut bone = Bone::new(format!("Metatarsal {}", i), BoneType::Long);
            bone.length_mm = 70.0;
            bone.mass_g = 10.0;
            bone
        }).collect();

        let phalanges: Vec<Bone> = (1..=14).map(|i| {
            let mut bone = Bone::new(format!("Toe Phalanx {}", i), BoneType::Long);
            bone.length_mm = 25.0;
            bone.mass_g = 2.0;
            bone
        }).collect();

        let mut fibula = Bone::new("Fibula".to_string(), BoneType::Long);
        fibula.length_mm = 390.0;
        fibula.mass_g = 150.0;

        LowerLimb {
            femur: Bone::femur(),
            tibia: Bone::tibia(),
            fibula,
            patella: Bone::patella(),
            tarsals,
            metatarsals,
            phalanges,
        }
    }

    pub fn bone_count(&self) -> usize {
        4 + self.tarsals.len() + self.metatarsals.len() + self.phalanges.len()
    }

    pub fn total_mass_g(&self) -> f64 {
        self.femur.mass_g
            + self.tibia.mass_g
            + self.fibula.mass_g
            + self.patella.mass_g
            + self.tarsals.iter().map(|b| b.mass_g).sum::<f64>()
            + self.metatarsals.iter().map(|b| b.mass_g).sum::<f64>()
            + self.phalanges.iter().map(|b| b.mass_g).sum::<f64>()
    }
}

impl PectoralGirdles {
    pub fn new_adult() -> Self {
        let mut left_clavicle = Bone::new("Left Clavicle".to_string(), BoneType::Long);
        left_clavicle.length_mm = 150.0;
        left_clavicle.mass_g = 50.0;

        let mut right_clavicle = Bone::new("Right Clavicle".to_string(), BoneType::Long);
        right_clavicle.length_mm = 150.0;
        right_clavicle.mass_g = 50.0;

        let mut left_scapula = Bone::new("Left Scapula".to_string(), BoneType::Flat);
        left_scapula.mass_g = 100.0;

        let mut right_scapula = Bone::new("Right Scapula".to_string(), BoneType::Flat);
        right_scapula.mass_g = 100.0;

        PectoralGirdles {
            left_clavicle,
            right_clavicle,
            left_scapula,
            right_scapula,
        }
    }

    pub fn total_mass_g(&self) -> f64 {
        self.left_clavicle.mass_g + self.right_clavicle.mass_g
            + self.left_scapula.mass_g + self.right_scapula.mass_g
    }
}

impl PelvicGirdle {
    pub fn new_adult() -> Self {
        let mut left_hip = Bone::new("Left Hip Bone".to_string(), BoneType::Irregular);
        left_hip.mass_g = 350.0;

        let mut right_hip = Bone::new("Right Hip Bone".to_string(), BoneType::Irregular);
        right_hip.mass_g = 350.0;

        PelvicGirdle {
            left_hip_bone: left_hip,
            right_hip_bone: right_hip,
        }
    }

    pub fn total_mass_g(&self) -> f64 {
        self.left_hip_bone.mass_g + self.right_hip_bone.mass_g
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adult_skeleton() {
        let skeleton = Skeleton::new_adult();
        assert_eq!(skeleton.bone_count(), 206);
        assert!(skeleton.total_bone_mass_kg > 0.0);
    }

    #[test]
    fn test_vertebral_column() {
        let column = VertebralColumn::new_adult();
        assert_eq!(column.cervical.len(), 7);
        assert_eq!(column.thoracic.len(), 12);
        assert_eq!(column.lumbar.len(), 5);
        assert_eq!(column.total_vertebrae(), 24);
    }

    #[test]
    fn test_skull_bones() {
        let skull = SkullBones::new_adult();
        assert_eq!(skull.cranium.len(), 8);
        assert_eq!(skull.facial_bones.len(), 14);
        assert_eq!(skull.auditory_ossicles.len(), 6);
        assert_eq!(skull.bone_count(), 29);
    }

    #[test]
    fn test_rib_cage() {
        let ribs = RibCage::new_adult();
        assert_eq!(ribs.ribs.len(), 24);
        assert_eq!(ribs.bone_count(), 25);
    }

    #[test]
    fn test_upper_limb() {
        let limb = UpperLimb::new_adult();
        assert_eq!(limb.carpals.len(), 8);
        assert_eq!(limb.metacarpals.len(), 5);
        assert_eq!(limb.phalanges.len(), 14);
        assert_eq!(limb.bone_count(), 30);
    }

    #[test]
    fn test_lower_limb() {
        let limb = LowerLimb::new_adult();
        assert_eq!(limb.tarsals.len(), 7);
        assert_eq!(limb.metatarsals.len(), 5);
        assert_eq!(limb.phalanges.len(), 14);
        assert_eq!(limb.bone_count(), 30);
    }

    #[test]
    fn test_skeletal_health() {
        let skeleton = Skeleton::new_adult();
        let health = skeleton.overall_skeletal_health_score();
        assert!(health > 0.0 && health <= 1.0);
    }

    #[test]
    fn test_bone_mineral_density() {
        let skeleton = Skeleton::new_adult();
        let avg_bmd = skeleton.average_bone_mineral_density();
        assert!(avg_bmd > 1.0 && avg_bmd < 2.0);
    }
}

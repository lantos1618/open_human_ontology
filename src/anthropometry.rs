use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BodyMeasurements {
    pub height_cm: f64,
    pub weight_kg: f64,
    pub waist_circumference_cm: f64,
    pub hip_circumference_cm: f64,
    pub neck_circumference_cm: f64,
    pub chest_circumference_cm: f64,
    pub shoulder_width_cm: f64,
    pub arm_length_cm: f64,
    pub leg_length_cm: f64,
    pub head_circumference_cm: f64,
}

impl BodyMeasurements {
    pub fn new(
        height: f64,
        weight: f64,
        waist: f64,
        hip: f64,
        neck: f64,
        chest: f64,
        shoulder: f64,
        arm: f64,
        leg: f64,
        head: f64,
    ) -> Self {
        Self {
            height_cm: height,
            weight_kg: weight,
            waist_circumference_cm: waist,
            hip_circumference_cm: hip,
            neck_circumference_cm: neck,
            chest_circumference_cm: chest,
            shoulder_width_cm: shoulder,
            arm_length_cm: arm,
            leg_length_cm: leg,
            head_circumference_cm: head,
        }
    }

    pub fn bmi(&self) -> f64 {
        let height_m = self.height_cm / 100.0;
        self.weight_kg / (height_m * height_m)
    }

    pub fn bmi_category(&self) -> BMICategory {
        let bmi = self.bmi();
        if bmi < 18.5 {
            BMICategory::Underweight
        } else if bmi < 25.0 {
            BMICategory::Normal
        } else if bmi < 30.0 {
            BMICategory::Overweight
        } else if bmi < 35.0 {
            BMICategory::ObeseClass1
        } else if bmi < 40.0 {
            BMICategory::ObeseClass2
        } else {
            BMICategory::ObeseClass3
        }
    }

    pub fn waist_to_hip_ratio(&self) -> f64 {
        self.waist_circumference_cm / self.hip_circumference_cm
    }

    pub fn waist_to_height_ratio(&self) -> f64 {
        self.waist_circumference_cm / self.height_cm
    }

    pub fn body_adiposity_index(&self) -> f64 {
        let hip_m = self.hip_circumference_cm / 100.0;
        let height_m = self.height_cm / 100.0;
        (hip_m / (height_m.powf(1.5))) - 18.0
    }

    pub fn sitting_height_ratio(&self) -> f64 {
        let sitting_height = self.height_cm - self.leg_length_cm;
        sitting_height / self.height_cm
    }

    pub fn is_android_fat_distribution(&self, biological_sex: BiologicalSex) -> bool {
        let whr = self.waist_to_hip_ratio();
        match biological_sex {
            BiologicalSex::Male => whr > 0.90,
            BiologicalSex::Female => whr > 0.85,
            BiologicalSex::Intersex => whr > 0.87,
        }
    }

    pub fn metabolic_risk_from_waist(&self, biological_sex: BiologicalSex) -> MetabolicRisk {
        match biological_sex {
            BiologicalSex::Male => {
                if self.waist_circumference_cm > 102.0 {
                    MetabolicRisk::High
                } else if self.waist_circumference_cm > 94.0 {
                    MetabolicRisk::Increased
                } else {
                    MetabolicRisk::Normal
                }
            }
            BiologicalSex::Female => {
                if self.waist_circumference_cm > 88.0 {
                    MetabolicRisk::High
                } else if self.waist_circumference_cm > 80.0 {
                    MetabolicRisk::Increased
                } else {
                    MetabolicRisk::Normal
                }
            }
            BiologicalSex::Intersex => {
                if self.waist_circumference_cm > 95.0 {
                    MetabolicRisk::High
                } else if self.waist_circumference_cm > 87.0 {
                    MetabolicRisk::Increased
                } else {
                    MetabolicRisk::Normal
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BMICategory {
    Underweight,
    Normal,
    Overweight,
    ObeseClass1,
    ObeseClass2,
    ObeseClass3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
    Intersex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetabolicRisk {
    Normal,
    Increased,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BodyComposition {
    pub lean_mass_kg: f64,
    pub fat_mass_kg: f64,
    pub bone_mass_kg: f64,
    pub water_percentage: f64,
}

impl BodyComposition {
    pub fn new(lean: f64, fat: f64, bone: f64, water_pct: f64) -> Self {
        Self {
            lean_mass_kg: lean,
            fat_mass_kg: fat,
            bone_mass_kg: bone,
            water_percentage: water_pct,
        }
    }

    pub fn total_mass(&self) -> f64 {
        self.lean_mass_kg + self.fat_mass_kg + self.bone_mass_kg
    }

    pub fn body_fat_percentage(&self) -> f64 {
        (self.fat_mass_kg / self.total_mass()) * 100.0
    }

    pub fn fat_free_mass_index(&self, height_cm: f64) -> f64 {
        let height_m = height_cm / 100.0;
        (self.lean_mass_kg + self.bone_mass_kg) / (height_m * height_m)
    }

    pub fn fat_mass_index(&self, height_cm: f64) -> f64 {
        let height_m = height_cm / 100.0;
        self.fat_mass_kg / (height_m * height_m)
    }

    pub fn essential_fat_percentage(biological_sex: BiologicalSex) -> f64 {
        match biological_sex {
            BiologicalSex::Male => 3.0,
            BiologicalSex::Female => 12.0,
            BiologicalSex::Intersex => 7.5,
        }
    }

    pub fn is_healthy_body_fat(&self, biological_sex: BiologicalSex, age: u32) -> bool {
        let bf_pct = self.body_fat_percentage();

        match biological_sex {
            BiologicalSex::Male => {
                if age < 40 {
                    (8.0..=20.0).contains(&bf_pct)
                } else {
                    (11.0..=22.0).contains(&bf_pct)
                }
            }
            BiologicalSex::Female => {
                if age < 40 {
                    (21.0..=33.0).contains(&bf_pct)
                } else {
                    (23.0..=35.0).contains(&bf_pct)
                }
            }
            BiologicalSex::Intersex => (14.0..=27.0).contains(&bf_pct),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Somatotype {
    Ectomorph,
    Mesomorph,
    Endomorph,
    EctoMesomorph,
    MesoEndomorph,
    Balanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropometricProfile {
    pub measurements: BodyMeasurements,
    pub composition: BodyComposition,
    pub biological_sex: BiologicalSex,
    pub age: u32,
    pub ethnicity: Ethnicity,
}

impl AnthropometricProfile {
    pub fn new(
        measurements: BodyMeasurements,
        composition: BodyComposition,
        sex: BiologicalSex,
        age: u32,
        ethnicity: Ethnicity,
    ) -> Self {
        Self {
            measurements,
            composition,
            biological_sex: sex,
            age,
            ethnicity,
        }
    }

    pub fn frame_size(&self) -> FrameSize {
        let wrist_cm = self.measurements.neck_circumference_cm * 0.4;
        let height_cm = self.measurements.height_cm;

        let r_value = height_cm / wrist_cm;

        match self.biological_sex {
            BiologicalSex::Male => {
                if r_value > 10.4 {
                    FrameSize::Small
                } else if r_value > 9.6 {
                    FrameSize::Medium
                } else {
                    FrameSize::Large
                }
            }
            BiologicalSex::Female => {
                if r_value > 11.0 {
                    FrameSize::Small
                } else if r_value > 10.1 {
                    FrameSize::Medium
                } else {
                    FrameSize::Large
                }
            }
            BiologicalSex::Intersex => {
                if r_value > 10.7 {
                    FrameSize::Small
                } else if r_value > 9.85 {
                    FrameSize::Medium
                } else {
                    FrameSize::Large
                }
            }
        }
    }

    pub fn ideal_weight_range_kg(&self) -> (f64, f64) {
        let height_cm = self.measurements.height_cm;
        let frame = self.frame_size();

        let base_weight = match self.biological_sex {
            BiologicalSex::Male => 50.0 + 0.91 * (height_cm - 152.4),
            BiologicalSex::Female => 45.5 + 0.91 * (height_cm - 152.4),
            BiologicalSex::Intersex => 47.75 + 0.91 * (height_cm - 152.4),
        };

        match frame {
            FrameSize::Small => (base_weight * 0.9, base_weight),
            FrameSize::Medium => (base_weight * 0.95, base_weight * 1.05),
            FrameSize::Large => (base_weight, base_weight * 1.1),
        }
    }

    pub fn surface_area_m2(&self) -> f64 {
        let h = self.measurements.height_cm;
        let w = self.measurements.weight_kg;
        0.007184 * h.powf(0.725) * w.powf(0.425)
    }

    pub fn basal_metabolic_rate(&self) -> f64 {
        let w = self.measurements.weight_kg;
        let h = self.measurements.height_cm;
        let a = self.age as f64;

        match self.biological_sex {
            BiologicalSex::Male => 88.362 + (13.397 * w) + (4.799 * h) - (5.677 * a),
            BiologicalSex::Female => 447.593 + (9.247 * w) + (3.098 * h) - (4.330 * a),
            BiologicalSex::Intersex => 267.9775 + (11.322 * w) + (3.9485 * h) - (5.0035 * a),
        }
    }

    pub fn ethnicity_adjusted_bmi_threshold(&self) -> f64 {
        match self.ethnicity {
            Ethnicity::EastAsian | Ethnicity::SouthAsian | Ethnicity::SoutheastAsian => 23.0,
            Ethnicity::Black => 25.5,
            Ethnicity::Hispanic => 25.0,
            Ethnicity::White | Ethnicity::MiddleEastern | Ethnicity::Other => 25.0,
        }
    }

    pub fn has_metabolic_syndrome(
        &self,
        systolic_bp: f64,
        glucose: f64,
        triglycerides: f64,
        hdl: f64,
    ) -> bool {
        let mut criteria = 0;

        if self.measurements.waist_circumference_cm
            > match self.biological_sex {
                BiologicalSex::Male => 102.0,
                BiologicalSex::Female => 88.0,
                BiologicalSex::Intersex => 95.0,
            }
        {
            criteria += 1;
        }

        if systolic_bp >= 130.0 {
            criteria += 1;
        }

        if glucose >= 100.0 {
            criteria += 1;
        }

        if triglycerides >= 150.0 {
            criteria += 1;
        }

        if hdl
            < match self.biological_sex {
                BiologicalSex::Male => 40.0,
                BiologicalSex::Female => 50.0,
                BiologicalSex::Intersex => 45.0,
            }
        {
            criteria += 1;
        }

        criteria >= 3
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FrameSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ethnicity {
    EastAsian,
    SouthAsian,
    SoutheastAsian,
    Black,
    White,
    Hispanic,
    MiddleEastern,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmi_calculation() {
        let measurements =
            BodyMeasurements::new(175.0, 70.0, 80.0, 95.0, 38.0, 95.0, 45.0, 75.0, 90.0, 56.0);

        let bmi = measurements.bmi();
        assert!((bmi - 22.86).abs() < 0.1);
        assert_eq!(measurements.bmi_category(), BMICategory::Normal);
    }

    #[test]
    fn test_waist_to_hip_ratio() {
        let measurements =
            BodyMeasurements::new(170.0, 65.0, 75.0, 95.0, 35.0, 90.0, 42.0, 72.0, 88.0, 55.0);

        let whr = measurements.waist_to_hip_ratio();
        assert!((whr - 0.789).abs() < 0.01);
    }

    #[test]
    fn test_body_composition() {
        let composition = BodyComposition::new(50.0, 15.0, 5.0, 60.0);

        assert_eq!(composition.total_mass(), 70.0);
        assert!((composition.body_fat_percentage() - 21.43).abs() < 0.1);
    }

    #[test]
    fn test_metabolic_risk() {
        let measurements = BodyMeasurements::new(
            175.0, 90.0, 105.0, 100.0, 42.0, 110.0, 48.0, 75.0, 90.0, 58.0,
        );

        assert_eq!(
            measurements.metabolic_risk_from_waist(BiologicalSex::Male),
            MetabolicRisk::High
        );
    }

    #[test]
    fn test_basal_metabolic_rate() {
        let measurements =
            BodyMeasurements::new(175.0, 70.0, 80.0, 95.0, 38.0, 95.0, 45.0, 75.0, 90.0, 56.0);
        let composition = BodyComposition::new(55.0, 12.0, 3.0, 62.0);

        let profile = AnthropometricProfile::new(
            measurements,
            composition,
            BiologicalSex::Male,
            30,
            Ethnicity::White,
        );

        let bmr = profile.basal_metabolic_rate();
        assert!(bmr > 1600.0 && bmr < 1800.0);
    }

    #[test]
    fn test_surface_area() {
        let measurements =
            BodyMeasurements::new(170.0, 70.0, 80.0, 95.0, 38.0, 95.0, 45.0, 75.0, 88.0, 56.0);
        let composition = BodyComposition::new(55.0, 12.0, 3.0, 62.0);

        let profile = AnthropometricProfile::new(
            measurements,
            composition,
            BiologicalSex::Male,
            25,
            Ethnicity::White,
        );

        let bsa = profile.surface_area_m2();
        assert!(bsa > 1.7 && bsa < 1.9);
    }
}

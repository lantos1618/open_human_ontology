use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GastroenterologyProfile {
    pub conditions: Vec<GICondition>,
    pub liver_function: LiverFunction,
    pub pancreas_function: PancreasFunction,
    pub bowel_habits: BowelHabits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GICondition {
    pub diagnosis: GIDiagnosis,
    pub severity: GISeverity,
    pub location: GILocation,
    pub complications: Vec<GIComplication>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GIDiagnosis {
    GERD,
    PepticUlcer,
    CrohnsDisease,
    UlcerativeColitis,
    IrritableBowelSyndrome,
    Celiac,
    Diverticulitis,
    Pancreatitis,
    Cirrhosis,
    HepatitisB,
    HepatitisC,
    FattyLiverDisease,
    Cholecystitis,
    Cholelithiasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GISeverity {
    Mild,
    Moderate,
    Severe,
    Fulminant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GILocation {
    Esophagus,
    Stomach,
    Duodenum,
    Jejunum,
    Ileum,
    Colon,
    Rectum,
    Liver,
    Pancreas,
    Gallbladder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GIComplication {
    Bleeding,
    Perforation,
    Obstruction,
    Stricture,
    Fistula,
    Abscess,
    Malignancy,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LiverFunction {
    pub alt_u_l: f64,
    pub ast_u_l: f64,
    pub alkaline_phosphatase_u_l: f64,
    pub bilirubin_total_mg_dl: f64,
    pub albumin_g_dl: f64,
    pub inr: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PancreasFunction {
    pub amylase_u_l: f64,
    pub lipase_u_l: f64,
    pub fecal_elastase_ug_g: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowelHabits {
    pub frequency_per_day: f64,
    pub consistency: StoolConsistency,
    pub blood_present: bool,
    pub mucus_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoolConsistency {
    Type1Hard,
    Type2Lumpy,
    Type3Cracked,
    Type4Smooth,
    Type5Soft,
    Type6Fluffy,
    Type7Watery,
}

impl GastroenterologyProfile {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            liver_function: LiverFunction::normal(),
            pancreas_function: PancreasFunction::normal(),
            bowel_habits: BowelHabits::normal(),
        }
    }

    pub fn add_condition(&mut self, condition: GICondition) {
        self.conditions.push(condition);
    }

    pub fn child_pugh_score(&self) -> Option<ChildPughScore> {
        let has_cirrhosis = self
            .conditions
            .iter()
            .any(|c| c.diagnosis == GIDiagnosis::Cirrhosis);

        if !has_cirrhosis {
            return None;
        }

        let mut score = 0;

        score += match self.liver_function.bilirubin_total_mg_dl {
            x if x < 2.0 => 1,
            x if x < 3.0 => 2,
            _ => 3,
        };

        score += match self.liver_function.albumin_g_dl {
            x if x > 3.5 => 1,
            x if x > 2.8 => 2,
            _ => 3,
        };

        score += match self.liver_function.inr {
            x if x < 1.7 => 1,
            x if x < 2.3 => 2,
            _ => 3,
        };

        Some(ChildPughScore::from_score(score))
    }

    pub fn ibd_activity_index(&self) -> Option<IBDActivity> {
        let has_ibd = self.conditions.iter().any(|c| {
            matches!(
                c.diagnosis,
                GIDiagnosis::CrohnsDisease | GIDiagnosis::UlcerativeColitis
            )
        });

        if !has_ibd {
            return None;
        }

        let activity = match (
            self.bowel_habits.frequency_per_day,
            self.bowel_habits.blood_present,
        ) {
            (freq, true) if freq > 6.0 => IBDActivity::Severe,
            (freq, _) if freq > 4.0 => IBDActivity::Moderate,
            (freq, _) if freq > 2.0 => IBDActivity::Mild,
            _ => IBDActivity::Remission,
        };

        Some(activity)
    }

    pub fn hepatic_encephalopathy_risk(&self) -> HepaticEncephalopathyRisk {
        if self.liver_function.albumin_g_dl < 2.8 && self.liver_function.inr > 1.7 {
            HepaticEncephalopathyRisk::High
        } else if self.liver_function.albumin_g_dl < 3.5 {
            HepaticEncephalopathyRisk::Moderate
        } else {
            HepaticEncephalopathyRisk::Low
        }
    }

    pub fn pancreatitis_severity(&self) -> Option<PancreatitisSeverity> {
        let has_pancreatitis = self
            .conditions
            .iter()
            .any(|c| c.diagnosis == GIDiagnosis::Pancreatitis);

        if !has_pancreatitis {
            return None;
        }

        let severity = if self.pancreas_function.lipase_u_l > 1000.0 {
            PancreatitisSeverity::Severe
        } else if self.pancreas_function.lipase_u_l > 300.0 {
            PancreatitisSeverity::Moderate
        } else {
            PancreatitisSeverity::Mild
        };

        Some(severity)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildPughScore {
    ClassA,
    ClassB,
    ClassC,
}

impl ChildPughScore {
    fn from_score(score: u32) -> Self {
        match score {
            5..=6 => ChildPughScore::ClassA,
            7..=9 => ChildPughScore::ClassB,
            _ => ChildPughScore::ClassC,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IBDActivity {
    Remission,
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HepaticEncephalopathyRisk {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PancreatitisSeverity {
    Mild,
    Moderate,
    Severe,
}

impl Default for GastroenterologyProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl LiverFunction {
    pub fn normal() -> Self {
        Self {
            alt_u_l: 25.0,
            ast_u_l: 25.0,
            alkaline_phosphatase_u_l: 80.0,
            bilirubin_total_mg_dl: 0.8,
            albumin_g_dl: 4.0,
            inr: 1.0,
        }
    }

    pub fn is_elevated(&self) -> bool {
        self.alt_u_l > 40.0 || self.ast_u_l > 40.0
    }

    pub fn cholestasis_pattern(&self) -> bool {
        self.alkaline_phosphatase_u_l > 120.0 && self.bilirubin_total_mg_dl > 1.2
    }

    pub fn hepatocellular_pattern(&self) -> bool {
        self.alt_u_l > 3.0 * 40.0 || self.ast_u_l > 3.0 * 40.0
    }

    pub fn synthetic_function_impaired(&self) -> bool {
        self.albumin_g_dl < 3.5 || self.inr > 1.3
    }
}

impl PancreasFunction {
    pub fn normal() -> Self {
        Self {
            amylase_u_l: 70.0,
            lipase_u_l: 30.0,
            fecal_elastase_ug_g: 300.0,
        }
    }

    pub fn pancreatitis_present(&self) -> bool {
        self.lipase_u_l > 3.0 * 60.0
    }

    pub fn exocrine_insufficiency(&self) -> bool {
        self.fecal_elastase_ug_g < 200.0
    }
}

impl BowelHabits {
    pub fn normal() -> Self {
        Self {
            frequency_per_day: 1.0,
            consistency: StoolConsistency::Type4Smooth,
            blood_present: false,
            mucus_present: false,
        }
    }

    pub fn constipation(&self) -> bool {
        self.frequency_per_day < 0.3
            || matches!(
                self.consistency,
                StoolConsistency::Type1Hard | StoolConsistency::Type2Lumpy
            )
    }

    pub fn diarrhea(&self) -> bool {
        self.frequency_per_day > 3.0
            || matches!(
                self.consistency,
                StoolConsistency::Type6Fluffy | StoolConsistency::Type7Watery
            )
    }

    pub fn alarm_symptoms(&self) -> bool {
        self.blood_present || (self.diarrhea() && self.mucus_present)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gastroenterology_profile_creation() {
        let profile = GastroenterologyProfile::new();
        assert!(profile.conditions.is_empty());
        assert_eq!(profile.liver_function.alt_u_l, 25.0);
    }

    #[test]
    fn test_child_pugh_scoring() {
        let mut profile = GastroenterologyProfile::new();

        profile.add_condition(GICondition {
            diagnosis: GIDiagnosis::Cirrhosis,
            severity: GISeverity::Severe,
            location: GILocation::Liver,
            complications: vec![],
        });

        profile.liver_function.bilirubin_total_mg_dl = 4.0;
        profile.liver_function.albumin_g_dl = 2.5;
        profile.liver_function.inr = 2.5;

        let score = profile.child_pugh_score();
        assert_eq!(score, Some(ChildPughScore::ClassB));
    }

    #[test]
    fn test_ibd_activity() {
        let mut profile = GastroenterologyProfile::new();

        profile.add_condition(GICondition {
            diagnosis: GIDiagnosis::UlcerativeColitis,
            severity: GISeverity::Moderate,
            location: GILocation::Colon,
            complications: vec![],
        });

        profile.bowel_habits.frequency_per_day = 5.0;
        profile.bowel_habits.blood_present = false;

        let activity = profile.ibd_activity_index();
        assert_eq!(activity, Some(IBDActivity::Moderate));
    }

    #[test]
    fn test_liver_function_patterns() {
        let mut liver = LiverFunction::normal();
        liver.alt_u_l = 150.0;
        liver.ast_u_l = 120.0;

        assert!(liver.is_elevated());
        assert!(liver.hepatocellular_pattern());
    }

    #[test]
    fn test_pancreatitis_detection() {
        let mut profile = GastroenterologyProfile::new();

        profile.add_condition(GICondition {
            diagnosis: GIDiagnosis::Pancreatitis,
            severity: GISeverity::Severe,
            location: GILocation::Pancreas,
            complications: vec![],
        });

        profile.pancreas_function.lipase_u_l = 1200.0;

        assert_eq!(
            profile.pancreatitis_severity(),
            Some(PancreatitisSeverity::Severe)
        );
    }

    #[test]
    fn test_bowel_habits() {
        let mut habits = BowelHabits::normal();
        habits.frequency_per_day = 5.0;
        habits.consistency = StoolConsistency::Type7Watery;

        assert!(habits.diarrhea());
        assert!(!habits.constipation());
    }
}

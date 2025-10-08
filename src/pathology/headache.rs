use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::biology::{BiologyResult, BiologyError};
use super::symptom::Symptom;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeadacheType {
    Migraine(MigraineSubtype),
    ClusterHeadache,
    TensionType,
    Hemicrania,
    SUNCT,
    Paroxysmal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigraineSubtype {
    WithoutAura,
    WithAura,
    Chronic,
    Hemiplegic,
    Basilar,
    Retinal,
    Vestibular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migraine {
    pub subtype: MigraineSubtype,
    pub frequency_per_month: f64,
    pub duration_hours: f64,
    pub intensity: PainIntensity,
    pub triggers: Vec<MigraineTrigger>,
    pub aura_symptoms: Vec<AuraSymptom>,
    pub genetic_variants: Vec<String>,
    pub comorbidities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PainIntensity {
    Mild,
    Moderate,
    Severe,
    Debilitating,
}

impl PainIntensity {
    pub fn score(&self) -> u8 {
        match self {
            PainIntensity::Mild => 3,
            PainIntensity::Moderate => 5,
            PainIntensity::Severe => 7,
            PainIntensity::Debilitating => 10,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigraineTrigger {
    Stress,
    LackOfSleep,
    Dehydration,
    Caffeine,
    Alcohol,
    Chocolate,
    AgedCheese,
    MSG,
    AspartameFoods,
    BrightLights,
    LoudNoise,
    StrongOdors,
    WeatherChanges,
    HormonalChanges,
    Menstruation,
    Exercise,
    Fasting,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuraSymptom {
    VisualDisturbances,
    Scotoma,
    Scintillations,
    ZigzagLines,
    TunnelVision,
    Hemianopia,
    Numbness,
    Tingling,
    SpeechDifficulty,
    Aphasia,
    MotorWeakness,
    Vertigo,
}

impl Migraine {
    pub fn new(subtype: MigraineSubtype) -> Self {
        Self {
            subtype,
            frequency_per_month: 0.0,
            duration_hours: 4.0,
            intensity: PainIntensity::Moderate,
            triggers: Vec::new(),
            aura_symptoms: Vec::new(),
            genetic_variants: Vec::new(),
            comorbidities: Vec::new(),
        }
    }

    pub fn is_chronic(&self) -> bool {
        matches!(self.subtype, MigraineSubtype::Chronic) || self.frequency_per_month >= 15.0
    }

    pub fn disability_score(&self) -> f64 {
        let frequency_score = (self.frequency_per_month / 30.0).min(1.0);
        let intensity_score = self.intensity.score() as f64 / 10.0;
        let duration_score = (self.duration_hours / 72.0).min(1.0);

        (frequency_score + intensity_score + duration_score) / 3.0 * 100.0
    }

    pub fn known_genetic_variants() -> Vec<(&'static str, &'static str)> {
        vec![
            ("CACNA1A", "Familial hemiplegic migraine type 1"),
            ("ATP1A2", "Familial hemiplegic migraine type 2"),
            ("SCN1A", "Familial hemiplegic migraine type 3"),
            ("MTHFR C677T", "Increased migraine risk with aura"),
            ("ESR1", "Menstrual migraine association"),
            ("PRDM16", "Common migraine susceptibility"),
            ("TRPM8", "Migraine susceptibility"),
            ("LRP1", "Migraine susceptibility"),
            ("TGFBR2", "Migraine with aura"),
            ("PHACTR1", "Migraine susceptibility"),
        ]
    }

    pub fn has_genetic_risk(&self, variant: &str) -> bool {
        self.genetic_variants.iter().any(|v| v == variant)
    }

    pub fn add_trigger(&mut self, trigger: MigraineTrigger) {
        if !self.triggers.contains(&trigger) {
            self.triggers.push(trigger);
        }
    }

    pub fn prophylactic_candidates(&self) -> Vec<&'static str> {
        let mut medications = Vec::new();

        if self.frequency_per_month >= 4.0 {
            medications.push("Propranolol");
            medications.push("Topiramate");
            medications.push("Amitriptyline");
        }

        if self.is_chronic() {
            medications.push("CGRP inhibitors (Erenumab, Fremanezumab)");
            medications.push("OnabotulinumtoxinA");
        }

        if matches!(self.subtype, MigraineSubtype::Vestibular) {
            medications.push("Verapamil");
        }

        medications
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHeadache {
    pub episodic: bool,
    pub attacks_per_day: f64,
    pub duration_minutes: f64,
    pub intensity: PainIntensity,
    pub cluster_period_weeks: f64,
    pub autonomic_symptoms: Vec<AutonomicSymptom>,
    pub genetic_variants: Vec<String>,
    pub circadian_pattern: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutonomicSymptom {
    Lacrimation,
    ConjunctivalInjection,
    NasalCongestion,
    Rhinorrhea,
    EyelidEdema,
    Ptosis,
    Miosis,
    FacialSweating,
    Restlessness,
}

impl ClusterHeadache {
    pub fn new() -> Self {
        Self {
            episodic: true,
            attacks_per_day: 1.0,
            duration_minutes: 60.0,
            intensity: PainIntensity::Debilitating,
            cluster_period_weeks: 6.0,
            autonomic_symptoms: Vec::new(),
            genetic_variants: Vec::new(),
            circadian_pattern: true,
        }
    }

    pub fn is_chronic(&self) -> bool {
        !self.episodic
    }

    pub fn meets_diagnostic_criteria(&self) -> bool {
        self.attacks_per_day >= 1.0 &&
        self.duration_minutes >= 15.0 &&
        self.duration_minutes <= 180.0 &&
        !self.autonomic_symptoms.is_empty()
    }

    pub fn known_genetic_variants() -> Vec<(&'static str, &'static str)> {
        vec![
            ("HCRTR2", "Hypocretin receptor 2 - circadian rhythm"),
            ("ADH4", "Alcohol dehydrogenase - alcohol trigger sensitivity"),
            ("CLOCK", "Circadian clock gene"),
            ("PER3", "Period circadian regulator"),
            ("MTHFR", "Associated with cluster headache risk"),
        ]
    }

    pub fn acute_treatment_options(&self) -> Vec<&'static str> {
        vec![
            "100% Oxygen 12-15 L/min",
            "Sumatriptan 6mg SC",
            "Zolmitriptan nasal spray",
        ]
    }

    pub fn prophylactic_candidates(&self) -> Vec<&'static str> {
        let mut medications = vec!["Verapamil"];

        if self.is_chronic() {
            medications.push("Lithium");
            medications.push("Topiramate");
        }

        if self.episodic {
            medications.push("Prednisone taper");
        }

        medications.push("Greater occipital nerve block");

        medications
    }

    pub fn disability_score(&self) -> f64 {
        let frequency_score = (self.attacks_per_day / 8.0).min(1.0);
        let intensity_score = self.intensity.score() as f64 / 10.0;
        let chronicity = if self.is_chronic() { 1.5 } else { 1.0 };

        frequency_score * intensity_score * chronicity * 100.0
    }
}

impl Default for ClusterHeadache {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadacheProfile {
    pub primary_diagnosis: Option<HeadacheType>,
    pub headache_days_per_month: f64,
    pub medication_overuse: bool,
    pub genetic_testing: HashMap<String, String>,
    pub treatment_history: Vec<String>,
    pub current_prophylaxis: Vec<String>,
}

impl HeadacheProfile {
    pub fn new() -> Self {
        Self {
            primary_diagnosis: None,
            headache_days_per_month: 0.0,
            medication_overuse: false,
            genetic_testing: HashMap::new(),
            treatment_history: Vec::new(),
            current_prophylaxis: Vec::new(),
        }
    }

    pub fn medication_overuse_headache_risk(&self) -> bool {
        self.headache_days_per_month >= 15.0 && self.medication_overuse
    }

    pub fn requires_prophylaxis(&self) -> bool {
        self.headache_days_per_month >= 4.0
    }

    pub fn genetic_risk_assessment(&self) -> Vec<String> {
        let mut risks = Vec::new();

        for (gene, variant) in &self.genetic_testing {
            match gene.as_str() {
                "CACNA1A" | "ATP1A2" | "SCN1A" => {
                    risks.push(format!("Familial hemiplegic migraine risk: {} {}", gene, variant));
                },
                "MTHFR" if variant.contains("C677T") => {
                    risks.push("Increased migraine with aura risk".to_string());
                },
                "HCRTR2" => {
                    risks.push("Cluster headache circadian pattern risk".to_string());
                },
                _ => {}
            }
        }

        risks
    }
}

impl Default for HeadacheProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migraine_creation() {
        let migraine = Migraine::new(MigraineSubtype::WithAura);
        assert_eq!(migraine.subtype, MigraineSubtype::WithAura);
    }

    #[test]
    fn test_migraine_chronic() {
        let mut migraine = Migraine::new(MigraineSubtype::WithoutAura);
        migraine.frequency_per_month = 16.0;
        assert!(migraine.is_chronic());
    }

    #[test]
    fn test_migraine_disability_score() {
        let mut migraine = Migraine::new(MigraineSubtype::Chronic);
        migraine.frequency_per_month = 20.0;
        migraine.intensity = PainIntensity::Severe;
        migraine.duration_hours = 48.0;
        let score = migraine.disability_score();
        assert!(score > 50.0);
    }

    #[test]
    fn test_migraine_triggers() {
        let mut migraine = Migraine::new(MigraineSubtype::WithoutAura);
        migraine.add_trigger(MigraineTrigger::Stress);
        migraine.add_trigger(MigraineTrigger::Caffeine);
        assert_eq!(migraine.triggers.len(), 2);
    }

    #[test]
    fn test_migraine_genetic_variants() {
        let variants = Migraine::known_genetic_variants();
        assert!(!variants.is_empty());
        assert!(variants.iter().any(|(gene, _)| gene == &"CACNA1A"));
    }

    #[test]
    fn test_cluster_headache() {
        let cluster = ClusterHeadache::new();
        assert!(cluster.episodic);
        assert_eq!(cluster.intensity, PainIntensity::Debilitating);
    }

    #[test]
    fn test_cluster_diagnostic_criteria() {
        let mut cluster = ClusterHeadache::new();
        cluster.autonomic_symptoms.push(AutonomicSymptom::Lacrimation);
        cluster.duration_minutes = 60.0;
        assert!(cluster.meets_diagnostic_criteria());
    }

    #[test]
    fn test_cluster_chronic() {
        let mut cluster = ClusterHeadache::new();
        cluster.episodic = false;
        assert!(cluster.is_chronic());
    }

    #[test]
    fn test_cluster_treatments() {
        let cluster = ClusterHeadache::new();
        let acute = cluster.acute_treatment_options();
        assert!(acute.contains(&"100% Oxygen 12-15 L/min"));

        let prophylactic = cluster.prophylactic_candidates();
        assert!(prophylactic.contains(&"Verapamil"));
    }

    #[test]
    fn test_headache_profile() {
        let mut profile = HeadacheProfile::new();
        profile.headache_days_per_month = 16.0;
        profile.medication_overuse = true;
        assert!(profile.medication_overuse_headache_risk());
        assert!(profile.requires_prophylaxis());
    }

    #[test]
    fn test_pain_intensity() {
        assert_eq!(PainIntensity::Mild.score(), 3);
        assert_eq!(PainIntensity::Debilitating.score(), 10);
    }

    #[test]
    fn test_genetic_risk_assessment() {
        let mut profile = HeadacheProfile::new();
        profile.genetic_testing.insert("CACNA1A".to_string(), "mutation".to_string());
        profile.genetic_testing.insert("MTHFR".to_string(), "C677T".to_string());

        let risks = profile.genetic_risk_assessment();
        assert_eq!(risks.len(), 2);
    }
}

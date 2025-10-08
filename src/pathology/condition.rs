use serde::{Deserialize, Serialize};
use super::symptom::Symptom;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MedicalCondition {
    pub name: String,
    pub is_chronic: bool,
    pub symptoms: Vec<Symptom>,
    pub onset_age: Option<f64>,
    pub duration_days: Option<f64>,
    pub affected_systems: Vec<String>,
}

impl MedicalCondition {
    pub fn new(name: String, is_chronic: bool) -> Self {
        MedicalCondition {
            name,
            is_chronic,
            symptoms: Vec::new(),
            onset_age: None,
            duration_days: None,
            affected_systems: Vec::new(),
        }
    }

    pub fn add_symptom(&mut self, symptom: Symptom) {
        self.symptoms.push(symptom);
    }

    pub fn with_onset_age(mut self, age: f64) -> Self {
        self.onset_age = Some(age);
        self
    }

    pub fn with_duration(mut self, days: f64) -> Self {
        self.duration_days = Some(days);
        self
    }

    pub fn add_affected_system(&mut self, system: String) {
        if !self.affected_systems.contains(&system) {
            self.affected_systems.push(system);
        }
    }

    pub fn symptom_count(&self) -> usize {
        self.symptoms.len()
    }

    pub fn is_acute(&self) -> bool {
        !self.is_chronic
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Migraine {
    pub frequency_per_month: f64,
    pub average_duration_hours: f64,
    pub triggers: Vec<String>,
    pub aura: bool,
    pub severity_score: u8,
}

impl Migraine {
    pub fn new() -> Self {
        Migraine {
            frequency_per_month: 0.0,
            average_duration_hours: 4.0,
            triggers: Vec::new(),
            aura: false,
            severity_score: 5,
        }
    }

    pub fn with_frequency(mut self, freq: f64) -> Self {
        self.frequency_per_month = freq;
        self
    }

    pub fn with_aura(mut self) -> Self {
        self.aura = true;
        self
    }

    pub fn add_trigger(&mut self, trigger: String) {
        if !self.triggers.contains(&trigger) {
            self.triggers.push(trigger);
        }
    }

    pub fn is_chronic(&self) -> bool {
        self.frequency_per_month >= 15.0
    }

    pub fn disability_score(&self) -> f64 {
        let frequency_factor = (self.frequency_per_month / 30.0).min(1.0);
        let duration_factor = (self.average_duration_hours / 72.0).min(1.0);
        let severity_factor = self.severity_score as f64 / 10.0;

        (frequency_factor + duration_factor + severity_factor) / 3.0
    }
}

impl Default for Migraine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterHeadache {
    pub cluster_period_weeks: f64,
    pub attacks_per_day: u8,
    pub attack_duration_minutes: f64,
    pub circadian_pattern: bool,
}

impl ClusterHeadache {
    pub fn new() -> Self {
        ClusterHeadache {
            cluster_period_weeks: 8.0,
            attacks_per_day: 2,
            attack_duration_minutes: 60.0,
            circadian_pattern: true,
        }
    }

    pub fn total_attacks_per_cluster(&self) -> f64 {
        self.cluster_period_weeks * 7.0 * self.attacks_per_day as f64
    }

    pub fn is_episodic(&self) -> bool {
        self.cluster_period_weeks < 52.0
    }

    pub fn is_chronic(&self) -> bool {
        !self.is_episodic()
    }
}

impl Default for ClusterHeadache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medical_condition() {
        let mut condition = MedicalCondition::new(
            "Hypertension".to_string(),
            true,
        ).with_onset_age(45.0);

        condition.add_affected_system("Cardiovascular".to_string());

        assert!(condition.is_chronic);
        assert!(!condition.is_acute());
        assert_eq!(condition.onset_age, Some(45.0));
    }

    #[test]
    fn test_migraine() {
        let mut migraine = Migraine::new()
            .with_frequency(20.0)
            .with_aura();

        migraine.add_trigger("Stress".to_string());
        migraine.add_trigger("Caffeine".to_string());

        assert!(migraine.is_chronic());
        assert!(migraine.aura);
        assert_eq!(migraine.triggers.len(), 2);
        assert!(migraine.disability_score() > 0.0);
    }

    #[test]
    fn test_cluster_headache() {
        let cluster = ClusterHeadache::new();

        assert!(cluster.is_episodic());
        assert!(!cluster.is_chronic());
        assert!(cluster.total_attacks_per_cluster() > 0.0);
        assert!(cluster.circadian_pattern);
    }
}

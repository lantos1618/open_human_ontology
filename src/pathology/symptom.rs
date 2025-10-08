use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymptomCategory {
    Pain,
    Neurological,
    Cardiovascular,
    Respiratory,
    Gastrointestinal,
    Constitutional,
    Psychological,
    Dermatological,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Symptom {
    pub name: String,
    pub category: SymptomCategory,
    pub severity: u8,
    pub onset: Onset,
    pub duration: Duration,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Onset {
    Acute,
    Subacute,
    Chronic,
    Sudden,
    Gradual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Duration {
    Transient,
    Hours(f64),
    Days(f64),
    Weeks(f64),
    Months(f64),
    Persistent,
}

impl Symptom {
    pub fn new(name: String, category: SymptomCategory) -> Self {
        Symptom {
            name,
            category,
            severity: 5,
            onset: Onset::Gradual,
            duration: Duration::Transient,
            location: None,
        }
    }

    pub fn with_severity(mut self, severity: u8) -> Self {
        self.severity = severity.clamp(1, 10);
        self
    }

    pub fn with_onset(mut self, onset: Onset) -> Self {
        self.onset = onset;
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    pub fn is_severe(&self) -> bool {
        self.severity >= 8
    }

    pub fn is_mild(&self) -> bool {
        self.severity <= 3
    }

    pub fn is_acute(&self) -> bool {
        matches!(self.onset, Onset::Acute | Onset::Sudden)
    }

    pub fn is_chronic(&self) -> bool {
        matches!(self.onset, Onset::Chronic) ||
        matches!(self.duration, Duration::Months(_) | Duration::Persistent)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymptomCluster {
    pub name: String,
    pub symptoms: Vec<Symptom>,
    pub temporal_pattern: TemporalPattern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalPattern {
    Continuous,
    Episodic,
    Intermittent,
    Progressive,
    Relapsing,
}

impl SymptomCluster {
    pub fn new(name: String, temporal_pattern: TemporalPattern) -> Self {
        SymptomCluster {
            name,
            symptoms: Vec::new(),
            temporal_pattern,
        }
    }

    pub fn add_symptom(&mut self, symptom: Symptom) {
        self.symptoms.push(symptom);
    }

    pub fn symptom_count(&self) -> usize {
        self.symptoms.len()
    }

    pub fn average_severity(&self) -> f64 {
        if self.symptoms.is_empty() {
            return 0.0;
        }
        let total: u32 = self.symptoms.iter().map(|s| s.severity as u32).sum();
        total as f64 / self.symptoms.len() as f64
    }

    pub fn has_severe_symptoms(&self) -> bool {
        self.symptoms.iter().any(|s| s.is_severe())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symptom_creation() {
        let symptom = Symptom::new(
            "Headache".to_string(),
            SymptomCategory::Pain,
        )
        .with_severity(7)
        .with_onset(Onset::Sudden)
        .with_duration(Duration::Hours(4.0))
        .with_location("Frontal".to_string());

        assert_eq!(symptom.severity, 7);
        assert!(!symptom.is_severe());
        assert!(!symptom.is_mild());
        assert!(symptom.is_acute());
        assert_eq!(symptom.location, Some("Frontal".to_string()));
    }

    #[test]
    fn test_symptom_cluster() {
        let mut cluster = SymptomCluster::new(
            "Flu-like symptoms".to_string(),
            TemporalPattern::Continuous,
        );

        cluster.add_symptom(Symptom::new(
            "Fever".to_string(),
            SymptomCategory::Constitutional,
        ).with_severity(6));

        cluster.add_symptom(Symptom::new(
            "Fatigue".to_string(),
            SymptomCategory::Constitutional,
        ).with_severity(8));

        assert_eq!(cluster.symptom_count(), 2);
        assert_eq!(cluster.average_severity(), 7.0);
        assert!(cluster.has_severe_symptoms());
    }
}

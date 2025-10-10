use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysiologicalTimeSeries {
    pub snapshots: BTreeMap<u64, PhysiologicalSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysiologicalSnapshot {
    pub timestamp: u64,
    pub cardiovascular: CardiovascularSnapshot,
    pub metabolic: MetabolicSnapshot,
    pub body_composition: BodyCompositionSnapshot,
    pub biomarkers: BiomarkerSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularSnapshot {
    pub systolic_bp_mmhg: f64,
    pub diastolic_bp_mmhg: f64,
    pub heart_rate_bpm: f64,
    pub pulse_pressure_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicSnapshot {
    pub fasting_glucose_mg_dl: f64,
    pub hba1c_percent: f64,
    pub total_cholesterol_mg_dl: f64,
    pub ldl_mg_dl: f64,
    pub hdl_mg_dl: f64,
    pub triglycerides_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyCompositionSnapshot {
    pub weight_kg: f64,
    pub bmi: f64,
    pub body_fat_percent: f64,
    pub lean_mass_kg: f64,
    pub waist_circumference_cm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomarkerSnapshot {
    pub vitamin_d_ng_ml: f64,
    pub vitamin_b12_pg_ml: f64,
    pub folate_ng_ml: f64,
    pub iron_ug_dl: f64,
    pub ferritin_ng_ml: f64,
    pub crp_mg_l: f64,
}

impl PhysiologicalTimeSeries {
    pub fn new() -> Self {
        Self {
            snapshots: BTreeMap::new(),
        }
    }

    pub fn add_snapshot(&mut self, snapshot: PhysiologicalSnapshot) {
        self.snapshots.insert(snapshot.timestamp, snapshot);
    }

    pub fn get_latest(&self) -> Option<&PhysiologicalSnapshot> {
        self.snapshots.values().last()
    }

    pub fn get_earliest(&self) -> Option<&PhysiologicalSnapshot> {
        self.snapshots.values().next()
    }

    pub fn get_range(&self, start_time: u64, end_time: u64) -> Vec<&PhysiologicalSnapshot> {
        self.snapshots
            .range(start_time..=end_time)
            .map(|(_, v)| v)
            .collect()
    }

    pub fn calculate_trend(&self, metric: MetricType) -> Option<Trend> {
        if self.snapshots.len() < 2 {
            return None;
        }

        let values: Vec<(u64, f64)> = self
            .snapshots
            .values()
            .map(|s| (s.timestamp, metric.extract_value(s)))
            .collect();

        if values.is_empty() {
            return None;
        }

        let first_val = values.first()?.1;
        let last_val = values.last()?.1;
        let delta = last_val - first_val;
        let percent_change = (delta / first_val) * 100.0;

        let direction = if delta.abs() < 0.01 {
            TrendDirection::Stable
        } else if delta > 0.0 {
            TrendDirection::Increasing
        } else {
            TrendDirection::Decreasing
        };

        Some(Trend {
            metric,
            direction,
            absolute_change: delta,
            percent_change,
            initial_value: first_val,
            current_value: last_val,
        })
    }

    pub fn assess_cardiovascular_risk_change(&self) -> Option<RiskChangeAssessment> {
        let latest = self.get_latest()?;
        let earliest = self.get_earliest()?;

        let initial_risk =
            Self::calculate_cv_risk_score(&earliest.cardiovascular, &earliest.metabolic);
        let current_risk = Self::calculate_cv_risk_score(&latest.cardiovascular, &latest.metabolic);

        let risk_change = current_risk - initial_risk;

        Some(RiskChangeAssessment {
            initial_risk_score: initial_risk,
            current_risk_score: current_risk,
            risk_delta: risk_change,
            risk_category: Self::categorize_risk(current_risk),
            improving: risk_change < -0.1,
        })
    }

    fn calculate_cv_risk_score(cv: &CardiovascularSnapshot, met: &MetabolicSnapshot) -> f64 {
        let mut score = 0.0;

        if cv.systolic_bp_mmhg > 140.0 {
            score += 2.0;
        } else if cv.systolic_bp_mmhg > 130.0 {
            score += 1.0;
        }

        if met.ldl_mg_dl > 160.0 {
            score += 2.0;
        } else if met.ldl_mg_dl > 130.0 {
            score += 1.0;
        }

        if met.hdl_mg_dl < 40.0 {
            score += 1.5;
        }

        if met.fasting_glucose_mg_dl > 126.0 {
            score += 2.0;
        } else if met.fasting_glucose_mg_dl > 100.0 {
            score += 1.0;
        }

        score
    }

    fn categorize_risk(score: f64) -> RiskCategory {
        if score < 2.0 {
            RiskCategory::Low
        } else if score < 4.0 {
            RiskCategory::Moderate
        } else if score < 6.0 {
            RiskCategory::High
        } else {
            RiskCategory::VeryHigh
        }
    }
}

impl Default for PhysiologicalTimeSeries {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    SystolicBP,
    DiastolicBP,
    HeartRate,
    BMI,
    BodyFatPercent,
    FastingGlucose,
    HbA1c,
    LDL,
    HDL,
    VitaminD,
}

impl MetricType {
    fn extract_value(&self, snapshot: &PhysiologicalSnapshot) -> f64 {
        match self {
            MetricType::SystolicBP => snapshot.cardiovascular.systolic_bp_mmhg,
            MetricType::DiastolicBP => snapshot.cardiovascular.diastolic_bp_mmhg,
            MetricType::HeartRate => snapshot.cardiovascular.heart_rate_bpm,
            MetricType::BMI => snapshot.body_composition.bmi,
            MetricType::BodyFatPercent => snapshot.body_composition.body_fat_percent,
            MetricType::FastingGlucose => snapshot.metabolic.fasting_glucose_mg_dl,
            MetricType::HbA1c => snapshot.metabolic.hba1c_percent,
            MetricType::LDL => snapshot.metabolic.ldl_mg_dl,
            MetricType::HDL => snapshot.metabolic.hdl_mg_dl,
            MetricType::VitaminD => snapshot.biomarkers.vitamin_d_ng_ml,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub metric: MetricType,
    pub direction: TrendDirection,
    pub absolute_change: f64,
    pub percent_change: f64,
    pub initial_value: f64,
    pub current_value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskChangeAssessment {
    pub initial_risk_score: f64,
    pub current_risk_score: f64,
    pub risk_delta: f64,
    pub risk_category: RiskCategory,
    pub improving: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskCategory {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_snapshot(timestamp: u64, sbp: f64, ldl: f64, bmi: f64) -> PhysiologicalSnapshot {
        PhysiologicalSnapshot {
            timestamp,
            cardiovascular: CardiovascularSnapshot {
                systolic_bp_mmhg: sbp,
                diastolic_bp_mmhg: 80.0,
                heart_rate_bpm: 70.0,
                pulse_pressure_mmhg: sbp - 80.0,
            },
            metabolic: MetabolicSnapshot {
                fasting_glucose_mg_dl: 90.0,
                hba1c_percent: 5.4,
                total_cholesterol_mg_dl: 200.0,
                ldl_mg_dl: ldl,
                hdl_mg_dl: 55.0,
                triglycerides_mg_dl: 100.0,
            },
            body_composition: BodyCompositionSnapshot {
                weight_kg: 70.0,
                bmi,
                body_fat_percent: 20.0,
                lean_mass_kg: 56.0,
                waist_circumference_cm: 85.0,
            },
            biomarkers: BiomarkerSnapshot {
                vitamin_d_ng_ml: 35.0,
                vitamin_b12_pg_ml: 450.0,
                folate_ng_ml: 12.0,
                iron_ug_dl: 90.0,
                ferritin_ng_ml: 100.0,
                crp_mg_l: 1.0,
            },
        }
    }

    #[test]
    fn test_time_series_creation() {
        let ts = PhysiologicalTimeSeries::new();
        assert!(ts.snapshots.is_empty());
    }

    #[test]
    fn test_add_and_retrieve_snapshot() {
        let mut ts = PhysiologicalTimeSeries::new();
        let snap1 = create_test_snapshot(1000, 120.0, 100.0, 23.0);
        let snap2 = create_test_snapshot(2000, 118.0, 95.0, 22.5);

        ts.add_snapshot(snap1);
        ts.add_snapshot(snap2);

        assert_eq!(ts.snapshots.len(), 2);
        assert_eq!(ts.get_latest().unwrap().timestamp, 2000);
        assert_eq!(ts.get_earliest().unwrap().timestamp, 1000);
    }

    #[test]
    fn test_trend_calculation() {
        let mut ts = PhysiologicalTimeSeries::new();
        ts.add_snapshot(create_test_snapshot(1000, 140.0, 150.0, 28.0));
        ts.add_snapshot(create_test_snapshot(2000, 130.0, 120.0, 26.0));
        ts.add_snapshot(create_test_snapshot(3000, 125.0, 110.0, 24.5));

        let trend = ts.calculate_trend(MetricType::SystolicBP).unwrap();
        assert_eq!(trend.direction, TrendDirection::Decreasing);
        assert_eq!(trend.initial_value, 140.0);
        assert_eq!(trend.current_value, 125.0);
        assert!(trend.absolute_change < 0.0);
    }

    #[test]
    fn test_bmi_trend() {
        let mut ts = PhysiologicalTimeSeries::new();
        ts.add_snapshot(create_test_snapshot(1000, 120.0, 100.0, 28.0));
        ts.add_snapshot(create_test_snapshot(2000, 120.0, 100.0, 24.5));

        let trend = ts.calculate_trend(MetricType::BMI).unwrap();
        assert_eq!(trend.direction, TrendDirection::Decreasing);
        assert!(trend.percent_change < 0.0);
    }

    #[test]
    fn test_cardiovascular_risk_assessment() {
        let mut ts = PhysiologicalTimeSeries::new();

        ts.add_snapshot(create_test_snapshot(1000, 150.0, 170.0, 30.0));
        ts.add_snapshot(create_test_snapshot(5000, 125.0, 110.0, 24.0));

        let risk = ts.assess_cardiovascular_risk_change().unwrap();
        assert!(risk.improving);
        assert!(risk.current_risk_score < risk.initial_risk_score);
        assert!(matches!(
            risk.risk_category,
            RiskCategory::Low | RiskCategory::Moderate
        ));
    }

    #[test]
    fn test_range_query() {
        let mut ts = PhysiologicalTimeSeries::new();
        ts.add_snapshot(create_test_snapshot(1000, 120.0, 100.0, 23.0));
        ts.add_snapshot(create_test_snapshot(2000, 118.0, 95.0, 22.5));
        ts.add_snapshot(create_test_snapshot(3000, 115.0, 90.0, 22.0));
        ts.add_snapshot(create_test_snapshot(4000, 120.0, 95.0, 22.5));

        let range = ts.get_range(1500, 3500);
        assert_eq!(range.len(), 2);
        assert_eq!(range[0].timestamp, 2000);
        assert_eq!(range[1].timestamp, 3000);
    }
}

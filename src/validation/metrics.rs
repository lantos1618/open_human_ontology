use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionError {
    pub parameter_name: String,
    pub predicted_value: f64,
    pub actual_value: f64,
    pub absolute_error: f64,
    pub relative_error_percent: f64,
}

impl PredictionError {
    pub fn new(parameter_name: String, predicted: f64, actual: f64) -> Self {
        let absolute_error = (predicted - actual).abs();
        let relative_error = if actual.abs() > 1e-10 {
            (absolute_error / actual.abs()) * 100.0
        } else {
            0.0
        };

        Self {
            parameter_name,
            predicted_value: predicted,
            actual_value: actual,
            absolute_error,
            relative_error_percent: relative_error,
        }
    }

    pub fn is_acceptable(&self, max_relative_error_percent: f64) -> bool {
        self.relative_error_percent <= max_relative_error_percent
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationMetrics {
    pub pearson_r: f64,
    pub r_squared: f64,
    pub sample_size: usize,
}

impl CorrelationMetrics {
    pub fn calculate(predicted: &[f64], actual: &[f64]) -> Option<Self> {
        if predicted.len() != actual.len() || predicted.is_empty() {
            return None;
        }

        let n = predicted.len() as f64;

        let mean_predicted = predicted.iter().sum::<f64>() / n;
        let mean_actual = actual.iter().sum::<f64>() / n;

        let mut numerator = 0.0;
        let mut sum_sq_predicted = 0.0;
        let mut sum_sq_actual = 0.0;

        for i in 0..predicted.len() {
            let diff_predicted = predicted[i] - mean_predicted;
            let diff_actual = actual[i] - mean_actual;

            numerator += diff_predicted * diff_actual;
            sum_sq_predicted += diff_predicted * diff_predicted;
            sum_sq_actual += diff_actual * diff_actual;
        }

        let denominator = (sum_sq_predicted * sum_sq_actual).sqrt();

        if denominator < 1e-10 {
            return None;
        }

        let pearson_r = numerator / denominator;
        let r_squared = pearson_r * pearson_r;

        Some(Self {
            pearson_r,
            r_squared,
            sample_size: predicted.len(),
        })
    }

    pub fn correlation_strength(&self) -> &'static str {
        let abs_r = self.pearson_r.abs();
        if abs_r > 0.9 {
            "Very strong"
        } else if abs_r > 0.7 {
            "Strong"
        } else if abs_r > 0.5 {
            "Moderate"
        } else if abs_r > 0.3 {
            "Weak"
        } else {
            "Very weak"
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelAccuracy {
    pub parameter_name: String,
    pub mean_absolute_error: f64,
    pub root_mean_square_error: f64,
    pub mean_absolute_percentage_error: f64,
    pub within_10_percent: f64,
    pub within_20_percent: f64,
    pub sample_size: usize,
}

impl ModelAccuracy {
    pub fn calculate(parameter_name: String, predicted: &[f64], actual: &[f64]) -> Option<Self> {
        if predicted.len() != actual.len() || predicted.is_empty() {
            return None;
        }

        let n = predicted.len() as f64;
        let mut sum_abs_error = 0.0;
        let mut sum_sq_error = 0.0;
        let mut sum_percentage_error = 0.0;
        let mut within_10 = 0;
        let mut within_20 = 0;

        for i in 0..predicted.len() {
            let error = predicted[i] - actual[i];
            let abs_error = error.abs();
            sum_abs_error += abs_error;
            sum_sq_error += error * error;

            let percentage_error = if actual[i].abs() > 1e-10 {
                (abs_error / actual[i].abs()) * 100.0
            } else {
                0.0
            };
            sum_percentage_error += percentage_error;

            if percentage_error <= 10.0 {
                within_10 += 1;
            }
            if percentage_error <= 20.0 {
                within_20 += 1;
            }
        }

        Some(Self {
            parameter_name,
            mean_absolute_error: sum_abs_error / n,
            root_mean_square_error: (sum_sq_error / n).sqrt(),
            mean_absolute_percentage_error: sum_percentage_error / n,
            within_10_percent: (within_10 as f64 / n) * 100.0,
            within_20_percent: (within_20 as f64 / n) * 100.0,
            sample_size: predicted.len(),
        })
    }

    pub fn performance_grade(&self) -> &'static str {
        if self.mean_absolute_percentage_error < 5.0 {
            "Excellent"
        } else if self.mean_absolute_percentage_error < 10.0 {
            "Good"
        } else if self.mean_absolute_percentage_error < 20.0 {
            "Acceptable"
        } else if self.mean_absolute_percentage_error < 30.0 {
            "Poor"
        } else {
            "Unacceptable"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prediction_error() {
        let error = PredictionError::new("test".to_string(), 100.0, 90.0);
        assert_eq!(error.absolute_error, 10.0);
        assert!((error.relative_error_percent - 11.11).abs() < 0.01);
    }

    #[test]
    fn test_correlation_metrics() {
        let predicted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let actual = vec![1.1, 2.0, 2.9, 4.1, 5.0];

        let metrics = CorrelationMetrics::calculate(&predicted, &actual).unwrap();
        assert!(metrics.pearson_r > 0.99);
        assert_eq!(metrics.sample_size, 5);
    }

    #[test]
    fn test_model_accuracy() {
        let predicted = vec![100.0, 200.0, 300.0];
        let actual = vec![95.0, 210.0, 290.0];

        let accuracy = ModelAccuracy::calculate("test".to_string(), &predicted, &actual).unwrap();
        assert!(accuracy.mean_absolute_error > 0.0);
        assert_eq!(accuracy.sample_size, 3);
    }

    #[test]
    fn test_perfect_correlation() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let metrics = CorrelationMetrics::calculate(&values, &values).unwrap();
        assert_eq!(metrics.pearson_r, 1.0);
        assert_eq!(metrics.r_squared, 1.0);
    }
}

use super::ground_truth::GroundTruthDatabase;
use super::metrics::{CorrelationMetrics, ModelAccuracy, PredictionError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub test_name: String,
    pub passed: bool,
    pub errors: Vec<PredictionError>,
    pub accuracy_metrics: Option<ModelAccuracy>,
    pub correlation_metrics: Option<CorrelationMetrics>,
    pub notes: Vec<String>,
}

impl ValidationResult {
    pub fn new(test_name: String) -> Self {
        Self {
            test_name,
            passed: true,
            errors: Vec::new(),
            accuracy_metrics: None,
            correlation_metrics: None,
            notes: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: PredictionError) {
        self.errors.push(error);
    }

    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }

    pub fn set_accuracy(&mut self, accuracy: ModelAccuracy) {
        self.accuracy_metrics = Some(accuracy);
    }

    pub fn set_correlation(&mut self, correlation: CorrelationMetrics) {
        self.correlation_metrics = Some(correlation);
    }

    pub fn mark_failed(&mut self) {
        self.passed = false;
    }

    pub fn summary(&self) -> String {
        let mut summary = format!("Test: {}\n", self.test_name);
        summary.push_str(&format!(
            "Status: {}\n",
            if self.passed {
                "✓ PASSED"
            } else {
                "✗ FAILED"
            }
        ));

        if let Some(acc) = &self.accuracy_metrics {
            summary.push_str(&format!(
                "  MAPE: {:.2}% ({})\n",
                acc.mean_absolute_percentage_error,
                acc.performance_grade()
            ));
            summary.push_str(&format!("  Within 10%: {:.1}%\n", acc.within_10_percent));
        }

        if let Some(corr) = &self.correlation_metrics {
            summary.push_str(&format!(
                "  R²: {:.3} ({})\n",
                corr.r_squared,
                corr.correlation_strength()
            ));
        }

        if !self.errors.is_empty() {
            summary.push_str(&format!("  Errors: {}\n", self.errors.len()));
        }

        summary
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetrics {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub overall_accuracy: f64,
    pub pass_rate: f64,
}

impl ValidationMetrics {
    pub fn from_results(results: &[ValidationResult]) -> Self {
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;

        let overall_accuracy = if !results.is_empty() {
            let sum: f64 = results
                .iter()
                .filter_map(|r| r.accuracy_metrics.as_ref())
                .map(|a| 100.0 - a.mean_absolute_percentage_error)
                .sum();
            sum / results.len() as f64
        } else {
            0.0
        };

        let pass_rate = if total_tests > 0 {
            (passed_tests as f64 / total_tests as f64) * 100.0
        } else {
            0.0
        };

        Self {
            total_tests,
            passed_tests,
            failed_tests,
            overall_accuracy,
            pass_rate,
        }
    }
}

pub struct ValidationFramework {
    ground_truth: GroundTruthDatabase,
    results: Vec<ValidationResult>,
}

impl ValidationFramework {
    pub fn new() -> Self {
        Self {
            ground_truth: GroundTruthDatabase::new(),
            results: Vec::new(),
        }
    }

    pub fn validate_parameter(
        &mut self,
        category: &str,
        parameter_name: &str,
        predicted_value: f64,
        max_error_percent: f64,
    ) -> bool {
        let mut result = ValidationResult::new(format!("{}::{}", category, parameter_name));

        if let Some(dataset) = self.ground_truth.get_dataset(category) {
            if let Some(expected) = dataset.get_expected_value(parameter_name) {
                let error =
                    PredictionError::new(parameter_name.to_string(), predicted_value, expected);

                let within_range =
                    dataset.is_within_expected_range(parameter_name, predicted_value);

                if !error.is_acceptable(max_error_percent) || !within_range {
                    result.mark_failed();
                    result.add_note(format!(
                        "Predicted {:.2}, expected {:.2} (error: {:.2}%)",
                        predicted_value, expected, error.relative_error_percent
                    ));
                }

                result.add_error(error);
            } else {
                result.add_note(format!("No ground truth for parameter: {}", parameter_name));
            }
        } else {
            result.add_note(format!(
                "No ground truth dataset for category: {}",
                category
            ));
        }

        let passed = result.passed;
        self.results.push(result);
        passed
    }

    pub fn validate_series(
        &mut self,
        test_name: String,
        predicted: &[f64],
        actual: &[f64],
        max_mape: f64,
    ) -> bool {
        let mut result = ValidationResult::new(test_name);

        if let Some(accuracy) = ModelAccuracy::calculate("series".to_string(), predicted, actual) {
            if accuracy.mean_absolute_percentage_error > max_mape {
                result.mark_failed();
            }
            result.set_accuracy(accuracy);
        }

        if let Some(correlation) = CorrelationMetrics::calculate(predicted, actual) {
            result.set_correlation(correlation);
        }

        let passed = result.passed;
        self.results.push(result);
        passed
    }

    pub fn get_metrics(&self) -> ValidationMetrics {
        ValidationMetrics::from_results(&self.results)
    }

    pub fn print_report(&self) {
        println!("\n╔═══════════════════════════════════════════════════════════╗");
        println!("║              Model Validation Report                     ║");
        println!("╚═══════════════════════════════════════════════════════════╝");

        let metrics = self.get_metrics();
        println!("\nOverall Metrics:");
        println!("  Total Tests: {}", metrics.total_tests);
        println!(
            "  Passed: {} ({:.1}%)",
            metrics.passed_tests, metrics.pass_rate
        );
        println!("  Failed: {}", metrics.failed_tests);
        println!("  Overall Accuracy: {:.2}%", metrics.overall_accuracy);

        println!("\n\nDetailed Results:");
        println!("{}", "─".repeat(60));

        for result in &self.results {
            println!("\n{}", result.summary());

            if !result.notes.is_empty() {
                println!("  Notes:");
                for note in &result.notes {
                    println!("    • {}", note);
                }
            }
        }

        println!("\n{}", "═".repeat(60));
    }

    pub fn ground_truth_database(&self) -> &GroundTruthDatabase {
        &self.ground_truth
    }
}

impl Default for ValidationFramework {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_framework() {
        let mut framework = ValidationFramework::new();

        let passed =
            framework.validate_parameter("cardiovascular", "resting_heart_rate_bpm", 72.0, 10.0);

        assert!(passed);
    }

    #[test]
    fn test_validation_metrics() {
        let mut results = Vec::new();

        let mut r1 = ValidationResult::new("test1".to_string());
        r1.passed = true;
        results.push(r1);

        let mut r2 = ValidationResult::new("test2".to_string());
        r2.passed = false;
        results.push(r2);

        let metrics = ValidationMetrics::from_results(&results);
        assert_eq!(metrics.total_tests, 2);
        assert_eq!(metrics.passed_tests, 1);
        assert_eq!(metrics.pass_rate, 50.0);
    }
}

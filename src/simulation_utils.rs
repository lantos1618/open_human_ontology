use crate::validation::ground_truth::GroundTruthDatabase;

pub struct BiomarkerTrajectory {
    pub baseline: f64,
    pub current: f64,
    pub target: f64,
    pub rate_constant: f64,
}

impl BiomarkerTrajectory {
    pub fn new(baseline: f64, target: f64, rate_constant: f64) -> Self {
        Self {
            baseline,
            current: baseline,
            target,
            rate_constant,
        }
    }

    pub fn from_ground_truth(
        db: &GroundTruthDatabase,
        category: &str,
        baseline_param: &str,
        target_param: &str,
        rate_constant: f64,
    ) -> Option<Self> {
        let dataset = db.get_dataset(category)?;
        let baseline = dataset.get_expected_value(baseline_param)?;
        let target = dataset.get_expected_value(target_param)?;
        Some(Self::new(baseline, target, rate_constant))
    }

    pub fn step_exponential(&mut self, dt: f64) {
        let delta = (self.target - self.current) * self.rate_constant * dt;
        self.current += delta;
    }

    pub fn step_sigmoid(&mut self, time: f64, midpoint: f64, steepness: f64) {
        let sigmoid = 1.0 / (1.0 + (-steepness * (time - midpoint)).exp());
        self.current = self.baseline + (self.target - self.baseline) * sigmoid;
    }

    pub fn step_linear(&mut self, dt: f64) {
        let delta = (self.target - self.baseline) * dt;
        self.current += delta;
    }

    pub fn fold_change(&self) -> f64 {
        if self.baseline.abs() < 1e-10 {
            return 0.0;
        }
        self.current / self.baseline
    }

    pub fn percent_change(&self) -> f64 {
        if self.baseline.abs() < 1e-10 {
            return 0.0;
        }
        ((self.current - self.baseline) / self.baseline) * 100.0
    }
}

pub struct DiseaseStage {
    pub name: String,
    pub duration_years: f64,
    pub biomarkers: Vec<BiomarkerState>,
}

pub struct BiomarkerState {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub reference_range: (f64, f64),
    pub clinical_interpretation: String,
}

impl BiomarkerState {
    pub fn is_abnormal(&self) -> bool {
        self.value < self.reference_range.0 || self.value > self.reference_range.1
    }

    pub fn severity(&self) -> String {
        if !self.is_abnormal() {
            return "Normal".to_string();
        }

        let deviation = if self.value < self.reference_range.0 {
            (self.reference_range.0 - self.value) / self.reference_range.0
        } else {
            (self.value - self.reference_range.1) / self.reference_range.1
        };

        if deviation < 0.5 {
            "Mild".to_string()
        } else if deviation < 2.0 {
            "Moderate".to_string()
        } else {
            "Severe".to_string()
        }
    }
}

pub fn simulate_exponential_decay(
    initial: f64,
    half_life: f64,
    time: f64,
) -> f64 {
    initial * 0.5_f64.powf(time / half_life)
}

pub fn simulate_exponential_growth(
    initial: f64,
    doubling_time: f64,
    time: f64,
) -> f64 {
    initial * 2.0_f64.powf(time / doubling_time)
}

pub fn simulate_logistic_growth(
    initial: f64,
    carrying_capacity: f64,
    growth_rate: f64,
    time: f64,
) -> f64 {
    carrying_capacity / (1.0 + ((carrying_capacity - initial) / initial) * (-growth_rate * time).exp())
}

pub fn simulate_first_order_kinetics(
    initial: f64,
    rate_constant: f64,
    time: f64,
) -> f64 {
    initial * (-rate_constant * time).exp()
}

pub fn calculate_hill_equation(
    max_response: f64,
    concentration: f64,
    ec50: f64,
    hill_coefficient: f64,
) -> f64 {
    let conc_n = concentration.powf(hill_coefficient);
    let ec50_n = ec50.powf(hill_coefficient);
    (max_response * conc_n) / (ec50_n + conc_n)
}

pub fn calculate_michaelis_menten(
    vmax: f64,
    substrate: f64,
    km: f64,
) -> f64 {
    (vmax * substrate) / (km + substrate)
}

pub struct TimeSeriesData {
    pub times: Vec<f64>,
    pub values: Vec<f64>,
}

impl TimeSeriesData {
    pub fn new() -> Self {
        Self {
            times: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn add_point(&mut self, time: f64, value: f64) {
        self.times.push(time);
        self.values.push(value);
    }

    pub fn mean(&self) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        self.values.iter().sum::<f64>() / self.values.len() as f64
    }

    pub fn max(&self) -> Option<f64> {
        self.values.iter().copied().max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn min(&self) -> Option<f64> {
        self.values.iter().copied().min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn integrate_trapezoidal(&self) -> f64 {
        if self.times.len() < 2 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..self.times.len() - 1 {
            let dt = self.times[i + 1] - self.times[i];
            let avg_value = (self.values[i] + self.values[i + 1]) / 2.0;
            area += avg_value * dt;
        }
        area
    }
}

pub fn format_biomarker_output(
    name: &str,
    value: f64,
    unit: &str,
    baseline: Option<f64>,
    fold_change: Option<f64>,
) -> String {
    let mut output = format!("{}: {:.2} {}", name, value, unit);

    if let Some(fc) = fold_change {
        output.push_str(&format!(" ({:.1}× baseline)", fc));
    } else if let Some(base) = baseline {
        let fc = value / base;
        output.push_str(&format!(" ({:.1}× baseline)", fc));
    }

    output
}

pub fn calculate_percent_change(initial: f64, current: f64) -> f64 {
    if initial.abs() < 1e-10 {
        return 0.0;
    }
    ((current - initial) / initial) * 100.0
}

pub fn calculate_fold_change(initial: f64, current: f64) -> f64 {
    if initial.abs() < 1e-10 {
        return 0.0;
    }
    current / initial
}

pub struct OrdinaryDifferentialEquation {
    pub state: Vec<f64>,
    pub derivatives: Box<dyn Fn(&[f64], f64) -> Vec<f64>>,
}

impl OrdinaryDifferentialEquation {
    pub fn new(
        initial_state: Vec<f64>,
        derivatives: Box<dyn Fn(&[f64], f64) -> Vec<f64>>,
    ) -> Self {
        Self {
            state: initial_state,
            derivatives,
        }
    }

    pub fn step_euler(&mut self, dt: f64, time: f64) {
        let dstate = (self.derivatives)(&self.state, time);
        for (i, ds) in dstate.iter().enumerate() {
            self.state[i] += ds * dt;
        }
    }

    pub fn step_rk4(&mut self, dt: f64, time: f64) {
        let k1 = (self.derivatives)(&self.state, time);

        let mut state_k2 = self.state.clone();
        for (i, k) in k1.iter().enumerate() {
            state_k2[i] += k * dt / 2.0;
        }
        let k2 = (self.derivatives)(&state_k2, time + dt / 2.0);

        let mut state_k3 = self.state.clone();
        for (i, k) in k2.iter().enumerate() {
            state_k3[i] += k * dt / 2.0;
        }
        let k3 = (self.derivatives)(&state_k3, time + dt / 2.0);

        let mut state_k4 = self.state.clone();
        for (i, k) in k3.iter().enumerate() {
            state_k4[i] += k * dt;
        }
        let k4 = (self.derivatives)(&state_k4, time + dt);

        for i in 0..self.state.len() {
            self.state[i] += (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]) * dt / 6.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biomarker_trajectory_exponential() {
        let mut traj = BiomarkerTrajectory::new(1.0, 10.0, 0.1);
        for _ in 0..100 {
            traj.step_exponential(0.1);
        }
        let expected = 10.0 - (10.0 - 1.0) * (-0.1 * 10.0_f64).exp();
        assert!((traj.current - expected).abs() < 0.5);
    }

    #[test]
    fn test_exponential_decay() {
        let value = simulate_exponential_decay(100.0, 2.0, 2.0);
        assert!((value - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_exponential_growth() {
        let value = simulate_exponential_growth(100.0, 2.0, 2.0);
        assert!((value - 200.0).abs() < 0.1);
    }

    #[test]
    fn test_time_series_integration() {
        let mut ts = TimeSeriesData::new();
        ts.add_point(0.0, 0.0);
        ts.add_point(1.0, 1.0);
        ts.add_point(2.0, 2.0);

        let area = ts.integrate_trapezoidal();
        assert!((area - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_ode_euler() {
        let mut ode = OrdinaryDifferentialEquation::new(
            vec![1.0],
            Box::new(|state, _time| vec![-0.1 * state[0]]),
        );

        for _ in 0..100 {
            ode.step_euler(0.1, 0.0);
        }

        assert!(ode.state[0] < 1.0);
        assert!(ode.state[0] > 0.0);
    }
}

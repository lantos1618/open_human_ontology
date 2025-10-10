use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ALDH2Genotype {
    WildType,
    Heterozygous,
    Homozygous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ADH1BGenotype {
    Slow,
    Intermediate,
    Fast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholMetabolismPathway {
    pub adh1b_genotype: ADH1BGenotype,
    pub aldh2_genotype: ALDH2Genotype,

    pub ethanol_concentration_mmol_l: f64,
    pub acetaldehyde_concentration_umol_l: f64,
    pub acetate_concentration_mmol_l: f64,

    pub adh_activity_relative: f64,
    pub aldh2_activity_relative: f64,

    pub blood_alcohol_concentration_mg_dl: f64,
    pub time_since_ingestion_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholMetabolismSimulation {
    pub pathway: AlcoholMetabolismPathway,
    pub timeline: Vec<MetabolismTimePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolismTimePoint {
    pub time_hours: f64,
    pub ethanol_mmol_l: f64,
    pub acetaldehyde_umol_l: f64,
    pub acetate_mmol_l: f64,
    pub flush_response_score: f64,
    pub symptoms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholIngestion {
    pub volume_ml: f64,
    pub alcohol_percentage: f64,
    pub body_weight_kg: f64,
    pub sex: Sex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
}

impl ALDH2Genotype {
    pub fn aldh2_activity_multiplier(&self) -> f64 {
        match self {
            ALDH2Genotype::WildType => 1.0,
            ALDH2Genotype::Heterozygous => 0.15,
            ALDH2Genotype::Homozygous => 0.01,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ALDH2Genotype::WildType => "ALDH2*1/*1 - Normal acetaldehyde clearance",
            ALDH2Genotype::Heterozygous => "ALDH2*1/*2 - Reduced acetaldehyde clearance (Asian flush)",
            ALDH2Genotype::Homozygous => "ALDH2*2/*2 - Severely reduced acetaldehyde clearance (very rare)",
        }
    }

    pub fn cancer_risk_multiplier(&self, alcohol_consumption: AlcoholConsumptionLevel) -> f64 {
        match (self, alcohol_consumption) {
            (ALDH2Genotype::WildType, _) => 1.0,
            (ALDH2Genotype::Heterozygous, AlcoholConsumptionLevel::None) => 1.0,
            (ALDH2Genotype::Heterozygous, AlcoholConsumptionLevel::Light) => 2.0,
            (ALDH2Genotype::Heterozygous, AlcoholConsumptionLevel::Moderate) => 5.0,
            (ALDH2Genotype::Heterozygous, AlcoholConsumptionLevel::Heavy) => 10.0,
            (ALDH2Genotype::Homozygous, AlcoholConsumptionLevel::None) => 1.0,
            (ALDH2Genotype::Homozygous, _) => 20.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlcoholConsumptionLevel {
    None,
    Light,
    Moderate,
    Heavy,
}

impl ADH1BGenotype {
    pub fn adh_activity_multiplier(&self) -> f64 {
        match self {
            ADH1BGenotype::Slow => 0.5,
            ADH1BGenotype::Intermediate => 1.0,
            ADH1BGenotype::Fast => 2.5,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ADH1BGenotype::Slow => "ADH1B*1/*1 - Slow ethanol → acetaldehyde conversion",
            ADH1BGenotype::Intermediate => "ADH1B*1/*2 - Intermediate ethanol metabolism",
            ADH1BGenotype::Fast => "ADH1B*2/*2 - Fast ethanol → acetaldehyde conversion (common in East Asia)",
        }
    }
}

impl AlcoholIngestion {
    pub fn calculate_peak_bac(&self) -> f64 {
        let ethanol_grams = self.volume_ml * self.alcohol_percentage / 100.0 * 0.789;

        let vd = match self.sex {
            Sex::Male => 0.68,
            Sex::Female => 0.55,
        };

        let body_water_l = self.body_weight_kg * vd;

        let bac_g_l = ethanol_grams / body_water_l;
        bac_g_l * 100.0
    }

    pub fn ethanol_mmol_l(&self) -> f64 {
        let bac_mg_dl = self.calculate_peak_bac();
        let bac_g_l = bac_mg_dl / 100.0;
        (bac_g_l / 46.07) * 1000.0
    }
}

impl AlcoholMetabolismPathway {
    pub fn new(adh1b_genotype: ADH1BGenotype, aldh2_genotype: ALDH2Genotype) -> Self {
        let adh_activity = adh1b_genotype.adh_activity_multiplier();
        let aldh2_activity = aldh2_genotype.aldh2_activity_multiplier();

        Self {
            adh1b_genotype,
            aldh2_genotype,
            ethanol_concentration_mmol_l: 0.0,
            acetaldehyde_concentration_umol_l: 0.0,
            acetate_concentration_mmol_l: 0.0,
            adh_activity_relative: adh_activity,
            aldh2_activity_relative: aldh2_activity,
            blood_alcohol_concentration_mg_dl: 0.0,
            time_since_ingestion_hours: 0.0,
        }
    }

    pub fn ingest_alcohol(&mut self, ingestion: &AlcoholIngestion) {
        self.ethanol_concentration_mmol_l = ingestion.ethanol_mmol_l();
        self.blood_alcohol_concentration_mg_dl = ingestion.calculate_peak_bac();
        self.time_since_ingestion_hours = 0.0;
    }

    pub fn step(&mut self, delta_hours: f64) {
        self.time_since_ingestion_hours += delta_hours;

        let k_adh = 0.15 * self.adh_activity_relative;
        let k_aldh = 0.8 * self.aldh2_activity_relative;

        let ethanol_metabolized = self.ethanol_concentration_mmol_l * k_adh * delta_hours;
        let acetaldehyde_produced = ethanol_metabolized * 1000.0;

        let acetaldehyde_clearance = self.acetaldehyde_concentration_umol_l * k_aldh * delta_hours;

        self.ethanol_concentration_mmol_l = (self.ethanol_concentration_mmol_l - ethanol_metabolized).max(0.0);

        self.acetaldehyde_concentration_umol_l =
            (self.acetaldehyde_concentration_umol_l + acetaldehyde_produced - acetaldehyde_clearance).max(0.0);

        self.acetate_concentration_mmol_l += acetaldehyde_clearance / 1000.0;

        self.blood_alcohol_concentration_mg_dl = self.ethanol_concentration_mmol_l * 46.07 / 10.0;
    }

    pub fn flush_response_score(&self) -> f64 {
        let acetaldehyde_factor = (self.acetaldehyde_concentration_umol_l / 10.0).min(10.0);

        let genotype_sensitivity = match self.aldh2_genotype {
            ALDH2Genotype::WildType => 1.0,
            ALDH2Genotype::Heterozygous => 3.0,
            ALDH2Genotype::Homozygous => 5.0,
        };

        (acetaldehyde_factor * genotype_sensitivity).min(10.0)
    }

    pub fn current_symptoms(&self) -> Vec<String> {
        let mut symptoms = Vec::new();

        let flush_score = self.flush_response_score();

        if flush_score > 2.0 {
            symptoms.push("Facial flushing".to_string());
        }
        if flush_score > 3.0 {
            symptoms.push("Increased heart rate".to_string());
        }
        if flush_score > 4.0 {
            symptoms.push("Nausea".to_string());
        }
        if flush_score > 5.0 {
            symptoms.push("Headache".to_string());
        }
        if flush_score > 6.0 {
            symptoms.push("Dizziness".to_string());
        }
        if flush_score > 7.0 {
            symptoms.push("Vomiting".to_string());
        }

        if self.blood_alcohol_concentration_mg_dl > 80.0 {
            symptoms.push("Intoxication".to_string());
        }

        symptoms
    }

    pub fn time_to_clear_hours(&self) -> f64 {
        if self.ethanol_concentration_mmol_l < 0.1 && self.acetaldehyde_concentration_umol_l < 1.0 {
            return 0.0;
        }

        let ethanol_clearance_hours = self.ethanol_concentration_mmol_l / (0.15 * self.adh_activity_relative);

        let acetaldehyde_hours = if self.aldh2_activity_relative > 0.1 {
            self.acetaldehyde_concentration_umol_l / (0.8 * self.aldh2_activity_relative * 1000.0)
        } else {
            10.0
        };

        ethanol_clearance_hours + acetaldehyde_hours
    }
}

impl AlcoholMetabolismSimulation {
    pub fn simulate(
        adh1b_genotype: ADH1BGenotype,
        aldh2_genotype: ALDH2Genotype,
        ingestion: &AlcoholIngestion,
        duration_hours: f64,
        step_size_hours: f64,
    ) -> Self {
        let mut pathway = AlcoholMetabolismPathway::new(adh1b_genotype, aldh2_genotype);
        pathway.ingest_alcohol(ingestion);

        let mut timeline = Vec::new();

        timeline.push(MetabolismTimePoint {
            time_hours: 0.0,
            ethanol_mmol_l: pathway.ethanol_concentration_mmol_l,
            acetaldehyde_umol_l: pathway.acetaldehyde_concentration_umol_l,
            acetate_mmol_l: pathway.acetate_concentration_mmol_l,
            flush_response_score: pathway.flush_response_score(),
            symptoms: pathway.current_symptoms(),
        });

        let mut current_time = 0.0;
        while current_time < duration_hours {
            pathway.step(step_size_hours);
            current_time += step_size_hours;

            timeline.push(MetabolismTimePoint {
                time_hours: current_time,
                ethanol_mmol_l: pathway.ethanol_concentration_mmol_l,
                acetaldehyde_umol_l: pathway.acetaldehyde_concentration_umol_l,
                acetate_mmol_l: pathway.acetate_concentration_mmol_l,
                flush_response_score: pathway.flush_response_score(),
                symptoms: pathway.current_symptoms(),
            });
        }

        Self { pathway, timeline }
    }

    pub fn peak_acetaldehyde(&self) -> f64 {
        self.timeline.iter()
            .map(|tp| tp.acetaldehyde_umol_l)
            .fold(0.0, f64::max)
    }

    pub fn area_under_curve_acetaldehyde(&self) -> f64 {
        let mut auc = 0.0;
        for i in 1..self.timeline.len() {
            let dt = self.timeline[i].time_hours - self.timeline[i-1].time_hours;
            let avg_concentration = (self.timeline[i].acetaldehyde_umol_l +
                                    self.timeline[i-1].acetaldehyde_umol_l) / 2.0;
            auc += avg_concentration * dt;
        }
        auc
    }

    pub fn time_above_threshold(&self, threshold_umol_l: f64) -> f64 {
        let mut time_above = 0.0;
        for i in 1..self.timeline.len() {
            if self.timeline[i].acetaldehyde_umol_l > threshold_umol_l {
                time_above += self.timeline[i].time_hours - self.timeline[i-1].time_hours;
            }
        }
        time_above
    }

    pub fn print_summary(&self) {
        println!("\n=== Alcohol Metabolism Simulation ===");
        println!("ADH1B Genotype: {}", self.pathway.adh1b_genotype.description());
        println!("ALDH2 Genotype: {}", self.pathway.aldh2_genotype.description());
        println!("\nPeak BAC: {:.2} mg/dL", self.timeline[0].ethanol_mmol_l * 46.07 / 10.0);
        println!("Peak Acetaldehyde: {:.1} µmol/L", self.peak_acetaldehyde());
        println!("Acetaldehyde AUC: {:.1} µmol·h/L", self.area_under_curve_acetaldehyde());
        println!("Time with Acetaldehyde >10 µmol/L: {:.2} hours", self.time_above_threshold(10.0));

        if let Some(worst) = self.timeline.iter().max_by(|a, b|
            a.flush_response_score.partial_cmp(&b.flush_response_score).unwrap()
        ) {
            println!("\nPeak Flush Response: {:.1}/10 at {:.1} hours",
                worst.flush_response_score, worst.time_hours);
            if !worst.symptoms.is_empty() {
                println!("Symptoms: {}", worst.symptoms.join(", "));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aldh2_activity() {
        assert_eq!(ALDH2Genotype::WildType.aldh2_activity_multiplier(), 1.0);
        assert_eq!(ALDH2Genotype::Heterozygous.aldh2_activity_multiplier(), 0.15);
        assert_eq!(ALDH2Genotype::Homozygous.aldh2_activity_multiplier(), 0.01);
    }

    #[test]
    fn test_adh1b_activity() {
        assert_eq!(ADH1BGenotype::Slow.adh_activity_multiplier(), 0.5);
        assert_eq!(ADH1BGenotype::Intermediate.adh_activity_multiplier(), 1.0);
        assert_eq!(ADH1BGenotype::Fast.adh_activity_multiplier(), 2.5);
    }

    #[test]
    fn test_bac_calculation() {
        let ingestion = AlcoholIngestion {
            volume_ml: 355.0,
            alcohol_percentage: 5.0,
            body_weight_kg: 70.0,
            sex: Sex::Male,
        };

        let bac = ingestion.calculate_peak_bac();
        assert!(bac > 0.0 && bac < 100.0);
    }

    #[test]
    fn test_wildtype_vs_deficient() {
        let ingestion = AlcoholIngestion {
            volume_ml: 355.0,
            alcohol_percentage: 5.0,
            body_weight_kg: 70.0,
            sex: Sex::Male,
        };

        let wildtype_sim = AlcoholMetabolismSimulation::simulate(
            ADH1BGenotype::Fast,
            ALDH2Genotype::WildType,
            &ingestion,
            8.0,
            0.1,
        );

        let deficient_sim = AlcoholMetabolismSimulation::simulate(
            ADH1BGenotype::Fast,
            ALDH2Genotype::Heterozygous,
            &ingestion,
            8.0,
            0.1,
        );

        let wildtype_peak = wildtype_sim.peak_acetaldehyde();
        let deficient_peak = deficient_sim.peak_acetaldehyde();

        assert!(deficient_peak > wildtype_peak * 1.5,
            "ALDH2 deficiency should cause at least 1.5x higher acetaldehyde (literature: 2-10x)");
        assert!(deficient_peak < wildtype_peak * 15.0,
            "Peak should be reasonable (literature: 2-10x)");
    }

    #[test]
    fn test_cancer_risk() {
        let genotype = ALDH2Genotype::Heterozygous;

        assert_eq!(genotype.cancer_risk_multiplier(AlcoholConsumptionLevel::None), 1.0);
        assert_eq!(genotype.cancer_risk_multiplier(AlcoholConsumptionLevel::Light), 2.0);
        assert_eq!(genotype.cancer_risk_multiplier(AlcoholConsumptionLevel::Heavy), 10.0);
    }

    #[test]
    fn test_metabolism_pathway() {
        let mut pathway = AlcoholMetabolismPathway::new(
            ADH1BGenotype::Fast,
            ALDH2Genotype::Heterozygous,
        );

        let ingestion = AlcoholIngestion {
            volume_ml: 355.0,
            alcohol_percentage: 5.0,
            body_weight_kg: 70.0,
            sex: Sex::Male,
        };

        pathway.ingest_alcohol(&ingestion);
        assert!(pathway.ethanol_concentration_mmol_l > 0.0);

        pathway.step(1.0);
        assert!(pathway.acetaldehyde_concentration_umol_l > 0.0);
    }
}

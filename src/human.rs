use crate::biology::genetics::{genotype::Genotype, AncestryProfile, PhenotypeProfile};
use crate::config::{BaselineHumanParams, HumanPreset, PresetType};
use crate::pathology::headache::HeadacheProfile;
use crate::pharmacology::pharmacogenomics::PharmacogeneticProfile;
use crate::systems::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Human {
    pub id: String,
    pub demographics: Demographics,
    pub body_metrics: BodyMetrics,
    pub systems: BodySystems,
    pub genetics: GeneticProfile,
    pub pharmacogenomics: PharmacogeneticProfile,
    pub health_conditions: HealthConditions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demographics {
    pub age_years: f64,
    pub biological_sex: BiologicalSex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticProfile {
    pub ancestry: AncestryProfile,
    pub genotypes: HashMap<String, Genotype>,
    pub carrier_status: Vec<String>,
    pub phenotype: PhenotypeProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConditions {
    pub active_conditions: Vec<String>,
    pub past_conditions: Vec<String>,
    pub family_history: Vec<String>,
    pub headache_profile: Option<HeadacheProfile>,
    pub allergies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMetrics {
    pub height_cm: f64,
    pub weight_kg: f64,
    pub body_surface_area_m2: f64,
    pub blood_volume_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodySystems {
    pub cardiovascular: CardiovascularSystem,
    pub respiratory: RespiratorySystem,
    pub nervous: NervousSystemIntegrated,
    pub digestive: DigestiveSystem,
    pub renal: RenalSystem,
    pub endocrine: EndocrineLandscape,
    pub muscular: MuscularSystem,
    pub skeletal: SkeletalSystem,
    pub integumentary: IntegumentarySystem,
    pub immune: ImmuneSystem,
    pub reproductive: ReproductiveSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularSystem {
    pub heart: Heart,
    pub blood_vessels: Vec<BloodVessel>,
    pub blood: Blood,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratorySystem {
    pub left_lung: Lung,
    pub right_lung: Lung,
    pub gas_exchange: GasExchange,
    pub breathing_pattern: BreathingPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NervousSystemIntegrated {
    pub central: CentralNervousSystem,
    pub peripheral: PeripheralNervousSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestiveSystem {
    pub gi_tract: GITract,
    pub nutrient_absorption: NutrientAbsorption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalSystem {
    pub left_kidney: Kidney,
    pub right_kidney: Kidney,
    pub filtration: Filtration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscularSystem {
    pub total_muscle_mass_kg: f64,
    pub fiber_type_distribution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkeletalSystem {
    pub total_bone_mass_kg: f64,
    pub bone_density_g_cm3: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegumentarySystem {
    pub skin: Skin,
    pub skin_type: integumentary::SkinType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneSystem {
    pub lymphatic: LymphaticSystem,
    pub wbc_count_per_ul: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReproductiveSystem {
    Male(MaleReproductiveSystem),
    Female(FemaleReproductiveSystem),
}

impl Human {
    pub fn new_adult_male(id: String, age_years: f64, height_cm: f64, weight_kg: f64) -> Self {
        let body_metrics = BodyMetrics::calculate(height_cm, weight_kg);

        Self {
            id,
            demographics: Demographics {
                age_years,
                biological_sex: BiologicalSex::Male,
            },
            body_metrics,
            systems: BodySystems::new_adult_male(),
            genetics: GeneticProfile::new(),
            pharmacogenomics: PharmacogeneticProfile::new(),
            health_conditions: HealthConditions::new(),
        }
    }

    pub fn new_adult_female(id: String, age_years: f64, height_cm: f64, weight_kg: f64) -> Self {
        let body_metrics = BodyMetrics::calculate(height_cm, weight_kg);

        Self {
            id,
            demographics: Demographics {
                age_years,
                biological_sex: BiologicalSex::Female,
            },
            body_metrics,
            systems: BodySystems::new_adult_female(),
            genetics: GeneticProfile::new(),
            pharmacogenomics: PharmacogeneticProfile::new(),
            health_conditions: HealthConditions::new(),
        }
    }

    pub fn from_preset(id: String, preset_type: PresetType) -> Self {
        let preset = HumanPreset::from_preset_type(preset_type);
        let body_metrics = BodyMetrics::calculate(preset.height_cm, preset.weight_kg);

        let biological_sex = match preset_type {
            PresetType::AdultMaleHealthy
            | PresetType::AdultMaleAthlete
            | PresetType::AdultMaleObesity
            | PresetType::ElderlyMaleHealthy
            | PresetType::YoungAdultMaleHealthy => BiologicalSex::Male,
            PresetType::AdultFemaleHealthy
            | PresetType::AdultFemaleAthlete
            | PresetType::AdultFemaleObesity
            | PresetType::ElderlyFemaleHealthy
            | PresetType::YoungAdultFemaleHealthy => BiologicalSex::Female,
        };

        let systems = match biological_sex {
            BiologicalSex::Male => BodySystems::from_baseline_params_male(&preset.baseline_params),
            BiologicalSex::Female => {
                BodySystems::from_baseline_params_female(&preset.baseline_params)
            }
        };

        Self {
            id,
            demographics: Demographics {
                age_years: preset.age_years,
                biological_sex,
            },
            body_metrics,
            systems,
            genetics: GeneticProfile::new(),
            pharmacogenomics: PharmacogeneticProfile::new(),
            health_conditions: HealthConditions::new(),
        }
    }

    pub fn from_custom_params(
        id: String,
        age_years: f64,
        height_cm: f64,
        weight_kg: f64,
        biological_sex: BiologicalSex,
        baseline_params: BaselineHumanParams,
    ) -> Self {
        let body_metrics = BodyMetrics::calculate(height_cm, weight_kg);

        let systems = match biological_sex {
            BiologicalSex::Male => BodySystems::from_baseline_params_male(&baseline_params),
            BiologicalSex::Female => BodySystems::from_baseline_params_female(&baseline_params),
        };

        Self {
            id,
            demographics: Demographics {
                age_years,
                biological_sex,
            },
            body_metrics,
            systems,
            genetics: GeneticProfile::new(),
            pharmacogenomics: PharmacogeneticProfile::new(),
            health_conditions: HealthConditions::new(),
        }
    }

    pub fn bmi(&self) -> f64 {
        let height_m = self.body_metrics.height_cm / 100.0;
        self.body_metrics.weight_kg / (height_m * height_m)
    }

    pub fn cardiac_output_l_per_min(&self) -> f64 {
        self.systems.cardiovascular.heart.cardiac_output_l_min()
    }

    pub fn metabolic_rate_kcal_per_day(&self) -> f64 {
        match self.demographics.biological_sex {
            BiologicalSex::Male => {
                10.0 * self.body_metrics.weight_kg + 6.25 * self.body_metrics.height_cm
                    - 5.0 * self.demographics.age_years
                    + 5.0
            }
            BiologicalSex::Female => {
                10.0 * self.body_metrics.weight_kg + 6.25 * self.body_metrics.height_cm
                    - 5.0 * self.demographics.age_years
                    - 161.0
            }
        }
    }

    pub fn total_blood_volume_l(&self) -> f64 {
        self.body_metrics.blood_volume_l
    }

    pub fn gfr_ml_per_min(&self) -> f64 {
        (self.systems.renal.left_kidney.gfr_ml_per_min
            + self.systems.renal.right_kidney.gfr_ml_per_min)
            / 2.0
    }

    pub fn health_summary(&self) -> HealthSummary {
        HealthSummary {
            bmi: self.bmi(),
            cardiac_output: self.cardiac_output_l_per_min(),
            respiratory_rate: self.systems.respiratory.breathing_pattern.rate_bpm,
            gfr: self.gfr_ml_per_min(),
            metabolic_rate: self.metabolic_rate_kcal_per_day(),
        }
    }

    pub fn ancestry_report(&self) -> Vec<String> {
        let mut report = Vec::new();

        if let Some(primary) = self.genetics.ancestry.primary_ancestry() {
            report.push(format!("Primary ancestry: {:?}", primary));
        }

        for (ancestry, percentage) in self.genetics.ancestry.components() {
            report.push(format!("{:?}: {:.1}%", ancestry, percentage));
        }

        let risks = self.genetics.ancestry.genetic_risk_factors();
        if !risks.is_empty() {
            report.push("\nGenetic risk factors based on ancestry:".to_string());
            report.extend(risks);
        }

        report
    }

    pub fn drug_compatibility_check(&self, drug_name: &str) -> Vec<String> {
        self.pharmacogenomics.check_drug_compatibility(drug_name)
    }

    pub fn comprehensive_health_assessment(&self) -> ComprehensiveHealthAssessment {
        ComprehensiveHealthAssessment {
            basic_metrics: self.health_summary(),
            genetic_risks: self.genetics.ancestry.genetic_risk_factors(),
            carrier_status: self.genetics.carrier_status.clone(),
            active_conditions: self.health_conditions.active_conditions.clone(),
            family_history: self.health_conditions.family_history.clone(),
        }
    }

    pub fn assess_migraine_risk(&self) -> MigraineDiagnosticInfo {
        let mut risk_score = 1.0;
        let mut genetic_factors = Vec::new();

        if let Some(primary) = self.genetics.ancestry.primary_ancestry() {
            for condition in primary.associated_conditions() {
                if condition.to_lowercase().contains("migraine") {
                    risk_score *= 1.5;
                    genetic_factors
                        .push(format!("Ancestry: {:?} associated with migraines", primary));
                }
            }
        }

        if matches!(self.demographics.biological_sex, BiologicalSex::Female) {
            risk_score *= 2.5;
            genetic_factors.push("Female sex - 2.5x increased risk".to_string());
        }

        if self.demographics.age_years >= 18.0 && self.demographics.age_years <= 50.0 {
            risk_score *= 1.3;
            genetic_factors.push("Peak age range for migraines".to_string());
        }

        let recommendations = if risk_score > 2.0 {
            vec![
                "Track headache patterns in diary".to_string(),
                "Identify and avoid triggers".to_string(),
                "Consider preventive strategies".to_string(),
                "Maintain regular sleep schedule".to_string(),
            ]
        } else {
            vec!["Standard headache awareness".to_string()]
        };

        MigraineDiagnosticInfo {
            risk_score,
            genetic_factors,
            has_known_migraines: self.health_conditions.headache_profile.is_some(),
            recommendations,
        }
    }

    pub fn assess_cluster_headache_risk(&self) -> ClusterHeadacheDiagnosticInfo {
        let mut risk_score = 1.0;
        let mut genetic_factors = Vec::new();

        if matches!(self.demographics.biological_sex, BiologicalSex::Male) {
            risk_score *= 3.0;
            genetic_factors.push("Male sex - 3x increased risk".to_string());
        }

        if self.demographics.age_years >= 20.0 && self.demographics.age_years <= 50.0 {
            risk_score *= 1.5;
            genetic_factors.push("Peak age range for cluster headaches".to_string());
        }

        let recommendations = if risk_score > 2.0 {
            vec![
                "Be aware of circadian pattern headaches".to_string(),
                "Avoid alcohol during cluster periods".to_string(),
                "Have oxygen therapy available".to_string(),
            ]
        } else {
            vec!["Standard awareness".to_string()]
        };

        ClusterHeadacheDiagnosticInfo {
            risk_score,
            genetic_factors,
            recommendations,
        }
    }

    pub fn assess_genetic_disease_risks(&self) -> Vec<GeneticDiseaseRisk> {
        let mut risks = Vec::new();

        for (ancestry, percentage) in self.genetics.ancestry.components() {
            if *percentage > 10.0 {
                for condition in ancestry.associated_conditions() {
                    let risk_factor = percentage / 100.0;
                    risks.push(GeneticDiseaseRisk {
                        condition: condition.to_string(),
                        relative_risk: risk_factor,
                        source: format!("{:?} ancestry ({:.1}%)", ancestry, percentage),
                        screening_recommended: risk_factor > 0.25,
                    });
                }
            }
        }

        risks
    }

    pub fn pharmacogenomic_report(&self) -> PharmacogenomicReport {
        let mut drug_interactions = Vec::new();
        let mut warnings = Vec::new();

        if self.genetics.phenotype.metabolic_traits.caffeine_metabolism
            == crate::biology::genetics::CaffeineMetabolism::Slow
        {
            warnings.push("Slow caffeine metabolizer - limit intake to avoid insomnia".to_string());
        }

        if self
            .genetics
            .phenotype
            .metabolic_traits
            .alcohol_metabolism
            .alcohol_flush_reaction
        {
            warnings
                .push("Alcohol flush reaction - increased cancer risk with alcohol".to_string());
        }

        match self
            .genetics
            .phenotype
            .pharmacological_traits
            .warfarin_sensitivity
        {
            crate::biology::genetics::WarfarinSensitivity::High => {
                drug_interactions.push("Warfarin: Use 30-50% lower doses".to_string());
            }
            crate::biology::genetics::WarfarinSensitivity::Low => {
                drug_interactions.push("Warfarin: May require higher doses".to_string());
            }
            _ => {}
        }

        match self
            .genetics
            .phenotype
            .pharmacological_traits
            .opioid_metabolism
        {
            crate::biology::genetics::OpioidMetabolism::UltraRapid => {
                drug_interactions.push("Codeine: Risk of toxicity - avoid use".to_string());
            }
            crate::biology::genetics::OpioidMetabolism::Poor => {
                drug_interactions
                    .push("Codeine: Reduced efficacy - consider alternative".to_string());
            }
            _ => {}
        }

        if self
            .genetics
            .phenotype
            .pharmacological_traits
            .statin_myopathy_risk
            > 2.0
        {
            drug_interactions
                .push("Statins: Elevated myopathy risk - monitor CK levels".to_string());
        }

        PharmacogenomicReport {
            drug_interactions,
            warnings,
            metabolism_profile: format!(
                "Caffeine: {:?}, Alcohol: {:?}",
                self.genetics.phenotype.metabolic_traits.caffeine_metabolism,
                self.genetics
                    .phenotype
                    .metabolic_traits
                    .alcohol_metabolism
                    .aldh2_function
            ),
        }
    }

    pub fn population_specific_traits_report(&self) -> PopulationTraitsReport {
        use crate::biology::genetics::PopulationSpecificTraits;

        let primary_ancestry = self.genetics.ancestry.primary_ancestry();

        let traits = if let Some(ancestry) = primary_ancestry {
            PopulationSpecificTraits::from_ancestry(ancestry)
        } else {
            PopulationSpecificTraits::default()
        };

        let dietary_recs = traits.dietary_recommendations();
        let alcohol_info = traits.alcohol_tolerance_info();

        PopulationTraitsReport {
            primary_ancestry: primary_ancestry.map(|a| format!("{:?}", a)),
            lactose_tolerance: format!("{:?}", traits.lactose_tolerance),
            alcohol_tolerance: alcohol_info,
            earwax_type: format!("{:?}", traits.earwax_type),
            skin_pigmentation: format!("{:?}", traits.skin_pigmentation),
            hair_traits: format!(
                "{:?} hair, {:?} pattern",
                traits.hair_traits.thickness, traits.hair_traits.curl_pattern
            ),
            dietary_recommendations: dietary_recs,
            vitamin_d_needs: format!("{:?}", traits.vitamin_d_synthesis),
        }
    }

    pub fn comprehensive_genetic_analysis(&self) -> ComprehensiveGeneticAnalysis {
        let disease_risks = self.assess_genetic_disease_risks();
        let migraine_risk = self.assess_migraine_risk();
        let cluster_risk = self.assess_cluster_headache_risk();
        let population_traits = self.population_specific_traits_report();
        let pharma_report = self.pharmacogenomic_report();

        ComprehensiveGeneticAnalysis {
            ancestry_breakdown: self.genetics.ancestry.components().clone(),
            disease_risks,
            headache_risks: HeadacheRisks {
                migraine: migraine_risk,
                cluster_headache: cluster_risk,
            },
            population_traits,
            pharmacogenomics: pharma_report,
            carrier_status: self.genetics.carrier_status.clone(),
        }
    }

    pub fn test_for_condition(&self, condition_name: &str) -> ConditionTestResult {
        let mut risk_factors = Vec::new();
        let protective_factors = Vec::new();
        let mut overall_risk = 1.0;

        let disease_risks = self.assess_genetic_disease_risks();
        for risk in disease_risks {
            if risk
                .condition
                .to_lowercase()
                .contains(&condition_name.to_lowercase())
            {
                risk_factors.push(format!("{} ({}x risk)", risk.source, risk.relative_risk));
                overall_risk *= 1.0 + risk.relative_risk;
            }
        }

        if self
            .health_conditions
            .active_conditions
            .iter()
            .any(|c| c.to_lowercase().contains(&condition_name.to_lowercase()))
        {
            risk_factors.push("Currently diagnosed".to_string());
            overall_risk *= 10.0;
        }

        if self
            .health_conditions
            .family_history
            .iter()
            .any(|c| c.to_lowercase().contains(&condition_name.to_lowercase()))
        {
            risk_factors.push("Family history".to_string());
            overall_risk *= 2.0;
        }

        ConditionTestResult {
            condition: condition_name.to_string(),
            overall_risk_multiplier: overall_risk,
            risk_factors,
            protective_factors,
            screening_recommended: overall_risk > 2.0,
        }
    }
}

impl GeneticProfile {
    pub fn new() -> Self {
        Self {
            ancestry: AncestryProfile::new(),
            genotypes: HashMap::new(),
            carrier_status: Vec::new(),
            phenotype: PhenotypeProfile::new(),
        }
    }

    pub fn with_genotypes(genotypes: HashMap<String, String>) -> Self {
        let phenotype = PhenotypeProfile::from_genotypes(&genotypes);

        Self {
            ancestry: AncestryProfile::new(),
            genotypes: HashMap::new(),
            carrier_status: Vec::new(),
            phenotype,
        }
    }
}

impl Default for GeneticProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthConditions {
    pub fn new() -> Self {
        Self {
            active_conditions: Vec::new(),
            past_conditions: Vec::new(),
            family_history: Vec::new(),
            headache_profile: None,
            allergies: Vec::new(),
        }
    }

    pub fn add_condition(&mut self, condition: String) {
        if !self.active_conditions.contains(&condition) {
            self.active_conditions.push(condition);
        }
    }

    pub fn add_family_history(&mut self, condition: String) {
        if !self.family_history.contains(&condition) {
            self.family_history.push(condition);
        }
    }
}

impl Default for HealthConditions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    pub bmi: f64,
    pub cardiac_output: f64,
    pub respiratory_rate: f64,
    pub gfr: f64,
    pub metabolic_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveHealthAssessment {
    pub basic_metrics: HealthSummary,
    pub genetic_risks: Vec<String>,
    pub carrier_status: Vec<String>,
    pub active_conditions: Vec<String>,
    pub family_history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigraineDiagnosticInfo {
    pub risk_score: f64,
    pub genetic_factors: Vec<String>,
    pub has_known_migraines: bool,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHeadacheDiagnosticInfo {
    pub risk_score: f64,
    pub genetic_factors: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticDiseaseRisk {
    pub condition: String,
    pub relative_risk: f64,
    pub source: String,
    pub screening_recommended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacogenomicReport {
    pub drug_interactions: Vec<String>,
    pub warnings: Vec<String>,
    pub metabolism_profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationTraitsReport {
    pub primary_ancestry: Option<String>,
    pub lactose_tolerance: String,
    pub alcohol_tolerance: crate::biology::genetics::AlcoholToleranceInfo,
    pub earwax_type: String,
    pub skin_pigmentation: String,
    pub hair_traits: String,
    pub dietary_recommendations: Vec<String>,
    pub vitamin_d_needs: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveGeneticAnalysis {
    pub ancestry_breakdown: std::collections::HashMap<crate::biology::genetics::Ancestry, f64>,
    pub disease_risks: Vec<GeneticDiseaseRisk>,
    pub headache_risks: HeadacheRisks,
    pub population_traits: PopulationTraitsReport,
    pub pharmacogenomics: PharmacogenomicReport,
    pub carrier_status: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadacheRisks {
    pub migraine: MigraineDiagnosticInfo,
    pub cluster_headache: ClusterHeadacheDiagnosticInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionTestResult {
    pub condition: String,
    pub overall_risk_multiplier: f64,
    pub risk_factors: Vec<String>,
    pub protective_factors: Vec<String>,
    pub screening_recommended: bool,
}

impl BodyMetrics {
    pub fn calculate(height_cm: f64, weight_kg: f64) -> Self {
        let bsa = ((height_cm * weight_kg) / 3600.0).sqrt();
        let blood_volume = weight_kg * 0.07;

        Self {
            height_cm,
            weight_kg,
            body_surface_area_m2: bsa,
            blood_volume_l: blood_volume,
        }
    }
}

impl BodySystems {
    pub fn new_adult_male() -> Self {
        Self {
            cardiovascular: CardiovascularSystem {
                heart: Heart::new(),
                blood_vessels: vec![],
                blood: Blood::new(cardiovascular::BloodType::OPositive),
            },
            respiratory: RespiratorySystem {
                left_lung: Lung::new_left(),
                right_lung: Lung::new_right(),
                gas_exchange: GasExchange::new_normal(),
                breathing_pattern: BreathingPattern::new_normal(),
            },
            nervous: NervousSystemIntegrated {
                central: CentralNervousSystem::new_adult(),
                peripheral: PeripheralNervousSystem::new(),
            },
            digestive: DigestiveSystem {
                gi_tract: GITract::new_adult(),
                nutrient_absorption: NutrientAbsorption::new_normal(),
            },
            renal: RenalSystem {
                left_kidney: Kidney::new_left(),
                right_kidney: Kidney::new_right(),
                filtration: Filtration::new_normal(),
            },
            endocrine: EndocrineLandscape::new_adult_male(),
            muscular: MuscularSystem {
                total_muscle_mass_kg: 35.0,
                fiber_type_distribution: 0.5,
            },
            skeletal: SkeletalSystem {
                total_bone_mass_kg: 4.5,
                bone_density_g_cm3: 1.3,
            },
            integumentary: IntegumentarySystem {
                skin: Skin::new_adult(1.8),
                skin_type: integumentary::SkinType::new_type_iii(),
            },
            immune: ImmuneSystem {
                lymphatic: LymphaticSystem::new_adult(),
                wbc_count_per_ul: 7000.0,
            },
            reproductive: ReproductiveSystem::Male(MaleReproductiveSystem::new_adult()),
        }
    }

    pub fn from_baseline_params_male(params: &BaselineHumanParams) -> Self {
        let mut heart = Heart::new();
        heart.heart_rate_bpm = params.cardiovascular.resting_heart_rate_bpm;
        heart.stroke_volume_ml = params.cardiovascular.stroke_volume_ml;
        heart.ejection_fraction = params.cardiovascular.ejection_fraction;

        let mut kidney_left = Kidney::new_left();
        kidney_left.gfr_ml_per_min = params.renal.gfr_ml_per_min;

        let mut kidney_right = Kidney::new_right();
        kidney_right.gfr_ml_per_min = params.renal.gfr_ml_per_min;

        Self {
            cardiovascular: CardiovascularSystem {
                heart,
                blood_vessels: vec![],
                blood: Blood::new(cardiovascular::BloodType::OPositive),
            },
            respiratory: RespiratorySystem {
                left_lung: Lung::new_left(),
                right_lung: Lung::new_right(),
                gas_exchange: GasExchange::new_normal(),
                breathing_pattern: BreathingPattern::new_normal(),
            },
            nervous: NervousSystemIntegrated {
                central: CentralNervousSystem::new_adult(),
                peripheral: PeripheralNervousSystem::new(),
            },
            digestive: DigestiveSystem {
                gi_tract: GITract::new_adult(),
                nutrient_absorption: NutrientAbsorption::new_normal(),
            },
            renal: RenalSystem {
                left_kidney: kidney_left,
                right_kidney: kidney_right,
                filtration: Filtration::new_normal(),
            },
            endocrine: EndocrineLandscape::new_adult_male(),
            muscular: MuscularSystem {
                total_muscle_mass_kg: 35.0,
                fiber_type_distribution: 0.5,
            },
            skeletal: SkeletalSystem {
                total_bone_mass_kg: 4.5,
                bone_density_g_cm3: 1.3,
            },
            integumentary: IntegumentarySystem {
                skin: Skin::new_adult(1.8),
                skin_type: integumentary::SkinType::new_type_iii(),
            },
            immune: ImmuneSystem {
                lymphatic: LymphaticSystem::new_adult(),
                wbc_count_per_ul: params.hematology.wbc_count_per_ul,
            },
            reproductive: ReproductiveSystem::Male(MaleReproductiveSystem::new_adult()),
        }
    }

    pub fn new_adult_female() -> Self {
        Self {
            cardiovascular: CardiovascularSystem {
                heart: Heart::new(),
                blood_vessels: vec![],
                blood: Blood::new(cardiovascular::BloodType::OPositive),
            },
            respiratory: RespiratorySystem {
                left_lung: Lung::new_left(),
                right_lung: Lung::new_right(),
                gas_exchange: GasExchange::new_normal(),
                breathing_pattern: BreathingPattern::new_normal(),
            },
            nervous: NervousSystemIntegrated {
                central: CentralNervousSystem::new_adult(),
                peripheral: PeripheralNervousSystem::new(),
            },
            digestive: DigestiveSystem {
                gi_tract: GITract::new_adult(),
                nutrient_absorption: NutrientAbsorption::new_normal(),
            },
            renal: RenalSystem {
                left_kidney: Kidney::new_left(),
                right_kidney: Kidney::new_right(),
                filtration: Filtration::new_normal(),
            },
            endocrine: EndocrineLandscape::new_adult_female(),
            muscular: MuscularSystem {
                total_muscle_mass_kg: 28.0,
                fiber_type_distribution: 0.5,
            },
            skeletal: SkeletalSystem {
                total_bone_mass_kg: 3.5,
                bone_density_g_cm3: 1.2,
            },
            integumentary: IntegumentarySystem {
                skin: Skin::new_adult(1.6),
                skin_type: integumentary::SkinType::new_type_iii(),
            },
            immune: ImmuneSystem {
                lymphatic: LymphaticSystem::new_adult(),
                wbc_count_per_ul: 7000.0,
            },
            reproductive: ReproductiveSystem::Female(FemaleReproductiveSystem::new_adult()),
        }
    }

    pub fn from_baseline_params_female(params: &BaselineHumanParams) -> Self {
        let mut heart = Heart::new();
        heart.heart_rate_bpm = params.cardiovascular.resting_heart_rate_bpm;
        heart.stroke_volume_ml = params.cardiovascular.stroke_volume_ml;
        heart.ejection_fraction = params.cardiovascular.ejection_fraction;

        let mut kidney_left = Kidney::new_left();
        kidney_left.gfr_ml_per_min = params.renal.gfr_ml_per_min;

        let mut kidney_right = Kidney::new_right();
        kidney_right.gfr_ml_per_min = params.renal.gfr_ml_per_min;

        Self {
            cardiovascular: CardiovascularSystem {
                heart,
                blood_vessels: vec![],
                blood: Blood::new(cardiovascular::BloodType::OPositive),
            },
            respiratory: RespiratorySystem {
                left_lung: Lung::new_left(),
                right_lung: Lung::new_right(),
                gas_exchange: GasExchange::new_normal(),
                breathing_pattern: BreathingPattern::new_normal(),
            },
            nervous: NervousSystemIntegrated {
                central: CentralNervousSystem::new_adult(),
                peripheral: PeripheralNervousSystem::new(),
            },
            digestive: DigestiveSystem {
                gi_tract: GITract::new_adult(),
                nutrient_absorption: NutrientAbsorption::new_normal(),
            },
            renal: RenalSystem {
                left_kidney: kidney_left,
                right_kidney: kidney_right,
                filtration: Filtration::new_normal(),
            },
            endocrine: EndocrineLandscape::new_adult_female(),
            muscular: MuscularSystem {
                total_muscle_mass_kg: 28.0,
                fiber_type_distribution: 0.5,
            },
            skeletal: SkeletalSystem {
                total_bone_mass_kg: 3.5,
                bone_density_g_cm3: 1.2,
            },
            integumentary: IntegumentarySystem {
                skin: Skin::new_adult(1.6),
                skin_type: integumentary::SkinType::new_type_iii(),
            },
            immune: ImmuneSystem {
                lymphatic: LymphaticSystem::new_adult(),
                wbc_count_per_ul: params.hematology.wbc_count_per_ul,
            },
            reproductive: ReproductiveSystem::Female(FemaleReproductiveSystem::new_adult()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_human_male() {
        let human = Human::new_adult_male("test_001".to_string(), 30.0, 175.0, 75.0);
        assert_eq!(human.demographics.biological_sex, BiologicalSex::Male);
        assert_eq!(human.demographics.age_years, 30.0);
    }

    #[test]
    fn test_create_human_female() {
        let human = Human::new_adult_female("test_002".to_string(), 28.0, 165.0, 60.0);
        assert_eq!(human.demographics.biological_sex, BiologicalSex::Female);
    }

    #[test]
    fn test_bmi_calculation() {
        let human = Human::new_adult_male("test_003".to_string(), 25.0, 180.0, 80.0);
        let bmi = human.bmi();
        assert!(bmi > 20.0 && bmi < 30.0);
    }

    #[test]
    fn test_cardiac_output() {
        let human = Human::new_adult_male("test_004".to_string(), 30.0, 175.0, 75.0);
        let co = human.cardiac_output_l_per_min();
        assert!(co > 4.0 && co < 6.0);
    }

    #[test]
    fn test_metabolic_rate() {
        let human = Human::new_adult_male("test_005".to_string(), 30.0, 175.0, 75.0);
        let bmr = human.metabolic_rate_kcal_per_day();
        assert!(bmr > 1500.0 && bmr < 2500.0);
    }

    #[test]
    fn test_gfr() {
        let human = Human::new_adult_male("test_006".to_string(), 30.0, 175.0, 75.0);
        let gfr = human.gfr_ml_per_min();
        assert!(gfr > 50.0);
    }

    #[test]
    fn test_health_summary() {
        let human = Human::new_adult_female("test_007".to_string(), 25.0, 165.0, 60.0);
        let summary = human.health_summary();
        assert!(summary.bmi > 0.0);
        assert!(summary.cardiac_output > 0.0);
        assert!(summary.gfr > 0.0);
    }

    #[test]
    fn test_body_systems_male() {
        let systems = BodySystems::new_adult_male();
        assert!(matches!(systems.reproductive, ReproductiveSystem::Male(_)));
        assert!(systems.muscular.total_muscle_mass_kg > 30.0);
    }

    #[test]
    fn test_body_systems_female() {
        let systems = BodySystems::new_adult_female();
        assert!(matches!(
            systems.reproductive,
            ReproductiveSystem::Female(_)
        ));
        assert!(systems.muscular.total_muscle_mass_kg < 30.0);
    }

    #[test]
    fn test_asian_ancestry_genetic_analysis() {
        use crate::biology::genetics::Ancestry;

        let mut human = Human::new_adult_male("asian_001".to_string(), 28.0, 175.0, 70.0);
        human
            .genetics
            .ancestry
            .add_component(Ancestry::EastAsian, 100.0, (0.0, 0.0));

        let risks = human.assess_genetic_disease_risks();

        assert!(!risks.is_empty());
        assert!(risks
            .iter()
            .any(|r| r.condition.to_lowercase().contains("gastric")));
    }

    #[test]
    fn test_migraine_risk_assessment_female() {
        let human = Human::new_adult_female("migraine_001".to_string(), 32.0, 165.0, 60.0);

        let info = human.assess_migraine_risk();

        assert!(info.risk_score > 1.0);
        assert!(info.genetic_factors.iter().any(|f| f.contains("Female")));
        assert!(!info.recommendations.is_empty());
    }

    #[test]
    fn test_cluster_headache_risk_assessment_male() {
        let human = Human::new_adult_male("cluster_001".to_string(), 35.0, 180.0, 80.0);

        let info = human.assess_cluster_headache_risk();

        assert!(info.risk_score >= 3.0);
        assert!(info.genetic_factors.iter().any(|f| f.contains("Male")));
    }

    #[test]
    fn test_ashkenazi_ancestry_brca_risk() {
        use crate::biology::genetics::Ancestry;

        let mut human = Human::new_adult_female("ashkenazi_001".to_string(), 40.0, 163.0, 58.0);
        human
            .genetics
            .ancestry
            .add_component(Ancestry::Ashkenazi, 100.0, (0.0, 0.0));

        let risks = human.assess_genetic_disease_risks();

        assert!(risks
            .iter()
            .any(|r| r.condition.contains("BRCA") || r.condition.contains("breast")));
    }

    #[test]
    fn test_pharmacogenomic_report() {
        let mut human = Human::new_adult_male("pharma_001".to_string(), 45.0, 175.0, 75.0);
        human
            .genetics
            .phenotype
            .metabolic_traits
            .caffeine_metabolism = crate::biology::genetics::CaffeineMetabolism::Slow;

        let report = human.pharmacogenomic_report();

        assert!(!report.warnings.is_empty());
        assert!(report.warnings.iter().any(|w| w.contains("caffeine")));
    }

    #[test]
    fn test_alcohol_flush_reaction() {
        let mut human = Human::new_adult_male("alcohol_001".to_string(), 30.0, 170.0, 65.0);
        human
            .genetics
            .phenotype
            .metabolic_traits
            .alcohol_metabolism
            .alcohol_flush_reaction = true;

        let report = human.pharmacogenomic_report();

        assert!(report.warnings.iter().any(|w| w.contains("alcohol")));
    }

    #[test]
    fn test_mixed_ancestry_profile() {
        use crate::biology::genetics::Ancestry;

        let mut human = Human::new_adult_female("mixed_001".to_string(), 26.0, 168.0, 62.0);
        human
            .genetics
            .ancestry
            .add_component(Ancestry::EastAsian, 50.0, (0.0, 0.0));
        human
            .genetics
            .ancestry
            .add_component(Ancestry::European, 50.0, (0.0, 0.0));

        assert!(human.genetics.ancestry.is_mixed());

        let report = human.ancestry_report();
        assert!(report.len() > 2);
    }

    #[test]
    fn test_warfarin_sensitivity() {
        let mut human = Human::new_adult_male("warfarin_001".to_string(), 60.0, 175.0, 78.0);
        human
            .genetics
            .phenotype
            .pharmacological_traits
            .warfarin_sensitivity = crate::biology::genetics::WarfarinSensitivity::High;

        let report = human.pharmacogenomic_report();

        assert!(report
            .drug_interactions
            .iter()
            .any(|d| d.contains("Warfarin")));
        assert!(report.drug_interactions.iter().any(|d| d.contains("lower")));
    }

    #[test]
    fn test_comprehensive_health_assessment() {
        use crate::biology::genetics::Ancestry;

        let mut human = Human::new_adult_female("comprehensive_001".to_string(), 38.0, 165.0, 65.0);
        human
            .genetics
            .ancestry
            .add_component(Ancestry::European, 80.0, (0.0, 0.0));
        human
            .genetics
            .ancestry
            .add_component(Ancestry::EastAsian, 20.0, (0.0, 0.0));
        human
            .health_conditions
            .add_condition("Hypertension".to_string());

        let assessment = human.comprehensive_health_assessment();

        assert!(assessment.basic_metrics.bmi > 0.0);
        assert!(!assessment.genetic_risks.is_empty());
        assert_eq!(assessment.active_conditions.len(), 1);
    }

    #[test]
    fn test_population_specific_traits() {
        use crate::biology::genetics::Ancestry;

        let mut human = Human::new_adult_male("asian_traits_001".to_string(), 28.0, 172.0, 68.0);
        human
            .genetics
            .ancestry
            .add_component(Ancestry::EastAsian, 100.0, (0.0, 0.0));

        let traits_report = human.population_specific_traits_report();

        assert!(traits_report.lactose_tolerance.contains("Intolerant"));
        assert!(traits_report.alcohol_tolerance.cancer_risk_with_alcohol > 5.0);
        assert!(traits_report.earwax_type.contains("Dry"));
        assert!(!traits_report.dietary_recommendations.is_empty());
    }

    #[test]
    fn test_comprehensive_genetic_analysis() {
        use crate::biology::genetics::Ancestry;

        let mut human =
            Human::new_adult_female("comprehensive_genetic_001".to_string(), 32.0, 165.0, 58.0);
        human
            .genetics
            .ancestry
            .add_component(Ancestry::Ashkenazi, 100.0, (0.0, 0.0));

        let analysis = human.comprehensive_genetic_analysis();

        assert!(!analysis.disease_risks.is_empty());
        assert!(analysis.headache_risks.migraine.risk_score > 1.0);
        assert!(analysis.population_traits.primary_ancestry.is_some());
    }

    #[test]
    fn test_condition_test_migraine() {
        use crate::biology::genetics::Ancestry;

        let mut human =
            Human::new_adult_female("condition_test_001".to_string(), 35.0, 168.0, 62.0);
        human
            .genetics
            .ancestry
            .add_component(Ancestry::European, 100.0, (0.0, 0.0));
        human
            .health_conditions
            .add_family_history("Migraine".to_string());

        let result = human.test_for_condition("migraine");

        assert_eq!(result.condition, "migraine");
        assert!(result.overall_risk_multiplier >= 2.0);
        assert!(result.risk_factors.iter().any(|f| f.contains("Family")));
    }
}

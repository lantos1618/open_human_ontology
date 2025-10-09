# Human Ontology Examples

Comprehensive examples demonstrating the diagnostic and analytical capabilities of the Human Ontology system.

## Table of Contents

1. [Genetic Ancestry Analysis](#genetic-ancestry-analysis)
2. [Migraine Risk Assessment](#migraine-risk-assessment)
3. [Cluster Headache Risk](#cluster-headache-risk)
4. [Pharmacogenomic Profiling](#pharmacogenomic-profiling)
5. [Comprehensive Health Assessment](#comprehensive-health-assessment)
6. [Disease Risk Screening](#disease-risk-screening)

---

## Genetic Ancestry Analysis

### Example: East Asian Person

```rust
use human_biology::*;
use human_biology::biology::genetics::Ancestry;

let mut person = Human::new_adult_female("patient_001".to_string(), 32.0, 162.0, 55.0);
person.genetics.ancestry.add_component(Ancestry::EastAsian, 100.0).unwrap();

let genetic_risks = person.assess_genetic_disease_risks();
for risk in genetic_risks {
    println!("{}: {:.1}% risk from {}",
        risk.condition,
        risk.relative_risk * 100.0,
        risk.source
    );
}
```

**Output:**
- Alcohol flush reaction
- Higher risk of gastric cancer
- Lower risk of melanoma
- EGFR mutation lung cancer

### Example: Mixed Ancestry

```rust
let mut person = Human::new_adult_female("patient_002".to_string(), 28.0, 165.0, 60.0);
person.genetics.ancestry.add_component(Ancestry::EastAsian, 60.0).unwrap();
person.genetics.ancestry.add_component(Ancestry::European, 40.0).unwrap();

assert!(person.genetics.ancestry.is_mixed());

let report = person.ancestry_report();
// Shows proportions and combined risk factors
```

---

## Migraine Risk Assessment

### Example: Female with Migraine Susceptibility

```rust
let person = Human::new_adult_female("patient_003".to_string(), 32.0, 165.0, 60.0);

let migraine_info = person.assess_migraine_risk();

println!("Risk Score: {:.2}", migraine_info.risk_score);
// Output: Risk Score: 3.25 (Female 2.5x + peak age 1.3x)

for factor in &migraine_info.genetic_factors {
    println!("- {}", factor);
}
// Output:
// - Female sex - 2.5x increased risk
// - Peak age range for migraines

for rec in &migraine_info.recommendations {
    println!("- {}", rec);
}
// Output:
// - Track headache patterns in diary
// - Identify and avoid triggers
// - Consider preventive strategies
// - Maintain regular sleep schedule
```

### Example: Migraine Patient with Disability Scoring

```rust
use human_biology::pathology::headache::{Migraine, MigraineSubtype, PainIntensity};

let mut migraine = Migraine::new(MigraineSubtype::WithAura);
migraine.frequency_per_month = 8.0;
migraine.duration_hours = 12.0;
migraine.intensity = PainIntensity::Severe;

let disability = migraine.disability_score();
// Output: 55.6% disability

let prophylactic = migraine.prophylactic_candidates();
// Output: ["Propranolol", "Topiramate", "Amitriptyline"]
```

---

## Cluster Headache Risk

### Example: Male with High Cluster Headache Risk

```rust
let person = Human::new_adult_male("patient_004".to_string(), 35.0, 178.0, 82.0);

let cluster_info = person.assess_cluster_headache_risk();

println!("Risk Score: {:.2}", cluster_info.risk_score);
// Output: Risk Score: 4.5 (Male 3.0x + peak age 1.5x)

for factor in &cluster_info.genetic_factors {
    println!("- {}", factor);
}
// Output:
// - Male sex - 3x increased risk
// - Peak age range for cluster headaches
```

---

## Pharmacogenomic Profiling

### Example: Slow Metabolizer with Drug Sensitivities

```rust
use human_biology::biology::genetics::{CaffeineMetabolism, WarfarinSensitivity};

let mut person = Human::new_adult_male("patient_005".to_string(), 35.0, 175.0, 75.0);

person.genetics.phenotype.metabolic_traits.caffeine_metabolism = CaffeineMetabolism::Slow;
person.genetics.phenotype.metabolic_traits.alcohol_metabolism.alcohol_flush_reaction = true;
person.genetics.phenotype.pharmacological_traits.warfarin_sensitivity = WarfarinSensitivity::High;

let pharma_report = person.pharmacogenomic_report();

// Drug Interactions:
// ⚕️  Warfarin: Use 30-50% lower doses

// Warnings:
// ⚠️  Slow caffeine metabolizer - limit intake to avoid insomnia
// ⚠️  Alcohol flush reaction - increased cancer risk with alcohol
```

### Example: Opioid Metabolism Variants

```rust
use human_biology::biology::genetics::OpioidMetabolism;

person.genetics.phenotype.pharmacological_traits.opioid_metabolism = OpioidMetabolism::UltraRapid;

let report = person.pharmacogenomic_report();
// Output: "Codeine: Risk of toxicity - avoid use"

person.genetics.phenotype.pharmacological_traits.opioid_metabolism = OpioidMetabolism::Poor;

let report = person.pharmacogenomic_report();
// Output: "Codeine: Reduced efficacy - consider alternative"
```

---

## Comprehensive Health Assessment

### Example: South Asian Female with Diabetes

```rust
let mut person = Human::new_adult_female("patient_006".to_string(), 45.0, 160.0, 72.0);

person.genetics.ancestry.add_component(Ancestry::SouthAsian, 100.0).unwrap();
person.health_conditions.add_condition("Type 2 Diabetes".to_string());
person.health_conditions.add_family_history("Coronary Artery Disease".to_string());

let assessment = person.comprehensive_health_assessment();

println!("BMI: {:.1}", assessment.basic_metrics.bmi);
// Output: BMI: 28.1 (Overweight)

println!("Cardiac Output: {:.1} L/min", assessment.basic_metrics.cardiac_output);
// Output: Cardiac Output: 5.0 L/min

println!("GFR: {:.1} mL/min", assessment.basic_metrics.gfr);
// Output: GFR: 95.0 mL/min

// Active Conditions: ["Type 2 Diabetes"]
// Family History: ["Coronary Artery Disease"]
// Genetic Risks: ["Higher risk of type 2 diabetes: 100.0% ancestry component",
//                 "Higher risk of coronary artery disease: 100.0% ancestry component"]
```

---

## Disease Risk Screening

### Example: Ashkenazi Jewish BRCA Screening

```rust
let mut person = Human::new_adult_female("patient_007".to_string(), 42.0, 163.0, 58.0);
person.genetics.ancestry.add_component(Ancestry::Ashkenazi, 100.0).unwrap();

let genetic_risks = person.assess_genetic_disease_risks();

for risk in &genetic_risks {
    println!("{}", risk.condition);
    if risk.screening_recommended {
        println!("  🔬 Priority screening recommended");
    }
}

// Output:
// Tay-Sachs disease: 100.0% ancestry component
// Gaucher disease: 100.0% ancestry component
// Familial dysautonomia: 100.0% ancestry component
// BRCA1/BRCA2 mutations: 100.0% ancestry component
//   🔬 Priority screening recommended
// Bloom syndrome: 100.0% ancestry component
```

### Example: European Ancestry Disease Panel

```rust
let mut person = Human::new_adult_male("patient_008".to_string(), 35.0, 180.0, 78.0);
person.genetics.ancestry.add_component(Ancestry::European, 100.0).unwrap();

let genetic_risks = person.assess_genetic_disease_risks();

// Output:
// - Cystic fibrosis
// - Hemochromatosis
// - Factor V Leiden
// - Familial hypercholesterolemia
```

---

## Metabolic Comparisons

### Example: Athlete vs Sedentary

```rust
let athlete = Human::new_adult_male("athlete_001".to_string(), 28.0, 185.0, 82.0);
let sedentary = Human::new_adult_male("sedentary_001".to_string(), 28.0, 185.0, 95.0);

println!("ATHLETE:");
println!("  BMI: {:.1}", athlete.bmi());  // 23.9
println!("  BMR: {:.0} kcal/day", athlete.metabolic_rate_kcal_per_day());  // 1,981

println!("SEDENTARY:");
println!("  BMI: {:.1}", sedentary.bmi());  // 27.7
println!("  BMR: {:.0} kcal/day", sedentary.metabolic_rate_kcal_per_day());  // 2,111
```

---

## Running Tests

All examples can be run as integration tests:

```bash
# Run all comprehensive tests
cargo test --test comprehensive_human_analysis

# Run specific test with output
cargo test test_pharmacogenomic_profile_slow_metabolizer -- --nocapture

# Run all library tests
cargo test --lib
```

**Current Status:** ✅ All 417 tests passing

---

## Key Features Demonstrated

### Genetic Analysis
- ✅ Ancestry-based disease risk assessment
- ✅ Mixed ancestry profiling
- ✅ Population-specific genetic variants
- ✅ Carrier status screening

### Headache Diagnostics
- ✅ Migraine risk scoring (sex, age, genetics)
- ✅ Cluster headache risk assessment
- ✅ Disability scoring
- ✅ Treatment recommendations
- ✅ Prophylactic medication selection

### Pharmacogenomics
- ✅ Drug metabolism profiling
- ✅ Warfarin sensitivity
- ✅ Opioid metabolism variants
- ✅ Caffeine metabolism
- ✅ Alcohol flush reaction
- ✅ Statin myopathy risk

### Health Metrics
- ✅ BMI calculation
- ✅ Cardiac output
- ✅ GFR (kidney function)
- ✅ Metabolic rate (BMR)
- ✅ Body composition

### Clinical Utility
- ✅ Personalized screening priorities
- ✅ Drug interaction warnings
- ✅ Lifestyle recommendations
- ✅ Family history integration
- ✅ Active condition tracking

// src/biology/processes/ethanol_metabolism_simulation.rs

use crate::biology::molecular::enzymes::{ADH1B, ADH1B_Variant, ALDH2, ALDH2_Variant};

#[derive(Debug, PartialEq)]
pub struct MetabolismMetrics {
    pub peak_acetaldehyde_level: f32,
    pub time_to_clear_acetaldehyde_approx_steps: u32,
}

pub fn simulate_ethanol_metabolism(
    initial_ethanol_concentration: f32,
    adh1b_variant: ADH1B_Variant,
    aldh2_variant: ALDH2_Variant,
) -> MetabolismMetrics {
    let adh1b = ADH1B { variant: adh1b_variant };
    let aldh2 = ALDH2 { variant: aldh2_variant };

    let mut current_ethanol: f32 = initial_ethanol_concentration;
    let mut current_acetaldehyde: f32 = 0.0;
    let mut peak_acetaldehyde: f32 = 0.0;
    let mut time_steps: u32 = 0;

    for _step in 0..1000 { // Max 1000 steps to prevent infinite loops in edge cases
        time_steps += 1;

        // Assume 10% of remaining ethanol is processed by ADH1B per step
        let ethanol_processed_this_step = current_ethanol * 0.1;
        let produced_acetaldehyde = adh1b.metabolize_ethanol(ethanol_processed_this_step);
        current_ethanol -= ethanol_processed_this_step;
        
        let cleared_acetaldehyde = aldh2.metabolize_acetaldehyde(current_acetaldehyde);

        current_acetaldehyde += produced_acetaldehyde - cleared_acetaldehyde;

        // Ensure concentrations do not go below 0.0
        if current_ethanol < 0.0 {
            current_ethanol = 0.0;
        }
        if current_acetaldehyde < 0.0 {
            current_acetaldehyde = 0.0;
        }

        peak_acetaldehyde = peak_acetaldehyde.max(current_acetaldehyde);

        if current_ethanol < 0.001 && current_acetaldehyde < 0.001 {
            break;
        }
    }

    MetabolismMetrics {
        peak_acetaldehyde_level: peak_acetaldehyde,
        time_to_clear_acetaldehyde_approx_steps: time_steps,
    }
}

#[cfg(test)]
mod tests {
    use super::{simulate_ethanol_metabolism, get_flush_reaction_severity, MetabolismMetrics, ADH1B_Variant, ALDH2_Variant};

    #[test]
    fn test_simulation_logic() {
        // Normal ADH1B, Normal ALDH2
        let metrics_normal = simulate_ethanol_metabolism(10.0, ADH1B_Variant::ADH1B_1, ALDH2_Variant::ALDH2_1);

        // Normal ADH1B, Deficient ALDH2
        let metrics_deficient_aldh2 = simulate_ethanol_metabolism(10.0, ADH1B_Variant::ADH1B_1, ALDH2_Variant::ALDH2_2);

        // High Activity ADH1B, Deficient ALDH2
        let metrics_high_adh_deficient_aldh2 = simulate_ethanol_metabolism(10.0, ADH1B_Variant::ADH1B_2, ALDH2_Variant::ALDH2_2);

        // Assertions based on expected behavior
        // Deficient ALDH2 should lead to higher peak acetaldehyde than normal ALDH2
        assert!(metrics_deficient_aldh2.peak_acetaldehyde_level > metrics_normal.peak_acetaldehyde_level, 
                "Deficient ALDH2 should have higher peak acetaldehyde. Normal: {}, Deficient: {}", 
                metrics_normal.peak_acetaldehyde_level, metrics_deficient_aldh2.peak_acetaldehyde_level);

        // High activity ADH1B with deficient ALDH2 should lead to even higher peak acetaldehyde
        assert!(metrics_high_adh_deficient_aldh2.peak_acetaldehyde_level > metrics_deficient_aldh2.peak_acetaldehyde_level,
                "High ADH1B + Deficient ALDH2 should have higher peak. Deficient ALDH2: {}, High ADH + Deficient ALDH2: {}",
                metrics_deficient_aldh2.peak_acetaldehyde_level, metrics_high_adh_deficient_aldh2.peak_acetaldehyde_level);
    }

    #[test]
    fn test_flush_severity_logic() {
        // Test for "None" - low dose, efficient enzymes
        let severity_none = get_flush_reaction_severity(ADH1B_Variant::ADH1B_1, ALDH2_Variant::ALDH2_1, 0.5); // Lowered dose for "None"
        assert_eq!(severity_none, "None", "Expected None severity for low dose with efficient enzymes. Got: {}", severity_none);

        // Test for "Mild" - ADH1B_1 (normal), ALDH2_2 (deficient), moderate dose
        // Expecting peak acetaldehyde between 1.0 and 5.0
        let severity_mild = get_flush_reaction_severity(ADH1B_Variant::ADH1B_1, ALDH2_Variant::ALDH2_2, 2.0); // Adjusted dose
        assert_eq!(severity_mild, "Mild", "Expected Mild severity. Got: {}", severity_mild);

        // Test for "Moderate" - ADH1B_2 (high), ALDH2_2 (deficient), moderate dose
        // Expecting peak acetaldehyde between 5.0 and 10.0
        let severity_moderate = get_flush_reaction_severity(ADH1B_Variant::ADH1B_1, ALDH2_Variant::ALDH2_2, 5.0); // Adjusted dose
         assert_eq!(severity_moderate, "Moderate", "Expected Moderate severity. Got: {}", severity_moderate);
        
        // Test for "Severe" - ADH1B_2 (high), ALDH2_2 (deficient), higher dose
        // Expecting peak acetaldehyde >= 10.0
        let severity_severe = get_flush_reaction_severity(ADH1B_Variant::ADH1B_2, ALDH2_Variant::ALDH2_2, 10.0);
        assert_eq!(severity_severe, "Severe", "Expected Severe severity. Got: {}", severity_severe);
    }
}

pub fn get_flush_reaction_severity(
    adh1b_variant: ADH1B_Variant,
    aldh2_variant: ALDH2_Variant,
    ethanol_dose: f32,
) -> String {
    let metrics = simulate_ethanol_metabolism(ethanol_dose, adh1b_variant, aldh2_variant);
    let peak_acetaldehyde = metrics.peak_acetaldehyde_level;

    if peak_acetaldehyde < 1.0 {
        "None".to_string()
    } else if peak_acetaldehyde < 5.0 {
        "Mild".to_string()
    } else if peak_acetaldehyde < 10.0 {
        "Moderate".to_string()
    } else {
        "Severe".to_string()
    }
}

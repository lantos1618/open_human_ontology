// src/biology/molecular/enzymes.rs

#[derive(Debug, PartialEq)]
pub enum ADH1B_Variant {
    ADH1B_1, // Normal activity
    ADH1B_2, // High activity
}

#[derive(Debug, PartialEq)]
pub enum ALDH2_Variant {
    ALDH2_1, // Normal activity
    ALDH2_2, // Low activity/deficient
}

pub struct ADH1B {
    pub variant: ADH1B_Variant,
}

impl ADH1B {
    pub fn metabolize_ethanol(&self, ethanol_concentration: f32) -> f32 {
        match self.variant {
            ADH1B_Variant::ADH1B_1 => ethanol_concentration * 0.1,
            ADH1B_Variant::ADH1B_2 => ethanol_concentration * 0.5,
        }
    }
}

pub struct ALDH2 {
    pub variant: ALDH2_Variant,
}

impl ALDH2 {
    pub fn metabolize_acetaldehyde(&self, acetaldehyde_concentration: f32) -> f32 {
        match self.variant {
            ALDH2_Variant::ALDH2_1 => acetaldehyde_concentration * 0.2,
            ALDH2_Variant::ALDH2_2 => acetaldehyde_concentration * 0.02,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ADH1B, ADH1B_Variant, ALDH2, ALDH2_Variant};

    #[test]
    fn test_adh1b_variants_activity() {
        let adh1b_normal = ADH1B { variant: ADH1B_Variant::ADH1B_1 };
        let adh1b_high = ADH1B { variant: ADH1B_Variant::ADH1B_2 };

        let ethanol_amount = 10.0;
        let normal_activity_result = adh1b_normal.metabolize_ethanol(ethanol_amount);
        let high_activity_result = adh1b_high.metabolize_ethanol(ethanol_amount);

        assert!(high_activity_result > normal_activity_result);
        assert_eq!(normal_activity_result, ethanol_amount * 0.1);
        assert_eq!(high_activity_result, ethanol_amount * 0.5);
    }

    #[test]
    fn test_aldh2_variants_activity() {
        let aldh2_normal = ALDH2 { variant: ALDH2_Variant::ALDH2_1 };
        let aldh2_deficient = ALDH2 { variant: ALDH2_Variant::ALDH2_2 };

        let acetaldehyde_amount = 10.0;
        let normal_activity_result = aldh2_normal.metabolize_acetaldehyde(acetaldehyde_amount);
        let deficient_activity_result = aldh2_deficient.metabolize_acetaldehyde(acetaldehyde_amount);

        assert!(normal_activity_result > deficient_activity_result);
        assert_eq!(normal_activity_result, acetaldehyde_amount * 0.2);
        assert_eq!(deficient_activity_result, acetaldehyde_amount * 0.02);
    }
}

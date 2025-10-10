use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenstrualCycle {
    pub cycle_length_days: f64,
    pub current_day: f64,
    pub phase: CyclePhase,
    pub hormone_levels: HormoneLevels,
    pub symptoms: Vec<CycleSymptom>,
    pub fertility_window: FertilityWindow,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CyclePhase {
    Menstrual,
    Follicular,
    Ovulatory,
    Luteal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormoneLevels {
    pub estradiol_pg_ml: f64,
    pub progesterone_ng_ml: f64,
    pub lh_miu_ml: f64,
    pub fsh_miu_ml: f64,
    pub testosterone_ng_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CycleSymptom {
    Cramping,
    Bloating,
    BreastTenderness,
    Headache,
    MoodSwings,
    Fatigue,
    Acne,
    BackPain,
    FoodCravings,
    Spotting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FertilityWindow {
    pub is_fertile: bool,
    pub days_until_ovulation: f64,
    pub fertility_probability: f64,
    pub ovulation_predicted_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OvulationTracking {
    pub basal_body_temperature_c: Vec<f64>,
    pub cervical_mucus_quality: CervicalMucusQuality,
    pub ovulation_test_positive: bool,
    pub mittelschmerz_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CervicalMucusQuality {
    Dry,
    Sticky,
    Creamy,
    Watery,
    EggWhite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FertilityProfile {
    pub age_years: f64,
    pub ovarian_reserve: OvarianReserve,
    pub cycle_regularity: CycleRegularity,
    pub fertility_score: f64,
    pub conception_probability_per_cycle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OvarianReserve {
    pub amh_ng_ml: f64,
    pub antral_follicle_count: usize,
    pub fsh_day3_miu_ml: f64,
    pub reserve_status: ReserveStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ReserveStatus {
    Excellent,
    Good,
    Fair,
    Diminished,
    VeryLow,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CycleRegularity {
    Regular,
    SlightlyIrregular,
    Irregular,
    Anovulatory,
}

impl MenstrualCycle {
    pub fn new(cycle_length_days: f64) -> Self {
        Self {
            cycle_length_days,
            current_day: 1.0,
            phase: CyclePhase::Menstrual,
            hormone_levels: HormoneLevels::menstrual_phase(),
            symptoms: vec![],
            fertility_window: FertilityWindow::new(cycle_length_days, 1.0),
        }
    }

    pub fn advance_day(&mut self) {
        self.current_day += 1.0;
        if self.current_day > self.cycle_length_days {
            self.current_day = 1.0;
        }

        self.update_phase();
        self.update_hormones();
        self.update_fertility_window();
    }

    fn update_phase(&mut self) {
        self.phase = match self.current_day {
            d if d <= 5.0 => CyclePhase::Menstrual,
            d if d <= 13.0 => CyclePhase::Follicular,
            d if d <= 16.0 => CyclePhase::Ovulatory,
            _ => CyclePhase::Luteal,
        };
    }

    fn update_hormones(&mut self) {
        self.hormone_levels = match self.phase {
            CyclePhase::Menstrual => HormoneLevels::menstrual_phase(),
            CyclePhase::Follicular => HormoneLevels::follicular_phase(self.current_day),
            CyclePhase::Ovulatory => HormoneLevels::ovulatory_phase(),
            CyclePhase::Luteal => HormoneLevels::luteal_phase(self.current_day),
        };
    }

    fn update_fertility_window(&mut self) {
        self.fertility_window = FertilityWindow::new(self.cycle_length_days, self.current_day);
    }

    pub fn is_fertile(&self) -> bool {
        self.fertility_window.is_fertile
    }

    pub fn ovulation_day(&self) -> f64 {
        self.cycle_length_days - 14.0
    }

    pub fn expected_symptoms(&self) -> Vec<CycleSymptom> {
        match self.phase {
            CyclePhase::Menstrual => vec![
                CycleSymptom::Cramping,
                CycleSymptom::Bloating,
                CycleSymptom::Fatigue,
                CycleSymptom::BackPain,
            ],
            CyclePhase::Follicular => vec![],
            CyclePhase::Ovulatory => vec![CycleSymptom::BreastTenderness],
            CyclePhase::Luteal => vec![
                CycleSymptom::Bloating,
                CycleSymptom::BreastTenderness,
                CycleSymptom::MoodSwings,
                CycleSymptom::Acne,
                CycleSymptom::FoodCravings,
            ],
        }
    }
}

impl HormoneLevels {
    fn menstrual_phase() -> Self {
        Self {
            estradiol_pg_ml: 35.0,
            progesterone_ng_ml: 0.3,
            lh_miu_ml: 5.0,
            fsh_miu_ml: 6.0,
            testosterone_ng_dl: 25.0,
        }
    }

    fn follicular_phase(day: f64) -> Self {
        let estradiol = 35.0 + (day - 5.0) * 20.0;
        Self {
            estradiol_pg_ml: estradiol,
            progesterone_ng_ml: 0.5,
            lh_miu_ml: 5.0 + (day - 5.0) * 1.5,
            fsh_miu_ml: 6.0,
            testosterone_ng_dl: 30.0,
        }
    }

    fn ovulatory_phase() -> Self {
        Self {
            estradiol_pg_ml: 250.0,
            progesterone_ng_ml: 1.5,
            lh_miu_ml: 30.0,
            fsh_miu_ml: 15.0,
            testosterone_ng_dl: 45.0,
        }
    }

    fn luteal_phase(day: f64) -> Self {
        let days_since_ovulation = day - 14.0;
        let progesterone = 1.5 + days_since_ovulation * 1.5;
        Self {
            estradiol_pg_ml: 180.0,
            progesterone_ng_ml: progesterone.min(20.0),
            lh_miu_ml: 5.0,
            fsh_miu_ml: 5.0,
            testosterone_ng_dl: 35.0,
        }
    }
}

impl FertilityWindow {
    fn new(cycle_length: f64, current_day: f64) -> Self {
        let ovulation_day = cycle_length - 14.0;
        let days_until_ovulation = ovulation_day - current_day;

        let fertile_start = ovulation_day - 5.0;
        let fertile_end = ovulation_day + 1.0;

        let is_fertile = current_day >= fertile_start && current_day <= fertile_end;

        let fertility_probability = if is_fertile {
            let days_from_ovulation = (current_day - ovulation_day).abs();
            match days_from_ovulation {
                d if d <= 0.5 => 0.33,
                d if d <= 1.0 => 0.27,
                d if d <= 2.0 => 0.19,
                d if d <= 3.0 => 0.12,
                d if d <= 4.0 => 0.08,
                _ => 0.04,
            }
        } else {
            0.0
        };

        Self {
            is_fertile,
            days_until_ovulation,
            fertility_probability,
            ovulation_predicted_day: ovulation_day,
        }
    }
}

impl FertilityProfile {
    pub fn new(age_years: f64) -> Self {
        let amh = Self::estimate_amh_for_age(age_years);
        let conception_prob = Self::conception_probability_for_age(age_years);

        Self {
            age_years,
            ovarian_reserve: OvarianReserve {
                amh_ng_ml: amh,
                antral_follicle_count: Self::estimate_afc_for_age(age_years),
                fsh_day3_miu_ml: Self::estimate_fsh_for_age(age_years),
                reserve_status: Self::classify_reserve(amh),
            },
            cycle_regularity: CycleRegularity::Regular,
            fertility_score: conception_prob,
            conception_probability_per_cycle: conception_prob,
        }
    }

    fn estimate_amh_for_age(age: f64) -> f64 {
        match age {
            a if a < 25.0 => 4.0,
            a if a < 30.0 => 3.5,
            a if a < 35.0 => 2.5,
            a if a < 40.0 => 1.5,
            a if a < 45.0 => 0.5,
            _ => 0.1,
        }
    }

    fn estimate_afc_for_age(age: f64) -> usize {
        match age {
            a if a < 25.0 => 25,
            a if a < 30.0 => 20,
            a if a < 35.0 => 15,
            a if a < 40.0 => 10,
            a if a < 45.0 => 5,
            _ => 2,
        }
    }

    fn estimate_fsh_for_age(age: f64) -> f64 {
        match age {
            a if a < 35.0 => 6.0,
            a if a < 40.0 => 8.0,
            a if a < 45.0 => 12.0,
            _ => 20.0,
        }
    }

    fn classify_reserve(amh: f64) -> ReserveStatus {
        match amh {
            a if a >= 3.0 => ReserveStatus::Excellent,
            a if a >= 1.5 => ReserveStatus::Good,
            a if a >= 1.0 => ReserveStatus::Fair,
            a if a >= 0.5 => ReserveStatus::Diminished,
            _ => ReserveStatus::VeryLow,
        }
    }

    fn conception_probability_for_age(age: f64) -> f64 {
        match age {
            a if a < 25.0 => 0.25,
            a if a < 30.0 => 0.23,
            a if a < 35.0 => 0.20,
            a if a < 40.0 => 0.12,
            a if a < 45.0 => 0.05,
            _ => 0.01,
        }
    }

    pub fn time_to_conception_months_50th_percentile(&self) -> f64 {
        if self.conception_probability_per_cycle == 0.0 {
            return f64::INFINITY;
        }

        (0.693 / self.conception_probability_per_cycle).ceil()
    }

    pub fn recommend_fertility_preservation(&self) -> bool {
        self.age_years >= 35.0
            || matches!(
                self.ovarian_reserve.reserve_status,
                ReserveStatus::Diminished | ReserveStatus::VeryLow
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menstrual_cycle_creation() {
        let cycle = MenstrualCycle::new(28.0);
        assert_eq!(cycle.current_day, 1.0);
        assert_eq!(cycle.phase, CyclePhase::Menstrual);
    }

    #[test]
    fn test_cycle_advancement() {
        let mut cycle = MenstrualCycle::new(28.0);
        cycle.advance_day();
        assert_eq!(cycle.current_day, 2.0);
    }

    #[test]
    fn test_ovulation_day() {
        let cycle = MenstrualCycle::new(28.0);
        assert_eq!(cycle.ovulation_day(), 14.0);
    }

    #[test]
    fn test_fertility_window() {
        let mut cycle = MenstrualCycle::new(28.0);
        for _ in 0..13 {
            cycle.advance_day();
        }
        assert!(cycle.is_fertile());
    }

    #[test]
    fn test_phase_progression() {
        let mut cycle = MenstrualCycle::new(28.0);

        assert_eq!(cycle.phase, CyclePhase::Menstrual);

        for _ in 0..7 {
            cycle.advance_day();
        }
        assert_eq!(cycle.phase, CyclePhase::Follicular);

        for _ in 0..7 {
            cycle.advance_day();
        }
        assert_eq!(cycle.phase, CyclePhase::Ovulatory);

        for _ in 0..5 {
            cycle.advance_day();
        }
        assert_eq!(cycle.phase, CyclePhase::Luteal);
    }

    #[test]
    fn test_fertility_profile_young() {
        let profile = FertilityProfile::new(25.0);
        assert!(profile.conception_probability_per_cycle > 0.2);
        assert_eq!(
            profile.ovarian_reserve.reserve_status,
            ReserveStatus::Excellent
        );
    }

    #[test]
    fn test_fertility_profile_older() {
        let profile = FertilityProfile::new(40.0);
        assert!(profile.conception_probability_per_cycle < 0.15);
        assert!(profile.recommend_fertility_preservation());
    }

    #[test]
    fn test_ovarian_reserve() {
        let profile = FertilityProfile::new(38.0);
        assert!(profile.ovarian_reserve.amh_ng_ml > 0.0);
        assert!(profile.ovarian_reserve.antral_follicle_count > 0);
    }

    #[test]
    fn test_time_to_conception() {
        let profile = FertilityProfile::new(28.0);
        let months = profile.time_to_conception_months_50th_percentile();
        assert!(months > 0.0 && months < 12.0);
    }

    #[test]
    fn test_hormone_levels_vary_by_phase() {
        let menstrual = HormoneLevels::menstrual_phase();
        let ovulatory = HormoneLevels::ovulatory_phase();

        assert!(ovulatory.lh_miu_ml > menstrual.lh_miu_ml);
        assert!(ovulatory.estradiol_pg_ml > menstrual.estradiol_pg_ml);
    }
}

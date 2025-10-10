use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SleepGene {
    PER1,
    PER2,
    PER3,
    CLOCK,
    BMAL1,
    CRY1,
    CRY2,
    NPAS2,
    DEC1,
    DEC2,
    HCRT,
    HCRTR2,
    ADORA2A,
    COMT,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SleepDisorder {
    Insomnia,
    NarcolepsyType1,
    NarcolepsyType2,
    ObstructiveSleepApnea,
    RestlessLegsSyndrome,
    DelayedSleepPhase,
    AdvancedSleepPhase,
    NonNine24Hour,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Chronotype {
    ExtremeMorningLark,
    MorningLark,
    Intermediate,
    EveningOwl,
    ExtremeEveningOwl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepGeneticVariant {
    pub gene: SleepGene,
    pub rs_id: String,
    pub variant: String,
    pub effect: SleepEffect,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SleepEffect {
    ShorterSleepDuration,
    LongerSleepDuration,
    EarlierChronotype,
    LaterChronotype,
    IncreasedInsomnia,
    NarcolepsyRisk,
    CaffeinesSensitivity,
    DeepSleepIncrease,
    RapidEyeMovementChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepProfile {
    pub chronotype: Chronotype,
    pub optimal_sleep_duration_hours: f64,
    pub sleep_efficiency: f64,
    pub deep_sleep_tendency: f64,
    pub rem_sleep_tendency: f64,
    pub caffeine_metabolism_rate: f64,
    pub insomnia_risk_score: f64,
    pub narcolepsy_risk: f64,
    pub circadian_period_hours: f64,
    pub variants: Vec<SleepGeneticVariant>,
}

impl SleepProfile {
    pub fn new() -> Self {
        Self {
            chronotype: Chronotype::Intermediate,
            optimal_sleep_duration_hours: 8.0,
            sleep_efficiency: 0.85,
            deep_sleep_tendency: 1.0,
            rem_sleep_tendency: 1.0,
            caffeine_metabolism_rate: 1.0,
            insomnia_risk_score: 1.0,
            narcolepsy_risk: 1.0,
            circadian_period_hours: 24.0,
            variants: Vec::new(),
        }
    }

    pub fn add_variant(&mut self, variant: SleepGeneticVariant) {
        match variant.effect {
            SleepEffect::ShorterSleepDuration => {
                self.optimal_sleep_duration_hours -= 0.5;
            }
            SleepEffect::LongerSleepDuration => {
                self.optimal_sleep_duration_hours += 0.5;
            }
            SleepEffect::EarlierChronotype => {
                self.circadian_period_hours -= 0.2;
                self.chronotype = self.shift_chronotype_earlier();
            }
            SleepEffect::LaterChronotype => {
                self.circadian_period_hours += 0.2;
                self.chronotype = self.shift_chronotype_later();
            }
            SleepEffect::IncreasedInsomnia => {
                self.insomnia_risk_score *= 1.5;
                self.sleep_efficiency *= 0.9;
            }
            SleepEffect::NarcolepsyRisk => {
                self.narcolepsy_risk *= 3.0;
            }
            SleepEffect::CaffeinesSensitivity => {
                self.caffeine_metabolism_rate *= 0.6;
            }
            SleepEffect::DeepSleepIncrease => {
                self.deep_sleep_tendency *= 1.2;
            }
            SleepEffect::RapidEyeMovementChange => {
                self.rem_sleep_tendency *= 1.15;
            }
        }

        match variant.gene {
            SleepGene::PER3 => {
                if variant.variant.contains("5/5") {
                    self.deep_sleep_tendency *= 1.3;
                }
            }
            SleepGene::ADORA2A => {
                self.caffeine_metabolism_rate *= 0.7;
            }
            SleepGene::HCRTR2 => {
                self.narcolepsy_risk *= 2.5;
            }
            _ => {}
        }

        self.variants.push(variant);
    }

    fn shift_chronotype_earlier(&self) -> Chronotype {
        match self.chronotype {
            Chronotype::ExtremeEveningOwl => Chronotype::EveningOwl,
            Chronotype::EveningOwl => Chronotype::Intermediate,
            Chronotype::Intermediate => Chronotype::MorningLark,
            Chronotype::MorningLark => Chronotype::ExtremeMorningLark,
            Chronotype::ExtremeMorningLark => Chronotype::ExtremeMorningLark,
        }
    }

    fn shift_chronotype_later(&self) -> Chronotype {
        match self.chronotype {
            Chronotype::ExtremeMorningLark => Chronotype::MorningLark,
            Chronotype::MorningLark => Chronotype::Intermediate,
            Chronotype::Intermediate => Chronotype::EveningOwl,
            Chronotype::EveningOwl => Chronotype::ExtremeEveningOwl,
            Chronotype::ExtremeEveningOwl => Chronotype::ExtremeEveningOwl,
        }
    }

    pub fn optimal_bedtime_range(&self) -> (u8, u8) {
        match self.chronotype {
            Chronotype::ExtremeMorningLark => (20, 21),
            Chronotype::MorningLark => (21, 22),
            Chronotype::Intermediate => (22, 23),
            Chronotype::EveningOwl => (23, 24),
            Chronotype::ExtremeEveningOwl => (0, 1),
        }
    }

    pub fn optimal_wake_time(&self) -> u8 {
        let (bedtime_start, _) = self.optimal_bedtime_range();
        ((bedtime_start as f64 + self.optimal_sleep_duration_hours) % 24.0) as u8
    }

    pub fn caffeine_cutoff_time(&self) -> u8 {
        let (bedtime, _) = self.optimal_bedtime_range();
        let hours_before = 6.0 / self.caffeine_metabolism_rate;
        ((bedtime as f64 - hours_before + 24.0) % 24.0) as u8
    }

    pub fn sleep_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        recommendations.push(format!(
            "Optimal bedtime: {:02}:00-{:02}:00",
            self.optimal_bedtime_range().0,
            self.optimal_bedtime_range().1
        ));

        recommendations.push(format!(
            "Aim for {} hours of sleep",
            self.optimal_sleep_duration_hours
        ));

        if self.caffeine_metabolism_rate < 0.8 {
            recommendations.push(format!(
                "Avoid caffeine after {:02}:00 (slow metabolizer)",
                self.caffeine_cutoff_time()
            ));
        }

        if self.insomnia_risk_score > 1.5 {
            recommendations
                .push("High insomnia risk: practice sleep hygiene, consider CBT-I".to_string());
        }

        if self.chronotype == Chronotype::ExtremeEveningOwl
            || self.chronotype == Chronotype::EveningOwl
        {
            recommendations.push(
                "Evening chronotype: get bright light in morning, dim lights in evening"
                    .to_string(),
            );
        }

        if self.chronotype == Chronotype::ExtremeMorningLark
            || self.chronotype == Chronotype::MorningLark
        {
            recommendations.push("Morning chronotype: avoid bright light in evening".to_string());
        }

        if self.deep_sleep_tendency > 1.2 {
            recommendations
                .push("Good deep sleep genetics: maintain with regular exercise".to_string());
        }

        recommendations
    }

    pub fn assess_disorder_risk(&self, disorder: SleepDisorder) -> f64 {
        match disorder {
            SleepDisorder::Insomnia => self.insomnia_risk_score,
            SleepDisorder::NarcolepsyType1 | SleepDisorder::NarcolepsyType2 => self.narcolepsy_risk,
            SleepDisorder::DelayedSleepPhase => {
                if matches!(
                    self.chronotype,
                    Chronotype::EveningOwl | Chronotype::ExtremeEveningOwl
                ) {
                    2.5
                } else {
                    1.0
                }
            }
            SleepDisorder::AdvancedSleepPhase => {
                if matches!(
                    self.chronotype,
                    Chronotype::MorningLark | Chronotype::ExtremeMorningLark
                ) {
                    2.0
                } else {
                    1.0
                }
            }
            _ => 1.0,
        }
    }
}

impl Default for SleepProfile {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_sleep_variants() -> Vec<SleepGeneticVariant> {
    vec![
        SleepGeneticVariant {
            gene: SleepGene::PER3,
            rs_id: "rs57875989".to_string(),
            variant: "5-repeat/5-repeat".to_string(),
            effect: SleepEffect::EarlierChronotype,
            description: "PER3 long variant: morning chronotype, more deep sleep, cognitive decline with sleep deprivation".to_string(),
        },
        SleepGeneticVariant {
            gene: SleepGene::PER3,
            rs_id: "rs57875989".to_string(),
            variant: "4-repeat/4-repeat".to_string(),
            effect: SleepEffect::LaterChronotype,
            description: "PER3 short variant: evening chronotype, resilient to sleep deprivation".to_string(),
        },
        SleepGeneticVariant {
            gene: SleepGene::CLOCK,
            rs_id: "rs1801260".to_string(),
            variant: "T allele".to_string(),
            effect: SleepEffect::LaterChronotype,
            description: "CLOCK variant associated with evening preference and delayed sleep phase".to_string(),
        },
        SleepGeneticVariant {
            gene: SleepGene::ADORA2A,
            rs_id: "rs5751876".to_string(),
            variant: "T allele".to_string(),
            effect: SleepEffect::CaffeinesSensitivity,
            description: "Adenosine receptor variant: increased caffeine sensitivity, deeper sleep".to_string(),
        },
        SleepGeneticVariant {
            gene: SleepGene::DEC2,
            rs_id: "rs121912617".to_string(),
            variant: "P385R".to_string(),
            effect: SleepEffect::ShorterSleepDuration,
            description: "Rare DEC2 variant: natural short sleeper (~6 hours), no adverse effects".to_string(),
        },
        SleepGeneticVariant {
            gene: SleepGene::HCRTR2,
            rs_id: "rs2653349".to_string(),
            variant: "A allele".to_string(),
            effect: SleepEffect::NarcolepsyRisk,
            description: "Hypocretin receptor 2 variant: increased narcolepsy risk".to_string(),
        },
        SleepGeneticVariant {
            gene: SleepGene::PER1,
            rs_id: "rs2735611".to_string(),
            variant: "G allele".to_string(),
            effect: SleepEffect::EarlierChronotype,
            description: "PER1 variant associated with morning preference".to_string(),
        },
        SleepGeneticVariant {
            gene: SleepGene::CRY1,
            rs_id: "rs2287161".to_string(),
            variant: "C allele".to_string(),
            effect: SleepEffect::LaterChronotype,
            description: "CRY1 variant: delayed sleep phase, longer circadian period".to_string(),
        },
        SleepGeneticVariant {
            gene: SleepGene::COMT,
            rs_id: "rs4680".to_string(),
            variant: "Met/Met".to_string(),
            effect: SleepEffect::IncreasedInsomnia,
            description: "COMT Met/Met: more vulnerable to sleep deprivation effects".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_profile_creation() {
        let profile = SleepProfile::new();
        assert_eq!(profile.chronotype, Chronotype::Intermediate);
        assert_eq!(profile.optimal_sleep_duration_hours, 8.0);
    }

    #[test]
    fn test_per3_long_variant() {
        let mut profile = SleepProfile::new();

        profile.add_variant(SleepGeneticVariant {
            gene: SleepGene::PER3,
            rs_id: "rs57875989".to_string(),
            variant: "5/5".to_string(),
            effect: SleepEffect::EarlierChronotype,
            description: "Test".to_string(),
        });

        assert!(matches!(
            profile.chronotype,
            Chronotype::MorningLark | Chronotype::ExtremeMorningLark
        ));
        assert!(profile.deep_sleep_tendency > 1.0);
    }

    #[test]
    fn test_dec2_short_sleeper() {
        let mut profile = SleepProfile::new();

        profile.add_variant(SleepGeneticVariant {
            gene: SleepGene::DEC2,
            rs_id: "rs121912617".to_string(),
            variant: "P385R".to_string(),
            effect: SleepEffect::ShorterSleepDuration,
            description: "Test".to_string(),
        });

        assert!(profile.optimal_sleep_duration_hours < 8.0);
    }

    #[test]
    fn test_caffeine_sensitivity() {
        let mut profile = SleepProfile::new();

        profile.add_variant(SleepGeneticVariant {
            gene: SleepGene::ADORA2A,
            rs_id: "rs5751876".to_string(),
            variant: "T allele".to_string(),
            effect: SleepEffect::CaffeinesSensitivity,
            description: "Test".to_string(),
        });

        assert!(profile.caffeine_metabolism_rate < 0.8);
    }

    #[test]
    fn test_optimal_bedtime_ranges() {
        let mut profile = SleepProfile::new();
        profile.chronotype = Chronotype::MorningLark;

        let (start, end) = profile.optimal_bedtime_range();
        assert_eq!(start, 21);
        assert_eq!(end, 22);
    }

    #[test]
    fn test_narcolepsy_risk() {
        let mut profile = SleepProfile::new();

        profile.add_variant(SleepGeneticVariant {
            gene: SleepGene::HCRTR2,
            rs_id: "rs2653349".to_string(),
            variant: "A allele".to_string(),
            effect: SleepEffect::NarcolepsyRisk,
            description: "Test".to_string(),
        });

        assert!(profile.narcolepsy_risk > 2.0);
        let risk = profile.assess_disorder_risk(SleepDisorder::NarcolepsyType1);
        assert!(risk > 2.0);
    }

    #[test]
    fn test_sleep_recommendations() {
        let mut profile = SleepProfile::new();
        profile.insomnia_risk_score = 2.0;
        profile.caffeine_metabolism_rate = 0.6;

        let recs = profile.sleep_recommendations();
        assert!(!recs.is_empty());
        assert!(recs.iter().any(|r| r.contains("caffeine")));
        assert!(recs.iter().any(|r| r.contains("insomnia")));
    }

    #[test]
    fn test_delayed_sleep_phase_risk() {
        let mut profile = SleepProfile::new();
        profile.chronotype = Chronotype::ExtremeEveningOwl;

        let risk = profile.assess_disorder_risk(SleepDisorder::DelayedSleepPhase);
        assert!(risk > 2.0);
    }

    #[test]
    fn test_chronotype_shifting() {
        let mut profile = SleepProfile::new();
        assert_eq!(profile.chronotype, Chronotype::Intermediate);

        profile.add_variant(SleepGeneticVariant {
            gene: SleepGene::CLOCK,
            rs_id: "rs1801260".to_string(),
            variant: "T allele".to_string(),
            effect: SleepEffect::LaterChronotype,
            description: "Test".to_string(),
        });

        assert_eq!(profile.chronotype, Chronotype::EveningOwl);
    }
}

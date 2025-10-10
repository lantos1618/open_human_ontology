use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ENTProfile {
    pub hearing: HearingAssessment,
    pub sinus: SinusHealth,
    pub throat: ThroatHealth,
    pub voice: VoiceAssessment,
    pub balance: VestibularFunction,
    pub conditions: Vec<ENTCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HearingAssessment {
    pub right_ear: Audiogram,
    pub left_ear: Audiogram,
    pub hearing_loss_type: Option<HearingLossType>,
    pub speech_discrimination: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audiogram {
    pub frequencies_hz: Vec<u32>,
    pub thresholds_db: Vec<f64>,
    pub air_conduction: Vec<f64>,
    pub bone_conduction: Vec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HearingLossType {
    Conductive,
    Sensorineural,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinusHealth {
    pub maxillary: SinusStatus,
    pub frontal: SinusStatus,
    pub ethmoid: SinusStatus,
    pub sphenoid: SinusStatus,
    pub chronic_sinusitis: bool,
    pub nasal_polyps: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SinusStatus {
    Clear,
    PartiallyOpacified,
    CompletelyOpacified,
    AirFluidLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroatHealth {
    pub tonsil_grade: TonsilGrade,
    pub pharyngeal_inflammation: InflammationLevel,
    pub vocal_cord_mobility: VocalCordMobility,
    pub swallowing_function: SwallowingFunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TonsilGrade {
    Grade0,
    Grade1,
    Grade2,
    Grade3,
    Grade4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InflammationLevel {
    None,
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VocalCordMobility {
    Normal,
    ReducedUnilateral,
    ReducedBilateral,
    ParalyzedUnilateral,
    ParalyzedBilateral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwallowingFunction {
    Normal,
    MildDysphasia,
    ModerateDysphasia,
    SevereDysphasia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceAssessment {
    pub quality: VoiceQuality,
    pub pitch_range: PitchRange,
    pub volume_range: VolumeRange,
    pub vocal_fatigue: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceQuality {
    Normal,
    Hoarse,
    Breathy,
    Strained,
    Tremulous,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PitchRange {
    pub lowest_hz: f64,
    pub highest_hz: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VolumeRange {
    pub softest_db: f64,
    pub loudest_db: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestibularFunction {
    pub balance_score: f64,
    pub vertigo_present: bool,
    pub nystagmus: NystagmusType,
    pub romberg_test: RombergResult,
    pub dix_hallpike: DixHallpikeResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NystagmusType {
    None,
    Horizontal,
    Vertical,
    Rotatory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RombergResult {
    Negative,
    Positive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DixHallpikeResult {
    Negative,
    PositiveRight,
    PositiveLeft,
    PositiveBilateral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ENTCondition {
    pub condition_type: ENTConditionType,
    pub severity: InflammationLevel,
    pub onset: ConditionOnset,
    pub affected_side: AffectedSide,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ENTConditionType {
    OtitisMedia,
    OtitisExterna,
    ChronicSinusitis,
    AcuteRhinosinusitis,
    NasalPolyps,
    DeviatedSeptum,
    Tonsillitis,
    Pharyngitis,
    Laryngitis,
    VocalCordNodules,
    VocalCordPolyps,
    BPPV,
    Meniere,
    AcousticNeuroma,
    Cholesteatoma,
    Epistaxis,
    Rhinitis,
    AlergicRhinitis,
    SleepApnea,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionOnset {
    Acute,
    Chronic,
    Recurrent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AffectedSide {
    Right,
    Left,
    Bilateral,
    Midline,
}

impl ENTProfile {
    pub fn new() -> Self {
        Self {
            hearing: HearingAssessment::normal(),
            sinus: SinusHealth::normal(),
            throat: ThroatHealth::normal(),
            voice: VoiceAssessment::normal(),
            balance: VestibularFunction::normal(),
            conditions: Vec::new(),
        }
    }

    pub fn add_condition(&mut self, condition: ENTCondition) {
        self.conditions.push(condition);
    }

    pub fn hearing_disability(&self) -> HearingDisability {
        let right_avg = self.hearing.right_ear.pure_tone_average();
        let left_avg = self.hearing.left_ear.pure_tone_average();
        let better_ear = right_avg.min(left_avg);

        match better_ear as u32 {
            0..=25 => HearingDisability::None,
            26..=40 => HearingDisability::Mild,
            41..=55 => HearingDisability::Moderate,
            56..=70 => HearingDisability::ModeratelySevere,
            71..=90 => HearingDisability::Severe,
            _ => HearingDisability::Profound,
        }
    }

    pub fn requires_ent_referral(&self) -> bool {
        for condition in &self.conditions {
            match condition.condition_type {
                ENTConditionType::AcousticNeuroma => return true,
                ENTConditionType::Cholesteatoma => return true,
                ENTConditionType::VocalCordPolyps => return true,
                _ if condition.severity == InflammationLevel::Severe => return true,
                _ => {}
            }
        }

        if self.hearing.speech_discrimination < 0.5 {
            return true;
        }

        if matches!(
            self.throat.vocal_cord_mobility,
            VocalCordMobility::ParalyzedUnilateral | VocalCordMobility::ParalyzedBilateral
        ) {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HearingDisability {
    None,
    Mild,
    Moderate,
    ModeratelySevere,
    Severe,
    Profound,
}

impl Default for ENTProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl HearingAssessment {
    pub fn normal() -> Self {
        Self {
            right_ear: Audiogram::normal(),
            left_ear: Audiogram::normal(),
            hearing_loss_type: None,
            speech_discrimination: 1.0,
        }
    }
}

impl Audiogram {
    pub fn normal() -> Self {
        let frequencies = vec![250, 500, 1000, 2000, 4000, 8000];
        let thresholds = vec![10.0, 10.0, 10.0, 10.0, 10.0, 10.0];

        Self {
            frequencies_hz: frequencies.clone(),
            thresholds_db: thresholds.clone(),
            air_conduction: thresholds.clone(),
            bone_conduction: thresholds,
        }
    }

    pub fn pure_tone_average(&self) -> f64 {
        let target_freqs = [500, 1000, 2000];
        let mut sum = 0.0;
        let mut count = 0;

        for (i, &freq) in self.frequencies_hz.iter().enumerate() {
            if target_freqs.contains(&freq) {
                sum += self.thresholds_db[i];
                count += 1;
            }
        }

        if count > 0 {
            sum / count as f64
        } else {
            0.0
        }
    }

    pub fn air_bone_gap(&self) -> f64 {
        let air_avg: f64 =
            self.air_conduction.iter().sum::<f64>() / self.air_conduction.len() as f64;
        let bone_avg: f64 =
            self.bone_conduction.iter().sum::<f64>() / self.bone_conduction.len() as f64;

        air_avg - bone_avg
    }
}

impl SinusHealth {
    pub fn normal() -> Self {
        Self {
            maxillary: SinusStatus::Clear,
            frontal: SinusStatus::Clear,
            ethmoid: SinusStatus::Clear,
            sphenoid: SinusStatus::Clear,
            chronic_sinusitis: false,
            nasal_polyps: false,
        }
    }
}

impl ThroatHealth {
    pub fn normal() -> Self {
        Self {
            tonsil_grade: TonsilGrade::Grade1,
            pharyngeal_inflammation: InflammationLevel::None,
            vocal_cord_mobility: VocalCordMobility::Normal,
            swallowing_function: SwallowingFunction::Normal,
        }
    }
}

impl VoiceAssessment {
    pub fn normal() -> Self {
        Self {
            quality: VoiceQuality::Normal,
            pitch_range: PitchRange {
                lowest_hz: 80.0,
                highest_hz: 1000.0,
            },
            volume_range: VolumeRange {
                softest_db: 40.0,
                loudest_db: 100.0,
            },
            vocal_fatigue: false,
        }
    }
}

impl VestibularFunction {
    pub fn normal() -> Self {
        Self {
            balance_score: 100.0,
            vertigo_present: false,
            nystagmus: NystagmusType::None,
            romberg_test: RombergResult::Negative,
            dix_hallpike: DixHallpikeResult::Negative,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ent_profile_creation() {
        let profile = ENTProfile::new();
        assert_eq!(profile.hearing.speech_discrimination, 1.0);
        assert!(!profile.balance.vertigo_present);
    }

    #[test]
    fn test_pure_tone_average() {
        let audiogram = Audiogram::normal();
        let pta = audiogram.pure_tone_average();
        assert_eq!(pta, 10.0);
    }

    #[test]
    fn test_hearing_disability_classification() {
        let mut profile = ENTProfile::new();
        profile.hearing.right_ear.thresholds_db = vec![50.0, 50.0, 50.0, 50.0, 50.0, 50.0];
        profile.hearing.left_ear.thresholds_db = vec![50.0, 50.0, 50.0, 50.0, 50.0, 50.0];

        let disability = profile.hearing_disability();
        assert_eq!(disability, HearingDisability::Moderate);
    }

    #[test]
    fn test_air_bone_gap() {
        let mut audiogram = Audiogram::normal();
        audiogram.air_conduction = vec![40.0, 40.0, 40.0, 40.0, 40.0, 40.0];
        audiogram.bone_conduction = vec![10.0, 10.0, 10.0, 10.0, 10.0, 10.0];

        let gap = audiogram.air_bone_gap();
        assert_eq!(gap, 30.0);
    }

    #[test]
    fn test_ent_referral_criteria() {
        let mut profile = ENTProfile::new();
        profile.add_condition(ENTCondition {
            condition_type: ENTConditionType::AcousticNeuroma,
            severity: InflammationLevel::Moderate,
            onset: ConditionOnset::Chronic,
            affected_side: AffectedSide::Right,
        });

        assert!(profile.requires_ent_referral());
    }
}

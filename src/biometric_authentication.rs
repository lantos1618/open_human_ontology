use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricProfile {
    pub fingerprint: Option<FingerprintData>,
    pub facial_recognition: Option<FacialRecognitionData>,
    pub iris_scan: Option<IrisData>,
    pub voice_print: Option<VoicePrintData>,
    pub gait_analysis: Option<GaitData>,
    pub retinal_scan: Option<RetinalData>,
    pub palm_print: Option<PalmPrintData>,
    pub vein_pattern: Option<VeinPatternData>,
    pub dna_profile: Option<DNABiometricData>,
    pub ear_shape: Option<EarShapeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintData {
    pub patterns: Vec<FingerprintPattern>,
    pub minutiae_count: usize,
    pub ridge_density: f64,
    pub pattern_type: FingerPrintPatternType,
    pub quality_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FingerPrintPatternType {
    Loop,
    Whorl,
    Arch,
    Composite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintPattern {
    pub x: f64,
    pub y: f64,
    pub pattern_type: MinutiaeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinutiaeType {
    RidgeEnding,
    Bifurcation,
    Lake,
    Spur,
    Crossover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacialRecognitionData {
    pub facial_landmarks: Vec<FacialLandmark>,
    pub interocular_distance_mm: f64,
    pub nose_bridge_length_mm: f64,
    pub jawline_angle_degrees: f64,
    pub face_shape: FaceShape,
    pub skin_texture_features: Vec<f64>,
    pub facial_geometry_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacialLandmark {
    pub landmark_type: LandmarkType,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LandmarkType {
    LeftEyeCenter,
    RightEyeCenter,
    NoseTip,
    LeftMouthCorner,
    RightMouthCorner,
    ChinTip,
    LeftEyebrowOuter,
    RightEyebrowOuter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FaceShape {
    Oval,
    Round,
    Square,
    Heart,
    Diamond,
    Oblong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrisData {
    pub iris_texture_features: Vec<f64>,
    pub iris_color: IrisColor,
    pub crypts_count: usize,
    pub furrows_count: usize,
    pub collarette_features: Vec<f64>,
    pub iris_diameter_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrisColor {
    Blue,
    Green,
    Brown,
    Hazel,
    Gray,
    Amber,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoicePrintData {
    pub fundamental_frequency_hz: f64,
    pub formant_frequencies: Vec<f64>,
    pub vocal_tract_length_cm: f64,
    pub pitch_range: (f64, f64),
    pub speaking_rate_wpm: f64,
    pub voice_quality: VoiceQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoiceQuality {
    Clear,
    Breathy,
    Creaky,
    Tense,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaitData {
    pub stride_length_cm: f64,
    pub step_width_cm: f64,
    pub cadence_steps_per_min: f64,
    pub gait_cycle_duration_s: f64,
    pub asymmetry_index: f64,
    pub biomechanical_features: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetinalData {
    pub blood_vessel_pattern: Vec<VesselPoint>,
    pub optic_disc_position: (f64, f64),
    pub fovea_position: (f64, f64),
    pub vessel_bifurcations: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VesselPoint {
    pub x: f64,
    pub y: f64,
    pub vessel_diameter_um: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalmPrintData {
    pub principal_lines: Vec<PalmLine>,
    pub ridge_patterns: Vec<RidgePattern>,
    pub palm_area_cm2: f64,
    pub texture_features: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalmLine {
    pub line_type: PalmLineType,
    pub length_mm: f64,
    pub curvature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PalmLineType {
    Heart,
    Head,
    Life,
    Fate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RidgePattern {
    pub x: f64,
    pub y: f64,
    pub orientation: f64,
    pub frequency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VeinPatternData {
    pub vein_map: Vec<VeinSegment>,
    pub bifurcation_points: Vec<(f64, f64)>,
    pub vein_density: f64,
    pub scan_location: VeinScanLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VeinSegment {
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub thickness_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VeinScanLocation {
    Finger,
    Palm,
    Wrist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNABiometricData {
    pub str_markers: HashMap<String, (String, String)>,
    pub snp_profile: Vec<String>,
    pub mitochondrial_haplogroup: String,
    pub y_chromosome_haplogroup: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarShapeData {
    pub ear_length_mm: f64,
    pub ear_width_mm: f64,
    pub lobe_type: EarLobeType,
    pub helix_curvature: f64,
    pub antihelix_features: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EarLobeType {
    Attached,
    Free,
    PartiallyAttached,
}

impl BiometricProfile {
    pub fn new() -> Self {
        Self {
            fingerprint: None,
            facial_recognition: None,
            iris_scan: None,
            voice_print: None,
            gait_analysis: None,
            retinal_scan: None,
            palm_print: None,
            vein_pattern: None,
            dna_profile: None,
            ear_shape: None,
        }
    }

    pub fn authentication_strength(&self) -> f64 {
        let mut strength = 0.0;

        if let Some(fp) = &self.fingerprint {
            strength += fp.quality_score * 0.15;
        }
        if self.facial_recognition.is_some() {
            strength += 0.15;
        }
        if self.iris_scan.is_some() {
            strength += 0.20;
        }
        if self.voice_print.is_some() {
            strength += 0.10;
        }
        if self.gait_analysis.is_some() {
            strength += 0.05;
        }
        if self.retinal_scan.is_some() {
            strength += 0.20;
        }
        if self.palm_print.is_some() {
            strength += 0.10;
        }
        if self.vein_pattern.is_some() {
            strength += 0.15;
        }
        if self.dna_profile.is_some() {
            strength += 0.30;
        }
        if self.ear_shape.is_some() {
            strength += 0.05;
        }

        strength.min(1.0)
    }

    pub fn available_modalities(&self) -> Vec<String> {
        let mut modalities = Vec::new();

        if self.fingerprint.is_some() {
            modalities.push("Fingerprint".to_string());
        }
        if self.facial_recognition.is_some() {
            modalities.push("Facial Recognition".to_string());
        }
        if self.iris_scan.is_some() {
            modalities.push("Iris Scan".to_string());
        }
        if self.voice_print.is_some() {
            modalities.push("Voice Print".to_string());
        }
        if self.gait_analysis.is_some() {
            modalities.push("Gait Analysis".to_string());
        }
        if self.retinal_scan.is_some() {
            modalities.push("Retinal Scan".to_string());
        }
        if self.palm_print.is_some() {
            modalities.push("Palm Print".to_string());
        }
        if self.vein_pattern.is_some() {
            modalities.push("Vein Pattern".to_string());
        }
        if self.dna_profile.is_some() {
            modalities.push("DNA Profile".to_string());
        }
        if self.ear_shape.is_some() {
            modalities.push("Ear Shape".to_string());
        }

        modalities
    }

    pub fn multi_factor_score(&self) -> f64 {
        let count = self.available_modalities().len() as f64;
        (count / 10.0).min(1.0)
    }

    pub fn recommended_use_cases(&self) -> Vec<String> {
        let mut use_cases = Vec::new();
        let strength = self.authentication_strength();

        if strength >= 0.8 {
            use_cases.push("High-security facility access".to_string());
            use_cases.push("Financial transactions".to_string());
            use_cases.push("Medical record access".to_string());
        }

        if strength >= 0.5 {
            use_cases.push("Device unlock".to_string());
            use_cases.push("Building access".to_string());
        }

        if strength >= 0.3 {
            use_cases.push("Low-security authentication".to_string());
        }

        use_cases
    }
}

impl Default for BiometricProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_profile() {
        let profile = BiometricProfile::new();
        assert_eq!(profile.available_modalities().len(), 0);
        assert_eq!(profile.authentication_strength(), 0.0);
    }

    #[test]
    fn test_fingerprint_only() {
        let mut profile = BiometricProfile::new();
        profile.fingerprint = Some(FingerprintData {
            patterns: vec![],
            minutiae_count: 45,
            ridge_density: 0.8,
            pattern_type: FingerPrintPatternType::Loop,
            quality_score: 0.9,
        });

        assert_eq!(profile.available_modalities().len(), 1);
        assert!(profile.authentication_strength() > 0.0);
    }

    #[test]
    fn test_multi_factor() {
        let mut profile = BiometricProfile::new();
        profile.fingerprint = Some(FingerprintData {
            patterns: vec![],
            minutiae_count: 45,
            ridge_density: 0.8,
            pattern_type: FingerPrintPatternType::Loop,
            quality_score: 0.95,
        });
        profile.iris_scan = Some(IrisData {
            iris_texture_features: vec![],
            iris_color: IrisColor::Brown,
            crypts_count: 12,
            furrows_count: 8,
            collarette_features: vec![],
            iris_diameter_mm: 11.5,
        });
        profile.facial_recognition = Some(FacialRecognitionData {
            facial_landmarks: vec![],
            interocular_distance_mm: 63.0,
            nose_bridge_length_mm: 25.0,
            jawline_angle_degrees: 120.0,
            face_shape: FaceShape::Oval,
            skin_texture_features: vec![],
            facial_geometry_hash: "hash123".to_string(),
        });

        assert_eq!(profile.available_modalities().len(), 3);
        assert!(profile.authentication_strength() > 0.4);
        assert!(profile.multi_factor_score() > 0.0);
    }

    #[test]
    fn test_high_security_profile() {
        let mut profile = BiometricProfile::new();
        profile.fingerprint = Some(FingerprintData {
            patterns: vec![],
            minutiae_count: 50,
            ridge_density: 0.85,
            pattern_type: FingerPrintPatternType::Whorl,
            quality_score: 0.98,
        });
        profile.iris_scan = Some(IrisData {
            iris_texture_features: vec![],
            iris_color: IrisColor::Blue,
            crypts_count: 15,
            furrows_count: 10,
            collarette_features: vec![],
            iris_diameter_mm: 12.0,
        });
        profile.retinal_scan = Some(RetinalData {
            blood_vessel_pattern: vec![],
            optic_disc_position: (0.0, 0.0),
            fovea_position: (0.0, 0.0),
            vessel_bifurcations: vec![],
        });
        profile.dna_profile = Some(DNABiometricData {
            str_markers: HashMap::new(),
            snp_profile: vec![],
            mitochondrial_haplogroup: "H1a".to_string(),
            y_chromosome_haplogroup: Some("R1b".to_string()),
        });

        let use_cases = profile.recommended_use_cases();
        assert!(use_cases.contains(&"High-security facility access".to_string()));
        assert!(profile.authentication_strength() > 0.8);
    }

    #[test]
    fn test_voice_print() {
        let voice = VoicePrintData {
            fundamental_frequency_hz: 120.0,
            formant_frequencies: vec![800.0, 1200.0, 2500.0],
            vocal_tract_length_cm: 17.5,
            pitch_range: (100.0, 250.0),
            speaking_rate_wpm: 150.0,
            voice_quality: VoiceQuality::Clear,
        };

        assert_eq!(voice.formant_frequencies.len(), 3);
        assert!(voice.fundamental_frequency_hz > 0.0);
    }

    #[test]
    fn test_gait_analysis() {
        let gait = GaitData {
            stride_length_cm: 150.0,
            step_width_cm: 12.0,
            cadence_steps_per_min: 110.0,
            gait_cycle_duration_s: 1.1,
            asymmetry_index: 0.05,
            biomechanical_features: vec![0.8, 0.9, 0.7],
        };

        assert!(gait.stride_length_cm > 0.0);
        assert!(gait.asymmetry_index < 1.0);
    }
}

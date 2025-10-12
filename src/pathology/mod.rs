pub mod allergy_immunology;
pub mod cardiovascular;
pub mod condition;
pub mod dermatology;
pub mod diagnosis;
pub mod disease;
pub mod gastroenterology;
pub mod headache;
pub mod infectious_disease;
pub mod metabolic;
pub mod neurological;
pub mod neuromuscular;
pub mod ophthalmology;
pub mod organ_pathophysiology;
pub mod orthopedics;
pub mod otolaryngology;
pub mod pain;
pub mod psychiatry;
pub mod pulmonology;
pub mod rheumatology;
pub mod sports_medicine;
pub mod symptom;
pub mod toxicology;
pub mod transplant;

// Type aliases to resolve conflicts
// ArrhythmiaType conflict: cardiovascular vs organ_pathophysiology
pub use cardiovascular::ArrhythmiaType as CardiovascularArrhythmiaType;
pub use organ_pathophysiology::ArrhythmiaType as OrganArrhythmiaType;

// ValveType conflict: cardiovascular vs organ_pathophysiology
pub use cardiovascular::ValveType as CardiovascularValveType;
pub use organ_pathophysiology::ValveType as OrganValveType;

// ValvularDysfunction conflict: cardiovascular vs organ_pathophysiology
pub use cardiovascular::ValvularDysfunction as CardiovascularValvularDysfunction;
pub use organ_pathophysiology::ValvularDysfunction as OrganValvularDysfunction;

// HypertensionStage conflict: cardiovascular vs metabolic
pub use cardiovascular::HypertensionStage as CardiovascularHypertensionStage;
pub use metabolic::HypertensionStage as MetabolicHypertensionStage;

// BiologicalSex conflict: cardiovascular vs metabolic
pub use cardiovascular::BiologicalSex as CardiovascularBiologicalSex;
pub use metabolic::BiologicalSex as MetabolicBiologicalSex;

// StrokeType conflict: cardiovascular vs neurological
pub use cardiovascular::StrokeType as CardiovascularStrokeType;
pub use neurological::StrokeType as NeurologicalStrokeType;

// APOEStatus conflict: cardiovascular vs neurological
pub use cardiovascular::APOEStatus as CardiovascularAPOEStatus;
pub use neurological::APOEStatus as NeurologicalAPOEStatus;

// Migraine conflict: condition vs headache
pub use condition::Migraine as ConditionMigraine;
pub use headache::Migraine as HeadacheMigraine;

// ClusterHeadache conflict: condition vs headache
pub use condition::ClusterHeadache as ConditionClusterHeadache;
pub use headache::ClusterHeadache as HeadacheClusterHeadache;

// Severity conflict: dermatology vs disease
pub use dermatology::Severity as DermatologySeverity;
pub use disease::Severity as DiseaseSeverity;

// RiskFactor conflict: dermatology vs disease
pub use dermatology::RiskFactor as DermatologyRiskFactor;
pub use disease::RiskFactor as DiseaseRiskFactor;

// PainIntensity conflict: headache vs pain
pub use headache::PainIntensity as HeadachePainIntensity;
pub use pain::PainIntensity as PainPainIntensity;

// ThyroidStatus conflict: metabolic vs organ_pathophysiology
pub use metabolic::ThyroidStatus as MetabolicThyroidStatus;
pub use organ_pathophysiology::ThyroidStatus as OrganThyroidStatus;

// CognitiveFunction conflict: organ_pathophysiology vs psychiatry
pub use organ_pathophysiology::CognitiveFunction as OrganCognitiveFunction;
pub use psychiatry::CognitiveFunction as PsychiatryCognitiveFunction;

// InjuryType conflict: orthopedics vs sports_medicine
pub use orthopedics::InjuryType as OrthopedicsInjuryType;
pub use sports_medicine::InjuryType as SportsMedicineInjuryType;

// InjurySeverity conflict: orthopedics vs sports_medicine
pub use orthopedics::InjurySeverity as OrthopedicsInjurySeverity;
pub use sports_medicine::InjurySeverity as SportsMedicineInjurySeverity;

// InjuryMechanism conflict: orthopedics vs sports_medicine
pub use orthopedics::InjuryMechanism as OrthopedicsInjuryMechanism;
pub use sports_medicine::InjuryMechanism as SportsMedicineInjuryMechanism;

// FractureType conflict: orthopedics vs sports_medicine
pub use orthopedics::FractureType as OrthopedicsFractureType;
pub use sports_medicine::FractureType as SportsMedicineFractureType;

// AnatomicalLocation conflict: orthopedics vs pain
pub use orthopedics::AnatomicalLocation as OrthopedicsAnatomicalLocation;
pub use pain::AnatomicalLocation as PainAnatomicalLocation;

// TemporalPattern conflict: pain vs symptom
pub use pain::TemporalPattern as PainTemporalPattern;
pub use symptom::TemporalPattern as SymptomTemporalPattern;

// Keep modules with conflicts as explicit exports, others as glob exports
pub use allergy_immunology::*;

// cardiovascular exports only non-conflicting items (ArrhythmiaType, ValveType, ValvularDysfunction, HypertensionStage, BiologicalSex, StrokeType, APOEStatus aliased above)
pub use cardiovascular::{CardiovascularCondition, MIType, AnginaType, HeartFailureType, CardiomyopathyType, CongenitalDefect, CardiovascularRiskProfile, PhysicalActivityLevel, RiskCategory, GeneticCardiovascularRisk, MTHFRStatus};

// condition exports only non-conflicting items (Migraine, ClusterHeadache aliased above)
pub use condition::{MedicalCondition};

// dermatology exports only non-conflicting items (Severity, RiskFactor aliased above)
pub use dermatology::{DermatologyProfile, FitzpatrickType, SkinCondition, SkinLesion, LesionType, LesionColor, BorderCharacteristic, EvolutionPattern, BodyRegion, MelanomaRisk, ABCDEScore, RiskLevel};

pub use diagnosis::*;

// disease exports only non-conflicting items (Severity, RiskFactor aliased above)
pub use disease::{DiseaseCategory, Disease, GeneticComponent, InheritancePattern, DiseaseProgression, DiseaseStage};

pub use gastroenterology::*;

// headache exports only non-conflicting items (Migraine, ClusterHeadache, PainIntensity aliased above)
pub use headache::{HeadacheType, MigraineSubtype, MigraineTrigger, AuraSymptom, AutonomicSymptom, HeadacheProfile};

pub use infectious_disease::*;

// metabolic exports only non-conflicting items (HypertensionStage, BiologicalSex, ThyroidStatus aliased above)
pub use metabolic::{MetabolicCondition, ObesityClass, LipidAbnormality, NAFLDStage, DiabetesProfile, DiabetesType, DiabeticComplication, MetabolicSyndromeAssessment, ThyroidProfile, ThyroidAntibodies, LipidPanel, LipidRiskCategory};

// neurological exports only non-conflicting items (StrokeType, APOEStatus aliased above)
pub use neurological::{NeurologicalCondition, MigraineType, EpilepsyType, NeuropathyType, PainType, NeurologicalProfile, NeurologicalSymptom, GeneticNeurologicalRisk};

pub use neuromuscular::*;
pub use ophthalmology::*;

// organ_pathophysiology exports only non-conflicting items (ArrhythmiaType, ValveType, ValvularDysfunction, ThyroidStatus, CognitiveFunction aliased above)
pub use organ_pathophysiology::{OrganPathophysiology, CardiacPathology, CardiacOutputStatus, ValveDysfunction, DysfunctionSeverity, CADSeverity, HeartFailureStage, PulmonaryPathology, ChronicPulmonaryCondition, HepaticPathology, LiverFunctionTests, FibrosisStage, SteatosisGrade, SyntheticFunction, RenalPathology, CKDStage, ElectrolyteDisturbance, Electrolyte, ElectrolyteAbnormality, AcidBaseStatus, AcidBaseDisorder, NeurologicalPathology, MotorFunction, CoordinationStatus, TremorType, SensoryFunction, SensoryStatus, NeurodegenerativeCondition, EndocrinePathology, ThyroidFunction, AdrenalFunction, AdrenalStatus, GlucoseMetabolismStatus, DiabetesStatus, BoneMetabolismStatus, BoneHealthStatus};

// orthopedics exports only non-conflicting items (InjuryType, AnatomicalLocation, InjurySeverity, InjuryMechanism, FractureType aliased above)
pub use orthopedics::{OrthopedicProfile, OrthopedicInjury, HealingStatus, Fracture, Bone, FractureHealing, UnionStatus, JointReplacement, ImplantType, ImplantComplication, SpinalCondition, SpinalConditionType, SpinalLevel, SpinalSeverity, FunctionalStatus};

pub use otolaryngology::*;

// pain exports only non-conflicting items (AnatomicalLocation, PainIntensity, TemporalPattern aliased above)
pub use pain::{PainProfile, PainSite, Side, PainQuality, PainEpisode, ChronicPainCondition, ChronicPainType, FlareFrequency, OpioidTolerance, NeuropathicPainAssessment, PainManagementPlan, NonPharmacologicalIntervention, PharmacologicalIntervention, MedicationClass, RouteOfAdministration, InterventionalProcedure, PsychologicalIntervention, DisabilityImpact, PainSeverity};

// psychiatry exports only non-conflicting items (CognitiveFunction aliased above)
pub use psychiatry::{PsychiatricProfile, PsychiatricDiagnosis, DisorderType, PsychiatricSeverity, EpisodeType, SymptomSeverity, DepressionScore, AnxietyScore, PsychosisScore, CognitiveScore, MoodState, Mood, Affect, EnergyLevel, RiskAssessments, SuicideRisk, ViolenceRisk, SelfHarmRisk, GlobalFunctioning};

pub use pulmonology::*;
pub use rheumatology::*;

// sports_medicine exports only non-conflicting items (InjuryType, InjurySeverity, InjuryMechanism, FractureType aliased above)
pub use sports_medicine::{InjuryProfile, Injury, MuscleStrain, LigamentSprain, TendinopathyType, StressInjuryType, CartilageType, NerveInjuryType, InjuryLocation, FunctionalLimitation, HealingStage, PastInjury, InjuryRiskFactor, BiomechanicalIssue, BiomechanicalIssueType, ReturnToPlayStage, RehabilitationProtocol, RehabPhase, RehabExercise, ActivityRestriction, RehabGoal, InjuryRisk};

// symptom exports only non-conflicting items (TemporalPattern aliased above)
pub use symptom::{SymptomCategory, Symptom, Onset, Duration, SymptomCluster};

pub use toxicology::*;
pub use transplant::*;

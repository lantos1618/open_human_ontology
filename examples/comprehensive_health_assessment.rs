use human_biology::systems::cardiovascular::hematology::{HematologyProfile, BiologicalSex};
use human_biology::systems::nervous::pain_pathways::PainProcessingSystem;
use human_biology::systems::sensory::{VestibularSystem, MechanoreceptorSystem};
use human_biology::biology::microbiome::Microbiome;

fn main() {
    println!("=== Comprehensive Health Assessment System ===\n");

    println!("1. HEMATOLOGY ASSESSMENT");
    println!("{}", "=".repeat(50));
    assess_hematology();

    println!("\n2. PAIN PROCESSING EVALUATION");
    println!("{}", "=".repeat(50));
    assess_pain_system();

    println!("\n3. VESTIBULAR & BALANCE ASSESSMENT");
    println!("{}", "=".repeat(50));
    assess_vestibular();

    println!("\n4. MECHANORECEPTION & PROPRIOCEPTION");
    println!("{}", "=".repeat(50));
    assess_mechanoreception();

    println!("\n5. GUT MICROBIOME ANALYSIS");
    println!("{}", "=".repeat(50));
    assess_microbiome();
}

fn assess_hematology() {
    let profile = HematologyProfile::new_normal(BiologicalSex::Male);

    println!("Red Blood Cells:");
    println!("  Hemoglobin: {:.1} g/dL", profile.complete_blood_count.red_blood_cells.hemoglobin_g_dl);
    println!("  Hematocrit: {:.1}%", profile.complete_blood_count.red_blood_cells.hematocrit_percent);
    println!("  MCV: {:.1} fL", profile.complete_blood_count.red_blood_cells.mean_corpuscular_volume_fl);

    println!("\nWhite Blood Cells:");
    println!("  Total: {:.0}/μL", profile.complete_blood_count.white_blood_cells.total_count_per_ul);
    println!("  Neutrophils: {:.0}/μL ({:.1}%)",
        profile.complete_blood_count.white_blood_cells.neutrophils.absolute_count_per_ul,
        profile.complete_blood_count.white_blood_cells.neutrophils.percent);
    println!("  Lymphocytes: {:.0}/μL ({:.1}%)",
        profile.complete_blood_count.white_blood_cells.lymphocytes.absolute_count_per_ul,
        profile.complete_blood_count.white_blood_cells.lymphocytes.percent);

    println!("\nPlatelet Count: {:.0}/μL", profile.complete_blood_count.platelets.count_per_ul);

    println!("\nCoagulation:");
    println!("  PT: {:.1} seconds", profile.coagulation.coagulation_tests.prothrombin_time_seconds);
    println!("  INR: {:.2}", profile.coagulation.coagulation_tests.inr);
    println!("  aPTT: {:.1} seconds", profile.coagulation.coagulation_tests.activated_partial_thromboplastin_time_seconds);

    println!("\nRisk Assessment:");
    println!("  Anemia Risk: {:?}", profile.assess_anemia_risk());
    println!("  Bleeding Risk: {:?}", profile.assess_bleeding_risk());
    println!("  Thrombosis Risk: {:?}", profile.assess_thrombosis_risk());
}

fn assess_pain_system() {
    let mut pain_system = PainProcessingSystem::new_normal();

    println!("Peripheral Nociception:");
    println!("  A-delta fibers: {}", pain_system.peripheral_nociception.a_delta_fibers.fiber_count);
    println!("  C-fibers: {}", pain_system.peripheral_nociception.c_fibers.fiber_count);
    println!("  Sensitization: {:.2}", pain_system.peripheral_nociception.a_delta_fibers.sensitization_level);

    println!("\nSpinal Processing:");
    println!("  Central sensitization: {:.2}", pain_system.spinal_processing.dorsal_horn.central_sensitization);
    println!("  Gate open: {:.0}%", pain_system.spinal_processing.gate_control.gate_open_percentage * 100.0);

    println!("\nDescending Modulation:");
    println!("  PAG activity: {:.2}", pain_system.descending_modulation.periaqueductal_gray.activity_level);
    println!("  Endogenous opioids: {:.2}", pain_system.descending_modulation.endogenous_opioids.beta_endorphin);

    let pain_intensity = pain_system.calculate_pain_intensity(5.0);
    let pain_tolerance = pain_system.calculate_pain_tolerance();
    let chronic_risk = pain_system.assess_chronic_pain_risk();

    println!("\nPain Metrics:");
    println!("  Baseline pain intensity (with 5.0 input): {:.2}/10", pain_intensity);
    println!("  Pain tolerance: {:.2}", pain_tolerance);
    println!("  Chronic pain risk: {:?}", chronic_risk);

    pain_system.spinal_processing.dorsal_horn.central_sensitization = 0.8;
    pain_system.pain_memory.pain_catastrophizing_score = 0.6;

    println!("\n--- Simulating Chronic Pain Condition ---");
    let sensitized_pain = pain_system.calculate_pain_intensity(5.0);
    let new_chronic_risk = pain_system.assess_chronic_pain_risk();

    println!("  Sensitized pain intensity: {:.2}/10", sensitized_pain);
    println!("  Updated chronic risk: {:?}", new_chronic_risk);
}

fn assess_vestibular() {
    let vestibular = VestibularSystem::new_normal();

    println!("Semicircular Canals:");
    println!("  Anterior: {} hair cells, sensitivity: {:.2}",
        vestibular.semicircular_canals.anterior_canal.hair_cells,
        vestibular.semicircular_canals.anterior_canal.sensitivity);
    println!("  Posterior: {} hair cells", vestibular.semicircular_canals.posterior_canal.hair_cells);
    println!("  Lateral: {} hair cells", vestibular.semicircular_canals.lateral_canal.hair_cells);

    println!("\nOtolith Organs:");
    println!("  Utricle: {} hair cells", vestibular.otolith_organs.utricle.hair_cells);
    println!("  Saccule: {} hair cells", vestibular.otolith_organs.saccule.hair_cells);

    println!("\nBalance Function:");
    println!("  Static balance: {:.2}", vestibular.balance_function.static_balance_score);
    println!("  Dynamic balance: {:.2}", vestibular.balance_function.dynamic_balance_score);
    println!("  VOR gain: {:.2}", vestibular.balance_function.vestibulo_ocular_reflex_gain);

    let vertigo_risk = vestibular.calculate_vertigo_risk();
    let fall_risk = vestibular.assess_fall_risk();

    println!("\nRisk Assessment:");
    println!("  Vertigo risk: {:.2}%", vertigo_risk * 100.0);
    println!("  Fall risk: {:?}", fall_risk);
}

fn assess_mechanoreception() {
    let mechanoreceptors = MechanoreceptorSystem::new_normal();

    println!("Cutaneous Receptors:");
    println!("  Meissner's corpuscles: {}", mechanoreceptors.cutaneous_receptors.meissners_corpuscles.count);
    println!("  Merkel's discs: {}", mechanoreceptors.cutaneous_receptors.merkels_discs.count);
    println!("  Pacinian corpuscles: {}", mechanoreceptors.cutaneous_receptors.pacinian_corpuscles.count);
    println!("  Ruffini endings: {}", mechanoreceptors.cutaneous_receptors.ruffini_endings.count);

    println!("\nDeep Tissue Receptors:");
    println!("  Muscle spindles: {}", mechanoreceptors.deep_tissue_receptors.muscle_spindles.total_count);
    println!("  Golgi tendon organs: {}", mechanoreceptors.deep_tissue_receptors.golgi_tendon_organs.count);

    println!("\nBaroreceptors:");
    println!("  Carotid sinus: {} receptors @ {:.1} mmHg",
        mechanoreceptors.visceral_mechanoreceptors.baroreceptors.carotid_sinus.receptor_count,
        mechanoreceptors.visceral_mechanoreceptors.baroreceptors.carotid_sinus.baseline_pressure_mmhg);
    println!("  Aortic arch: {} receptors",
        mechanoreceptors.visceral_mechanoreceptors.baroreceptors.aortic_arch.receptor_count);

    let tactile_acuity = mechanoreceptors.calculate_tactile_acuity();
    let proprioceptive_accuracy = mechanoreceptors.calculate_proprioceptive_accuracy();

    println!("\nFunctional Assessments:");
    println!("  Tactile acuity: {:.2}", tactile_acuity);
    println!("  Proprioceptive accuracy: {:.2}", proprioceptive_accuracy);
}

fn assess_microbiome() {
    let gut = Microbiome::new_healthy_gut();

    println!("Gut Microbiome Profile:");
    println!("  Diversity index: {:.2}", gut.diversity_index);
    println!("  Total bacterial load: {:.2e} CFU/mL", gut.total_count_cfu_ml);
    println!("  Number of species: {}", gut.bacteria.len());

    println!("\nKey Species:");
    for bacteria in gut.bacteria.iter().take(5) {
        println!("  {} ({:?}): {:.1}%",
            bacteria.species,
            bacteria.phylum,
            bacteria.relative_abundance_percent);
    }

    let fb_ratio = gut.firmicutes_bacteroidetes_ratio();
    let has_dysbiosis = gut.has_dysbiosis();
    let pathogenic_load = gut.pathogenic_load();
    let beneficial_count = gut.beneficial_bacteria_count();

    println!("\nMicrobiome Metrics:");
    println!("  Firmicutes/Bacteroidetes ratio: {:.2}", fb_ratio);
    println!("  Dysbiosis detected: {}", has_dysbiosis);
    println!("  Pathogenic load: {:.2}%", pathogenic_load);
    println!("  Beneficial species count: {}", beneficial_count);
}


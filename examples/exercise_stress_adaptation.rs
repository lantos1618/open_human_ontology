use std::fmt;

#[derive(Debug, Clone, Copy)]
struct OxidativeStressState {
    ros_h2o2_nm: f64,
    lipid_peroxides_mda_um: f64,
    superoxide_nm: f64,
    peroxynitrite_nm: f64,
}

#[derive(Debug, Clone, Copy)]
struct AntioxidantDefense {
    nrf2_nuclear_fold: f64,
    sod2_activity_u_mg: f64,
    gpx4_activity_nmol_min_mg: f64,
    catalase_activity_u_mg: f64,
    gsh_gssg_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
struct MitochondrialFunction {
    membrane_potential_mv: f64,
    atp_production_umol_min_g: f64,
    respiratory_control_ratio: f64,
    drp1_fission_events_hr: f64,
    mfn2_fusion_rate_min: f64,
    mitophagy_flux_percent_day: f64,
}

#[derive(Debug, Clone, Copy)]
struct InflammatoryResponse {
    nlrp3_asc_specks_per_cell: f64,
    il6_pg_ml: f64,
    tnf_alpha_pg_ml: f64,
    il10_pg_ml: f64,
    il10_tnf_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
struct MetabolicState {
    lactate_mm: f64,
    glucose_mg_dl: f64,
    free_fatty_acids_mm: f64,
    oxygen_consumption_ml_kg_min: f64,
    respiratory_exchange_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
struct HormoneProfile {
    cortisol_nm: f64,
    epinephrine_nm: f64,
    norepinephrine_nm: f64,
    growth_hormone_ng_ml: f64,
    testosterone_ng_dl: f64,
}

#[derive(Debug, Clone, Copy)]
struct CardiovascularResponse {
    heart_rate_bpm: f64,
    stroke_volume_ml: f64,
    cardiac_output_l_min: f64,
    systolic_bp_mmhg: f64,
    diastolic_bp_mmhg: f64,
    no_endothelial_nm: f64,
}

#[derive(Debug, Clone, Copy)]
struct MuscularAdaptation {
    calcium_cytosolic_nm: f64,
    pgc1_alpha_expression_fold: f64,
    mtor_s2448_phospho_fold: f64,
    ampk_t172_phospho_fold: f64,
    muscle_glycogen_mmol_kg: f64,
    creatine_phosphate_mmol_kg: f64,
}

#[derive(Debug, Clone, Copy)]
struct CircadianState {
    time_of_day_hours: f64,
    cortisol_amplitude_fold: f64,
    melatonin_pg_ml: f64,
    core_temp_celsius: f64,
}

#[derive(Debug)]
struct HumanExerciseState {
    time_minutes: f64,
    exercise_intensity_percent_vo2max: f64,
    oxidative_stress: OxidativeStressState,
    antioxidants: AntioxidantDefense,
    mitochondria: MitochondrialFunction,
    inflammation: InflammatoryResponse,
    metabolism: MetabolicState,
    hormones: HormoneProfile,
    cardiovascular: CardiovascularResponse,
    muscle: MuscularAdaptation,
    circadian: CircadianState,
}

impl HumanExerciseState {
    fn baseline() -> Self {
        HumanExerciseState {
            time_minutes: 0.0,
            exercise_intensity_percent_vo2max: 0.0,
            oxidative_stress: OxidativeStressState {
                ros_h2o2_nm: 10.0,
                lipid_peroxides_mda_um: 0.5,
                superoxide_nm: 5.0,
                peroxynitrite_nm: 8.0,
            },
            antioxidants: AntioxidantDefense {
                nrf2_nuclear_fold: 1.0,
                sod2_activity_u_mg: 25.0,
                gpx4_activity_nmol_min_mg: 180.0,
                catalase_activity_u_mg: 240.0,
                gsh_gssg_ratio: 100.0,
            },
            mitochondria: MitochondrialFunction {
                membrane_potential_mv: -180.0,
                atp_production_umol_min_g: 45.0,
                respiratory_control_ratio: 5.0,
                drp1_fission_events_hr: 0.8,
                mfn2_fusion_rate_min: 1.2,
                mitophagy_flux_percent_day: 5.0,
            },
            inflammation: InflammatoryResponse {
                nlrp3_asc_specks_per_cell: 0.01,
                il6_pg_ml: 1.5,
                tnf_alpha_pg_ml: 3.0,
                il10_pg_ml: 5.0,
                il10_tnf_ratio: 1.67,
            },
            metabolism: MetabolicState {
                lactate_mm: 1.0,
                glucose_mg_dl: 90.0,
                free_fatty_acids_mm: 0.4,
                oxygen_consumption_ml_kg_min: 3.5,
                respiratory_exchange_ratio: 0.85,
            },
            hormones: HormoneProfile {
                cortisol_nm: 150.0,
                epinephrine_nm: 0.5,
                norepinephrine_nm: 2.0,
                growth_hormone_ng_ml: 0.5,
                testosterone_ng_dl: 600.0,
            },
            cardiovascular: CardiovascularResponse {
                heart_rate_bpm: 70.0,
                stroke_volume_ml: 70.0,
                cardiac_output_l_min: 4.9,
                systolic_bp_mmhg: 120.0,
                diastolic_bp_mmhg: 80.0,
                no_endothelial_nm: 100.0,
            },
            muscle: MuscularAdaptation {
                calcium_cytosolic_nm: 100.0,
                pgc1_alpha_expression_fold: 1.0,
                mtor_s2448_phospho_fold: 1.0,
                ampk_t172_phospho_fold: 1.0,
                muscle_glycogen_mmol_kg: 400.0,
                creatine_phosphate_mmol_kg: 25.0,
            },
            circadian: CircadianState {
                time_of_day_hours: 10.0,
                cortisol_amplitude_fold: 1.5,
                melatonin_pg_ml: 5.0,
                core_temp_celsius: 37.0,
            },
        }
    }

    fn simulate_exercise(&mut self, intensity_percent_vo2max: f64, duration_min: f64) {
        self.exercise_intensity_percent_vo2max = intensity_percent_vo2max;
        self.time_minutes = duration_min;

        let intensity_factor = intensity_percent_vo2max / 100.0;

        self.metabolism.oxygen_consumption_ml_kg_min = 3.5 + (50.0 * intensity_factor);
        self.metabolism.lactate_mm = 1.0 + (12.0 * intensity_factor.powi(2));
        self.metabolism.glucose_mg_dl = 90.0 - (20.0 * intensity_factor);
        self.metabolism.free_fatty_acids_mm = 0.4 + (0.8 * intensity_factor);
        self.metabolism.respiratory_exchange_ratio = 0.75 + (0.25 * intensity_factor);

        self.cardiovascular.heart_rate_bpm = 70.0 + (120.0 * intensity_factor);
        self.cardiovascular.stroke_volume_ml = 70.0 + (50.0 * intensity_factor * (1.0 - 0.3 * intensity_factor));
        self.cardiovascular.cardiac_output_l_min = (self.cardiovascular.heart_rate_bpm * self.cardiovascular.stroke_volume_ml) / 1000.0;
        self.cardiovascular.systolic_bp_mmhg = 120.0 + (60.0 * intensity_factor);
        self.cardiovascular.diastolic_bp_mmhg = 80.0 - (10.0 * intensity_factor);
        self.cardiovascular.no_endothelial_nm = 100.0 + (200.0 * intensity_factor);

        self.oxidative_stress.ros_h2o2_nm = 10.0 * (1.0 + 8.0 * intensity_factor);
        self.oxidative_stress.superoxide_nm = 5.0 * (1.0 + 10.0 * intensity_factor);
        self.oxidative_stress.peroxynitrite_nm = 8.0 * (1.0 + 6.0 * intensity_factor);
        self.oxidative_stress.lipid_peroxides_mda_um = 0.5 * (1.0 + 4.0 * intensity_factor);

        let ros_fold = self.oxidative_stress.ros_h2o2_nm / 10.0;
        self.antioxidants.nrf2_nuclear_fold = 1.0 + (3.0 * intensity_factor);
        self.antioxidants.sod2_activity_u_mg = 25.0 * (1.0 + 0.8 * self.antioxidants.nrf2_nuclear_fold);
        self.antioxidants.gpx4_activity_nmol_min_mg = 180.0 * (1.0 + 0.6 * self.antioxidants.nrf2_nuclear_fold);
        self.antioxidants.catalase_activity_u_mg = 240.0 * (1.0 + 0.5 * self.antioxidants.nrf2_nuclear_fold);
        self.antioxidants.gsh_gssg_ratio = 100.0 / (1.0 + 0.8 * intensity_factor);

        self.mitochondria.membrane_potential_mv = -180.0 + (20.0 * intensity_factor);
        self.mitochondria.atp_production_umol_min_g = 45.0 * (1.0 + 2.5 * intensity_factor);
        self.mitochondria.respiratory_control_ratio = 5.0 * (1.0 - 0.3 * intensity_factor);
        self.mitochondria.drp1_fission_events_hr = 0.8 * (1.0 + 3.0 * intensity_factor);
        self.mitochondria.mfn2_fusion_rate_min = 1.2 * (1.0 - 0.4 * intensity_factor);

        if intensity_factor > 0.7 {
            self.inflammation.nlrp3_asc_specks_per_cell = 0.01 + (0.15 * (intensity_factor - 0.7));
            self.inflammation.il6_pg_ml = 1.5 * (1.0 + 20.0 * (intensity_factor - 0.7));
            self.inflammation.tnf_alpha_pg_ml = 3.0 * (1.0 + 8.0 * (intensity_factor - 0.7));
            self.inflammation.il10_pg_ml = 5.0 * (1.0 + 12.0 * (intensity_factor - 0.7));
            self.inflammation.il10_tnf_ratio = self.inflammation.il10_pg_ml / self.inflammation.tnf_alpha_pg_ml;
        }

        self.hormones.cortisol_nm = 150.0 * (1.0 + 2.0 * intensity_factor);
        self.hormones.epinephrine_nm = 0.5 * (1.0 + 40.0 * intensity_factor);
        self.hormones.norepinephrine_nm = 2.0 * (1.0 + 15.0 * intensity_factor);
        self.hormones.growth_hormone_ng_ml = 0.5 * (1.0 + 30.0 * intensity_factor);

        self.muscle.calcium_cytosolic_nm = 100.0 + (900.0 * intensity_factor);
        self.muscle.ampk_t172_phospho_fold = 1.0 + (4.0 * intensity_factor);
        self.muscle.pgc1_alpha_expression_fold = 1.0 + (3.0 * self.muscle.ampk_t172_phospho_fold * (duration_min / 60.0));
        self.muscle.muscle_glycogen_mmol_kg = 400.0 - (250.0 * intensity_factor * (duration_min / 60.0));
        self.muscle.creatine_phosphate_mmol_kg = 25.0 - (15.0 * intensity_factor);

        self.circadian.core_temp_celsius = 37.0 + (2.0 * intensity_factor);
    }

    fn simulate_recovery(&mut self, hours_post: f64) {
        let recovery_factor = (-hours_post / 24.0).exp();

        self.oxidative_stress.ros_h2o2_nm = 10.0 + (self.oxidative_stress.ros_h2o2_nm - 10.0) * recovery_factor;
        self.oxidative_stress.superoxide_nm = 5.0 + (self.oxidative_stress.superoxide_nm - 5.0) * recovery_factor;
        self.oxidative_stress.lipid_peroxides_mda_um = 0.5 + (self.oxidative_stress.lipid_peroxides_mda_um - 0.5) * recovery_factor;

        self.inflammation.il6_pg_ml = 1.5 + (self.inflammation.il6_pg_ml - 1.5) * recovery_factor;
        self.inflammation.tnf_alpha_pg_ml = 3.0 + (self.inflammation.tnf_alpha_pg_ml - 3.0) * recovery_factor;

        let training_adaptation = 1.0 - recovery_factor;
        self.antioxidants.sod2_activity_u_mg = 25.0 * (1.0 + 0.3 * training_adaptation);
        self.antioxidants.gpx4_activity_nmol_min_mg = 180.0 * (1.0 + 0.25 * training_adaptation);
        self.mitochondria.respiratory_control_ratio = 5.0 * (1.0 + 0.15 * training_adaptation);

        let glycogen_recovery_rate = 15.0;
        self.muscle.muscle_glycogen_mmol_kg = (self.muscle.muscle_glycogen_mmol_kg + glycogen_recovery_rate * hours_post).min(400.0);

        self.muscle.mtor_s2448_phospho_fold = 1.0 + (3.0 * (-hours_post / 2.0).exp());

        self.cardiovascular.heart_rate_bpm = 70.0;
        self.cardiovascular.stroke_volume_ml = 70.0;
        self.cardiovascular.cardiac_output_l_min = 4.9;
        self.metabolism.lactate_mm = 1.0;
    }
}

impl fmt::Display for HumanExerciseState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "=== HUMAN EXERCISE SIMULATION ===")?;
        writeln!(f, "Time: {:.1} min | Intensity: {:.0}% VO2max\n", self.time_minutes, self.exercise_intensity_percent_vo2max)?;

        writeln!(f, "CARDIOVASCULAR:")?;
        writeln!(f, "  HR: {:.0} bpm | SV: {:.0} ml | CO: {:.1} L/min",
            self.cardiovascular.heart_rate_bpm, self.cardiovascular.stroke_volume_ml, self.cardiovascular.cardiac_output_l_min)?;
        writeln!(f, "  BP: {:.0}/{:.0} mmHg | eNOS-NO: {:.0} nM\n",
            self.cardiovascular.systolic_bp_mmhg, self.cardiovascular.diastolic_bp_mmhg, self.cardiovascular.no_endothelial_nm)?;

        writeln!(f, "METABOLISM:")?;
        writeln!(f, "  VO2: {:.1} ml/kg/min | RER: {:.2}", self.metabolism.oxygen_consumption_ml_kg_min, self.metabolism.respiratory_exchange_ratio)?;
        writeln!(f, "  Lactate: {:.1} mM | Glucose: {:.0} mg/dL | FFA: {:.2} mM\n",
            self.metabolism.lactate_mm, self.metabolism.glucose_mg_dl, self.metabolism.free_fatty_acids_mm)?;

        writeln!(f, "MUSCLE:")?;
        writeln!(f, "  Ca²⁺: {:.0} nM | Glycogen: {:.0} mmol/kg | PCr: {:.1} mmol/kg",
            self.muscle.calcium_cytosolic_nm, self.muscle.muscle_glycogen_mmol_kg, self.muscle.creatine_phosphate_mmol_kg)?;
        writeln!(f, "  AMPK-pT172: {:.2}× | PGC-1α: {:.2}× | mTOR-pS2448: {:.2}×\n",
            self.muscle.ampk_t172_phospho_fold, self.muscle.pgc1_alpha_expression_fold, self.muscle.mtor_s2448_phospho_fold)?;

        writeln!(f, "MITOCHONDRIA:")?;
        writeln!(f, "  ΔΨm: {:.0} mV | ATP: {:.1} μmol/min/g | RCR: {:.2}",
            self.mitochondria.membrane_potential_mv, self.mitochondria.atp_production_umol_min_g, self.mitochondria.respiratory_control_ratio)?;
        writeln!(f, "  Drp1 fission: {:.2}/hr | Mfn2 fusion: {:.2}/min\n",
            self.mitochondria.drp1_fission_events_hr, self.mitochondria.mfn2_fusion_rate_min)?;

        writeln!(f, "OXIDATIVE STRESS:")?;
        writeln!(f, "  H₂O₂: {:.1} nM | O₂⁻: {:.1} nM | ONOO⁻: {:.1} nM | MDA: {:.2} μM\n",
            self.oxidative_stress.ros_h2o2_nm, self.oxidative_stress.superoxide_nm,
            self.oxidative_stress.peroxynitrite_nm, self.oxidative_stress.lipid_peroxides_mda_um)?;

        writeln!(f, "ANTIOXIDANT DEFENSE:")?;
        writeln!(f, "  NRF2: {:.2}× nuclear | SOD2: {:.1} U/mg | GPX4: {:.1} nmol/min/mg",
            self.antioxidants.nrf2_nuclear_fold, self.antioxidants.sod2_activity_u_mg, self.antioxidants.gpx4_activity_nmol_min_mg)?;
        writeln!(f, "  Catalase: {:.1} U/mg | GSH/GSSG: {:.1}\n",
            self.antioxidants.catalase_activity_u_mg, self.antioxidants.gsh_gssg_ratio)?;

        writeln!(f, "INFLAMMATION:")?;
        writeln!(f, "  NLRP3 ASC: {:.3}/cell | IL-6: {:.1} pg/ml | TNF-α: {:.1} pg/ml",
            self.inflammation.nlrp3_asc_specks_per_cell, self.inflammation.il6_pg_ml, self.inflammation.tnf_alpha_pg_ml)?;
        writeln!(f, "  IL-10: {:.1} pg/ml | IL-10/TNF-α: {:.2}\n",
            self.inflammation.il10_pg_ml, self.inflammation.il10_tnf_ratio)?;

        writeln!(f, "HORMONES:")?;
        writeln!(f, "  Cortisol: {:.0} nM | Epi: {:.1} nM | NE: {:.1} nM | GH: {:.1} ng/ml\n",
            self.hormones.cortisol_nm, self.hormones.epinephrine_nm,
            self.hormones.norepinephrine_nm, self.hormones.growth_hormone_ng_ml)?;

        Ok(())
    }
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║      HUMAN BIOLOGICAL RESPONSE TO EXERCISE STRESS                 ║");
    println!("║  Multi-System Integration: Molecular → Cellular → Physiological   ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("SCENARIO 1: MODERATE EXERCISE (60% VO2max, 30 min)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut human_moderate = HumanExerciseState::baseline();
    println!("BASELINE (Rest):\n{}", human_moderate);

    human_moderate.simulate_exercise(60.0, 30.0);
    println!("\n{}\nDURING EXERCISE (60% VO2max, 30 min):\n{}", "─".repeat(70), human_moderate);

    human_moderate.simulate_recovery(2.0);
    println!("\n{}\n2 HOURS POST-EXERCISE:\n{}", "─".repeat(70), human_moderate);

    human_moderate.simulate_recovery(24.0);
    println!("\n{}\n24 HOURS POST-EXERCISE (Training Adaptation):\n{}", "─".repeat(70), human_moderate);

    println!("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("SCENARIO 2: HIGH-INTENSITY INTERVAL TRAINING (85% VO2max, 20 min)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut human_hiit = HumanExerciseState::baseline();
    human_hiit.simulate_exercise(85.0, 20.0);
    println!("DURING HIIT (85% VO2max, 20 min):\n{}", human_hiit);

    human_hiit.simulate_recovery(1.0);
    println!("\n{}\n1 HOUR POST-HIIT:\n{}", "─".repeat(70), human_hiit);

    human_hiit.simulate_recovery(48.0);
    println!("\n{}\n48 HOURS POST-HIIT (Supercompensation):\n{}", "─".repeat(70), human_hiit);

    println!("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("SCENARIO 3: ULTRA-ENDURANCE (50% VO2max, 180 min)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut human_ultra = HumanExerciseState::baseline();
    human_ultra.simulate_exercise(50.0, 180.0);
    println!("DURING ULTRA-ENDURANCE (50% VO2max, 3 hours):\n{}", human_ultra);

    human_ultra.simulate_recovery(6.0);
    println!("\n{}\n6 HOURS POST-EXERCISE:\n{}", "─".repeat(70), human_ultra);

    human_ultra.simulate_recovery(72.0);
    println!("\n{}\n72 HOURS POST-EXERCISE (Mitochondrial Biogenesis):\n{}", "─".repeat(70), human_ultra);

    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║                         KEY INSIGHTS                              ║");
    println!("╠═══════════════════════════════════════════════════════════════════╣");
    println!("║ • Moderate exercise → hormesis: controlled oxidative stress       ║");
    println!("║   triggers NRF2 antioxidant upregulation and mitochondrial        ║");
    println!("║   biogenesis via AMPK/PGC-1α pathway                              ║");
    println!("║                                                                   ║");
    println!("║ • HIIT → maximal cardiovascular and metabolic stress with acute   ║");
    println!("║   inflammation (IL-6), balanced by anti-inflammatory IL-10        ║");
    println!("║                                                                   ║");
    println!("║ • Ultra-endurance → glycogen depletion, fat oxidation, sustained  ║");
    println!("║   mitochondrial stress → robust PGC-1α-driven adaptation          ║");
    println!("║                                                                   ║");
    println!("║ • Recovery period shows training adaptations: enhanced           ║");
    println!("║   antioxidant capacity, improved mitochondrial function,          ║");
    println!("║   glycogen supercompensation, muscle protein synthesis (mTOR)     ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  CIRCADIAN DISRUPTION & JET LAG SIMULATION                     ║");
    println!("║  Multi-system impact of circadian misalignment                 ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("Simulating the biological consequences of rapid time zone shift:");
    println!("  San Francisco → Tokyo (8-hour eastward shift)\n");
    println!("Systems affected:");
    println!("  • Cortisol diurnal rhythm & HPA axis");
    println!("  • Gut-brain axis & serotonin");
    println!("  • Inflammatory cytokines");
    println!("  • Metabolic regulation");
    println!("  • Sleep-wake homeostasis\n");

    simulate_baseline_circadian();
    simulate_day1_jetlag();
    simulate_day3_jetlag();
    simulate_day7_recovery();

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  KEY INSIGHTS                                                  ║");
    println!("║                                                                ║");
    println!("║  • Cortisol rhythm takes 3-5 days to realign                   ║");
    println!("║  • Gut microbiome serotonin/GABA disrupted → mood/GI issues    ║");
    println!("║  • Circadian misalignment → pro-inflammatory state             ║");
    println!("║  • Insulin sensitivity reduced → transient glucose intolerance ║");
    println!("║  • Recovery requires systematic re-entrainment                 ║");
    println!("║                                                                ║");
    println!("║  Clinical relevance: shift work, jet lag, sleep disorders      ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
}

fn simulate_baseline_circadian() {
    println!("████████████████████████████████████████████████████████████████");
    println!("BASELINE: Normal circadian rhythm (San Francisco, day 0)");
    println!("████████████████████████████████████████████████████████████████\n");

    let local_time_sf = "22:00 (night)";
    let melatonin_pg_ml = 80.0;
    let core_body_temp_c = 36.5;

    let cortisol_06h_morning_ug_dl = 15.0;
    let cortisol_12h_midday_ug_dl = 10.0;
    let cortisol_18h_evening_ug_dl = 5.0;
    let cortisol_00h_nadir_ug_dl = 2.0;
    let car_nmol_l = 12.0;
    let acth_morning_pg_ml = 25.0;
    let acth_evening_pg_ml = 10.0;
    let crh_hypothalamic_ng_ml = 1.0;

    let gut_serotonin_ng_ml = 120.0;
    let tryptophan_availability_um = 50.0;
    let microbiota_gaba_nm = 200.0;
    let scfa_butyrate_mm = 1.0;
    let glp1_fasting_pmol_l = 15.0;

    let tnf_alpha_pg_ml = 2.0;
    let il6_pg_ml = 1.2;
    let il1beta_pg_ml = 4.0;
    let il10_tnf_ratio = 5.0;

    let fasting_glucose_mg_dl = 85.0;
    let fasting_insulin_mu_u_ml = 5.0;
    let homa_ir = (fasting_glucose_mg_dl * fasting_insulin_mu_u_ml) / 405.0;
    let insulin_sensitivity_index = 100.0;

    let sleep_efficiency_pct = 88.0;
    let rem_sleep_pct = 22.0;
    let slow_wave_sleep_pct = 20.0;
    let sleep_latency_min = 12.0;

    println!("🕐 CIRCADIAN TIMING:");
    println!("  • Local time: {}", local_time_sf);
    println!("  • Melatonin: {:.0} pg/mL (nighttime peak)", melatonin_pg_ml);
    println!("  • Core temp: {:.1}°C (declining for sleep)", core_body_temp_c);

    println!("\n⏰ CORTISOL DIURNAL RHYTHM:");
    println!("  • 06:00 (wake): {:.0} μg/dL (morning peak)", cortisol_06h_morning_ug_dl);
    println!("  • 12:00: {:.0} μg/dL", cortisol_12h_midday_ug_dl);
    println!("  • 18:00: {:.0} μg/dL (evening decline)", cortisol_18h_evening_ug_dl);
    println!("  • 00:00: {:.0} μg/dL (nadir)", cortisol_00h_nadir_ug_dl);
    println!("  • CAR: {:.0} nmol/L (healthy awakening response)", car_nmol_l);
    println!("  • ACTH morning/evening: {:.0}/{:.0} pg/mL", acth_morning_pg_ml, acth_evening_pg_ml);
    println!("  • CRH: {:.1} ng/mL", crh_hypothalamic_ng_ml);

    println!("\n🧠 GUT-BRAIN AXIS:");
    println!("  • Gut serotonin: {:.0} ng/mL (entrained)", gut_serotonin_ng_ml);
    println!("  • Tryptophan: {:.0} μM", tryptophan_availability_um);
    println!("  • Microbiota GABA: {:.0} nM", microbiota_gaba_nm);
    println!("  • Butyrate: {:.1} mM (healthy microbiome)", scfa_butyrate_mm);
    println!("  • GLP-1: {:.0} pmol/L", glp1_fasting_pmol_l);

    println!("\n🔥 INFLAMMATORY STATUS:");
    println!("  • TNF-α: {:.1} pg/mL (baseline)", tnf_alpha_pg_ml);
    println!("  • IL-6: {:.1} pg/mL (baseline)", il6_pg_ml);
    println!("  • IL-1β: {:.0} pg/mL (baseline)", il1beta_pg_ml);
    println!("  • IL-10/TNF-α: {:.1} (homeostatic)", il10_tnf_ratio);

    println!("\n🔬 METABOLIC FUNCTION:");
    println!("  • Fasting glucose: {:.0} mg/dL", fasting_glucose_mg_dl);
    println!("  • Fasting insulin: {:.0} μU/mL", fasting_insulin_mu_u_ml);
    println!("  • HOMA-IR: {:.2} (excellent)", homa_ir);
    println!("  • Insulin sensitivity: {:.0}%", insulin_sensitivity_index);

    println!("\n💤 SLEEP QUALITY:");
    println!("  • Sleep efficiency: {:.0}%", sleep_efficiency_pct);
    println!("  • REM sleep: {:.0}%", rem_sleep_pct);
    println!("  • Slow-wave: {:.0}%", slow_wave_sleep_pct);
    println!("  • Sleep latency: {:.0} min", sleep_latency_min);

    println!("\n✓ ANALYSIS: Optimal circadian synchrony");
    println!("  All systems entrained to local zeitgebers (light/dark, meals)\n");
}

fn simulate_day1_jetlag() {
    println!("\n████████████████████████████████████████████████████████████████");
    println!("DAY 1: Acute jet lag (Tokyo arrival, 8-hour shift)");
    println!("████████████████████████████████████████████████████████████████\n");

    let local_time_tokyo = "14:00 (afternoon)";
    let body_time_sf = "22:00 (biological night)";
    let circadian_misalignment_hours = 8.0;

    let melatonin_pg_ml = 75.0;
    let core_body_temp_c = 36.4;
    let alertness_score = 35.0;

    let cortisol_current_ug_dl = 3.5;
    let expected_cortisol_ug_dl = 10.0;
    let cortisol_rhythm_amplitude = 0.4;
    let car_nmol_l = 6.0;
    let acth_pg_ml = 32.0;
    let crh_hypothalamic_ng_ml = 1.5;
    let gr_binding_affinity = 88.0;

    let gut_serotonin_ng_ml = 85.0;
    let tryptophan_availability_um = 38.0;
    let microbiota_gaba_nm = 140.0;
    let scfa_butyrate_mm = 0.7;
    let glp1_pmol_l = 12.0;
    let vagal_tone_dysregulation = 35.0;

    let tnf_alpha_pg_ml = 4.2;
    let il6_pg_ml = 2.8;
    let il1beta_pg_ml = 9.0;
    let il10_tnf_ratio = 2.4;
    let circadian_clock_gene_disruption = 60.0;

    let glucose_tolerance_pct = 78.0;
    let insulin_mu_u_ml = 8.0;
    let homa_ir = 1.68;
    let insulin_sensitivity_index = 78.0;

    let sleep_efficiency_pct = 45.0;
    let rem_sleep_pct = 8.0;
    let slow_wave_sleep_pct = 5.0;
    let sleep_latency_min = 65.0;
    let wake_after_sleep_onset_min = 95.0;

    println!("🕐 CIRCADIAN MISALIGNMENT:");
    println!("  • Tokyo time: {} | Body clock: {}", local_time_tokyo, body_time_sf);
    println!("  • Phase shift: {:.0} hours eastward", circadian_misalignment_hours);
    println!("  • Melatonin: {:.0} pg/mL ↓ (inappropriate timing)", melatonin_pg_ml);
    println!("  • Core temp: {:.1}°C (confused signal)", core_body_temp_c);
    println!("  • Alertness: {:.0}/100 ↓↓↓ (severe impairment)", alertness_score);

    println!("\n⏰ CORTISOL RHYTHM DISRUPTION:");
    println!("  • Current cortisol: {:.1} μg/dL (body thinks it's night)", cortisol_current_ug_dl);
    println!("  • Expected (Tokyo 14:00): {:.0} μg/dL → {:.0}% of expected ↓↓",
             expected_cortisol_ug_dl, (cortisol_current_ug_dl / expected_cortisol_ug_dl * 100.0));
    println!("  • Rhythm amplitude: {:.0}% of normal ↓↓↓", cortisol_rhythm_amplitude * 100.0);
    println!("  • CAR: {:.0} nmol/L ↓↓ (blunted awakening)", car_nmol_l);
    println!("  • ACTH: {:.0} pg/mL ↑ (attempting compensation)", acth_pg_ml);
    println!("  • CRH: {:.1} ng/mL ↑ (stress response)", crh_hypothalamic_ng_ml);
    println!("  • GR sensitivity: {:.0}% ↓", gr_binding_affinity);

    println!("\n🧠 GUT-BRAIN AXIS DISRUPTION:");
    println!("  • Gut serotonin: {:.0} ng/mL ↓↓ (mistimed)", gut_serotonin_ng_ml);
    println!("  • Tryptophan: {:.0} μM ↓", tryptophan_availability_um);
    println!("  • Microbiota GABA: {:.0} nM ↓↓", microbiota_gaba_nm);
    println!("  • Butyrate: {:.1} mM ↓ (gut dysregulation)", scfa_butyrate_mm);
    println!("  • GLP-1: {:.0} pmol/L ↓", glp1_pmol_l);
    println!("  • Vagal dysregulation: {:.0}% impaired", vagal_tone_dysregulation);

    println!("\n🔥 INFLAMMATORY ACTIVATION:");
    println!("  • TNF-α: {:.1} pg/mL ↑↑ (circadian stress)", tnf_alpha_pg_ml);
    println!("  • IL-6: {:.1} pg/mL ↑↑", il6_pg_ml);
    println!("  • IL-1β: {:.0} pg/mL ↑↑", il1beta_pg_ml);
    println!("  • IL-10/TNF-α: {:.1} ↓↓ (pro-inflammatory)", il10_tnf_ratio);
    println!("  • Clock gene disruption: {:.0}% ↑↑↑", circadian_clock_gene_disruption);

    println!("\n🔬 METABOLIC DYSFUNCTION:");
    println!("  • Glucose tolerance: {:.0}% ↓↓", glucose_tolerance_pct);
    println!("  • Compensatory insulin: {:.0} μU/mL ↑", insulin_mu_u_ml);
    println!("  • HOMA-IR: {:.2} ↑ (transient resistance)", homa_ir);
    println!("  • Insulin sensitivity: {:.0}% ↓↓", insulin_sensitivity_index);

    println!("\n💤 SLEEP DEVASTATION:");
    println!("  • Sleep efficiency: {:.0}% ↓↓↓ (fragmented)", sleep_efficiency_pct);
    println!("  • REM sleep: {:.0}% ↓↓↓", rem_sleep_pct);
    println!("  • Slow-wave: {:.0}% ↓↓↓", slow_wave_sleep_pct);
    println!("  • Sleep latency: {:.0} min ↑↑↑ (can't initiate)", sleep_latency_min);
    println!("  • WASO: {:.0} min ↑↑↑ (frequent awakenings)", wake_after_sleep_onset_min);

    println!("\n⚠ ANALYSIS: Severe circadian misalignment");
    println!("  Body clock out of phase with environment by 8 hours");
    println!("  Cortisol, serotonin, GABA all mistimed → mood/alertness impact");
    println!("  Inflammatory activation from circadian disruption");
    println!("  Transient insulin resistance from clock gene disruption");
    println!("  Sleep architecture severely impaired\n");
}

fn simulate_day3_jetlag() {
    println!("\n████████████████████████████████████████████████████████████════");
    println!("DAY 3: Partial adaptation (Tokyo, 48h post-arrival)");
    println!("████████████████████████████████████████████████████════════════\n");

    let local_time_tokyo = "14:00 (afternoon)";
    let body_time_sf = "00:00 (adjusted ~6h)";
    let circadian_misalignment_hours = 2.0;

    let melatonin_pg_ml = 15.0;
    let core_body_temp_c = 36.8;
    let alertness_score = 65.0;

    let cortisol_current_ug_dl = 8.0;
    let expected_cortisol_ug_dl = 10.0;
    let cortisol_rhythm_amplitude = 0.7;
    let car_nmol_l = 9.5;
    let acth_pg_ml = 28.0;
    let crh_hypothalamic_ng_ml = 1.2;
    let gr_binding_affinity = 92.0;

    let gut_serotonin_ng_ml = 105.0;
    let tryptophan_availability_um = 45.0;
    let microbiota_gaba_nm = 175.0;
    let scfa_butyrate_mm = 0.85;
    let glp1_pmol_l = 14.0;
    let vagal_tone_dysregulation = 18.0;

    let tnf_alpha_pg_ml = 3.0;
    let il6_pg_ml = 1.8;
    let il1beta_pg_ml = 6.0;
    let il10_tnf_ratio = 3.3;
    let circadian_clock_gene_disruption = 28.0;

    let glucose_tolerance_pct = 90.0;
    let insulin_mu_u_ml = 6.0;
    let homa_ir = 1.26;
    let insulin_sensitivity_index = 90.0;

    let sleep_efficiency_pct = 68.0;
    let rem_sleep_pct = 16.0;
    let slow_wave_sleep_pct = 14.0;
    let sleep_latency_min = 28.0;
    let wake_after_sleep_onset_min = 45.0;

    println!("🕐 RE-ENTRAINMENT IN PROGRESS:");
    println!("  • Tokyo time: {} | Body clock: ~{}", local_time_tokyo, body_time_sf);
    println!("  • Remaining phase shift: {:.0} hours", circadian_misalignment_hours);
    println!("  • Melatonin: {:.0} pg/mL (better timing)", melatonin_pg_ml);
    println!("  • Core temp: {:.1}°C (normalizing)", core_body_temp_c);
    println!("  • Alertness: {:.0}/100 (improved)", alertness_score);

    println!("\n⏰ CORTISOL PARTIAL RECOVERY:");
    println!("  • Current cortisol: {:.0} μg/dL", cortisol_current_ug_dl);
    println!("  • {:.0}% of expected (recovering)", (cortisol_current_ug_dl / expected_cortisol_ug_dl * 100.0));
    println!("  • Rhythm amplitude: {:.0}% ↑", cortisol_rhythm_amplitude * 100.0);
    println!("  • CAR: {:.1} nmol/L (improving)", car_nmol_l);
    println!("  • ACTH: {:.0} pg/mL (normalizing)", acth_pg_ml);
    println!("  • CRH: {:.1} ng/mL", crh_hypothalamic_ng_ml);

    println!("\n🧠 GUT-BRAIN RECOVERY:");
    println!("  • Gut serotonin: {:.0} ng/mL ↑", gut_serotonin_ng_ml);
    println!("  • Microbiota GABA: {:.0} nM ↑", microbiota_gaba_nm);
    println!("  • Butyrate: {:.2} mM (recovering)", scfa_butyrate_mm);
    println!("  • Vagal function: {:.0}% impaired (improving)", vagal_tone_dysregulation);

    println!("\n🔥 INFLAMMATION SUBSIDING:");
    println!("  • TNF-α: {:.1} pg/mL ↓", tnf_alpha_pg_ml);
    println!("  • IL-6: {:.1} pg/mL ↓", il6_pg_ml);
    println!("  • IL-10/TNF-α: {:.1} ↑ (recovering)", il10_tnf_ratio);
    println!("  • Clock gene disruption: {:.0}% ↓", circadian_clock_gene_disruption);

    println!("\n🔬 METABOLIC RECOVERY:");
    println!("  • Glucose tolerance: {:.0}% ↑", glucose_tolerance_pct);
    println!("  • Insulin: {:.0} μU/mL", insulin_mu_u_ml);
    println!("  • HOMA-IR: {:.2} (improving)", homa_ir);
    println!("  • Insulin sensitivity: {:.0}%", insulin_sensitivity_index);

    println!("\n💤 SLEEP IMPROVING:");
    println!("  • Sleep efficiency: {:.0}% ↑", sleep_efficiency_pct);
    println!("  • REM sleep: {:.0}% ↑", rem_sleep_pct);
    println!("  • Slow-wave: {:.0}% ↑", slow_wave_sleep_pct);
    println!("  • Sleep latency: {:.0} min ↓", sleep_latency_min);

    println!("\n✓ ANALYSIS: Active re-entrainment");
    println!("  ~75% adapted to Tokyo time through light exposure & meal timing");
    println!("  Cortisol rhythm re-establishing");
    println!("  Gut-brain axis recovering (serotonin, GABA production normalizing)");
    println!("  Inflammation resolving, metabolic function improving\n");
}

fn simulate_day7_recovery() {
    println!("\n████████████████████████████████████████████████████████════════");
    println!("DAY 7: Near-complete adaptation (Tokyo, 1 week)");
    println!("████████████████████████████████████████████████████████════════\n");

    let local_time_tokyo = "14:00 (afternoon)";
    let circadian_alignment_pct = 95.0;

    let melatonin_pg_ml = 8.0;
    let core_body_temp_c = 37.0;
    let alertness_score = 88.0;

    let cortisol_current_ug_dl = 9.5;
    let expected_cortisol_ug_dl = 10.0;
    let cortisol_rhythm_amplitude = 0.92;
    let car_nmol_l = 11.5;
    let acth_pg_ml = 26.0;
    let crh_hypothalamic_ng_ml = 1.0;

    let gut_serotonin_ng_ml = 118.0;
    let tryptophan_availability_um = 49.0;
    let microbiota_gaba_nm = 195.0;
    let scfa_butyrate_mm = 0.98;
    let glp1_pmol_l = 15.0;

    let tnf_alpha_pg_ml = 2.1;
    let il6_pg_ml = 1.3;
    let il1beta_pg_ml = 4.2;
    let il10_tnf_ratio = 4.8;
    let circadian_clock_gene_disruption = 5.0;

    let glucose_tolerance_pct = 98.0;
    let insulin_mu_u_ml = 5.2;
    let homa_ir = 1.09;
    let insulin_sensitivity_index = 98.0;

    let sleep_efficiency_pct = 85.0;
    let rem_sleep_pct = 21.0;
    let slow_wave_sleep_pct = 19.0;
    let sleep_latency_min = 14.0;

    println!("🕐 CIRCADIAN RE-ENTRAINMENT COMPLETE:");
    println!("  • Tokyo time: {}", local_time_tokyo);
    println!("  • Circadian alignment: {:.0}%", circadian_alignment_pct);
    println!("  • Melatonin: {:.0} pg/mL (appropriate low)", melatonin_pg_ml);
    println!("  • Core temp: {:.1}°C (normal)", core_body_temp_c);
    println!("  • Alertness: {:.0}/100 (restored)", alertness_score);

    println!("\n⏰ CORTISOL RHYTHM RESTORED:");
    println!("  • Current cortisol: {:.1} μg/dL ({:.0}% of expected)",
             cortisol_current_ug_dl, (cortisol_current_ug_dl / expected_cortisol_ug_dl * 100.0));
    println!("  • Rhythm amplitude: {:.0}% (nearly normal)", cortisol_rhythm_amplitude * 100.0);
    println!("  • CAR: {:.1} nmol/L (healthy)", car_nmol_l);
    println!("  • ACTH: {:.0} pg/mL (baseline)", acth_pg_ml);

    println!("\n🧠 GUT-BRAIN HOMEOSTASIS:");
    println!("  • Gut serotonin: {:.0} ng/mL (restored)", gut_serotonin_ng_ml);
    println!("  • Microbiota GABA: {:.0} nM (normal)", microbiota_gaba_nm);
    println!("  • Butyrate: {:.2} mM (healthy)", scfa_butyrate_mm);

    println!("\n🔥 INFLAMMATION RESOLVED:");
    println!("  • TNF-α: {:.1} pg/mL (baseline)", tnf_alpha_pg_ml);
    println!("  • IL-6: {:.1} pg/mL (baseline)", il6_pg_ml);
    println!("  • IL-10/TNF-α: {:.1} (homeostatic)", il10_tnf_ratio);
    println!("  • Clock genes: {:.0}% disruption (minimal)", circadian_clock_gene_disruption);

    println!("\n🔬 METABOLIC FUNCTION NORMALIZED:");
    println!("  • Glucose tolerance: {:.0}%", glucose_tolerance_pct);
    println!("  • Insulin: {:.1} μU/mL", insulin_mu_u_ml);
    println!("  • HOMA-IR: {:.2} (excellent)", homa_ir);
    println!("  • Insulin sensitivity: {:.0}%", insulin_sensitivity_index);

    println!("\n💤 SLEEP ARCHITECTURE RESTORED:");
    println!("  • Sleep efficiency: {:.0}%", sleep_efficiency_pct);
    println!("  • REM sleep: {:.0}%", rem_sleep_pct);
    println!("  • Slow-wave: {:.0}%", slow_wave_sleep_pct);
    println!("  • Sleep latency: {:.0} min", sleep_latency_min);

    println!("\n✓ ANALYSIS: Successful circadian adaptation");
    println!("  All systems re-entrained to Tokyo zeitgebers");
    println!("  Full recovery timeline: ~7 days for 8-hour eastward shift");
    println!("  Key factors: consistent light exposure, meal timing, sleep schedule");
    println!("  Clinical note: westward travel recovers faster (~5 days)\n");
}

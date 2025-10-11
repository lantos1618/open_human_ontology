fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  METABOLIC SYNDROME CASCADE SIMULATION                         ║");
    println!("║  From diet to inflammation to insulin resistance               ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("This simulation traces the pathophysiological cascade:");
    println!("  High-fat meal → Lipid absorption → Postprandial lipemia");
    println!("  → Oxidative stress → Inflammation → Insulin resistance\n");

    simulate_healthy_meal();
    simulate_high_fat_meal();
    simulate_chronic_overnutrition();

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  KEY INSIGHTS                                                  ║");
    println!("║                                                                ║");
    println!("║  • Postprandial lipemia drives oxidative stress                ║");
    println!("║  • IL-1β/TNF-α impair insulin receptor signaling               ║");
    println!("║  • Chronic inflammation → β-cell dysfunction                   ║");
    println!("║  • HPA axis dysregulation amplifies metabolic dysfunction      ║");
    println!("║                                                                ║");
    println!("║  Systems integrated: dietary lipids, gut-brain, inflammation,  ║");
    println!("║  insulin signaling, cortisol stress, cytokine networks         ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
}

fn simulate_healthy_meal() {
    println!("████████████████████████████████████████████████████████████████");
    println!("BASELINE: Healthy mixed meal (20g fat, 50g carbs)");
    println!("████████████████████████████████████████████████████████████████\n");

    let dietary_fat_g = 20.0;
    let postprandial_hours = 4.0;

    let pancreatic_lipase_activity: f64 = 100.0;
    let bile_salt_conc_mm: f64 = 15.0;
    let chylomicron_tg_mg_dl = 150.0 + 100.0 * (dietary_fat_g / 50.0);

    let apob48_particles_per_ml = 1e8 * (dietary_fat_g / 30.0);
    let postprandial_lipemia_auc = chylomicron_tg_mg_dl * postprandial_hours / 2.0;

    let npcil1_cholesterol_absorption_pct = 55.0;
    let cholesterol_absorbed_mg = dietary_fat_g * 2.0 * npcil1_cholesterol_absorption_pct / 100.0;

    let glp1_secretion_pmol_l = 15.0 + 25.0 * (dietary_fat_g / 40.0);
    let pyy_satiety_pmol_l = 20.0 + 30.0 * (dietary_fat_g / 40.0);
    let gut_serotonin_ng_ml = 100.0 + 50.0 * (dietary_fat_g / 40.0);
    let vagal_firing_hz = 5.0 + 3.0 * (glp1_secretion_pmol_l / 40.0);

    let plasma_glucose_mg_dl = 85.0 + 40.0;
    let insulin_secretion_mu_u_ml = 5.0 + 45.0;
    let homa_ir = (plasma_glucose_mg_dl * insulin_secretion_mu_u_ml) / 405.0;

    let tnf_alpha_pg_ml = 2.5;
    let il6_pg_ml = 1.5;
    let il1beta_pg_ml = 5.0;
    let il10_tnf_ratio = 10.0 / 2.5;

    let cortisol_morning_ug_dl = 15.0;
    let cortisol_evening_ug_dl = 5.0;
    let cortisol_awakening_response_nmol_l = 12.0;
    let acth_pg_ml = 25.0;

    println!("🍽 DIETARY LIPID METABOLISM:");
    println!("  • Pancreatic lipase: {:.0}% activity", pancreatic_lipase_activity);
    println!("  • Bile salts: {:.0} mM (optimal emulsification)", bile_salt_conc_mm);
    println!("  • Chylomicron TG: {:.0} mg/dL (normal postprandial)", chylomicron_tg_mg_dl);
    println!("  • ApoB-48: {:.1e} particles/mL", apob48_particles_per_ml);
    println!("  • Postprandial lipemia AUC: {:.0} mg·h/dL", postprandial_lipemia_auc);
    println!("  • NPC1L1 cholesterol absorption: {:.0}%", npcil1_cholesterol_absorption_pct);

    println!("\n🧠 GUT-BRAIN AXIS:");
    println!("  • GLP-1: {:.1} pmol/L (satiety signal)", glp1_secretion_pmol_l);
    println!("  • PYY: {:.0} pmol/L (appetite suppression)", pyy_satiety_pmol_l);
    println!("  • Gut serotonin: {:.0} ng/mL (90% of total)", gut_serotonin_ng_ml);
    println!("  • Vagal afferent firing: {:.1} Hz", vagal_firing_hz);

    println!("\n🔬 INSULIN SIGNALING:");
    println!("  • Fasting glucose: 85 mg/dL → Postprandial: {:.0} mg/dL", plasma_glucose_mg_dl);
    println!("  • Insulin: 5 → {:.0} μU/mL", insulin_secretion_mu_u_ml);
    println!("  • HOMA-IR: {:.2} (excellent insulin sensitivity)", homa_ir);

    println!("\n🔥 INFLAMMATORY STATUS:");
    println!("  • TNF-α: {:.1} pg/mL (baseline)", tnf_alpha_pg_ml);
    println!("  • IL-6: {:.1} pg/mL (baseline)", il6_pg_ml);
    println!("  • IL-1β: {:.0} pg/mL (baseline)", il1beta_pg_ml);
    println!("  • IL-10/TNF-α ratio: {:.1} (anti-inflammatory)", il10_tnf_ratio);

    println!("\n⏰ HPA AXIS / CORTISOL:");
    println!("  • Morning cortisol: {:.0} μg/dL", cortisol_morning_ug_dl);
    println!("  • Evening cortisol: {:.0} μg/dL (healthy circadian)", cortisol_evening_ug_dl);
    println!("  • CAR: {:.0} nmol/L", cortisol_awakening_response_nmol_l);
    println!("  • ACTH: {:.0} pg/mL", acth_pg_ml);

    println!("\n✓ ANALYSIS: Healthy metabolic state");
    println!("  Normal postprandial response, insulin sensitive, minimal inflammation\n");
}

fn simulate_high_fat_meal() {
    println!("\n████████████████████████████████████████████████████████████████");
    println!("SCENARIO 1: High-fat meal challenge (60g fat, 40g carbs)");
    println!("████████████████████████████████████████████████████████████████\n");

    let dietary_fat_g = 60.0;
    let postprandial_hours = 6.0;

    let pancreatic_lipase_activity: f64 = 100.0;
    let bile_salt_conc_mm: f64 = 20.0;
    let chylomicron_tg_mg_dl = 150.0 + 100.0 * (dietary_fat_g / 50.0);

    let apob48_particles_per_ml = 1e8 * (dietary_fat_g / 30.0);
    let postprandial_lipemia_auc = chylomicron_tg_mg_dl * postprandial_hours / 2.0;

    let npcil1_cholesterol_absorption_pct = 55.0;
    let cholesterol_absorbed_mg = dietary_fat_g * 2.0 * npcil1_cholesterol_absorption_pct / 100.0;

    let lipemia_excess: f64 = if chylomicron_tg_mg_dl > 200.0 { chylomicron_tg_mg_dl - 200.0 } else { 0.0 };
    let oxidative_stress_fold: f64 = 1.0 + lipemia_excess / 100.0;
    let ros_generation: f64 = 1.0 * oxidative_stress_fold;

    let glp1_secretion_pmol_l: f64 = 15.0 + 25.0 * (dietary_fat_g / 40.0);
    let pyy_satiety_pmol_l: f64 = 20.0 + 30.0 * (dietary_fat_g / 40.0);
    let gut_serotonin_ng_ml: f64 = 100.0 + 50.0 * (dietary_fat_g / 40.0);
    let scfa_butyrate_mm: f64 = 0.8;
    let vagal_firing_hz: f64 = 5.0 + 3.0 * (glp1_secretion_pmol_l / 40.0);

    let tnf_alpha_pg_ml: f64 = 2.5 * oxidative_stress_fold;
    let il6_pg_ml: f64 = 1.5 * oxidative_stress_fold.powf(1.2);
    let il1beta_pg_ml = 5.0 * (1.0 + (oxidative_stress_fold - 1.0) * 2.0);
    let il10_tnf_ratio = 10.0 / tnf_alpha_pg_ml;
    let ifn_gamma_pg_ml = 5.0 * oxidative_stress_fold;

    let plasma_glucose_mg_dl = 90.0 + 35.0;
    let insulin_secretion_mu_u_ml = 8.0 + 52.0;
    let insulin_receptor_phosphorylation = 100.0 - 15.0 * (tnf_alpha_pg_ml / 5.0);
    let homa_ir = (plasma_glucose_mg_dl * insulin_secretion_mu_u_ml) / 405.0;

    let cortisol_morning_ug_dl = 17.0;
    let cortisol_evening_ug_dl = 7.0;
    let cortisol_awakening_response_nmol_l = 14.0;
    let acth_pg_ml = 30.0;
    let gr_binding_affinity = 95.0;

    println!("🍽 DIETARY LIPID METABOLISM:");
    println!("  • Fat load: {:.0}g (3x baseline)", dietary_fat_g);
    println!("  • Bile salts: {:.0} mM ↑ (compensatory)", bile_salt_conc_mm);
    println!("  • Chylomicron TG: {:.0} mg/dL ↑↑ (elevated)", chylomicron_tg_mg_dl);
    println!("  • ApoB-48: {:.1e} particles/mL ↑↑", apob48_particles_per_ml);
    println!("  • Postprandial lipemia AUC: {:.0} mg·h/dL ↑↑↑", postprandial_lipemia_auc);
    println!("  • Cholesterol absorbed: {:.0} mg", cholesterol_absorbed_mg);
    println!("  • Oxidative stress: {:.2}x baseline", oxidative_stress_fold);

    println!("\n🧠 GUT-BRAIN AXIS:");
    println!("  • GLP-1: {:.1} pmol/L ↑↑ (maximal satiety)", glp1_secretion_pmol_l);
    println!("  • PYY: {:.0} pmol/L ↑↑", pyy_satiety_pmol_l);
    println!("  • Gut serotonin: {:.0} ng/mL ↑", gut_serotonin_ng_ml);
    println!("  • Vagal firing: {:.1} Hz ↑", vagal_firing_hz);
    println!("  • Butyrate: {:.1} mM (normal microbiome)", scfa_butyrate_mm);

    println!("\n🔥 INFLAMMATORY CYTOKINE NETWORK:");
    println!("  • TNF-α: {:.1} pg/mL ↑ (LPS-induced)", tnf_alpha_pg_ml);
    println!("  • IL-6: {:.1} pg/mL ↑ (acute phase)", il6_pg_ml);
    println!("  • IL-1β: {:.1} pg/mL ↑↑ (inflammasome)", il1beta_pg_ml);
    println!("  • IFN-γ: {:.1} pg/mL ↑ (Th1)", ifn_gamma_pg_ml);
    println!("  • IL-10/TNF-α: {:.2} ↓ (pro-inflammatory shift)", il10_tnf_ratio);

    println!("\n🔬 INSULIN SIGNALING:");
    println!("  • Postprandial glucose: {:.0} mg/dL", plasma_glucose_mg_dl);
    println!("  • Insulin: {:.0} μU/mL ↑ (compensatory)", insulin_secretion_mu_u_ml);
    println!("  • Insulin receptor phosphorylation: {:.0}% ↓", insulin_receptor_phosphorylation);
    println!("  • HOMA-IR: {:.2} ↑ (early insulin resistance)", homa_ir);

    println!("\n⏰ HPA AXIS / CORTISOL STRESS:");
    println!("  • Morning cortisol: {:.0} μg/dL ↑", cortisol_morning_ug_dl);
    println!("  • Evening: {:.0} μg/dL ↑ (blunted rhythm)", cortisol_evening_ug_dl);
    println!("  • CAR: {:.0} nmol/L ↑", cortisol_awakening_response_nmol_l);
    println!("  • ACTH: {:.0} pg/mL ↑", acth_pg_ml);
    println!("  • GR binding: {:.0}%", gr_binding_affinity);

    println!("\n⚠ ANALYSIS: Postprandial metabolic stress");
    println!("  High lipemia → oxidative stress → inflammation");
    println!("  TNF-α/IL-1β impair insulin signaling");
    println!("  Compensatory insulin secretion maintains glucose control");
    println!("  Recoverable with 12-16h fasting\n");
}

fn simulate_chronic_overnutrition() {
    println!("\n████████████████████████████████████████████████████████████████");
    println!("SCENARIO 2: Chronic overnutrition (months of high-fat diet)");
    println!("████████████████████████████████████████████████████████████████\n");

    let avg_dietary_fat_g = 80.0;
    let disease_duration_months = 6.0;

    let pancreatic_lipase_activity: f64 = 100.0;
    let bile_salt_conc_mm: f64 = 22.0;
    let chylomicron_tg_mg_dl = 400.0;
    let fasting_tg_mg_dl = 180.0;

    let apob48_particles_per_ml = 3.5e8;
    let remnant_clearance_pct = 60.0;

    let oxidative_stress_fold = 3.5;
    let advanced_glycation_endproducts = 2.5;

    let glp1_secretion_pmol_l = 52.0;
    let gut_serotonin_ng_ml = 190.0;
    let scfa_butyrate_mm = 0.4;
    let dysbiosis_index = 2.8;
    let vagal_firing_hz = 11.5;

    let tnf_alpha_pg_ml = 12.5;
    let il6_pg_ml = 8.2;
    let il1beta_pg_ml = 45.0;
    let il18_pg_ml = 280.0;
    let ifn_gamma_pg_ml = 18.0;
    let il17a_pg_ml = 12.0;
    let il10_tnf_ratio = 10.0 / tnf_alpha_pg_ml;
    let crp_mg_l = 8.5;

    let fasting_glucose_mg_dl = 108.0;
    let fasting_insulin_mu_u_ml = 22.0;
    let homa_ir = (fasting_glucose_mg_dl * fasting_insulin_mu_u_ml) / 405.0;
    let insulin_receptor_phosphorylation = 55.0;
    let akt_phosphorylation = 48.0;
    let glut4_translocation = 35.0;
    let beta_cell_function = 75.0;
    let hba1c_pct = 5.9;

    let cortisol_morning_ug_dl = 22.0;
    let cortisol_evening_ug_dl = 12.0;
    let cortisol_awakening_response_nmol_l = 8.0;
    let acth_pg_ml = 42.0;
    let crh_hypothalamic_ng_ml = 1.8;
    let gr_binding_affinity = 78.0;
    let gr_nuclear_translocation = 82.0;
    let hsd11b1_conversion_rate = 135.0;

    let adipose_tissue_kg = 32.0;
    let visceral_fat_pct = 35.0;
    let hepatic_steatosis_pct = 18.0;
    let alt_u_l = 48.0;

    println!("🍽 CHRONIC LIPID OVERLOAD:");
    println!("  • Average fat intake: {:.0}g/day (sustained)", avg_dietary_fat_g);
    println!("  • Fasting TG: {:.0} mg/dL ↑↑ (dyslipidemia)", fasting_tg_mg_dl);
    println!("  • Postprandial TG: {:.0} mg/dL ↑↑↑", chylomicron_tg_mg_dl);
    println!("  • ApoB-48: {:.1e} ↑↑ (impaired clearance)", apob48_particles_per_ml);
    println!("  • Remnant clearance: {:.0}% ↓↓", remnant_clearance_pct);
    println!("  • Oxidative stress: {:.1}x ↑↑↑", oxidative_stress_fold);
    println!("  • AGEs: {:.1}x baseline", advanced_glycation_endproducts);

    println!("\n🧠 GUT-BRAIN AXIS DYSREGULATION:");
    println!("  • GLP-1: {:.0} pmol/L (desensitized)", glp1_secretion_pmol_l);
    println!("  • Gut serotonin: {:.0} ng/mL", gut_serotonin_ng_ml);
    println!("  • Butyrate: {:.1} mM ↓↓ (dysbiosis)", scfa_butyrate_mm);
    println!("  • Dysbiosis index: {:.1} ↑↑", dysbiosis_index);
    println!("  • Vagal tone: {:.1} Hz (impaired)", vagal_firing_hz);

    println!("\n🔥 CHRONIC INFLAMMATION:");
    println!("  • TNF-α: {:.1} pg/mL ↑↑↑ (chronic elevation)", tnf_alpha_pg_ml);
    println!("  • IL-6: {:.1} pg/mL ↑↑↑ (persistent)", il6_pg_ml);
    println!("  • IL-1β: {:.0} pg/mL ↑↑↑ (metaflammation)", il1beta_pg_ml);
    println!("  • IL-18: {:.0} pg/mL ↑↑↑", il18_pg_ml);
    println!("  • IFN-γ: {:.0} pg/mL ↑↑ (Th1 shift)", ifn_gamma_pg_ml);
    println!("  • IL-17A: {:.0} pg/mL ↑ (Th17)", il17a_pg_ml);
    println!("  • IL-10/TNF-α: {:.2} ↓↓↓ (lost homeostasis)", il10_tnf_ratio);
    println!("  • CRP: {:.1} mg/L ↑↑ (systemic inflammation)", crp_mg_l);

    println!("\n💉 INSULIN RESISTANCE:");
    println!("  • Fasting glucose: {:.0} mg/dL ↑ (impaired fasting)", fasting_glucose_mg_dl);
    println!("  • Fasting insulin: {:.0} μU/mL ↑↑↑ (hyperinsulinemia)", fasting_insulin_mu_u_ml);
    println!("  • HOMA-IR: {:.2} ↑↑↑ (insulin resistant)", homa_ir);
    println!("  • HbA1c: {:.1}% (prediabetic)", hba1c_pct);
    println!("  • IR phosphorylation: {:.0}% ↓↓↓", insulin_receptor_phosphorylation);
    println!("  • AKT activation: {:.0}% ↓↓↓", akt_phosphorylation);
    println!("  • GLUT4 translocation: {:.0}% ↓↓↓", glut4_translocation);
    println!("  • β-cell function: {:.0}% ↓↓", beta_cell_function);

    println!("\n⏰ HPA AXIS DYSREGULATION:");
    println!("  • Morning cortisol: {:.0} μg/dL ↑↑ (elevated)", cortisol_morning_ug_dl);
    println!("  • Evening: {:.0} μg/dL ↑↑ (lost circadian rhythm)", cortisol_evening_ug_dl);
    println!("  • CAR: {:.0} nmol/L ↓↓ (blunted)", cortisol_awakening_response_nmol_l);
    println!("  • ACTH: {:.0} pg/mL ↑↑", acth_pg_ml);
    println!("  • CRH: {:.1} ng/mL ↑", crh_hypothalamic_ng_ml);
    println!("  • GR binding: {:.0}% ↓↓", gr_binding_affinity);
    println!("  • 11β-HSD1: {:.0}% ↑↑ (excess cortisol)", hsd11b1_conversion_rate);

    println!("\n🏥 METABOLIC SYNDROME FEATURES:");
    println!("  • Adipose tissue: {:.0} kg (obesity)", adipose_tissue_kg);
    println!("  • Visceral fat: {:.0}% ↑↑↑", visceral_fat_pct);
    println!("  • Hepatic steatosis: {:.0}% (NAFLD)", hepatic_steatosis_pct);
    println!("  • ALT: {:.0} U/L ↑ (liver inflammation)", alt_u_l);

    println!("\n☠ ANALYSIS: ESTABLISHED METABOLIC SYNDROME");
    println!("  • Chronic lipemia → sustained oxidative stress");
    println!("  • Cytokine storm: TNF-α/IL-1β/IL-6 block insulin signaling");
    println!("  • Compensatory hyperinsulinemia → β-cell exhaustion");
    println!("  • HPA axis dysregulation → excess cortisol → further IR");
    println!("  • Gut dysbiosis → reduced butyrate → intestinal inflammation");
    println!("  • Visceral adiposity → adipokine imbalance");
    println!("  • Trajectory: T2DM, CVD, NAFLD without intervention\n");
}

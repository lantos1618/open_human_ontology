fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║     Clinical Biomarker Analyzer - Differential Diagnosis        ║");
    println!("║   Lab Value Interpretation with Calculated Indices & Scores     ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("Patient: 48-year-old male presenting with fatigue and abdominal pain");
    println!("Duration: 3 months, progressive\n");

    println!("━━━ Complete Blood Count (CBC) ━━━\n");

    let wbc = 4.2;
    let hgb = 9.5;
    let hct = 28.5;
    let mcv = 68.0;
    let mch = 22.0;
    let mchc = 32.0;
    let rdw = 18.5;
    let platelet = 450.0;

    println!("{:>25} {:>12} {:>15} {:>15}",
             "Test", "Value", "Reference", "Interpretation");
    println!("{}", "─".repeat(70));

    let wbc_status = interpret_value(wbc, 4.5, 11.0, "");
    println!("{:>25} {:>12.1} {:>15} {:>15}", "WBC", wbc, "4.5-11.0 K/μL", wbc_status);

    let hgb_status = if hgb < 13.5 { "↓ Low" } else if hgb > 17.5 { "↑ High" } else { "Normal" };
    println!("{:>25} {:>12.1} {:>15} {:>15}", "Hemoglobin", hgb, "13.5-17.5 g/dL", hgb_status);

    let hct_status = if hct < 41.0 { "↓ Low" } else if hct > 50.0 { "↑ High" } else { "Normal" };
    println!("{:>25} {:>12.1} {:>15} {:>15}", "Hematocrit", hct, "41-50%", hct_status);

    let mcv_status = if mcv < 80.0 { "↓ Microcytic" } else if mcv > 100.0 { "↑ Macrocytic" } else { "Normocytic" };
    println!("{:>25} {:>12.1} {:>15} {:>15}", "MCV", mcv, "80-100 fL", mcv_status);

    let mch_status = if mch < 27.0 { "↓ Low" } else if mch > 31.0 { "↑ High" } else { "Normal" };
    println!("{:>25} {:>12.1} {:>15} {:>15}", "MCH", mch, "27-31 pg", mch_status);

    let mchc_status = if mchc < 32.0 { "↓ Hypochromic" } else if mchc > 36.0 { "↑ High" } else { "Normochromic" };
    println!("{:>25} {:>12.1} {:>15} {:>15}", "MCHC", mchc, "32-36 g/dL", mchc_status);

    let rdw_status = if rdw > 14.5 { "↑ High" } else { "Normal" };
    println!("{:>25} {:>12.1} {:>15} {:>15}", "RDW", rdw, "11.5-14.5%", rdw_status);

    let plt_status = if platelet < 150.0 { "↓ Low" } else if platelet > 400.0 { "↑ High" } else { "Normal" };
    println!("{:>25} {:>12.0} {:>15} {:>15}", "Platelets", platelet, "150-400 K/μL", plt_status);

    println!("\n🔍 CBC Pattern Recognition:");
    println!("   • Microcytic anemia (MCV 68, normal 80-100)");
    println!("   • Hypochromic RBCs (low MCH)");
    println!("   • High RDW (anisocytosis - varying cell sizes)");
    println!("   • Reactive thrombocytosis (platelets 450K)");
    println!("   → Classic for: Iron deficiency or chronic disease");

    println!("\n━━━ Iron Studies ━━━\n");

    let serum_iron = 35.0;
    let tibc = 450.0;
    let ferritin = 8.0;

    println!("{:>25} {:>12} {:>15} {:>15}",
             "Test", "Value", "Reference", "Interpretation");
    println!("{}", "─".repeat(70));

    let iron_status = if serum_iron < 60.0 { "↓ Low" } else if serum_iron > 170.0 { "↑ High" } else { "Normal" };
    println!("{:>25} {:>12.0} {:>15} {:>15}", "Serum Iron", serum_iron, "60-170 μg/dL", iron_status);

    let tibc_status = if tibc > 370.0 { "↑ High" } else if tibc < 250.0 { "↓ Low" } else { "Normal" };
    println!("{:>25} {:>12.0} {:>15} {:>15}", "TIBC", tibc, "250-370 μg/dL", tibc_status);

    let transferrin_saturation = (serum_iron / tibc) * 100.0;
    let tsat_status = if transferrin_saturation < 20.0 { "↓ Low" } else if transferrin_saturation > 50.0 { "↑ High" } else { "Normal" };
    println!("{:>25} {:>12.1} {:>15} {:>15}", "Transferrin Saturation", transferrin_saturation, "20-50%", tsat_status);

    let ferritin_status = if ferritin < 30.0 { "↓ Depleted" } else if ferritin > 300.0 { "↑ High" } else { "Normal" };
    println!("{:>25} {:>12.0} {:>15} {:>15}", "Ferritin", ferritin, "30-300 ng/mL", ferritin_status);

    println!("\n🔍 Iron Study Interpretation:");
    println!("   Equation: Transferrin Saturation = (Serum Iron / TIBC) × 100");
    println!("   Calculated TSAT = ({:.0} / {:.0}) × 100 = {:.1}%", serum_iron, tibc, transferrin_saturation);
    println!("\n   Pattern:");
    println!("   • ↓ Serum iron (35, normal 60-170)");
    println!("   • ↑ TIBC (450, normal 250-370) - body trying to capture more iron");
    println!("   • ↓ Transferrin saturation (7.8%, normal >20%)");
    println!("   • ↓ Ferritin (8, normal >30) - depleted iron stores");
    println!("   → DIAGNOSTIC: Iron deficiency anemia");

    println!("\n━━━ Comprehensive Metabolic Panel (CMP) ━━━\n");

    let glucose = 105.0;
    let bun = 28.0;
    let creatinine = 1.4;
    let sodium = 142.0;
    let potassium = 4.2;
    let chloride = 105.0;
    let co2 = 24.0;
    let calcium = 9.8;
    let albumin = 3.2;
    let total_protein = 6.8;
    let ast = 85.0;
    let alt = 120.0;
    let alk_phos = 145.0;
    let total_bilirubin = 1.8;
    let direct_bilirubin = 0.9;

    println!("{:>25} {:>12} {:>15} {:>15}",
             "Test", "Value", "Reference", "Interpretation");
    println!("{}", "─".repeat(70));

    println!("{:>25} {:>12.0} {:>15} {:>15}", "Glucose", glucose, "70-100 mg/dL",
             if glucose > 125.0 { "↑ Diabetic" } else if glucose > 100.0 { "↑ Prediabetic" } else { "Normal" });

    println!("{:>25} {:>12.0} {:>15} {:>15}", "BUN", bun, "7-20 mg/dL",
             if bun > 20.0 { "↑ Elevated" } else { "Normal" });

    println!("{:>25} {:>12.1} {:>15} {:>15}", "Creatinine", creatinine, "0.7-1.3 mg/dL",
             if creatinine > 1.3 { "↑ Elevated" } else { "Normal" });

    let bun_cr_ratio = bun / creatinine;
    println!("{:>25} {:>12.0} {:>15} {:>15}", "BUN/Cr Ratio", bun_cr_ratio, "10-20",
             if bun_cr_ratio > 20.0 { "↑ Prerenal" } else { "Normal" });

    let egfr = calculate_egfr(creatinine, 48, true, false);
    println!("{:>25} {:>12.0} {:>15} {:>15}", "eGFR (CKD-EPI)", egfr, ">60 mL/min",
             if egfr < 60.0 { "↓ CKD" } else if egfr < 90.0 { "Mild ↓" } else { "Normal" });

    println!("{:>25} {:>12.0} {:>15} {:>15}", "Sodium", sodium, "136-145 mEq/L", "Normal");
    println!("{:>25} {:>12.1} {:>15} {:>15}", "Potassium", potassium, "3.5-5.0 mEq/L", "Normal");
    println!("{:>25} {:>12.0} {:>15} {:>15}", "Chloride", chloride, "98-107 mEq/L", "Normal");
    println!("{:>25} {:>12.0} {:>15} {:>15}", "CO2", co2, "22-29 mEq/L", "Normal");
    println!("{:>25} {:>12.1} {:>15} {:>15}", "Calcium", calcium, "8.5-10.5 mg/dL", "Normal");

    println!("{:>25} {:>12.1} {:>15} {:>15}", "Albumin", albumin, "3.5-5.0 g/dL",
             if albumin < 3.5 { "↓ Low" } else { "Normal" });

    println!("{:>25} {:>12.1} {:>15} {:>15}", "Total Protein", total_protein, "6.0-8.0 g/dL", "Normal");

    let globulin = total_protein - albumin;
    let ag_ratio = albumin / globulin;
    println!("{:>25} {:>12.1} {:>15} {:>15}", "Globulin (calc)", globulin, "2.0-3.5 g/dL", "Normal");
    println!("{:>25} {:>12.2} {:>15} {:>15}", "A/G Ratio", ag_ratio, "1.0-2.5", "Normal");

    println!("\n🔍 Renal Function:");
    println!("   CKD-EPI eGFR equation (simplified):");
    println!("   eGFR = 141 × min(Cr/κ, 1)^α × max(Cr/κ, 1)^(-1.209) × 0.993^Age");
    println!("   where κ=0.9 (male), α=-0.411 (male)");
    println!("   eGFR = {:.0} mL/min/1.73m² → {}", egfr,
             if egfr < 60.0 { "Stage 3 CKD" } else { "Normal" });

    println!("\n━━━ Liver Function Tests ━━━\n");

    println!("{:>25} {:>12} {:>15} {:>15}",
             "Test", "Value", "Reference", "Interpretation");
    println!("{}", "─".repeat(70));

    let ast_status = if ast > 40.0 { "↑ Elevated" } else { "Normal" };
    println!("{:>25} {:>12.0} {:>15} {:>15}", "AST", ast, "10-40 U/L", ast_status);

    let alt_status = if alt > 40.0 { "↑ Elevated" } else { "Normal" };
    println!("{:>25} {:>12.0} {:>15} {:>15}", "ALT", alt, "10-40 U/L", alt_status);

    let ast_alt_ratio = ast / alt;
    println!("{:>25} {:>12.2} {:>15} {:>15}", "AST/ALT Ratio", ast_alt_ratio, "<1.0",
             if ast_alt_ratio > 2.0 { "Alcohol?" } else if ast_alt_ratio > 1.0 { "Cirrhosis?" } else { "Hepatitis?" });

    let alk_phos_status = if alk_phos > 120.0 { "↑ Elevated" } else { "Normal" };
    println!("{:>25} {:>12.0} {:>15} {:>15}", "Alkaline Phosphatase", alk_phos, "30-120 U/L", alk_phos_status);

    let tbili_status = if total_bilirubin > 1.2 { "↑ Elevated" } else { "Normal" };
    println!("{:>25} {:>12.1} {:>15} {:>15}", "Total Bilirubin", total_bilirubin, "0.1-1.2 mg/dL", tbili_status);

    let dbili_status = if direct_bilirubin > 0.3 { "↑ Elevated" } else { "Normal" };
    println!("{:>25} {:>12.1} {:>15} {:>15}", "Direct Bilirubin", direct_bilirubin, "0.0-0.3 mg/dL", dbili_status);

    let indirect_bilirubin = total_bilirubin - direct_bilirubin;
    println!("{:>25} {:>12.1} {:>15} {:>15}", "Indirect Bilirubin", indirect_bilirubin, "0.1-0.9 mg/dL", "Normal");

    let direct_fraction = (direct_bilirubin / total_bilirubin) * 100.0;
    println!("{:>25} {:>12.0} {:>15} {:>15}", "Direct Fraction %", direct_fraction, "<20%",
             if direct_fraction > 20.0 { "Conjugated" } else { "Unconjugated" });

    println!("\n🔍 Liver Pattern Analysis:");
    println!("   AST/ALT Ratio = {:.2}", ast_alt_ratio);
    println!("   Interpretation:");
    println!("   • AST/ALT < 1: Suggests viral hepatitis or NAFLD");
    println!("   • AST/ALT 1-2: Suggests chronic liver disease");
    println!("   • AST/ALT > 2: Suggests alcoholic liver disease");
    println!("\n   Bilirubin fractionation:");
    println!("   Direct Bilirubin = {:.1} mg/dL ({:.0}% of total)", direct_bilirubin, direct_fraction);
    println!("   • Direct >20%: Conjugated hyperbilirubinemia (cholestasis, hepatitis)");
    println!("   • Direct <20%: Unconjugated (hemolysis, Gilbert's)");
    println!("\n   Current pattern:");
    println!("   • ↑ ALT > ↑ AST (ratio 0.71) → Hepatocellular injury");
    println!("   • ↑ Alk Phos → Cholestatic component");
    println!("   • ↑ Direct bilirubin (50% of total) → Cholestasis");

    println!("\n━━━ Calculated Clinical Scores ━━━\n");

    let apri = (ast / 40.0) / (platelet / 100.0);
    println!("APRI Score (AST-to-Platelet Ratio Index):");
    println!("  Formula: (AST / ULN) / (Platelet count × 10^9/L) × 100");
    println!("  APRI = ({:.0}/40) / ({:.0}/100) = {:.3}", ast, platelet, apri);
    println!("  Interpretation:");
    println!("    <0.5: Low fibrosis probability");
    println!("    0.5-1.5: Indeterminate");
    println!("    >1.5: High fibrosis probability");
    println!("  This patient: {:.3} → {}", apri,
             if apri > 1.5 { "Significant fibrosis likely" } else if apri > 0.5 { "Indeterminate" } else { "Low fibrosis risk" });

    println!("\nFIB-4 Index (Age, AST, ALT, Platelet):");
    let fib4 = calculate_fib4(48, ast, alt, platelet);
    println!("  Formula: (Age × AST) / (Platelet × √ALT)");
    println!("  FIB-4 = (48 × {:.0}) / ({:.0} × √{:.0}) = {:.2}", ast, platelet, alt, fib4);
    println!("  Interpretation:");
    println!("    <1.45: F0-F1 (no/mild fibrosis)");
    println!("    1.45-3.25: Indeterminate");
    println!("    >3.25: F3-F4 (advanced fibrosis/cirrhosis)");
    println!("  This patient: {:.2} → {}", fib4,
             if fib4 > 3.25 { "Advanced fibrosis" } else if fib4 > 1.45 { "Indeterminate" } else { "Minimal fibrosis" });

    println!("\nMELD Score (Model for End-Stage Liver Disease):");
    let meld = calculate_meld(creatinine, total_bilirubin, 1.2);
    println!("  Formula: 3.78×ln(bilirubin) + 11.2×ln(INR) + 9.57×ln(creatinine) + 6.43");
    println!("  MELD = {:.0}", meld);
    println!("  Interpretation:");
    println!("    <10: 1.9% 3-month mortality");
    println!("    10-19: 6.0% 3-month mortality");
    println!("    20-29: 19.6% 3-month mortality");
    println!("    30-39: 52.6% 3-month mortality");
    println!("    >40: 71.3% 3-month mortality");
    println!("  This patient: {:.0} → Low-risk", meld);

    println!("\n━━━ Differential Diagnosis ━━━\n");

    println!("Primary diagnosis: IRON DEFICIENCY ANEMIA with LIVER DISEASE");
    println!("\nSupporting findings:");
    println!("  1. Iron deficiency anemia:");
    println!("     ✓ Microcytic, hypochromic anemia (MCV 68, MCH 22)");
    println!("     ✓ Low iron (35), high TIBC (450), low ferritin (8)");
    println!("     ✓ Transferrin saturation 7.8% (depleted iron)");
    println!("     ✓ Reactive thrombocytosis (450K)");
    println!("\n  2. Chronic liver disease:");
    println!("     ✓ Hepatocellular pattern (AST 85, ALT 120)");
    println!("     ✓ Cholestatic features (Alk Phos 145, direct bili 0.9)");
    println!("     ✓ Hypoalbuminemia (3.2) - synthetic dysfunction");
    println!("     ✓ Mild azotemia (BUN 28, Cr 1.4) - hepatorenal?");

    println!("\nDifferential for iron deficiency:");
    println!("  • GI blood loss (most common in men) →");
    println!("    - Peptic ulcer disease (NSAIDs?)");
    println!("    - Colorectal cancer (age appropriate)");
    println!("    - Esophageal varices (from portal hypertension?)");
    println!("  • Malabsorption (celiac, H. pylori gastritis)");
    println!("  • Dietary insufficiency (less likely in adults)");

    println!("\nDifferential for liver disease:");
    println!("  • Non-alcoholic fatty liver disease (NAFLD)");
    println!("  • Alcoholic liver disease (need history)");
    println!("  • Viral hepatitis (B, C)");
    println!("  • Hemochromatosis (paradoxical with iron deficiency)");
    println!("  • Autoimmune hepatitis");

    println!("\n━━━ Recommended Workup ━━━\n");

    println!("Immediate:");
    println!("  □ Upper endoscopy (EGD) - evaluate for varices, ulcers");
    println!("  □ Colonoscopy - screen for colon cancer");
    println!("  □ Hepatitis panel (HBsAg, HCV Ab)");
    println!("  □ Abdominal ultrasound - evaluate liver morphology, portal vein");
    println!("  □ INR/PT - assess hepatic synthetic function");

    println!("\nFollow-up:");
    println!("  □ Iron supplementation: Ferrous sulfate 325mg TID");
    println!("  □ Repeat CBC in 4-6 weeks (expect Hgb ↑ 1g/dL)");
    println!("  □ Liver biopsy if viral/autoimmune serologies negative");
    println!("  □ Esophagogastroduodenoscopy for variceal screening if cirrhosis");

    println!("\n━━━ Clinical Pearls ━━━");
    println!("\n✓ Iron deficiency in adult men → GI blood loss until proven otherwise");
    println!("✓ Microcytic anemia DDx: TAILS (Thalassemia, Anemia of chronic disease, Iron deficiency, Lead poisoning, Sideroblastic)");
    println!("✓ AST/ALT ratio: Mnemonic \"Alcoholics Smash Two (glasses), thus AST is 2× ALT\"");
    println!("✓ Ferritin is acute phase reactant → may be falsely normal in iron deficiency + inflammation");
    println!("✓ TIBC ↑ in iron deficiency (body trying to capture more iron)");
    println!("✓ TIBC ↓ in chronic disease (functional iron deficiency)");
    println!("✓ RDW elevated in iron deficiency (variable cell sizes during recovery)");
    println!("\nReferences:");
    println!("  - Harrison's Principles of Internal Medicine (21st ed)");
    println!("  - UpToDate: Approach to the adult with anemia");
    println!("  - Wai CT et al. Hepatology 2003;38:518-526 (FIB-4)");
    println!("  - Sterling RK et al. Clin Gastro Hepatol 2006;4:1214-1220 (APRI)\n");
}

fn interpret_value(value: f64, low: f64, high: f64, _unit: &str) -> &'static str {
    if value < low {
        "↓ Low"
    } else if value > high {
        "↑ High"
    } else {
        "Normal"
    }
}

fn calculate_egfr(creatinine: f64, age: i32, is_male: bool, is_black: bool) -> f64 {
    let kappa = if is_male { 0.9 } else { 0.7 };
    let alpha = if is_male { -0.411 } else { -0.329 };
    let sex_factor = if is_male { 1.0 } else { 1.018 };
    let race_factor = if is_black { 1.159 } else { 1.0 };

    let cr_ratio = creatinine / kappa;
    let min_term = cr_ratio.min(1.0_f64).powf(alpha);
    let max_term = cr_ratio.max(1.0_f64).powf(-1.209);
    let age_term = 0.993_f64.powf(age as f64);

    141.0 * min_term * max_term * age_term * sex_factor * race_factor
}

fn calculate_fib4(age: i32, ast: f64, alt: f64, platelet: f64) -> f64 {
    (age as f64 * ast) / (platelet * alt.sqrt())
}

fn calculate_meld(creatinine: f64, bilirubin: f64, inr: f64) -> f64 {
    let meld = 3.78 * bilirubin.ln() + 11.2 * inr.ln() + 9.57 * creatinine.ln() + 6.43;
    meld.max(1.0).round()
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║   Renin-Angiotensin-Aldosterone System (RAAS): BP Regulation    ║");
    println!("║     Baroreceptor Reflex + Hormonal Cascade + Renal Sodium       ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("━━━ RAAS Mathematical Model ━━━");
    println!("Multi-tier feedback system integrating neural and hormonal control:\n");

    println!("Baroreceptor reflex (fast, seconds-minutes):");
    println!("  dHR/dt = k₁·(BP_target - BP) - k₂·HR");
    println!("  dSVR/dt = k₃·(BP_target - BP) - k₄·SVR\n");

    println!("RAAS cascade (slow, hours-days):");
    println!("  dRenin/dt = k₅·(1 - BP/BP_target) - k₆·Renin");
    println!("  dAng2/dt = k₇·Renin - k₈·Ang2");
    println!("  dAldosterone/dt = k₉·Ang2 - k₁₀·Aldosterone");
    println!("  dNa_retention/dt = k₁₁·Aldosterone - k₁₂·Na_retention\n");

    println!("Blood pressure equation:");
    println!("  BP = CO × SVR");
    println!("  where CO = HR × SV × (1 + volume_effect)\n");

    println!("Where:");
    println!("  BP = blood pressure (mmHg)");
    println!("  HR = heart rate (bpm)");
    println!("  SVR = systemic vascular resistance (mmHg·min/L)");
    println!("  Renin = plasma renin activity (ng/mL/hr)");
    println!("  Ang2 = angiotensin II (pg/mL)");
    println!("  Aldosterone = serum aldosterone (ng/dL)\n");

    struct RAASModel {
        bp: f64,
        hr: f64,
        sv: f64,
        svr: f64,
        renin: f64,
        angiotensin2: f64,
        aldosterone: f64,
        sodium_retention: f64,
        blood_volume: f64,
        bp_target: f64,
        k1: f64,
        k2: f64,
        k3: f64,
        k4: f64,
        k5: f64,
        k6: f64,
        k7: f64,
        k8: f64,
        k9: f64,
        k10: f64,
        k11: f64,
        k12: f64,
        time: f64,
    }

    impl RAASModel {
        fn new_healthy() -> Self {
            Self {
                bp: 120.0,
                hr: 70.0,
                sv: 70.0,
                svr: 1.0,
                renin: 1.5,
                angiotensin2: 15.0,
                aldosterone: 10.0,
                sodium_retention: 0.0,
                blood_volume: 5.0,
                bp_target: 120.0,
                k1: 0.5,
                k2: 0.3,
                k3: 0.008,
                k4: 0.05,
                k5: 0.15,
                k6: 0.12,
                k7: 12.0,
                k8: 0.8,
                k9: 1.5,
                k10: 0.25,
                k11: 0.02,
                k12: 0.015,
                time: 0.0,
            }
        }

        fn new_hypertensive() -> Self {
            let mut model = Self::new_healthy();
            model.bp = 160.0;
            model.svr = 1.35;
            model.renin = 2.5;
            model.angiotensin2 = 28.0;
            model.aldosterone = 18.0;
            model.sodium_retention = 1.5;
            model.bp_target = 120.0;
            model
        }

        fn new_heart_failure() -> Self {
            let mut model = Self::new_healthy();
            model.bp = 95.0;
            model.hr = 95.0;
            model.sv = 45.0;
            model.renin = 8.5;
            model.angiotensin2 = 65.0;
            model.aldosterone = 35.0;
            model.sodium_retention = 3.0;
            model.blood_volume = 6.5;
            model
        }

        fn step(&mut self, dt: f64, ace_inhibitor: f64, arb: f64, diuretic: f64, beta_blocker: f64) {
            let bp_error = self.bp_target - self.bp;

            let d_hr = self.k1 * bp_error - self.k2 * (self.hr - 70.0) - beta_blocker * 20.0;
            let d_svr = self.k3 * bp_error - self.k4 * (self.svr - 1.0);

            let renal_perfusion_pressure = self.bp / 120.0;
            let renin_stimulus = if renal_perfusion_pressure < 0.9 {
                self.k5 * (1.0 - renal_perfusion_pressure)
            } else {
                0.0
            };
            let d_renin = renin_stimulus - self.k6 * (self.renin - 1.5);

            let ace_activity = 1.0 - ace_inhibitor;
            let d_ang2 = self.k7 * self.renin * ace_activity - self.k8 * (self.angiotensin2 - 15.0);

            let ang2_effect = if arb > 0.5 {
                self.angiotensin2 * 0.2
            } else {
                self.angiotensin2
            };

            let d_aldo = self.k9 * (ang2_effect / 15.0) - self.k10 * (self.aldosterone - 10.0);

            let d_na_retention = self.k11 * self.aldosterone - self.k12 * self.sodium_retention
                               - diuretic * 0.5;

            self.hr += d_hr * dt;
            self.svr += d_svr * dt;
            self.renin += d_renin * dt;
            self.angiotensin2 += d_ang2 * dt;
            self.aldosterone += d_aldo * dt;
            self.sodium_retention += d_na_retention * dt;

            self.hr = self.hr.max(40.0).min(160.0);
            self.svr = self.svr.max(0.5).min(2.0);
            self.renin = self.renin.max(0.1).min(20.0);
            self.angiotensin2 = self.angiotensin2.max(5.0).min(150.0);
            self.aldosterone = self.aldosterone.max(2.0).min(80.0);
            self.sodium_retention = self.sodium_retention.max(-2.0).min(5.0);

            let svr_from_ang2 = 1.0 + (self.angiotensin2 - 15.0) * 0.008;
            self.svr = self.svr * 0.7 + svr_from_ang2 * 0.3;

            self.blood_volume = 5.0 + self.sodium_retention * 0.25;

            let co = (self.hr * self.sv * (1.0 + (self.blood_volume - 5.0) * 0.08)) / 1000.0;
            self.bp = co * self.svr * 20.0;

            self.time += dt;
        }

        fn mean_arterial_pressure(&self) -> f64 {
            self.bp * 0.8 + 20.0
        }

        fn cardiac_output(&self) -> f64 {
            (self.hr * self.sv * (1.0 + (self.blood_volume - 5.0) * 0.08)) / 1000.0
        }

        fn interpret_bp(&self) -> (&str, &str) {
            let stage = match self.bp as i32 {
                0..=119 => "Normal",
                120..=129 => "Elevated",
                130..=139 => "Stage 1 HTN",
                140..=179 => "Stage 2 HTN",
                _ => "Hypertensive crisis",
            };

            let recommendation = match stage {
                "Normal" => "Continue healthy lifestyle",
                "Elevated" => "Lifestyle modifications",
                "Stage 1 HTN" => "Lifestyle + consider single agent",
                "Stage 2 HTN" => "Lifestyle + dual therapy (ACE-i/ARB + CCB/diuretic)",
                "Hypertensive crisis" => "Emergency evaluation, IV antihypertensives",
                _ => "Monitor",
            };

            (stage, recommendation)
        }
    }

    println!("━━━ Scenario 1: Acute Hemorrhage - Compensatory Response ━━━\n");
    println!("Simulating 1L blood loss (20% of circulating volume):");
    println!("  Immediate: Baroreceptor reflex (↑HR, ↑SVR)");
    println!("  Minutes: Sympathetic activation");
    println!("  Hours: RAAS activation (renin → Ang2 → aldosterone)\n");

    let mut hemorrhage = RAASModel::new_healthy();
    hemorrhage.blood_volume = 4.0;

    println!("{:>6} {:>8} {:>8} {:>8} {:>10} {:>12} {:>12} {:>15}",
             "Time", "BP", "HR", "CO", "Renin", "Ang2", "Aldo", "Status");
    println!("{:>6} {:>8} {:>8} {:>8} {:>10} {:>12} {:>12} {:>15}",
             "(min)", "(mmHg)", "(bpm)", "(L/min)", "(ng/mL/hr)", "(pg/mL)", "(ng/dL)", "");
    println!("{}", "─".repeat(95));

    for min in (0..=120).step_by(10) {
        if min % 10 == 0 {
            let (stage, _) = hemorrhage.interpret_bp();
            println!("{:>6} {:>8.0} {:>8.0} {:>8.2} {:>10.1} {:>12.1} {:>12.1} {:>15}",
                     min, hemorrhage.bp, hemorrhage.hr, hemorrhage.cardiac_output(),
                     hemorrhage.renin, hemorrhage.angiotensin2, hemorrhage.aldosterone, stage);
        }
        hemorrhage.step(1.0, 0.0, 0.0, 0.0, 0.0);
    }

    println!("\n  0-5 min: Baroreceptor reflex (↑HR from 70→95 bpm, ↑SVR)");
    println!("  10-30 min: Renin release from kidneys (↓ renal perfusion)");
    println!("  30-60 min: Ang2 rises → vasoconstriction + aldosterone secretion");
    println!("  60-120 min: Na+ retention begins → plasma volume restoration\n");

    println!("━━━ Scenario 2: Essential Hypertension - RAAS Overactivity ━━━\n");
    println!("Chronic elevated BP with inappropriate RAAS activation:");
    println!("  Baseline BP: 160/100 mmHg (Stage 2)");
    println!("  Elevated renin despite high BP (failed feedback)");
    println!("  ↑ Ang2 → vasoconstriction + Na+ retention\n");

    let mut hypertension = RAASModel::new_hypertensive();

    println!("{:>6} {:>8} {:>8} {:>10} {:>12} {:>12} {:>12}",
             "Time", "BP", "SVR", "Renin", "Ang2", "Aldo", "Na+ ret");
    println!("{:>6} {:>8} {:>8} {:>10} {:>12} {:>12} {:>12}",
             "(hr)", "(mmHg)", "(rel)", "(ng/mL/hr)", "(pg/mL)", "(ng/dL)", "(mEq)");
    println!("{}", "─".repeat(80));

    for hr in (0..=168).step_by(12) {
        if hr % 12 == 0 {
            println!("{:>6} {:>8.0} {:>8.2} {:>10.1} {:>12.1} {:>12.1} {:>12.2}",
                     hr, hypertension.bp, hypertension.svr, hypertension.renin,
                     hypertension.angiotensin2, hypertension.aldosterone,
                     hypertension.sodium_retention);
        }
        for _ in 0..60 {
            hypertension.step(1.0, 0.0, 0.0, 0.0, 0.0);
        }
    }

    println!("\n  Pathophysiology: Vicious cycle");
    println!("  ↑ BP → vascular remodeling → ↑ SVR → ↑ BP");
    println!("  RAAS fails to suppress despite ↑ BP");
    println!("  Target organ damage: LVH, nephrosclerosis, retinopathy\n");

    println!("━━━ Scenario 3: Heart Failure - Neurohormonal Activation ━━━\n");
    println!("Reduced cardiac output triggers compensatory mechanisms:");
    println!("  ↓ CO → ↓ renal perfusion → ↑↑ renin");
    println!("  Compensatory initially, maladaptive chronically");
    println!("  Goal: Support CO but causes fluid overload\n");

    let mut hf = RAASModel::new_heart_failure();

    println!("{:>6} {:>8} {:>8} {:>8} {:>10} {:>12} {:>12} {:>12}",
             "Time", "BP", "HR", "CO", "Renin", "Ang2", "Aldo", "Volume");
    println!("{:>6} {:>8} {:>8} {:>8} {:>10} {:>12} {:>12} {:>12}",
             "(hr)", "(mmHg)", "(bpm)", "(L/min)", "(ng/mL/hr)", "(pg/mL)", "(ng/dL)", "(L)");
    println!("{}", "─".repeat(95));

    for hr in (0..=72).step_by(6) {
        if hr % 6 == 0 {
            println!("{:>6} {:>8.0} {:>8.0} {:>8.2} {:>10.1} {:>12.1} {:>12.1} {:>12.2}",
                     hr, hf.bp, hf.hr, hf.cardiac_output(),
                     hf.renin, hf.angiotensin2, hf.aldosterone, hf.blood_volume);
        }
        for _ in 0..60 {
            hf.step(1.0, 0.0, 0.0, 0.0, 0.0);
        }
    }

    println!("\n  Markedly elevated renin (>8 ng/mL/hr, normal 0.5-3.3)");
    println!("  Very high Ang2 and aldosterone");
    println!("  Volume overload → pulmonary edema, peripheral edema");
    println!("  Clinical: Dyspnea, orthopnea, elevated JVP, rales\n");

    println!("━━━ Scenario 4: ACE Inhibitor Treatment (Lisinopril) ━━━\n");
    println!("Mechanism: Blocks angiotensin-converting enzyme (ACE)");
    println!("  Prevents Ang1 → Ang2 conversion");
    println!("  ↓ Ang2 → ↓ vasoconstriction, ↓ aldosterone");
    println!("  ↓ afterload and preload → improved cardiac function\n");

    let mut ace_treatment = RAASModel::new_hypertensive();

    println!("{:>6} {:>10} {:>8} {:>8} {:>10} {:>12} {:>12}",
             "Day", "ACE-i", "BP", "SVR", "Renin", "Ang2", "Aldo");
    println!("{:>6} {:>10} {:>8} {:>8} {:>10} {:>12} {:>12}",
             "", "(dose)", "(mmHg)", "(rel)", "(ng/mL/hr)", "(pg/mL)", "(ng/dL)");
    println!("{}", "─".repeat(75));

    for day in 0..=28 {
        let ace_inhibitor = if day >= 1 { 0.85 } else { 0.0 };
        let dose_mg = if day >= 1 { 20.0 } else { 0.0 };

        if day % 2 == 0 {
            println!("{:>6} {:>10.0} {:>8.0} {:>8.2} {:>10.1} {:>12.1} {:>12.1}",
                     day, dose_mg, ace_treatment.bp, ace_treatment.svr,
                     ace_treatment.renin, ace_treatment.angiotensin2, ace_treatment.aldosterone);
        }

        for _ in 0..(24 * 60) {
            ace_treatment.step(1.0, ace_inhibitor, 0.0, 0.0, 0.0);
        }
    }

    println!("\n  Day 0: Baseline hypertension (160/100 mmHg)");
    println!("  Day 1-3: Ang2 drops rapidly, BP begins to decline");
    println!("  Day 7-14: BP stabilizes ~130/80 mmHg (Stage 1 → Normal)");
    println!("  Day 14-28: Continued benefit, vascular remodeling reverses");
    println!("  Renin rises (compensatory, loss of Ang2 negative feedback)\n");

    println!("━━━ Scenario 5: Combination Therapy (ACE-i + Diuretic) ━━━\n");
    println!("Heart failure treatment - dual RAAS blockade:");
    println!("  ACE inhibitor: ↓ afterload (↓ Ang2, ↓ SVR)");
    println!("  Loop diuretic: ↓ preload (↓ volume, ↓ pulmonary edema)");
    println!("  Synergistic effect on BP and volume\n");

    let mut combo_treatment = RAASModel::new_heart_failure();

    println!("{:>6} {:>8} {:>8} {:>8} {:>8} {:>10} {:>12}",
             "Day", "BP", "HR", "CO", "Volume", "Renin", "Status");
    println!("{:>6} {:>8} {:>8} {:>8} {:>8} {:>10} {:>12}",
             "", "(mmHg)", "(bpm)", "(L/min)", "(L)", "(ng/mL/hr)", "");
    println!("{}", "─".repeat(75));

    for day in 0..=21 {
        let ace_inhibitor = if day >= 1 { 0.80 } else { 0.0 };
        let diuretic = if day >= 1 { 0.65 } else { 0.0 };

        if day % 3 == 0 {
            let status = if combo_treatment.blood_volume > 6.0 {
                "Volume overload"
            } else if combo_treatment.blood_volume > 5.5 {
                "Improving"
            } else if combo_treatment.blood_volume <= 5.3 {
                "Euvolemic"
            } else {
                "Compensated"
            };

            println!("{:>6} {:>8.0} {:>8.0} {:>8.2} {:>8.2} {:>10.1} {:>12}",
                     day, combo_treatment.bp, combo_treatment.hr, combo_treatment.cardiac_output(),
                     combo_treatment.blood_volume, combo_treatment.renin, status);
        }

        for _ in 0..(24 * 60) {
            combo_treatment.step(1.0, ace_inhibitor, 0.0, diuretic, 0.0);
        }
    }

    println!("\n  Day 0-3: Diuresis begins, volume decreases");
    println!("  Day 3-7: BP rises toward normal as afterload ↓");
    println!("  Day 7-14: CO improves, symptoms resolve");
    println!("  Day 14-21: Hemodynamic stability achieved\n");

    println!("━━━ Scenario 6: Beta-Blocker in Heart Failure ━━━\n");
    println!("Paradox: Negative inotrope improves heart failure");
    println!("  Mechanism: ↓ HR → ↑ diastolic filling time");
    println!("  Blocks sympathetic overdrive (maladaptive)");
    println!("  Start low, titrate slowly (\"start low, go slow\")\n");

    let mut beta_blocker_hf = RAASModel::new_heart_failure();

    println!("{:>6} {:>12} {:>8} {:>8} {:>8} {:>12}",
             "Week", "Dose (mg)", "HR", "BP", "CO", "Clinical");
    println!("{}", "─".repeat(60));

    for week in 0..=12 {
        let beta_blocker = if week == 0 {
            0.0
        } else if week <= 2 {
            0.15
        } else if week <= 4 {
            0.30
        } else if week <= 8 {
            0.45
        } else {
            0.60
        };

        let dose_mg = if week == 0 {
            0.0
        } else if week <= 2 {
            3.125
        } else if week <= 4 {
            6.25
        } else if week <= 8 {
            12.5
        } else {
            25.0
        };

        let status = if beta_blocker_hf.hr > 90.0 {
            "Tachycardic"
        } else if beta_blocker_hf.hr > 70.0 {
            "Improving"
        } else {
            "Target HR"
        };

        println!("{:>6} {:>12} {:>8.0} {:>8.0} {:>8.2} {:>12}",
                 week, dose_mg, beta_blocker_hf.hr, beta_blocker_hf.bp,
                 beta_blocker_hf.cardiac_output(), status);

        for _ in 0..(7 * 24 * 60) {
            beta_blocker_hf.step(1.0, 0.0, 0.0, 0.0, beta_blocker);
        }
    }

    println!("\n  Week 0-2: Start carvedilol 3.125 mg BID, HR ↓ gradually");
    println!("  Week 2-4: Uptitrate to 6.25 mg BID");
    println!("  Week 4-8: Uptitrate to 12.5 mg BID");
    println!("  Week 8-12: Target dose 25 mg BID (HR ~60-70 bpm)");
    println!("  Benefit: ↓ mortality by 35% (MERIT-HF, COPERNICUS trials)\n");

    println!("━━━ Pharmacology of RAAS Inhibitors ━━━\n");

    println!("ACE Inhibitors (Lisinopril, Enalapril):");
    println!("  Mechanism: Block ACE → ↓ Ang2, ↑ bradykinin");
    println!("  Benefits: ↓ BP, ↓ LV remodeling, ↓ proteinuria");
    println!("  Side effects: Dry cough (10-20%, ↑ bradykinin), hyperkalemia, AKI");
    println!("  Contraindications: Pregnancy, bilateral RAS, angioedema history\n");

    println!("Angiotensin Receptor Blockers (Losartan, Valsartan):");
    println!("  Mechanism: Block AT1 receptor (Ang2 receptor)");
    println!("  Benefits: Similar to ACE-i, no cough");
    println!("  Use: ACE-i intolerant patients\n");

    println!("Aldosterone Antagonists (Spironolactone, Eplerenone):");
    println!("  Mechanism: Block mineralocorticoid receptor");
    println!("  Benefits: K+-sparing, anti-fibrotic");
    println!("  Use: Resistant HTN, HF with EF ≤35%");
    println!("  Monitor: K+ (risk of hyperkalemia with ACE-i/ARB)\n");

    println!("Diuretics (Furosemide, HCTZ):");
    println!("  Loop: Block Na-K-2Cl in thick ascending limb");
    println!("  Thiazide: Block NaCl in DCT");
    println!("  Benefits: ↓ preload, ↓ BP");
    println!("  Side effects: Hypokalemia, hyperuricemia, AKI\n");

    println!("━━━ Clinical Guidelines (JNC 8, ACC/AHA 2017) ━━━\n");

    println!("{:>25} {:>15} {:>30}",
             "Condition", "BP Target", "First-line Therapy");
    println!("{}", "─".repeat(75));
    println!("{:>25} {:>15} {:>30}",
             "General population", "<140/90", "ACE-i/ARB/CCB/diuretic");
    println!("{:>25} {:>15} {:>30}",
             "Diabetes/CKD", "<130/80", "ACE-i or ARB (preferred)");
    println!("{:>25} {:>15} {:>30}",
             "Heart failure (HFrEF)", "Treat to SBP<130", "ACE-i + BB + diuretic");
    println!("{:>25} {:>15} {:>30}",
             "Post-MI", "<130/80", "ACE-i + BB (mandatory)");
    println!("{:>25} {:>15} {:>30}",
             "Resistant HTN", "<130/80", "3 drugs + spironolactone");

    println!("\n━━━ Advanced Concepts ━━━\n");

    println!("Pressure-Natriuresis Relationship:");
    println!("  ↑ BP → ↑ renal perfusion → ↑ Na+ excretion");
    println!("  Guyton model: BP set-point determined by kidney function");
    println!("  Chronic HTN → rightward shift of curve (impaired natriuresis)\n");

    println!("RAAS Escape:");
    println!("  Despite continuous Ang2, kidneys eventually excrete Na+");
    println!("  Prevents indefinite volume expansion");
    println!("  Mechanism: ANP, pressure-natriuresis override aldosterone\n");

    println!("Angiotensin 1-7 (Counter-regulatory):");
    println!("  ACE2 converts Ang2 → Ang 1-7");
    println!("  Effects: Vasodilation, anti-fibrotic (opposite of Ang2)");
    println!("  COVID-19: SARS-CoV-2 binds ACE2 → ↓ Ang 1-7 pathway\n");

    println!("Tissue RAAS:");
    println!("  Local Ang2 production in heart, vessels, kidneys");
    println!("  Autocrine/paracrine effects independent of circulating RAAS");
    println!("  Drives tissue remodeling, fibrosis\n");

    println!("━━━ Validation ━━━");
    println!("✓ Normal BP: 120/80 mmHg, MAP ~93 mmHg");
    println!("✓ Normal renin: 0.5-3.3 ng/mL/hr (upright)");
    println!("✓ Normal aldosterone: 4-31 ng/dL (upright)");
    println!("✓ ACE inhibitor effect: 10-15% BP reduction");
    println!("  Whelton PK et al. J Am Coll Cardiol 2018;71:e127-e248");
    println!("✓ Heart failure neurohormones: Renin 5-20× normal");
    println!("  Packer M et al. N Engl J Med 2001;344:1651-1658");
    println!("✓ Beta-blocker mortality benefit: 35% RRR");
    println!("  MERIT-HF. Lancet 1999;353:2001-2007");

    println!("\nReferences:");
    println!("  - Whelton PK et al. J Am Coll Cardiol 2018;71:e127-e248 (2017 HTN Guidelines)");
    println!("  - Yancy CW et al. J Am Coll Cardiol 2017;70:776-803 (HF Guidelines)");
    println!("  - Guyton AC. Annu Rev Physiol 1972;34:13-46 (Pressure-natriuresis)");
    println!("  - Packer M et al. N Engl J Med 2001;344:1651-1658 (Carvedilol HF trial)\n");
}

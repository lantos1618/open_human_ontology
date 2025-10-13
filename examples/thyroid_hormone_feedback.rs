fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║   Hypothalamic-Pituitary-Thyroid (HPT) Axis: Feedback Control   ║");
    println!("║     TRH → TSH → T4 → T3 with Negative Feedback                  ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("━━━ HPT Axis Mathematical Model ━━━");
    println!("Three-tier negative feedback system:\n");
    println!("  dTRH/dt = k₁ - k₂·TRH - k₃·TRH·T3");
    println!("  dTSH/dt = k₄·TRH - k₅·TSH - k₆·TSH·T4");
    println!("  dT4/dt = k₇·TSH - k₈·T4 - k₉·T4");
    println!("  dT3/dt = k₉·T4 - k₁₀·T3\n");

    println!("Where:");
    println!("  TRH = thyrotropin-releasing hormone (hypothalamus, ng/L)");
    println!("  TSH = thyroid-stimulating hormone (pituitary, mIU/L)");
    println!("  T4 = thyroxine (thyroid, μg/dL)");
    println!("  T3 = triiodothyronine (peripheral conversion, ng/dL)");
    println!("  k₁ = TRH synthesis rate");
    println!("  k₂ = TRH degradation");
    println!("  k₃ = T3 inhibition of TRH (negative feedback)");
    println!("  k₄ = TSH synthesis stimulated by TRH");
    println!("  k₅ = TSH clearance");
    println!("  k₆ = T4 inhibition of TSH (negative feedback)");
    println!("  k₇ = T4 synthesis stimulated by TSH");
    println!("  k₈ = T4 clearance");
    println!("  k₉ = T4→T3 conversion (5'-deiodinase)");
    println!("  k₁₀ = T3 clearance\n");

    struct ThyroidAxis {
        trh: f64,
        tsh: f64,
        t4: f64,
        t3: f64,
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
        time: f64,
    }

    impl ThyroidAxis {
        fn new_healthy() -> Self {
            Self {
                trh: 5.0,
                tsh: 2.0,
                t4: 8.0,
                t3: 120.0,
                k1: 0.5,
                k2: 0.08,
                k3: 0.002,
                k4: 0.6,
                k5: 0.15,
                k6: 0.01,
                k7: 0.8,
                k8: 0.05,
                k9: 0.025,
                k10: 0.08,
                time: 0.0,
            }
        }

        fn new_primary_hypothyroid() -> Self {
            let mut axis = Self::new_healthy();
            axis.k7 = 0.15;
            axis.tsh = 15.0;
            axis.t4 = 3.5;
            axis.t3 = 60.0;
            axis
        }

        fn new_hashimotos_early() -> Self {
            let mut axis = Self::new_healthy();
            axis.k7 = 0.4;
            axis.tsh = 6.5;
            axis.t4 = 6.0;
            axis.t3 = 95.0;
            axis
        }

        fn new_secondary_hypothyroid() -> Self {
            let mut axis = Self::new_healthy();
            axis.k4 = 0.15;
            axis.tsh = 0.5;
            axis.t4 = 4.0;
            axis.t3 = 70.0;
            axis
        }

        fn new_hyperthyroid_graves() -> Self {
            let mut axis = Self::new_healthy();
            axis.k7 = 2.5;
            axis.tsh = 0.01;
            axis.t4 = 18.0;
            axis.t3 = 280.0;
            axis
        }

        fn step(&mut self, dt: f64) {
            let d_trh = self.k1 - self.k2 * self.trh - self.k3 * self.trh * self.t3;
            let d_tsh = self.k4 * self.trh - self.k5 * self.tsh - self.k6 * self.tsh * self.t4;
            let d_t4 = self.k7 * self.tsh - self.k8 * self.t4 - self.k9 * self.t4;
            let d_t3 = self.k9 * self.t4 - self.k10 * self.t3;

            self.trh += d_trh * dt;
            self.tsh += d_tsh * dt;
            self.t4 += d_t4 * dt;
            self.t3 += d_t3 * dt;

            self.trh = self.trh.max(0.1);
            self.tsh = self.tsh.max(0.01);
            self.t4 = self.t4.max(0.5);
            self.t3 = self.t3.max(10.0);

            self.time += dt;
        }

        fn interpret_status(&self) -> (&str, &str) {
            let diagnosis = match (self.tsh, self.t4, self.t3) {
                (tsh, t4, _) if tsh > 4.5 && t4 < 5.0 => "Primary hypothyroidism",
                (tsh, t4, _) if tsh > 4.5 && t4 >= 5.0 && t4 < 7.5 => "Subclinical hypothyroidism",
                (tsh, _, _) if tsh < 0.4 => "Secondary hypothyroidism / TSH deficiency",
                (tsh, t4, _) if tsh < 0.1 && t4 > 12.0 => "Hyperthyroidism (Graves' or toxic nodule)",
                (tsh, t4, t3) if tsh >= 0.4 && tsh <= 4.5 && t4 >= 5.0 && t4 <= 12.0 && t3 >= 80.0 && t3 <= 200.0 => "Euthyroid (normal)",
                _ => "Borderline / Indeterminate",
            };

            let recommendation = match diagnosis {
                "Primary hypothyroidism" => "Levothyroxine replacement (1.6 μg/kg/day)",
                "Subclinical hypothyroidism" => "Monitor, consider treatment if TSH >10 or symptoms",
                "Secondary hypothyroidism / TSH deficiency" => "Pituitary MRI, endocrine consult",
                "Hyperthyroidism (Graves' or toxic nodule)" => "Methimazole, beta-blocker, RAI therapy",
                "Euthyroid (normal)" => "No treatment needed",
                _ => "Repeat testing in 4-6 weeks",
            };

            (diagnosis, recommendation)
        }

        fn free_t4_index(&self) -> f64 {
            self.t4 * 0.85
        }

        fn t3_uptake_ratio(&self) -> f64 {
            self.t3 / (self.t4 * 15.0)
        }
    }

    println!("━━━ Scenario 1: Healthy Individual (Euthyroid) ━━━\n");
    println!("Baseline steady-state with intact feedback:");
    println!("  TSH: 0.4-4.5 mIU/L (normal)");
    println!("  Free T4: 5-12 μg/dL (normal)");
    println!("  Total T3: 80-200 ng/dL (normal)\n");

    let mut healthy = ThyroidAxis::new_healthy();

    println!("{:>6} {:>8} {:>8} {:>8} {:>10} {:>20}",
             "Time", "TRH", "TSH", "T4", "T3", "Status");
    println!("{:>6} {:>8} {:>8} {:>8} {:>10} {:>20}",
             "(hr)", "(ng/L)", "(mIU/L)", "(μg/dL)", "(ng/dL)", "");
    println!("{}", "─".repeat(70));

    for i in 0..=120 {
        if i % 10 == 0 {
            let (status, _) = healthy.interpret_status();
            println!("{:>6} {:>8.2} {:>8.2} {:>8.2} {:>10.1} {:>20}",
                     i, healthy.trh, healthy.tsh, healthy.t4, healthy.t3, status);
        }
        healthy.step(1.0);
    }

    println!("\n  Negative feedback maintains homeostasis");
    println!("  When T4 ↑ → TSH ↓ (pituitary feedback)");
    println!("  When T3 ↑ → TRH ↓ (hypothalamic feedback)\n");

    println!("━━━ Scenario 2: Primary Hypothyroidism (Hashimoto's Thyroiditis) ━━━\n");
    println!("Thyroid gland dysfunction:");
    println!("  Autoimmune destruction → ↓ T4 synthesis (k₇ reduced 81%)");
    println!("  Compensatory ↑ TSH (feedback attempts to restore T4)");
    println!("  Treatment: Levothyroxine replacement\n");

    let mut hypothyroid = ThyroidAxis::new_primary_hypothyroid();

    println!("{:>6} {:>8} {:>8} {:>8} {:>10} {:>25}",
             "Time", "TSH", "T4", "T3", "FT4 Index", "Status");
    println!("{}", "─".repeat(75));

    for i in 0..=120 {
        if i % 10 == 0 {
            let (status, _) = hypothyroid.interpret_status();
            let fti = hypothyroid.free_t4_index();
            println!("{:>6} {:>8.2} {:>8.2} {:>10.1} {:>10.2} {:>25}",
                     i, hypothyroid.tsh, hypothyroid.t4, hypothyroid.t3, fti, status);
        }
        hypothyroid.step(1.0);
    }

    println!("\n  TSH remains elevated (>10 mIU/L) despite ↑ TRH");
    println!("  T4 and T3 low → symptoms: fatigue, weight gain, cold intolerance");
    println!("  Diagnosis: TSH >4.5 with low T4 (<5 μg/dL)");
    println!("  Treatment goal: TSH 0.5-2.5 mIU/L with levothyroxine\n");

    println!("━━━ Scenario 3: Subclinical Hypothyroidism (Early Hashimoto's) ━━━\n");
    println!("Mild thyroid dysfunction:");
    println!("  Slightly ↓ T4 synthesis (k₇ reduced 50%)");
    println!("  TSH elevated but T4 still in lower-normal range");
    println!("  Patient may be asymptomatic or have subtle symptoms\n");

    let mut subclinical = ThyroidAxis::new_hashimotos_early();

    println!("{:>6} {:>8} {:>8} {:>8} {:>10} {:>30}",
             "Time", "TSH", "T4", "T3", "T3/T4", "Status");
    println!("{}", "─".repeat(80));

    for i in 0..=120 {
        if i % 10 == 0 {
            let (status, _) = subclinical.interpret_status();
            let ratio = subclinical.t3_uptake_ratio();
            println!("{:>6} {:>8.2} {:>8.2} {:>10.1} {:>10.3} {:>30}",
                     i, subclinical.tsh, subclinical.t4, subclinical.t3, ratio, status);
        }
        subclinical.step(1.0);
    }

    println!("\n  TSH: 5-10 mIU/L (mildly elevated)");
    println!("  T4: Still within reference range (5-7 μg/dL)");
    println!("  Clinical dilemma: Treat or observe?");
    println!("  Guidelines: Consider treatment if TSH >10, symptomatic, or TPO+ antibodies\n");

    println!("━━━ Scenario 4: Secondary Hypothyroidism (Pituitary Failure) ━━━\n");
    println!("Pituitary dysfunction:");
    println!("  ↓ TSH synthesis (k₄ reduced 75%)");
    println!("  Low TSH → Low T4 (but TSH doesn't rise appropriately)");
    println!("  Cause: Pituitary tumor, Sheehan syndrome, radiation\n");

    let mut secondary = ThyroidAxis::new_secondary_hypothyroid();

    println!("{:>6} {:>8} {:>8} {:>8} {:>10} {:>30}",
             "Time", "TRH", "TSH", "T4", "T3", "Status");
    println!("{}", "─".repeat(80));

    for i in 0..=120 {
        if i % 10 == 0 {
            let (status, _) = secondary.interpret_status();
            println!("{:>6} {:>8.2} {:>8.2} {:>8.2} {:>10.1} {:>30}",
                     i, secondary.trh, secondary.tsh, secondary.t4, secondary.t3, status);
        }
        secondary.step(1.0);
    }

    println!("\n  Key finding: Low TSH with low T4 (paradoxical)");
    println!("  TRH elevated but TSH fails to respond → pituitary defect");
    println!("  Diagnosis: TSH <0.4 with low T4");
    println!("  Treatment: Levothyroxine (dose guided by free T4, not TSH)\n");

    println!("━━━ Scenario 5: Hyperthyroidism (Graves' Disease) ━━━\n");
    println!("Autoimmune thyroid stimulation:");
    println!("  TSH receptor antibodies (TRAb) → uncontrolled T4 synthesis");
    println!("  ↑↑ T4 and T3 → profound TSH suppression via feedback");
    println!("  Symptoms: Weight loss, anxiety, tremor, palpitations\n");

    let mut graves = ThyroidAxis::new_hyperthyroid_graves();

    println!("{:>6} {:>8} {:>8} {:>8} {:>10} {:>30}",
             "Time", "TSH", "T4", "T3", "T3/T4", "Status");
    println!("{}", "─".repeat(80));

    for i in 0..=120 {
        if i % 10 == 0 {
            let (status, _) = graves.interpret_status();
            let ratio = graves.t3_uptake_ratio();
            println!("{:>6} {:>8.3} {:>8.2} {:>10.1} {:>10.3} {:>30}",
                     i, graves.tsh, graves.t4, graves.t3, ratio, status);
        }
        graves.step(1.0);
    }

    println!("\n  TSH suppressed (<0.01 mIU/L) due to negative feedback");
    println!("  T4 >12 μg/dL, T3 >200 ng/dL (elevated)");
    println!("  T3/T4 ratio elevated (increased peripheral conversion)");
    println!("  Treatment: Methimazole 10-40 mg/day, propranolol for symptoms\n");

    println!("━━━ Levothyroxine Treatment Simulation ━━━\n");
    println!("Treating primary hypothyroidism:");
    println!("  Starting dose: 100 μg daily (1.6 μg/kg for 62.5 kg patient)");
    println!("  Titrate based on TSH at 6-8 weeks");
    println!("  Goal: TSH 0.5-2.5 mIU/L, free T4 in mid-normal range\n");

    let mut treated = ThyroidAxis::new_primary_hypothyroid();

    println!("{:>6} {:>10} {:>8} {:>8} {:>10} {:>20}",
             "Time", "LT4 dose", "TSH", "T4", "T3", "Clinical");
    println!("{:>6} {:>10} {:>8} {:>8} {:>10} {:>20}",
             "(days)", "(μg)", "(mIU/L)", "(μg/dL)", "(ng/dL)", "Response");
    println!("{}", "─".repeat(75));

    for day in (0..=84).step_by(7) {
        let lt4_dose = if day >= 7 {
            100.0
        } else {
            0.0
        };

        let lt4_effect = if day >= 7 {
            (100.0 / 100.0) * 0.015
        } else {
            0.0
        };

        if day % 7 == 0 {
            let response = if treated.tsh > 10.0 {
                "Undertreated"
            } else if treated.tsh > 2.5 {
                "Improving"
            } else if treated.tsh >= 0.5 {
                "Therapeutic"
            } else {
                "Overreplaced"
            };

            println!("{:>6} {:>10.0} {:>8.2} {:>8.2} {:>10.1} {:>20}",
                     day, lt4_dose, treated.tsh, treated.t4, treated.t3, response);
        }

        for _ in 0..24 {
            treated.k7 = 0.15 + lt4_effect;
            treated.step(1.0);
        }
    }

    println!("\n  Week 0-1: No treatment, persistent hypothyroidism");
    println!("  Week 2-4: TSH begins to decline, T4 rising");
    println!("  Week 6-8: Check TSH to assess dose adequacy");
    println!("  Week 12: Re-check if dose adjusted, then annually\n");

    println!("━━━ Clinical Interpretation Guidelines ━━━\n");

    println!("{:>25} {:>12} {:>15} {:>20}",
             "Condition", "TSH", "Free T4", "Diagnosis");
    println!("{}", "─".repeat(75));
    println!("{:>25} {:>12} {:>15} {:>20}",
             "Primary hypothyroid", ">4.5", "Low (<5)", "Hashimoto's, iodine deficiency");
    println!("{:>25} {:>12} {:>15} {:>20}",
             "Subclinical hypothyroid", "4.5-10", "Normal", "Early thyroid failure");
    println!("{:>25} {:>12} {:>15} {:>20}",
             "Secondary hypothyroid", "<0.4", "Low", "Pituitary adenoma");
    println!("{:>25} {:>12} {:>15} {:>20}",
             "Primary hyperthyroid", "<0.1", "High (>12)", "Graves', toxic nodule");
    println!("{:>25} {:>12} {:>15} {:>20}",
             "Subclinical hyperthyroid", "<0.4", "Normal", "Early hyperthyroidism");
    println!("{:>25} {:>12} {:>15} {:>20}",
             "Euthyroid", "0.4-4.5", "5-12", "Normal");

    println!("\n━━━ Pharmacology of Thyroid Medications ━━━\n");

    println!("Levothyroxine (T4 replacement):");
    println!("  Half-life: 7 days → Steady-state in 4-6 weeks");
    println!("  Absorption: 60-80%, take on empty stomach");
    println!("  Conversion: T4 → T3 via 5'-deiodinase (peripheral tissues)");
    println!("  Bioequivalence: Stick to same brand (±12.5% variation)\n");

    println!("Liothyronine (T3 replacement):");
    println!("  Half-life: 1 day (shorter → more fluctuations)");
    println!("  Use: Severe hypothyroid myxedema coma");
    println!("  Not preferred for chronic therapy (unstable levels)\n");

    println!("Methimazole (antithyroid drug):");
    println!("  Mechanism: Inhibits thyroid peroxidase (TPO)");
    println!("  Blocks T4 synthesis, doesn't affect TSH");
    println!("  Onset: 4-8 weeks (existing T4 must clear first)");
    println!("  Side effects: Agranulocytosis (rare, 0.3%)\n");

    println!("━━━ Advanced Concepts ━━━\n");

    println!("Set-point Regulation:");
    println!("  TSH set-point varies individually (genetic)");
    println!("  Optimal TSH for one person may be 1.5, another 3.0");
    println!("  Goal: Restore to patient's baseline, not lab reference\n");

    println!("Non-Thyroidal Illness Syndrome (Euthyroid Sick):");
    println!("  Critical illness → ↓ T3 (↓ conversion), ↓ TSH");
    println!("  T4 may be normal or low");
    println!("  Mechanism: Adaptive response to conserve energy");
    println!("  Management: Usually no treatment, resolve with recovery\n");

    println!("T3 Toxicosis:");
    println!("  Isolated T3 elevation with normal/high T4");
    println!("  ↑ Peripheral conversion (T4→T3)");
    println!("  Seen in: Graves' disease, toxic adenoma\n");

    println!("Thyroid Hormone Resistance:");
    println!("  Genetic: TRβ mutations → ↓ receptor sensitivity");
    println!("  Phenotype: ↑ TSH, ↑ T4, ↑ T3 (but no hyperthyroid symptoms)");
    println!("  Diagnosis: Molecular testing for TRβ mutations\n");

    println!("━━━ Validation ━━━");
    println!("✓ Normal ranges: TSH 0.4-4.5 mIU/L, T4 5-12 μg/dL, T3 80-200 ng/dL");
    println!("  American Thyroid Association guidelines 2014");
    println!("✓ Primary hypothyroid: TSH >10 with low T4");
    println!("  Prevalence: 4.6% (women), 0.5% (men) in US");
    println!("✓ Levothyroxine dosing: 1.6 μg/kg/day for replacement");
    println!("  Jonklaas J et al. Thyroid 2014;24:1670-1751");
    println!("✓ Feedback model: Based on Dietrich et al. homeostatic model");
    println!("  Dietrich JW et al. Front Endocrinol 2012;3:40");

    println!("\nReferences:");
    println!("  - Ross DS et al. Thyroid 2016;26:1343-1421 (ATA Hypothyroidism Guidelines)");
    println!("  - Bahn RS et al. Thyroid 2011;21:593-646 (ATA Hyperthyroidism Guidelines)");
    println!("  - Dietrich JW et al. Front Endocrinol 2012;3:40 (HPT Axis Model)");
    println!("  - Jonklaas J et al. Thyroid 2014;24:1670-1751 (Levothyroxine Guidelines)\n");
}

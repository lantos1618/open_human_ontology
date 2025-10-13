fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║   Calcium Homeostasis: PTH-Vitamin D-Calcitonin Feedback Loop   ║");
    println!("║     Bone-Kidney-GI Integration with Endocrine Control           ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("━━━ Calcium Homeostasis Mathematical Model ━━━");
    println!("Three-hormone feedback system regulating serum calcium:\n");

    println!("Parathyroid hormone (PTH) response:");
    println!("  dPTH/dt = k₁·(Ca_target - Ca_serum)² - k₂·PTH");
    println!("  (inverse sigmoidal relationship)\n");

    println!("Vitamin D activation:");
    println!("  d[1,25(OH)₂D]/dt = k₃·PTH - k₄·[1,25(OH)₂D]");
    println!("  (PTH stimulates 1α-hydroxylase in kidney)\n");

    println!("Calcitonin secretion:");
    println!("  dCalcitonin/dt = k₅·(Ca_serum - Ca_target) - k₆·Calcitonin");
    println!("  (activated when Ca high)\n");

    println!("Calcium fluxes:");
    println!("  dCa_serum/dt = GI_absorption + Bone_resorption - Renal_excretion - Bone_deposition");
    println!("  where:");
    println!("    GI_absorption = k₇·[1,25(OH)₂D]·Dietary_Ca");
    println!("    Bone_resorption = k₈·PTH");
    println!("    Bone_deposition = k₉·Calcitonin");
    println!("    Renal_excretion = k₁₀·Ca_serum·(1 - PTH_effect)\n");

    println!("Where:");
    println!("  Ca_serum = serum calcium (mg/dL)");
    println!("  PTH = parathyroid hormone (pg/mL)");
    println!("  1,25(OH)₂D = calcitriol (pg/mL)");
    println!("  Calcitonin = serum calcitonin (pg/mL)\n");

    struct CalciumHomeostasis {
        ca_serum: f64,
        pth: f64,
        calcitriol: f64,
        calcitonin: f64,
        bone_mass: f64,
        dietary_ca: f64,
        ca_target: f64,
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

    impl CalciumHomeostasis {
        fn new_healthy() -> Self {
            Self {
                ca_serum: 9.5,
                pth: 35.0,
                calcitriol: 45.0,
                calcitonin: 5.0,
                bone_mass: 1000.0,
                dietary_ca: 1000.0,
                ca_target: 9.5,
                k1: 80.0,
                k2: 0.15,
                k3: 0.8,
                k4: 0.25,
                k5: 12.0,
                k6: 0.8,
                k7: 0.00035,
                k8: 0.025,
                k9: 0.015,
                k10: 0.12,
                time: 0.0,
            }
        }

        fn new_primary_hyperparathyroid() -> Self {
            let mut model = Self::new_healthy();
            model.ca_serum = 11.2;
            model.pth = 95.0;
            model.calcitriol = 75.0;
            model.bone_mass = 850.0;
            model
        }

        fn new_hypoparathyroid() -> Self {
            let mut model = Self::new_healthy();
            model.ca_serum = 7.5;
            model.pth = 8.0;
            model.calcitriol = 18.0;
            model.k1 = 0.0;
            model
        }

        fn new_vitamin_d_deficiency() -> Self {
            let mut model = Self::new_healthy();
            model.ca_serum = 8.8;
            model.pth = 72.0;
            model.calcitriol = 15.0;
            model.k3 = 0.15;
            model.dietary_ca = 600.0;
            model
        }

        fn new_ckd_stage4() -> Self {
            let mut model = Self::new_healthy();
            model.ca_serum = 8.3;
            model.pth = 185.0;
            model.calcitriol = 12.0;
            model.k3 = 0.08;
            model.k10 = 0.04;
            model
        }

        fn step(&mut self, dt: f64, vit_d_supplement: f64, calcium_supplement: f64) {
            let ca_error = self.ca_target - self.ca_serum;

            let pth_stimulus = if ca_error > 0.0 {
                self.k1 * ca_error * ca_error
            } else {
                -self.k1 * ca_error.abs().powf(1.5) * 0.5
            };
            let d_pth = pth_stimulus - self.k2 * (self.pth - 35.0);

            let d_calcitriol = self.k3 * (self.pth / 35.0).min(4.0) - self.k4 * (self.calcitriol - 45.0)
                             + vit_d_supplement * 0.5;

            let calcitonin_stimulus = if self.ca_serum > self.ca_target {
                self.k5 * (self.ca_serum - self.ca_target)
            } else {
                0.0
            };
            let d_calcitonin = calcitonin_stimulus - self.k6 * (self.calcitonin - 5.0);

            let gi_absorption = self.k7 * self.calcitriol * (self.dietary_ca + calcium_supplement);
            let bone_resorption = self.k8 * (self.pth / 35.0);
            let bone_deposition = self.k9 * (self.calcitonin / 5.0);
            let pth_renal_effect = (self.pth / 100.0).min(1.0);
            let renal_excretion = self.k10 * self.ca_serum * (1.0 - pth_renal_effect * 0.7);

            let d_ca = gi_absorption + bone_resorption - bone_deposition - renal_excretion;
            let d_bone = bone_deposition - bone_resorption;

            self.pth += d_pth * dt;
            self.calcitriol += d_calcitriol * dt;
            self.calcitonin += d_calcitonin * dt;
            self.ca_serum += d_ca * dt;
            self.bone_mass += d_bone * dt;

            self.pth = self.pth.max(5.0).min(500.0);
            self.calcitriol = self.calcitriol.max(8.0).min(150.0);
            self.calcitonin = self.calcitonin.max(2.0).min(100.0);
            self.ca_serum = self.ca_serum.max(6.0).min(14.0);
            self.bone_mass = self.bone_mass.max(400.0).min(1200.0);

            self.time += dt;
        }

        fn ionized_calcium(&self) -> f64 {
            self.ca_serum * 0.5
        }

        fn phosphate_estimate(&self) -> f64 {
            let base_phos = 3.5;
            base_phos - (self.pth - 35.0) * 0.008
        }

        fn interpret_status(&self) -> (&str, &str) {
            let diagnosis = match (self.ca_serum, self.pth) {
                (ca, pth) if ca > 10.5 && pth > 65.0 => "Primary hyperparathyroidism",
                (ca, pth) if ca > 10.5 && pth < 20.0 => "Hypercalcemia of malignancy",
                (ca, pth) if ca > 10.5 => "Hypercalcemia (other)",
                (ca, pth) if ca < 8.5 && pth > 65.0 => "Secondary hyperparathyroidism (Vit D def/CKD)",
                (ca, pth) if ca < 8.5 && pth < 15.0 => "Hypoparathyroidism",
                (ca, pth) if ca >= 8.5 && ca <= 10.5 && pth >= 15.0 && pth <= 65.0 => "Normal calcium homeostasis",
                _ => "Indeterminate",
            };

            let recommendation = match diagnosis {
                "Primary hyperparathyroidism" => "Parathyroidectomy if symptomatic",
                "Hypercalcemia of malignancy" => "Treat underlying cancer, bisphosphonates",
                "Secondary hyperparathyroidism (Vit D def/CKD)" => "Vitamin D replacement, phosphate binders",
                "Hypoparathyroidism" => "Calcium + calcitriol supplementation",
                "Normal calcium homeostasis" => "No treatment needed",
                _ => "Further workup",
            };

            (diagnosis, recommendation)
        }
    }

    println!("━━━ Scenario 1: Healthy Calcium Homeostasis ━━━\n");
    println!("Baseline steady-state with intact feedback:");
    println!("  Serum Ca: 8.5-10.5 mg/dL (normal)");
    println!("  PTH: 15-65 pg/mL (normal)");
    println!("  1,25(OH)₂D: 20-60 pg/mL (normal)\n");

    let mut healthy = CalciumHomeostasis::new_healthy();

    println!("{:>6} {:>10} {:>10} {:>12} {:>12} {:>12} {:>20}",
             "Time", "Ca", "PTH", "Calcitriol", "Calcitonin", "Bone", "Status");
    println!("{:>6} {:>10} {:>10} {:>12} {:>12} {:>12} {:>20}",
             "(hr)", "(mg/dL)", "(pg/mL)", "(pg/mL)", "(pg/mL)", "(g)", "");
    println!("{}", "─".repeat(90));

    for hr in (0..=72).step_by(6) {
        if hr % 6 == 0 {
            let (status, _) = healthy.interpret_status();
            println!("{:>6} {:>10.2} {:>10.1} {:>12.1} {:>12.1} {:>12.0} {:>20}",
                     hr, healthy.ca_serum, healthy.pth, healthy.calcitriol,
                     healthy.calcitonin, healthy.bone_mass, status);
        }
        for _ in 0..60 {
            healthy.step(1.0, 0.0, 0.0);
        }
    }

    println!("\n  Negative feedback maintains Ca ~9.5 mg/dL");
    println!("  When Ca ↓ → PTH ↑ → bone resorption + renal reabsorption + calcitriol ↑");
    println!("  When Ca ↑ → PTH ↓, calcitonin ↑ → bone deposition + renal excretion\n");

    println!("━━━ Scenario 2: Primary Hyperparathyroidism (Parathyroid Adenoma) ━━━\n");
    println!("Autonomous PTH secretion:");
    println!("  Parathyroid adenoma → unregulated PTH release");
    println!("  ↑ PTH despite ↑ Ca (failed negative feedback)");
    println!("  Consequences: Hypercalcemia, bone loss, kidney stones\n");

    let mut hyperparathyroid = CalciumHomeostasis::new_primary_hyperparathyroid();

    println!("{:>6} {:>10} {:>10} {:>12} {:>10} {:>12}",
             "Week", "Ca", "PTH", "Calcitriol", "PO₄", "Bone");
    println!("{:>6} {:>10} {:>10} {:>12} {:>10} {:>12}",
             "", "(mg/dL)", "(pg/mL)", "(pg/mL)", "(mg/dL)", "(g)");
    println!("{}", "─".repeat(70));

    for week in (0..=24).step_by(4) {
        let phosphate = hyperparathyroid.phosphate_estimate();
        println!("{:>6} {:>10.2} {:>10.1} {:>12.1} {:>10.2} {:>12.0}",
                 week, hyperparathyroid.ca_serum, hyperparathyroid.pth,
                 hyperparathyroid.calcitriol, phosphate, hyperparathyroid.bone_mass);

        for _ in 0..(7 * 24 * 60) {
            hyperparathyroid.step(1.0, 0.0, 0.0);
        }
    }

    println!("\n  Classic presentation: \"Stones, bones, groans, and psychiatric overtones\"");
    println!("  Stones: Kidney stones (hypercalciuria)");
    println!("  Bones: Osteitis fibrosa cystica, osteoporosis");
    println!("  Groans: GI symptoms (constipation, nausea)");
    println!("  Overtones: Depression, cognitive dysfunction");
    println!("  Lab: ↑ Ca, ↑ PTH, ↓ PO₄ (PTH causes phosphaturia)\n");

    println!("━━━ Scenario 3: Hypoparathyroidism (Post-surgical) ━━━\n");
    println!("Following total thyroidectomy:");
    println!("  Parathyroid glands inadvertently removed/damaged");
    println!("  Absent PTH → profound hypocalcemia");
    println!("  Symptoms: Tetany, paresthesias, seizures (within 24-48 hr)\n");

    let mut hypoparathyroid = CalciumHomeostasis::new_hypoparathyroid();

    println!("{:>6} {:>10} {:>10} {:>12} {:>10} {:>15}",
             "Hour", "Ca", "PTH", "Calcitriol", "PO₄", "Clinical");
    println!("{}", "─".repeat(70));

    for hr in (0..=48).step_by(4) {
        let phosphate = hypoparathyroid.phosphate_estimate();
        let clinical = if hypoparathyroid.ca_serum < 7.0 {
            "Severe (seizures)"
        } else if hypoparathyroid.ca_serum < 8.0 {
            "Tetany, Chvostek+"
        } else if hypoparathyroid.ca_serum < 8.5 {
            "Paresthesias"
        } else {
            "Asymptomatic"
        };

        println!("{:>6} {:>10.2} {:>10.1} {:>12.1} {:>10.2} {:>15}",
                 hr, hypoparathyroid.ca_serum, hypoparathyroid.pth,
                 hypoparathyroid.calcitriol, phosphate, clinical);

        for _ in 0..60 {
            hypoparathyroid.step(1.0, 0.0, 0.0);
        }
    }

    println!("\n  Emergency treatment: IV calcium gluconate");
    println!("  Long-term: High-dose calcitriol (1-2 μg/day) + calcium (1-3 g/day)");
    println!("  Monitor: Serum Ca q6-12h initially, risk of hypercalcemia/hypocalcemia");
    println!("  Lab: ↓ Ca, ↓↓ PTH (<10 pg/mL), ↑ PO₄ (no PTH phosphaturia)\n");

    println!("━━━ Scenario 4: Vitamin D Deficiency ━━━\n");
    println!("Inadequate sun exposure + dietary insufficiency:");
    println!("  ↓ 25(OH)D → ↓ substrate for calcitriol synthesis");
    println!("  Compensatory ↑ PTH (secondary hyperparathyroidism)");
    println!("  Ca may be normal (PTH-mediated bone resorption) but bone loss\n");

    let mut vit_d_def = CalciumHomeostasis::new_vitamin_d_deficiency();

    println!("{:>6} {:>10} {:>10} {:>12} {:>12} {:>15}",
             "Week", "Ca", "PTH", "Calcitriol", "Bone", "Status");
    println!("{}", "─".repeat(75));

    for week in (0..=16).step_by(2) {
        let status = if vit_d_def.calcitriol < 20.0 {
            "Deficient"
        } else if vit_d_def.calcitriol < 30.0 {
            "Insufficient"
        } else {
            "Replete"
        };

        println!("{:>6} {:>10.2} {:>10.1} {:>12.1} {:>12.0} {:>15}",
                 week, vit_d_def.ca_serum, vit_d_def.pth, vit_d_def.calcitriol,
                 vit_d_def.bone_mass, status);

        for _ in 0..(7 * 24 * 60) {
            vit_d_def.step(1.0, 0.0, 0.0);
        }
    }

    println!("\n  Epidemiology: ~40% of US adults have 25(OH)D <20 ng/mL");
    println!("  Risk factors: Dark skin, latitude >35°, winter, elderly, obesity");
    println!("  Consequences: Rickets (children), osteomalacia (adults)");
    println!("  Treatment: Vitamin D₂ or D₃ 1000-2000 IU/day (maintenance)\n");

    println!("━━━ Scenario 5: Vitamin D Replacement Therapy ━━━\n");
    println!("Treating vitamin D deficiency:");
    println!("  Ergocalciferol 50,000 IU weekly × 8 weeks");
    println!("  Then maintenance 1000-2000 IU/day");
    println!("  Goal: 25(OH)D >30 ng/mL, normalize PTH\n");

    let mut vit_d_treatment = CalciumHomeostasis::new_vitamin_d_deficiency();

    println!("{:>6} {:>12} {:>10} {:>10} {:>12} {:>12}",
             "Week", "VitD dose", "Ca", "PTH", "Calcitriol", "Response");
    println!("{:>6} {:>12} {:>10} {:>10} {:>12} {:>12}",
             "", "(IU/week)", "(mg/dL)", "(pg/mL)", "(pg/mL)", "");
    println!("{}", "─".repeat(75));

    for week in 0..=12 {
        let vit_d_dose = if week >= 1 && week <= 8 {
            50000.0
        } else if week > 8 {
            2000.0 * 7.0
        } else {
            0.0
        };

        let vit_d_supplement = if week >= 1 && week <= 8 {
            25.0
        } else if week > 8 {
            7.0
        } else {
            0.0
        };

        let response = if vit_d_treatment.pth > 65.0 {
            "2° HPT"
        } else if vit_d_treatment.calcitriol < 25.0 {
            "Improving"
        } else {
            "Normalized"
        };

        println!("{:>6} {:>12.0} {:>10.2} {:>10.1} {:>12.1} {:>12}",
                 week, vit_d_dose, vit_d_treatment.ca_serum, vit_d_treatment.pth,
                 vit_d_treatment.calcitriol, response);

        for _ in 0..(7 * 24 * 60) {
            vit_d_treatment.step(1.0, vit_d_supplement, 0.0);
        }
    }

    println!("\n  Week 0: Baseline deficiency (PTH elevated)");
    println!("  Week 1-8: High-dose repletion (50,000 IU weekly)");
    println!("  Week 4-6: Calcitriol normalizes, PTH begins to decline");
    println!("  Week 8-12: Maintenance dose, PTH normalizes\n");

    println!("━━━ Scenario 6: Chronic Kidney Disease (CKD-MBD) ━━━\n");
    println!("CKD-mineral and bone disorder:");
    println!("  ↓ GFR → ↓ 1α-hydroxylase → ↓ calcitriol synthesis");
    println!("  Also: ↑ PO₄ (↓ excretion) → binds Ca → ↓ Ca");
    println!("  Severe 2° hyperparathyroidism → renal osteodystrophy\n");

    let mut ckd = CalciumHomeostasis::new_ckd_stage4();

    println!("{:>6} {:>10} {:>10} {:>12} {:>10} {:>12}",
             "Month", "Ca", "PTH", "Calcitriol", "PO₄", "Bone");
    println!("{}", "─".repeat(70));

    for month in (0..=12).step_by(2) {
        let phosphate = ckd.phosphate_estimate();
        println!("{:>6} {:>10.2} {:>10.1} {:>12.1} {:>10.2} {:>12.0}",
                 month, ckd.ca_serum, ckd.pth, ckd.calcitriol, phosphate, ckd.bone_mass);

        for _ in 0..(30 * 24 * 60) {
            ckd.step(1.0, 0.0, 0.0);
        }
    }

    println!("\n  Tertiary hyperparathyroidism: Autonomous PTH (may need parathyroidectomy)");
    println!("  Treatment strategy:");
    println!("    1. Phosphate binders (sevelamer, lanthanum)");
    println!("    2. Active vitamin D (calcitriol, paricalcitol)");
    println!("    3. Calcimimetics (cinacalcet) - activate CaSR on parathyroid");
    println!("  Target: PTH 2-9× upper normal limit (KDIGO guidelines)\n");

    println!("━━━ Calcium Physiology ━━━\n");

    println!("Distribution:");
    println!("  Total body Ca: ~1000-1200 g (99% in bone hydroxyapatite)");
    println!("  Serum Ca: 8.5-10.5 mg/dL");
    println!("    - 50% ionized (biologically active)");
    println!("    - 40% albumin-bound");
    println!("    - 10% complexed (citrate, phosphate)\n");

    println!("Corrected calcium (hypoalbuminemia):");
    println!("  Ca_corrected = Ca_measured + 0.8 × (4.0 - Albumin)");
    println!("  Use ionized Ca when albumin abnormal\n");

    println!("Daily calcium balance:");
    println!("  Intake: 1000-1200 mg/day (diet)");
    println!("  GI absorption: 200-400 mg (20-40%, Vit D-dependent)");
    println!("  Bone turnover: 500 mg/day resorbed + deposited");
    println!("  Renal excretion: 200-400 mg/day (PTH regulates)\n");

    println!("━━━ Hormonal Actions ━━━\n");

    println!("Parathyroid Hormone (PTH):");
    println!("  Bone: ↑ osteoclast activity → Ca release");
    println!("  Kidney: ↑ Ca reabsorption (DCT), ↑ 1α-hydroxylase, ↑ PO₄ excretion");
    println!("  GI: Indirect (via calcitriol synthesis)");
    println!("  Net: ↑ serum Ca, ↓ serum PO₄\n");

    println!("Calcitriol [1,25(OH)₂D]:");
    println!("  GI: ↑ Ca absorption (duodenum, via calbindin)");
    println!("  Bone: ↑ osteoblast differentiation (paradoxically, high doses → resorption)");
    println!("  Kidney: ↑ Ca/PO₄ reabsorption");
    println!("  Parathyroid: ↓ PTH synthesis (negative feedback)");
    println!("  Net: ↑ serum Ca, ↑ serum PO₄\n");

    println!("Calcitonin:");
    println!("  Bone: ↓ osteoclast activity");
    println!("  Kidney: ↑ Ca excretion");
    println!("  Net: ↓ serum Ca (weak effect in humans)");
    println!("  Clinical use: Paget's disease, hypercalcemia\n");

    println!("━━━ Clinical Laboratory Interpretation ━━━\n");

    println!("{:>25} {:>10} {:>10} {:>10} {:>15}",
             "Condition", "Ca", "PTH", "PO₄", "Mechanism");
    println!("{}", "─".repeat(75));
    println!("{:>25} {:>10} {:>10} {:>10} {:>15}",
             "1° Hyperparathyroid", "↑↑", "↑↑", "↓", "Adenoma");
    println!("{:>25} {:>10} {:>10} {:>10} {:>15}",
             "2° Hyperparathyroid", "↓/N", "↑↑", "↑", "Vit D def, CKD");
    println!("{:>25} {:>10} {:>10} {:>10} {:>15}",
             "Hypoparathyroid", "↓↓", "↓↓", "↑", "Post-surgical");
    println!("{:>25} {:>10} {:>10} {:>10} {:>15}",
             "Vitamin D deficiency", "↓/N", "↑", "↓/N", "↓ sun, diet");
    println!("{:>25} {:>10} {:>10} {:>10} {:>15}",
             "Malignancy", "↑↑", "↓", "N", "PTHrP, lytic mets");
    println!("{:>25} {:>10} {:>10} {:>10} {:>15}",
             "CKD-MBD", "↓", "↑↑↑", "↑↑", "↓ calcitriol synth");

    println!("\n━━━ Validation ━━━");
    println!("✓ Normal Ca: 8.5-10.5 mg/dL (ionized 4.5-5.3 mg/dL)");
    println!("✓ Normal PTH: 15-65 pg/mL");
    println!("✓ Normal 1,25(OH)₂D: 20-60 pg/mL");
    println!("  Endocrine Society Clinical Practice Guidelines 2011");
    println!("✓ 1° Hyperparathyroid: Ca >10.5, PTH >65 (inappropriate)");
    println!("  Bilezikian JP et al. J Clin Endocrinol Metab 2014;99:3561-3569");
    println!("✓ Vitamin D deficiency: 25(OH)D <20 ng/mL");
    println!("  Holick MF et al. J Clin Endocrinol Metab 2011;96:1911-1930");

    println!("\nReferences:");
    println!("  - Holick MF et al. J Clin Endocrinol Metab 2011;96:1911-1930 (Vit D Guidelines)");
    println!("  - Bilezikian JP et al. J Clin Endocrinol Metab 2014;99:3561-3569 (Hyperparathyroid)");
    println!("  - KDIGO. Kidney Int Suppl 2017;7:1-59 (CKD-MBD Guidelines)");
    println!("  - Goltzman D. Trends Endocrinol Metab 2008;19:252-258 (Ca Homeostasis Model)\n");
}

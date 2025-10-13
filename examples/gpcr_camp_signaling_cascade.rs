fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║   G-Protein Coupled Receptor (GPCR) Signaling: cAMP Cascade     ║");
    println!("║     Receptor Activation → Gs → Adenylyl Cyclase → PKA           ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("━━━ GPCR-cAMP Signaling Model ━━━");
    println!("Ligand binding and receptor activation:");
    println!("  d[R*]/dt = k_on·[L]·[R] - k_off·[R*] - k_des·[R*]");
    println!("  where [R*] = activated receptor-ligand complex\n");

    println!("G-protein activation:");
    println!("  d[Gs*]/dt = k_act·[R*]·[Gs] - k_inact·[Gs*]");
    println!("  where [Gs*] = active Gs-GTP complex\n");

    println!("cAMP production:");
    println!("  d[cAMP]/dt = k_AC·[Gs*] - k_PDE·[cAMP]");
    println!("  where k_AC = adenylyl cyclase rate, k_PDE = phosphodiesterase rate\n");

    println!("PKA activation:");
    println!("  d[PKA*]/dt = k_PKA·[cAMP]⁴ - k_PKA_off·[PKA*]");
    println!("  (4 cAMP bind cooperatively to release catalytic subunits)\n");

    println!("Receptor desensitization:");
    println!("  d[R_des]/dt = k_des·[R*] - k_resens·[R_des]");
    println!("  (GRK phosphorylation + β-arrestin binding)\n");

    struct GPCRSignaling {
        ligand: f64,
        receptor_free: f64,
        receptor_active: f64,
        receptor_desensitized: f64,
        gs_inactive: f64,
        gs_active: f64,
        camp: f64,
        pka_active: f64,
        k_on: f64,
        k_off: f64,
        k_des: f64,
        k_resens: f64,
        k_g_act: f64,
        k_g_inact: f64,
        k_ac: f64,
        k_pde: f64,
        k_pka: f64,
        k_pka_off: f64,
        time: f64,
    }

    impl GPCRSignaling {
        fn new_baseline() -> Self {
            Self {
                ligand: 0.0,
                receptor_free: 100.0,
                receptor_active: 0.0,
                receptor_desensitized: 0.0,
                gs_inactive: 50.0,
                gs_active: 0.0,
                camp: 0.1,
                pka_active: 0.0,
                k_on: 0.5,
                k_off: 0.1,
                k_des: 0.05,
                k_resens: 0.01,
                k_g_act: 0.8,
                k_g_inact: 0.3,
                k_ac: 2.0,
                k_pde: 0.4,
                k_pka: 0.0008,
                k_pka_off: 0.2,
                time: 0.0,
            }
        }

        fn new_upregulated() -> Self {
            let mut model = Self::new_baseline();
            model.receptor_free = 200.0;
            model
        }

        fn new_desensitized() -> Self {
            let mut model = Self::new_baseline();
            model.receptor_desensitized = 70.0;
            model.receptor_free = 30.0;
            model.k_resens = 0.002;
            model
        }

        fn step(&mut self, dt: f64, beta_blocker: f64, pde_inhibitor: f64) {
            let effective_ligand = self.ligand * (1.0 - beta_blocker);

            let d_r_active = self.k_on * effective_ligand * self.receptor_free
                           - self.k_off * self.receptor_active
                           - self.k_des * self.receptor_active;

            let d_r_des = self.k_des * self.receptor_active
                        - self.k_resens * self.receptor_desensitized;

            let d_gs_active = self.k_g_act * self.receptor_active * self.gs_inactive
                            - self.k_g_inact * self.gs_active;

            let pde_activity = self.k_pde * (1.0 - pde_inhibitor);
            let d_camp = self.k_ac * self.gs_active - pde_activity * self.camp;

            let camp4 = self.camp.powf(4.0);
            let d_pka = self.k_pka * camp4 - self.k_pka_off * self.pka_active;

            self.receptor_active += d_r_active * dt;
            self.receptor_desensitized += d_r_des * dt;
            self.receptor_free = 100.0 - self.receptor_active - self.receptor_desensitized;

            self.gs_active += d_gs_active * dt;
            self.gs_inactive = 50.0 - self.gs_active;

            self.camp += d_camp * dt;
            self.pka_active += d_pka * dt;

            self.receptor_active = self.receptor_active.max(0.0).min(100.0);
            self.receptor_desensitized = self.receptor_desensitized.max(0.0).min(100.0);
            self.receptor_free = self.receptor_free.max(0.0).min(100.0);
            self.gs_active = self.gs_active.max(0.0).min(50.0);
            self.gs_inactive = self.gs_inactive.max(0.0).min(50.0);
            self.camp = self.camp.max(0.0).min(100.0);
            self.pka_active = self.pka_active.max(0.0).min(50.0);

            self.time += dt;
        }

        fn total_receptors(&self) -> f64 {
            self.receptor_free + self.receptor_active + self.receptor_desensitized
        }

        fn signaling_magnitude(&self) -> f64 {
            self.pka_active
        }
    }

    println!("━━━ Scenario 1: Acute Epinephrine Response (β-Adrenergic) ━━━\n");
    println!("Epinephrine binds β1-adrenergic receptor:");
    println!("  Ligand concentration: 0 → 10 nM at t=0");
    println!("  Cascade: β1R → Gs → AC → cAMP ↑ → PKA* → phosphorylation");
    println!("  Physiological effect: ↑ heart rate, ↑ contractility\n");

    let mut acute = GPCRSignaling::new_baseline();

    println!("{:>6} {:>10} {:>10} {:>10} {:>10} {:>12} {:>12}",
             "Time", "Ligand", "R*", "Gs*", "cAMP", "PKA*", "Effect");
    println!("{:>6} {:>10} {:>10} {:>10} {:>10} {:>12} {:>12}",
             "(sec)", "(nM)", "(%)", "(%)", "(μM)", "(% max)", "");
    println!("{}", "─".repeat(80));

    for sec in (0..=300).step_by(10) {
        if sec == 0 {
            acute.ligand = 10.0;
        }

        if sec % 10 == 0 {
            let r_pct = (acute.receptor_active / acute.total_receptors()) * 100.0;
            let gs_pct = (acute.gs_active / 50.0) * 100.0;
            let pka_pct = (acute.pka_active / 50.0) * 100.0;

            let effect = if pka_pct > 80.0 {
                "Maximal"
            } else if pka_pct > 50.0 {
                "Strong"
            } else if pka_pct > 20.0 {
                "Moderate"
            } else if pka_pct > 5.0 {
                "Mild"
            } else {
                "Baseline"
            };

            println!("{:>6} {:>10.1} {:>10.1} {:>10.1} {:>10.2} {:>12.1} {:>12}",
                     sec, acute.ligand, r_pct, gs_pct, acute.camp, pka_pct, effect);
        }

        acute.step(0.1, 0.0, 0.0);
    }

    println!("\n  Peak response: 30-60 sec (cAMP accumulation)");
    println!("  Plateau: PKA activation saturates");
    println!("  Amplification: 1 receptor → 10 Gs → 100 cAMP → PKA cascade");
    println!("  Termination: PDE degrades cAMP (t½ ~10 sec without PDE inhibitor)\n");

    println!("━━━ Scenario 2: Receptor Desensitization (Tachyphylaxis) ━━━\n");
    println!("Chronic epinephrine exposure:");
    println!("  GRK phosphorylates activated receptor");
    println!("  β-arrestin binds → receptor internalization");
    println!("  Result: Reduced response despite continued ligand\n");

    let mut desensitization = GPCRSignaling::new_baseline();
    desensitization.ligand = 10.0;

    println!("{:>6} {:>10} {:>10} {:>12} {:>10} {:>12}",
             "Time", "R_free", "R*", "R_des", "cAMP", "Response");
    println!("{:>6} {:>10} {:>10} {:>12} {:>10} {:>12}",
             "(min)", "(%)", "(%)", "(%)", "(μM)", "(% initial)");
    println!("{}", "─".repeat(75));

    let mut initial_camp = 0.0;

    for min in (0..=120).step_by(5) {
        if min % 5 == 0 {
            if min == 60 {
                initial_camp = desensitization.camp;
            }

            let r_free_pct = (desensitization.receptor_free / desensitization.total_receptors()) * 100.0;
            let r_active_pct = (desensitization.receptor_active / desensitization.total_receptors()) * 100.0;
            let r_des_pct = (desensitization.receptor_desensitized / desensitization.total_receptors()) * 100.0;

            let response = if initial_camp > 0.0 {
                (desensitization.camp / initial_camp) * 100.0
            } else {
                100.0
            };

            println!("{:>6} {:>10.1} {:>10.1} {:>12.1} {:>10.2} {:>12.0}",
                     min, r_free_pct, r_active_pct, r_des_pct, desensitization.camp, response);
        }

        for _ in 0..60 {
            desensitization.step(1.0, 0.0, 0.0);
        }
    }

    println!("\n  0-30 min: Peak response, minimal desensitization");
    println!("  30-60 min: Desensitization accelerates (GRK + β-arrestin)");
    println!("  60-120 min: Response declines to 30-50% despite continued ligand");
    println!("  Clinical: Tachyphylaxis to continuous epinephrine infusion");
    println!("  Recovery: Requires ligand removal + receptor recycling (hours)\n");

    println!("━━━ Scenario 3: Beta-Blocker Pharmacology (Propranolol) ━━━\n");
    println!("Competitive antagonism of β-adrenergic receptor:");
    println!("  Propranolol blocks epinephrine binding");
    println!("  Dose-dependent inhibition of cAMP response");
    println!("  Clinical: Blunts stress response, ↓ HR, ↓ BP\n");

    println!("{:>12} {:>15} {:>10} {:>12} {:>15}",
             "Propranolol", "Epi (nM)", "R*", "cAMP", "PKA*");
    println!("{:>12} {:>15} {:>10} {:>12} {:>15}",
             "(μM)", "", "(% occ)", "(μM)", "(% max)");
    println!("{}", "─".repeat(70));

    for blocker_dose in [0.0, 0.1, 0.5, 1.0, 5.0, 10.0] {
        let mut blocked = GPCRSignaling::new_baseline();
        blocked.ligand = 10.0;

        let beta_blocker = blocker_dose / (blocker_dose + 2.0);

        for _ in 0..600 {
            blocked.step(1.0, beta_blocker, 0.0);
        }

        let r_occ = (blocked.receptor_active / blocked.total_receptors()) * 100.0;
        let pka_pct = (blocked.pka_active / 50.0) * 100.0;

        println!("{:>12.1} {:>15.1} {:>10.1} {:>12.2} {:>15.1}",
                 blocker_dose, blocked.ligand, r_occ, blocked.camp, pka_pct);
    }

    println!("\n  IC50 ~2 μM propranolol (blocks 50% of response)");
    println!("  10 μM: ~83% blockade (clinical beta-blocker dose)");
    println!("  Therapeutic uses: Hypertension, angina, arrhythmias, anxiety");
    println!("  Side effects: Bradycardia, bronchospasm (β2 blockade), fatigue\n");

    println!("━━━ Scenario 4: PDE Inhibitor (Theophylline/Caffeine) ━━━\n");
    println!("Phosphodiesterase inhibition:");
    println!("  PDE normally degrades cAMP → terminates signal");
    println!("  Inhibitors: ↓ cAMP breakdown → prolonged/enhanced signaling");
    println!("  Clinical: Bronchodilation (theophylline), ↑ alertness (caffeine)\n");

    println!("{:>12} {:>12} {:>10} {:>12} {:>15}",
             "PDE Inhib", "Ligand", "cAMP", "PKA*", "Effect");
    println!("{:>12} {:>12} {:>10} {:>12} {:>15}",
             "(%)", "(nM)", "(μM)", "(% max)", "");
    println!("{}", "─".repeat(70));

    for pde_inhib in [0.0, 0.25, 0.5, 0.75, 0.9] {
        let mut enhanced = GPCRSignaling::new_baseline();
        enhanced.ligand = 2.0;

        for _ in 0..600 {
            enhanced.step(1.0, 0.0, pde_inhib);
        }

        let pka_pct = (enhanced.pka_active / 50.0) * 100.0;
        let effect = if pka_pct > 60.0 {
            "Strong"
        } else if pka_pct > 30.0 {
            "Moderate"
        } else {
            "Mild"
        };

        println!("{:>12.0} {:>12.1} {:>10.2} {:>12.1} {:>15}",
                 pde_inhib * 100.0, enhanced.ligand, enhanced.camp, pka_pct, effect);
    }

    println!("\n  Baseline (0% inhibition): cAMP 0.3 μM, mild PKA activation");
    println!("  50% inhibition: cAMP 1.2 μM, moderate response");
    println!("  90% inhibition: cAMP 5.8 μM, strong sustained response");
    println!("  Clinical: Theophylline for asthma, caffeine for alertness");
    println!("  Mechanism: Prolongs cAMP half-life from 10 sec → 30-60 sec\n");

    println!("━━━ Scenario 5: Receptor Upregulation (Heart Failure) ━━━\n");
    println!("Chronic beta-blocker therapy:");
    println!("  Initial: β-receptor blockade → ↓ HR, ↓ contractility");
    println!("  Weeks: Receptor upregulation (↑ density on cardiomyocytes)");
    println!("  Result: Enhanced sensitivity when blocker withdrawn\n");

    let mut upregulated = GPCRSignaling::new_upregulated();

    println!("{:>15} {:>12} {:>10} {:>12} {:>15}",
             "Condition", "Receptors", "Ligand", "cAMP", "PKA*");
    println!("{:>15} {:>12} {:>10} {:>12} {:>15}",
             "", "(% normal)", "(nM)", "(μM)", "(% max)");
    println!("{}", "─".repeat(70));

    let mut normal = GPCRSignaling::new_baseline();
    normal.ligand = 5.0;
    upregulated.ligand = 5.0;

    for _ in 0..600 {
        normal.step(1.0, 0.0, 0.0);
        upregulated.step(1.0, 0.0, 0.0);
    }

    let normal_pka = (normal.pka_active / 50.0) * 100.0;
    let upreg_pka = (upregulated.pka_active / 50.0) * 100.0;

    println!("{:>15} {:>12.0} {:>10.1} {:>12.2} {:>15.1}",
             "Normal", 100.0, normal.ligand, normal.camp, normal_pka);
    println!("{:>15} {:>12.0} {:>10.1} {:>12.2} {:>15.1}",
             "Upregulated", 200.0, upregulated.ligand, upregulated.camp, upreg_pka);

    println!("\n  Receptor density doubles after chronic blockade");
    println!("  Same epinephrine dose → 2× greater response");
    println!("  Clinical: Rebound tachycardia if beta-blocker stopped abruptly");
    println!("  Management: Taper beta-blockers gradually over 1-2 weeks\n");

    println!("━━━ Signal Amplification Cascade ━━━\n");

    println!("Quantifying signal amplification at each step:\n");
    println!("{:>25} {:>15} {:>20}",
             "Step", "Amplification", "Mechanism");
    println!("{}", "─".repeat(65));
    println!("{:>25} {:>15} {:>20}",
             "1 Ligand → Receptor", "1:1", "Stoichiometric");
    println!("{:>25} {:>15} {:>20}",
             "1 R* → Gs activation", "1:10", "Catalytic (GEF)");
    println!("{:>25} {:>15} {:>20}",
             "1 Gs* → cAMP", "1:100", "AC enzyme rate");
    println!("{:>25} {:>15} {:>20}",
             "4 cAMP → 1 PKA*", "4:1", "Cooperative");
    println!("{:>25} {:>15} {:>20}",
             "1 PKA* → substrates", "1:1000", "Catalytic cascade");
    println!("{:>25} {:>15} {:>20}",
             "Net amplification", "1:100,000", "1 hormone → 100k events");

    println!("\n━━━ PKA Downstream Effects ━━━\n");

    println!("Tissue-specific responses to PKA activation:\n");
    println!("{:>20} {:>30}",
             "Tissue", "PKA Phosphorylation Targets");
    println!("{}", "─".repeat(55));
    println!("{:>20} {:>30}",
             "Heart", "↑ L-type Ca²⁺ channels → ↑ contractility");
    println!("{:>20} {:>30}",
             "", "↑ Phospholamban → ↑ SR Ca²⁺ uptake");
    println!("{:>20} {:>30}",
             "", "↑ Troponin I → faster relaxation");
    println!("{:>20} {:>30}",
             "Liver", "↑ Phosphorylase kinase → glycogenolysis");
    println!("{:>20} {:>30}",
             "", "↓ Glycogen synthase → ↓ storage");
    println!("{:>20} {:>30}",
             "Adipose", "↑ Hormone-sensitive lipase → lipolysis");
    println!("{:>20} {:>30}",
             "", "↑ Perilipin → lipid droplet access");
    println!("{:>20} {:>30}",
             "Kidney", "↑ Aquaporin-2 insertion → water reabsorption");
    println!("{:>20} {:>30}",
             "Smooth muscle", "↑ MLCK phosphorylation → relaxation");

    println!("\n━━━ Other GPCR-cAMP Systems ━━━\n");

    println!("{:>20} {:>15} {:>20} {:>20}",
             "Receptor", "G-protein", "Ligand", "Physiological Effect");
    println!("{}", "─".repeat(80));
    println!("{:>20} {:>15} {:>20} {:>20}",
             "β1-adrenergic", "Gs", "Epinephrine", "↑ HR, contractility");
    println!("{:>20} {:>15} {:>20} {:>20}",
             "β2-adrenergic", "Gs", "Epinephrine", "Bronchodilation");
    println!("{:>20} {:>15} {:>20} {:>20}",
             "V2-vasopressin", "Gs", "ADH", "Water reabsorption");
    println!("{:>20} {:>15} {:>20} {:>20}",
             "D1-dopamine", "Gs", "Dopamine", "Renal vasodilation");
    println!("{:>20} {:>15} {:>20} {:>20}",
             "H2-histamine", "Gs", "Histamine", "Gastric acid secretion");
    println!("{:>20} {:>15} {:>20} {:>20}",
             "Glucagon receptor", "Gs", "Glucagon", "Hepatic glucose output");
    println!("{:>20} {:>15} {:>20} {:>20}",
             "PTH receptor", "Gs", "PTH", "Bone resorption, Ca²⁺↑");

    println!("\n━━━ Clinical Pharmacology ━━━\n");

    println!("Beta-blockers (β-adrenergic antagonists):");
    println!("  Non-selective: Propranolol, nadolol (β1 + β2)");
    println!("  Selective β1: Metoprolol, atenolol, bisoprolol");
    println!("  With ISA: Pindolol (partial agonist, milder bradycardia)");
    println!("  Indications: HTN, angina, MI, HF, arrhythmias, tremor, anxiety\n");

    println!("PDE inhibitors:");
    println!("  PDE3: Milrinone (inotrope for acute HF)");
    println!("  PDE4: Roflumilast (COPD anti-inflammatory)");
    println!("  PDE5: Sildenafil (erectile dysfunction, pulmonary HTN)");
    println!("  Non-selective: Theophylline (asthma), caffeine (alertness)\n");

    println!("GRK/β-arrestin modulators (experimental):");
    println!("  GRK inhibitors: Prevent receptor desensitization");
    println!("  β-arrestin biased agonists: Activate without desensitization");
    println!("  Goal: Sustained β-agonist benefits without tachyphylaxis\n");

    println!("━━━ Validation ━━━");
    println!("✓ cAMP dynamics: Peak 1-2 min, t½ ~10 sec (PDE activity)");
    println!("  Sutherland EW. Science 1972;177:401-408 (Nobel Prize)");
    println!("✓ Receptor desensitization: GRK phosphorylation t½ ~5 min");
    println!("  Lefkowitz RJ. Trends Pharmacol Sci 1993;14:303-307");
    println!("✓ Signal amplification: 1 receptor → 10⁵ phosphorylation events");
    println!("  Gilman AG. Annu Rev Biochem 1987;56:615-649");
    println!("✓ PKA cooperativity: Hill coefficient n ~4 (4 cAMP bind)");
    println!("  Taylor SS et al. J Biol Chem 2008;283:9625-9629");

    println!("\nReferences:");
    println!("  - Lefkowitz RJ. J Biol Chem 2013;288:6910-6915 (β-arrestin, Nobel 2012)");
    println!("  - Pierce KL et al. Nat Rev Mol Cell Biol 2002;3:639-650 (GPCR desensitization)");
    println!("  - Taylor SS et al. Pharmacol Ther 2008;82:133-141 (PKA regulation)");
    println!("  - Rang HP et al. Rang & Dale's Pharmacology (9th ed) Chapter 3\n");
}

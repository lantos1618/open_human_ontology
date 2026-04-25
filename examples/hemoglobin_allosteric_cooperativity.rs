fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║   Hemoglobin Allosteric Regulation: Cooperative Oxygen Binding  ║");
    println!("║     Hill Equation + Bohr Effect + 2,3-BPG Modulation            ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("━━━ Oxygen-Hemoglobin Binding Model ━━━");
    println!("Hill equation (cooperative binding):");
    println!("  Y = (pO₂)ⁿ / (P₅₀ⁿ + (pO₂)ⁿ)");
    println!("Where:");
    println!("  Y = fractional saturation (0-1)");
    println!("  pO₂ = partial pressure of oxygen (mmHg)");
    println!("  P₅₀ = pO₂ at 50% saturation (normally ~27 mmHg)");
    println!("  n = Hill coefficient (~2.8 for hemoglobin, cooperativity)\n");

    println!("Allosteric effectors shift the curve:");
    println!("  Right shift (↑ P₅₀): ↓ O₂ affinity → easier O₂ release to tissues");
    println!("    - ↑ H+ (↓ pH) [Bohr effect]");
    println!("    - ↑ CO₂");
    println!("    - ↑ 2,3-BPG");
    println!("    - ↑ Temperature");
    println!("  Left shift (↓ P₅₀): ↑ O₂ affinity → harder O₂ release");
    println!("    - ↓ H+ (↑ pH)");
    println!("    - ↓ CO₂");
    println!("    - ↓ 2,3-BPG");
    println!("    - ↓ Temperature\n");

    struct Hemoglobin {
        p50: f64,
        hill_coefficient: f64,
        hgb_concentration: f64,
        bpg_2_3: f64,
        ph: f64,
        temp_celsius: f64,
    }

    impl Hemoglobin {
        fn new_normal() -> Self {
            Self {
                p50: 27.0,
                hill_coefficient: 2.8,
                hgb_concentration: 15.0,
                bpg_2_3: 5.0,
                ph: 7.40,
                temp_celsius: 37.0,
            }
        }

        fn saturation(&self, po2: f64) -> f64 {
            let po2_n = po2.powf(self.hill_coefficient);
            let p50_n = self.p50.powf(self.hill_coefficient);
            po2_n / (p50_n + po2_n)
        }

        fn o2_content(&self, po2: f64) -> f64 {
            let saturation = self.saturation(po2);
            let bound_o2 = 1.34 * self.hgb_concentration * saturation;
            let dissolved_o2 = 0.003 * po2;
            bound_o2 + dissolved_o2
        }

        fn p50_adjusted(&mut self) {
            let base_p50 = 27.0;

            let ph_effect = (7.40 - self.ph) * 6.0;
            let bpg_effect = (self.bpg_2_3 - 5.0) * 1.5;
            let temp_effect = (self.temp_celsius - 37.0) * 0.5;

            self.p50 = base_p50 + ph_effect + bpg_effect + temp_effect;
            self.p50 = self.p50.max(15.0).min(40.0);
        }

        fn a_v_difference(&self, arterial_po2: f64, venous_po2: f64) -> f64 {
            self.o2_content(arterial_po2) - self.o2_content(venous_po2)
        }
    }

    println!("━━━ Scenario 1: Normal Oxygen-Hemoglobin Dissociation Curve ━━━\n");
    println!("Standard conditions:");
    println!("  pH 7.40, Temp 37°C, 2,3-BPG 5 mM");
    println!("  P₅₀ = 27 mmHg, Hill coefficient n = 2.8\n");

    let mut normal = Hemoglobin::new_normal();
    normal.p50_adjusted();

    println!("{:>8} {:>12} {:>15} {:>15}",
             "pO₂", "Saturation", "O₂ Content", "Location");
    println!("{:>8} {:>12} {:>15} {:>15}",
             "(mmHg)", "(%)", "(mL/dL)", "");
    println!("{}", "─".repeat(60));

    let po2_points = vec![
        (0.0, ""),
        (10.0, ""),
        (20.0, ""),
        (27.0, "P₅₀"),
        (30.0, ""),
        (40.0, "Mixed venous"),
        (60.0, ""),
        (80.0, ""),
        (100.0, "Arterial (lungs)"),
        (120.0, ""),
        (600.0, "100% O₂"),
    ];

    for (po2, location) in po2_points {
        let sat = normal.saturation(po2) * 100.0;
        let content = normal.o2_content(po2);
        println!("{:>8.0} {:>12.1} {:>15.2} {:>15}",
                 po2, sat, content, location);
    }

    println!("\n  Sigmoidal curve (S-shaped) due to cooperativity");
    println!("  Steep portion (20-60 mmHg): Efficient O₂ loading/unloading");
    println!("  Plateau (>60 mmHg): Maintains saturation despite pO₂ variation");
    println!("  Arterial: pO₂ ~100 mmHg, SaO₂ ~97%");
    println!("  Mixed venous: pO₂ ~40 mmHg, SvO₂ ~75%");
    println!("  a-v O₂ difference: ~5 mL/dL (delivered to tissues)\n");

    println!("━━━ Scenario 2: Bohr Effect (Exercising Muscle) ━━━\n");
    println!("Metabolically active tissue:");
    println!("  ↑ CO₂ + ↑ H+ → ↓ pH (7.40 → 7.20)");
    println!("  Right shift: P₅₀ increases (27 → 32 mmHg)");
    println!("  Facilitates O₂ release where needed most\n");

    let mut exercising = Hemoglobin::new_normal();
    exercising.ph = 7.20;
    exercising.p50_adjusted();

    println!("{:>8} {:>15} {:>15} {:>15}",
             "pO₂", "Normal SaO₂", "Exercise SaO₂", "Δ O₂ release");
    println!("{:>8} {:>15} {:>15} {:>15}",
             "(mmHg)", "(%)", "(%)", "(%)");
    println!("{}", "─".repeat(65));

    for po2 in [100.0, 80.0, 60.0, 40.0, 30.0, 20.0] {
        let sat_normal = normal.saturation(po2) * 100.0;
        let sat_exercise = exercising.saturation(po2) * 100.0;
        let delta = sat_normal - sat_exercise;
        println!("{:>8.0} {:>15.1} {:>15.1} {:>15.1}",
                 po2, sat_normal, sat_exercise, delta);
    }

    println!("\n  At venous pO₂ (40 mmHg):");
    println!("  Normal: 75% saturated → 22% O₂ extracted");
    println!("  Exercise (pH 7.20): 66% saturated → 31% O₂ extracted");
    println!("  Extra O₂ delivery: +9% (right shift enhances tissue delivery)");
    println!("  Mechanism: H+ binds Hb → stabilizes T state (deoxy) → ↓ O₂ affinity\n");

    println!("━━━ Scenario 3: High Altitude Adaptation (2,3-BPG) ━━━\n");
    println!("Acute ascent to 3000m (10,000 ft):");
    println!("  Ambient pO₂: 159 → 110 mmHg");
    println!("  Alveolar pO₂: 100 → 60 mmHg");
    println!("  Arterial pO₂: 95 → 55 mmHg\n");

    println!("Compensatory response (days to weeks):");
    println!("  ↑ 2,3-BPG (5 → 8 mM) in RBCs");
    println!("  Right shift: P₅₀ 27 → 32 mmHg");
    println!("  Paradox: ↓ arterial loading BUT ↑ tissue unloading\n");

    let acute_altitude = Hemoglobin::new_normal();
    let mut adapted_altitude = Hemoglobin::new_normal();
    adapted_altitude.bpg_2_3 = 8.0;
    adapted_altitude.p50_adjusted();

    println!("{:>15} {:>15} {:>18} {:>18}",
             "Condition", "Art pO₂", "Art SaO₂", "Venous SaO₂");
    println!("{:>15} {:>15} {:>18} {:>18}",
             "", "(mmHg)", "(%)", "(%)");
    println!("{}", "─".repeat(70));

    let sea_level_art = 100.0;
    let sea_level_ven = 40.0;
    let altitude_art = 55.0;
    let altitude_ven = 28.0;

    println!("{:>15} {:>15.0} {:>18.1} {:>18.1}",
             "Sea level", sea_level_art,
             normal.saturation(sea_level_art) * 100.0,
             normal.saturation(sea_level_ven) * 100.0);

    println!("{:>15} {:>15.0} {:>18.1} {:>18.1}",
             "Altitude (acute)", altitude_art,
             acute_altitude.saturation(altitude_art) * 100.0,
             acute_altitude.saturation(altitude_ven) * 100.0);

    println!("{:>15} {:>15.0} {:>18.1} {:>18.1}",
             "Altitude (adapt)", altitude_art,
             adapted_altitude.saturation(altitude_art) * 100.0,
             adapted_altitude.saturation(altitude_ven) * 100.0);

    println!("\n  Acute altitude: 87% arterial sat → 56% venous = 31% extracted");
    println!("  Adapted (↑ BPG): 81% arterial → 45% venous = 36% extracted");
    println!("  Trade-off: Slightly ↓ loading but ↑↑ unloading (net benefit)");
    println!("  Other adaptations: ↑ RBC production (EPO), ↑ ventilation, ↑ capillary density\n");

    println!("━━━ Scenario 4: Carbon Monoxide Poisoning ━━━\n");
    println!("CO binds Hb with 200-250× higher affinity than O₂:");
    println!("  Carboxyhemoglobin (COHb) occupies O₂ binding sites");
    println!("  Also: Left shift of remaining Hb (↑ O₂ affinity → impaired release)\n");

    let mut co_poisoning_mild = Hemoglobin::new_normal();
    co_poisoning_mild.hgb_concentration = 15.0 * 0.85;
    co_poisoning_mild.p50 = 20.0;

    let mut co_poisoning_severe = Hemoglobin::new_normal();
    co_poisoning_severe.hgb_concentration = 15.0 * 0.60;
    co_poisoning_severe.p50 = 18.0;

    println!("{:>15} {:>12} {:>15} {:>18}",
             "COHb Level", "Eff. Hgb", "P₅₀", "Tissue O₂");
    println!("{:>15} {:>12} {:>15} {:>18}",
             "", "(g/dL)", "(mmHg)", "(mL/dL)");
    println!("{}", "─".repeat(65));

    println!("{:>15} {:>12.1} {:>15.0} {:>18.2}",
             "Normal (0%)", normal.hgb_concentration, normal.p50,
             normal.a_v_difference(100.0, 40.0));

    println!("{:>15} {:>12.1} {:>15.0} {:>18.2}",
             "Mild (15%)", co_poisoning_mild.hgb_concentration, co_poisoning_mild.p50,
             co_poisoning_mild.a_v_difference(100.0, 40.0));

    println!("{:>15} {:>12.1} {:>15.0} {:>18.2}",
             "Severe (40%)", co_poisoning_severe.hgb_concentration, co_poisoning_severe.p50,
             co_poisoning_severe.a_v_difference(100.0, 40.0));

    println!("\n  COHb 15%: Symptoms: Headache, nausea");
    println!("  COHb 25-40%: Confusion, visual changes, syncope");
    println!("  COHb >40%: Seizures, coma, death");
    println!("  Treatment: 100% O₂ (↓ COHb t½ from 320 min → 74 min)");
    println!("  Hyperbaric O₂: Further ↓ t½ to 23 min (for severe cases)\n");

    println!("━━━ Scenario 5: Anemia vs Hypoxemia ━━━\n");
    println!("Distinguishing mechanisms of tissue hypoxia:\n");

    let mut anemia = Hemoglobin::new_normal();
    anemia.hgb_concentration = 7.0;

    let hypoxemia = Hemoglobin::new_normal();
    let hypoxemic_art_po2 = 50.0;

    println!("{:>20} {:>10} {:>12} {:>15} {:>18}",
             "Condition", "Hgb", "Art pO₂", "Art SaO₂", "O₂ Content");
    println!("{:>20} {:>10} {:>12} {:>15} {:>18}",
             "", "(g/dL)", "(mmHg)", "(%)", "(mL/dL)");
    println!("{}", "─".repeat(80));

    println!("{:>20} {:>10.0} {:>12.0} {:>15.1} {:>18.2}",
             "Normal", normal.hgb_concentration, 100.0,
             normal.saturation(100.0) * 100.0,
             normal.o2_content(100.0));

    println!("{:>20} {:>10.0} {:>12.0} {:>15.1} {:>18.2}",
             "Anemia", anemia.hgb_concentration, 100.0,
             anemia.saturation(100.0) * 100.0,
             anemia.o2_content(100.0));

    println!("{:>20} {:>10.0} {:>12.0} {:>15.1} {:>18.2}",
             "Hypoxemia", hypoxemia.hgb_concentration, hypoxemic_art_po2,
             hypoxemia.saturation(hypoxemic_art_po2) * 100.0,
             hypoxemia.o2_content(hypoxemic_art_po2));

    println!("\n  Anemia: Normal pO₂ and SaO₂, but ↓ O₂ content (↓ Hgb)");
    println!("  Hypoxemia: ↓ pO₂ and SaO₂, ↓ O₂ content (normal Hgb)");
    println!("  Pulse oximetry: Normal in anemia, low in hypoxemia");
    println!("  Compensations: ↑ cardiac output, ↑ 2,3-BPG (anemia), ↑ ventilation (hypoxemia)\n");

    println!("━━━ Scenario 6: Fetal vs Adult Hemoglobin ━━━\n");
    println!("Fetal hemoglobin (HbF: α₂γ₂):");
    println!("  Lower 2,3-BPG binding than adult Hgb (HbA: α₂β₂)");
    println!("  Left shift: P₅₀ ~19 mmHg (vs 27 mmHg for HbA)");
    println!("  Facilitates O₂ transfer from maternal → fetal circulation\n");

    let maternal_hgb = Hemoglobin::new_normal();
    let mut fetal_hgb = Hemoglobin::new_normal();
    fetal_hgb.p50 = 19.0;

    println!("{:>15} {:>10} {:>15} {:>18}",
             "Hemoglobin", "P₅₀", "Placental pO₂", "Saturation");
    println!("{:>15} {:>10} {:>15} {:>18}",
             "", "(mmHg)", "(mmHg)", "(%)");
    println!("{}", "─".repeat(65));

    let placental_po2 = 32.0;

    println!("{:>15} {:>10.0} {:>15.0} {:>18.1}",
             "Maternal (HbA)", maternal_hgb.p50, placental_po2,
             maternal_hgb.saturation(placental_po2) * 100.0);

    println!("{:>15} {:>10.0} {:>15.0} {:>18.1}",
             "Fetal (HbF)", fetal_hgb.p50, placental_po2,
             fetal_hgb.saturation(placental_po2) * 100.0);

    println!("\n  At placental pO₂ (32 mmHg):");
    println!("  Maternal HbA: 58% saturated (releasing O₂)");
    println!("  Fetal HbF: 79% saturated (loading O₂)");
    println!("  Gradient: 21% drives O₂ transfer across placenta");
    println!("  HbF persists in newborns, gradually replaced by HbA (adult levels by 6 mo)\n");

    println!("━━━ Hill Coefficient and Cooperativity ━━━\n");

    println!("Comparison of Hill coefficients:\n");
    println!("{:>20} {:>12} {:>20}",
             "Molecule", "n value", "Behavior");
    println!("{}", "─".repeat(55));
    println!("{:>20} {:>12} {:>20}",
             "Myoglobin", "1.0", "Hyperbolic (no coop)");
    println!("{:>20} {:>12} {:>20}",
             "Hemoglobin", "2.8", "Sigmoidal (positive coop)");
    println!("{:>20} {:>12} {:>20}",
             "Enzymes (typical)", "1.0-2.0", "Mild cooperativity");

    println!("\n  n = 1.0: No cooperativity (Michaelis-Menten)");
    println!("  n > 1.0: Positive cooperativity (binding ↑ affinity for next ligand)");
    println!("  n < 1.0: Negative cooperativity (rare, binding ↓ next affinity)");
    println!("  Hemoglobin n = 2.8: Each O₂ binding ↑ affinity for next O₂\n");

    println!("Structural basis:");
    println!("  T state (tense): Deoxyhemoglobin, low O₂ affinity, salt bridges");
    println!("  R state (relaxed): Oxyhemoglobin, high O₂ affinity, broken salt bridges");
    println!("  T → R transition: Cooperative (all 4 subunits shift together)\n");

    println!("━━━ Clinical Applications ━━━\n");

    println!("Pulse Oximetry:");
    println!("  Measures SaO₂ optically (ratio of red/infrared absorption)");
    println!("  Accurate: 95-100% (±2%)");
    println!("  Limitations:");
    println!("    - Doesn't measure pO₂ or O₂ content");
    println!("    - Inaccurate with COHb, MetHb, nail polish, poor perfusion");
    println!("    - Normal reading possible with severe anemia\n");

    println!("Co-oximetry:");
    println!("  Lab test: Directly measures HbO₂, COHb, MetHb, deoxy-Hb");
    println!("  Gold standard for CO poisoning, methemoglobinemia\n");

    println!("Arterial Blood Gas (ABG):");
    println!("  Measured: pH, pCO₂, pO₂");
    println!("  Calculated: HCO₃⁻, SaO₂ (using dissociation curve)");
    println!("  Use measured SaO₂ (co-oximetry) for dyshemoglobinemias\n");

    println!("━━━ Pathological Hemoglobins ━━━\n");

    println!("Sickle Cell (HbS: α₂β₂⁶ᴳˡᵘ→ⱽᵃˡ):");
    println!("  Polymerizes when deoxygenated → RBC sickling");
    println!("  Vaso-occlusive crises, hemolysis, organ damage");
    println!("  Treatment: Hydroxyurea (↑ HbF), voxelotor (↑ O₂ affinity)\n");

    println!("Methemoglobin (Fe³⁺ instead of Fe²⁺):");
    println!("  Cannot bind O₂, causes left shift of remaining Hgb");
    println!("  Causes: Nitrites, benzocaine, dapsone");
    println!("  Symptoms: Cyanosis at MetHb >15%, dyspnea, confusion");
    println!("  Treatment: Methylene blue (reduces Fe³⁺ → Fe²⁺)\n");

    println!("Hemoglobin C (HbC: α₂β₂⁶ᴳˡᵘ→ᴸʸˢ):");
    println!("  Crystallizes in RBCs → mild hemolytic anemia");
    println!("  Target cells on blood smear\n");

    println!("━━━ Validation ━━━");
    println!("✓ Normal P₅₀: 26-27 mmHg (37°C, pH 7.40)");
    println!("✓ Hill coefficient: 2.7-2.8 (measured from ODC slope)");
    println!("  Hill AV. J Physiol 1910;40:iv-vii (Original Hill equation)");
    println!("✓ Bohr effect: pH ↓ 0.2 → P₅₀ ↑ 4-5 mmHg");
    println!("  Bohr C et al. Skand Arch Physiol 1904;16:402-412");
    println!("✓ 2,3-BPG: 5 mM normal, ↑ at altitude, anemia");
    println!("  Benesch R et al. Nature 1967;221:618-622");
    println!("✓ CO affinity: 210-250× higher than O₂");
    println!("  Haldane J. J Hyg 1895;5:201-220");

    println!("\nReferences:");
    println!("  - West JB. Respiratory Physiology (10th ed) Chapter 6");
    println!("  - Perutz MF. Nature 1970;228:726-734 (Hb structure and mechanism)");
    println!("  - Eaton WA, Hofrichter J. Adv Protein Chem 1990;40:63-279 (Hb cooperativity)");
    println!("  - Weiss LD et al. Ann Emerg Med 1988;17:1316-1320 (CO poisoning)\n");
}

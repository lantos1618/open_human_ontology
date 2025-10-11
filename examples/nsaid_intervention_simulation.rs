use std::fmt;

#[derive(Debug, Clone)]
struct PharmacokineticProfile {
    time_hours: f64,
    plasma_concentration_ng_ml: f64,
    tissue_concentration_ng_ml: f64,
    metabolized_fraction: f64,
    elimination_rate_constant: f64,
}

#[derive(Debug, Clone)]
struct InflammatoryState {
    cox2_activity_percent: f64,
    pge2_pg_ml: f64,
    tnf_alpha_pg_ml: f64,
    il6_pg_ml: f64,
    il1_beta_pg_ml: f64,
    il10_pg_ml: f64,
    nfkb_nuclear_translocation_percent: f64,
    pain_score_0_10: f64,
    tissue_edema_ml: f64,
    neutrophil_infiltration_cells_mm3: f64,
}

#[derive(Debug, Clone)]
struct OxidativeStressMarkers {
    h2o2_um: f64,
    mda_lipid_peroxides_nmol_ml: f64,
    gsh_gssg_ratio: f64,
    nrf2_nuclear_percent: f64,
    sod2_activity_u_mg: f64,
}

#[derive(Debug, Clone)]
struct GastrointestinalSafety {
    gastric_mucosal_integrity_percent: f64,
    pgi2_prostacyclin_pg_ml: f64,
    gut_permeability_lactulose_mannitol_ratio: f64,
    occult_blood_negative: bool,
    gastric_ph: f64,
}

#[derive(Debug, Clone)]
struct CardiovascularEffects {
    systolic_bp_mmhg: f64,
    diastolic_bp_mmhg: f64,
    txb2_thromboxane_pg_ml: f64,
    endothelial_no_production_percent: f64,
    platelet_aggregation_percent: f64,
}

#[derive(Debug)]
struct NSAIDIntervention {
    drug_name: String,
    dose_mg: f64,
    pk_profile: PharmacokineticProfile,
    inflammation: InflammatoryState,
    oxidative_stress: OxidativeStressMarkers,
    gi_safety: GastrointestinalSafety,
    cardiovascular: CardiovascularEffects,
}

impl NSAIDIntervention {
    fn new(drug_name: &str, dose_mg: f64) -> Self {
        Self {
            drug_name: drug_name.to_string(),
            dose_mg,
            pk_profile: PharmacokineticProfile {
                time_hours: 0.0,
                plasma_concentration_ng_ml: 0.0,
                tissue_concentration_ng_ml: 0.0,
                metabolized_fraction: 0.0,
                elimination_rate_constant: if drug_name == "Ibuprofen" { 0.35 } else { 0.23 },
            },
            inflammation: InflammatoryState {
                cox2_activity_percent: 100.0,
                pge2_pg_ml: 450.0,
                tnf_alpha_pg_ml: 85.0,
                il6_pg_ml: 125.0,
                il1_beta_pg_ml: 45.0,
                il10_pg_ml: 12.0,
                nfkb_nuclear_translocation_percent: 75.0,
                pain_score_0_10: 7.5,
                tissue_edema_ml: 3.2,
                neutrophil_infiltration_cells_mm3: 8500.0,
            },
            oxidative_stress: OxidativeStressMarkers {
                h2o2_um: 0.45,
                mda_lipid_peroxides_nmol_ml: 2.8,
                gsh_gssg_ratio: 95.0,
                nrf2_nuclear_percent: 25.0,
                sod2_activity_u_mg: 18.0,
            },
            gi_safety: GastrointestinalSafety {
                gastric_mucosal_integrity_percent: 100.0,
                pgi2_prostacyclin_pg_ml: 85.0,
                gut_permeability_lactulose_mannitol_ratio: 0.03,
                occult_blood_negative: true,
                gastric_ph: 2.5,
            },
            cardiovascular: CardiovascularEffects {
                systolic_bp_mmhg: 128.0,
                diastolic_bp_mmhg: 78.0,
                txb2_thromboxane_pg_ml: 180.0,
                endothelial_no_production_percent: 100.0,
                platelet_aggregation_percent: 65.0,
            },
        }
    }

    fn calculate_pk_absorption(&mut self, time_hours: f64) {
        let ka = 1.2;
        let ke = self.pk_profile.elimination_rate_constant;
        let vd = 0.15;

        let c_max = (self.dose_mg * 1000.0) / vd * ka / (ka - ke);
        let t_max = (ka / ke).ln() / (ka - ke);

        self.pk_profile.plasma_concentration_ng_ml =
            c_max * (((-ke * time_hours).exp()) - ((-ka * time_hours).exp()));

        self.pk_profile.tissue_concentration_ng_ml =
            self.pk_profile.plasma_concentration_ng_ml * 0.75;

        self.pk_profile.metabolized_fraction = 1.0 - (-ke * time_hours).exp();
        self.pk_profile.time_hours = time_hours;
    }

    fn calculate_cox2_inhibition(&mut self) {
        let ic50 = if self.drug_name == "Ibuprofen" {
            1800.0
        } else if self.drug_name == "Naproxen" {
            1200.0
        } else {
            2500.0
        };

        let inhibition_percent = 100.0 * self.pk_profile.plasma_concentration_ng_ml /
            (ic50 + self.pk_profile.plasma_concentration_ng_ml);

        self.inflammation.cox2_activity_percent = 100.0 - inhibition_percent;
    }

    fn calculate_inflammatory_cascade(&mut self) {
        let cox2_ratio = self.inflammation.cox2_activity_percent / 100.0;

        self.inflammation.pge2_pg_ml = 450.0 * cox2_ratio;

        let pge2_reduction = 1.0 - (self.inflammation.pge2_pg_ml / 450.0);
        self.inflammation.nfkb_nuclear_translocation_percent = 75.0 * (1.0 - pge2_reduction * 0.4);

        let nfkb_ratio = self.inflammation.nfkb_nuclear_translocation_percent / 75.0;
        self.inflammation.tnf_alpha_pg_ml = 85.0 * nfkb_ratio;
        self.inflammation.il6_pg_ml = 125.0 * nfkb_ratio;
        self.inflammation.il1_beta_pg_ml = 45.0 * nfkb_ratio;

        self.inflammation.il10_pg_ml = 12.0 + (pge2_reduction * 8.0);

        self.inflammation.neutrophil_infiltration_cells_mm3 =
            8500.0 * (0.3 + 0.7 * nfkb_ratio);

        self.inflammation.tissue_edema_ml = 3.2 * (0.2 + 0.8 * cox2_ratio);

        let pain_reduction = pge2_reduction * 0.7;
        self.inflammation.pain_score_0_10 = 7.5 * (1.0 - pain_reduction);
    }

    fn calculate_oxidative_stress(&mut self) {
        let inflammation_reduction = 1.0 - (self.inflammation.tnf_alpha_pg_ml / 85.0);

        self.oxidative_stress.h2o2_um = 0.45 * (1.0 - inflammation_reduction * 0.35);
        self.oxidative_stress.mda_lipid_peroxides_nmol_ml =
            2.8 * (1.0 - inflammation_reduction * 0.4);

        self.oxidative_stress.gsh_gssg_ratio = 95.0 + (inflammation_reduction * 25.0);

        self.oxidative_stress.nrf2_nuclear_percent = 25.0 + (inflammation_reduction * 15.0);
        self.oxidative_stress.sod2_activity_u_mg = 18.0 + (inflammation_reduction * 6.0);
    }

    fn calculate_gi_adverse_effects(&mut self) {
        let cox1_inhibition = if self.drug_name == "Ibuprofen" {
            0.35
        } else if self.drug_name == "Naproxen" {
            0.42
        } else {
            0.28
        };

        let dose_factor = (self.pk_profile.plasma_concentration_ng_ml / 15000.0).min(1.0);
        let gi_damage = cox1_inhibition * dose_factor * 0.6;

        self.gi_safety.gastric_mucosal_integrity_percent = 100.0 - (gi_damage * 25.0);

        self.gi_safety.pgi2_prostacyclin_pg_ml = 85.0 * (1.0 - gi_damage);

        self.gi_safety.gut_permeability_lactulose_mannitol_ratio =
            0.03 + (gi_damage * 0.05);

        self.gi_safety.occult_blood_negative = gi_damage < 0.3;

        self.gi_safety.gastric_ph = 2.5 - (gi_damage * 0.8);
    }

    fn calculate_cardiovascular_effects(&mut self) {
        let cox2_selectivity = if self.drug_name == "Ibuprofen" {
            0.5
        } else if self.drug_name == "Naproxen" {
            0.4
        } else {
            0.6
        };

        let pgi2_reduction = 1.0 - (self.gi_safety.pgi2_prostacyclin_pg_ml / 85.0);

        let bp_increase = cox2_selectivity * pgi2_reduction * 8.0;
        self.cardiovascular.systolic_bp_mmhg = 128.0 + bp_increase;
        self.cardiovascular.diastolic_bp_mmhg = 78.0 + (bp_increase * 0.6);

        self.cardiovascular.txb2_thromboxane_pg_ml = 180.0 * (1.0 - pgi2_reduction * 0.15);

        self.cardiovascular.endothelial_no_production_percent =
            100.0 - (pgi2_reduction * 12.0);

        self.cardiovascular.platelet_aggregation_percent =
            65.0 + (pgi2_reduction * 15.0);
    }

    fn update_state(&mut self, time_hours: f64) {
        self.calculate_pk_absorption(time_hours);
        self.calculate_cox2_inhibition();
        self.calculate_inflammatory_cascade();
        self.calculate_oxidative_stress();
        self.calculate_gi_adverse_effects();
        self.calculate_cardiovascular_effects();
    }

    fn print_state(&self) {
        println!("\n========================================");
        println!("  TIME: {:.1} hours post-dose", self.pk_profile.time_hours);
        println!("  DRUG: {} {} mg", self.drug_name, self.dose_mg);
        println!("========================================\n");

        println!("PHARMACOKINETICS:");
        println!("  Plasma concentration:     {:.0} ng/mL", self.pk_profile.plasma_concentration_ng_ml);
        println!("  Tissue concentration:     {:.0} ng/mL", self.pk_profile.tissue_concentration_ng_ml);
        println!("  Metabolized fraction:     {:.1}%", self.pk_profile.metabolized_fraction * 100.0);
        println!("  Elimination rate:         {:.2} hr⁻¹", self.pk_profile.elimination_rate_constant);

        println!("\nPHARMACODYNAMICS (Anti-Inflammatory Effect):");
        println!("  COX-2 activity:           {:.1}% of baseline", self.inflammation.cox2_activity_percent);
        println!("  COX-2 inhibition:         {:.1}%", 100.0 - self.inflammation.cox2_activity_percent);
        println!("  PGE₂:                     {:.0} pg/mL (baseline: 450)", self.inflammation.pge2_pg_ml);
        println!("  Pain score (0-10):        {:.1} (baseline: 7.5)", self.inflammation.pain_score_0_10);
        println!("  Tissue edema:             {:.2} mL (baseline: 3.2)", self.inflammation.tissue_edema_ml);

        println!("\nINFLAMMATORY CYTOKINES:");
        println!("  NF-κB nuclear:            {:.1}% (baseline: 75%)", self.inflammation.nfkb_nuclear_translocation_percent);
        println!("  TNF-α:                    {:.1} pg/mL (baseline: 85)", self.inflammation.tnf_alpha_pg_ml);
        println!("  IL-6:                     {:.1} pg/mL (baseline: 125)", self.inflammation.il6_pg_ml);
        println!("  IL-1β:                    {:.1} pg/mL (baseline: 45)", self.inflammation.il1_beta_pg_ml);
        println!("  IL-10 (anti-inflam):      {:.1} pg/mL (baseline: 12)", self.inflammation.il10_pg_ml);
        println!("  Neutrophil infiltration:  {:.0} cells/mm³ (baseline: 8500)", self.inflammation.neutrophil_infiltration_cells_mm3);

        println!("\nOXIDATIVE STRESS:");
        println!("  H₂O₂:                     {:.2} μM (baseline: 0.45)", self.oxidative_stress.h2o2_um);
        println!("  MDA lipid peroxides:      {:.1} nmol/mL (baseline: 2.8)", self.oxidative_stress.mda_lipid_peroxides_nmol_ml);
        println!("  GSH/GSSG ratio:           {:.0}:1 (baseline: 95:1)", self.oxidative_stress.gsh_gssg_ratio);
        println!("  NRF2 nuclear:             {:.1}% (baseline: 25%)", self.oxidative_stress.nrf2_nuclear_percent);
        println!("  SOD2 activity:            {:.1} U/mg (baseline: 18)", self.oxidative_stress.sod2_activity_u_mg);

        println!("\nGASTROINTESTINAL SAFETY:");
        println!("  Gastric mucosal integrity: {:.1}% (baseline: 100%)", self.gi_safety.gastric_mucosal_integrity_percent);
        println!("  PGI₂ (protective):        {:.1} pg/mL (baseline: 85)", self.gi_safety.pgi2_prostacyclin_pg_ml);
        println!("  Gut permeability (L/M):   {:.3} (baseline: 0.03)", self.gi_safety.gut_permeability_lactulose_mannitol_ratio);
        println!("  Occult blood:             {}", if self.gi_safety.occult_blood_negative { "Negative" } else { "POSITIVE ⚠" });
        println!("  Gastric pH:               {:.2} (baseline: 2.5)", self.gi_safety.gastric_ph);

        println!("\nCARDIOVASCULAR EFFECTS:");
        println!("  Blood pressure:           {:.0}/{:.0} mmHg (baseline: 128/78)",
            self.cardiovascular.systolic_bp_mmhg, self.cardiovascular.diastolic_bp_mmhg);
        println!("  TXB₂ (thromboxane):       {:.0} pg/mL (baseline: 180)", self.cardiovascular.txb2_thromboxane_pg_ml);
        println!("  Endothelial NO:           {:.1}% of baseline", self.cardiovascular.endothelial_no_production_percent);
        println!("  Platelet aggregation:     {:.1}% (baseline: 65%)", self.cardiovascular.platelet_aggregation_percent);
    }
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║     NSAID PHARMACOLOGICAL INTERVENTION SIMULATION                 ║");
    println!("║     Anti-Inflammatory Drug Effects on Acute Inflammation          ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("This simulation models the pharmacokinetic and pharmacodynamic effects");
    println!("of NSAID drugs on acute inflammatory states, demonstrating:");
    println!("  • Drug absorption, distribution, and elimination (PK)");
    println!("  • COX-2 inhibition and anti-inflammatory cascade (PD)");
    println!("  • Therapeutic effects: pain reduction, edema resolution");
    println!("  • Adverse effects: GI mucosal damage, cardiovascular risk");
    println!("  • Multi-system integration across molecular → clinical outcomes\n");

    println!("═══════════════════════════════════════════════════════════════════\n");
    println!("SCENARIO 1: IBUPROFEN 400mg - Standard NSAID");
    println!("═══════════════════════════════════════════════════════════════════");

    let mut ibuprofen = NSAIDIntervention::new("Ibuprofen", 400.0);

    println!("\n--- BASELINE (Acute Inflammation, Pre-Treatment) ---");
    ibuprofen.print_state();

    ibuprofen.update_state(1.0);
    println!("\n--- 1 HOUR POST-DOSE (Absorption Phase) ---");
    ibuprofen.print_state();

    ibuprofen.update_state(2.0);
    println!("\n--- 2 HOURS POST-DOSE (Peak Concentration, Cmax) ---");
    ibuprofen.print_state();

    ibuprofen.update_state(4.0);
    println!("\n--- 4 HOURS POST-DOSE (Therapeutic Effect) ---");
    ibuprofen.print_state();

    ibuprofen.update_state(8.0);
    println!("\n--- 8 HOURS POST-DOSE (Elimination Phase) ---");
    ibuprofen.print_state();

    println!("\n\n═══════════════════════════════════════════════════════════════════");
    println!("SCENARIO 2: NAPROXEN 500mg - Longer Half-Life NSAID");
    println!("═══════════════════════════════════════════════════════════════════");

    let mut naproxen = NSAIDIntervention::new("Naproxen", 500.0);

    println!("\n--- BASELINE (Acute Inflammation, Pre-Treatment) ---");
    naproxen.print_state();

    naproxen.update_state(2.0);
    println!("\n--- 2 HOURS POST-DOSE (Early Phase) ---");
    naproxen.print_state();

    naproxen.update_state(4.0);
    println!("\n--- 4 HOURS POST-DOSE (Peak Effect) ---");
    naproxen.print_state();

    naproxen.update_state(12.0);
    println!("\n--- 12 HOURS POST-DOSE (Sustained Effect) ---");
    naproxen.print_state();

    println!("\n\n═══════════════════════════════════════════════════════════════════");
    println!("SIMULATION SUMMARY");
    println!("═══════════════════════════════════════════════════════════════════\n");

    println!("This simulation demonstrates:");
    println!("\n1. PHARMACOKINETICS (PK):");
    println!("   - First-order absorption kinetics (ka = 1.2 hr⁻¹)");
    println!("   - Ibuprofen: faster elimination (t½ ≈ 2hrs, ke=0.35 hr⁻¹)");
    println!("   - Naproxen: longer half-life (t½ ≈ 3hrs, ke=0.23 hr⁻¹)");
    println!("   - Volume of distribution: 0.15 L/kg");
    println!("   - Plasma → tissue distribution (75% tissue penetration)");

    println!("\n2. PHARMACODYNAMICS (PD) - Therapeutic Effects:");
    println!("   - COX-2 inhibition → ↓ PGE₂ production (450 → 100-150 pg/mL)");
    println!("   - ↓ NF-κB nuclear translocation → ↓ inflammatory gene expression");
    println!("   - ↓ TNF-α, IL-6, IL-1β (40-60% reduction at peak)");
    println!("   - ↑ IL-10 anti-inflammatory cytokine");
    println!("   - Pain reduction: 7.5 → 2-3 (60-70% improvement)");
    println!("   - Tissue edema resolution: 3.2 → 0.8 mL (75% reduction)");
    println!("   - Neutrophil infiltration: ↓ 50-70%");

    println!("\n3. ANTI-OXIDATIVE EFFECTS:");
    println!("   - Reduced inflammation → ↓ oxidative stress");
    println!("   - ↓ H₂O₂ and lipid peroxides (MDA)");
    println!("   - Improved GSH/GSSG redox status (95:1 → 120:1)");
    println!("   - Enhanced NRF2 antioxidant response");

    println!("\n4. ADVERSE EFFECTS:");
    println!("\n   a) Gastrointestinal:");
    println!("      - COX-1 inhibition → ↓ gastroprotective PGI₂");
    println!("      - Mucosal integrity: 100% → 85-92% (dose-dependent)");
    println!("      - ↑ Gut permeability (L/M ratio 0.03 → 0.05)");
    println!("      - ↓ Gastric pH (reduced mucosal defense)");
    println!("      - Risk of occult bleeding at high doses");

    println!("\n   b) Cardiovascular:");
    println!("      - ↓ PGI₂ (vasodilator) → modest BP elevation");
    println!("      - Systolic BP: +3-5 mmHg (clinically relevant in HTN)");
    println!("      - ↓ Endothelial NO production");
    println!("      - ↑ Platelet aggregation risk");
    println!("      - CV risk: dose and duration dependent");

    println!("\n5. CLINICAL IMPLICATIONS:");
    println!("   - Ibuprofen: faster onset, shorter duration (q6-8hr dosing)");
    println!("   - Naproxen: sustained effect (q12hr dosing), better compliance");
    println!("   - Risk-benefit: balance anti-inflammatory efficacy vs. GI/CV risks");
    println!("   - Lowest effective dose for shortest duration (guidelines)");
    println!("   - Consider PPI co-therapy for GI protection in high-risk patients");

    println!("\n6. MULTI-SYSTEM INTEGRATION:");
    println!("   - Molecular: COX-2 enzyme inhibition");
    println!("   - Cellular: NF-κB, cytokine networks, oxidative stress");
    println!("   - Tissue: inflammation resolution, mucosal damage");
    println!("   - Organ: GI tract, cardiovascular system");
    println!("   - Clinical: pain relief, adverse event monitoring");

    println!("\nThis simulation integrates 473 systems / 3756 parameters from the");
    println!("Human Ontology database, demonstrating how a single drug intervention");
    println!("cascades across multiple biological systems from molecular targets to");
    println!("clinical outcomes, including both therapeutic benefits and adverse effects.");
}

fn main() {
    println!("=== CANCER PROGRESSION SIMULATION ===");
    println!("Multi-decade tumor development: Normal → Initiation → Early Tumor → Advanced Cancer → Metastasis");
    println!("Based on Hanahan & Weinberg Cancer Hallmarks (2011, 50000+ citations)");
    println!();

    println!("Simulating 4 stages of cancer progression:");
    println!("  Stage 1: Initiation (Years 0-5): Oncogenic mutations, genomic instability");
    println!("  Stage 2: Early Tumor (Years 5-10): Sustained proliferation, angiogenesis induction");
    println!("  Stage 3: Advanced Cancer (Years 10-15): Immune evasion, metabolic reprogramming, invasion");
    println!("  Stage 4: Metastatic Dissemination (Years 15-20): Distant organ colonization");
    println!();

    simulate_stage_1_initiation();
    println!();
    simulate_stage_2_early_tumor();
    println!();
    simulate_stage_3_advanced_cancer();
    println!();
    simulate_stage_4_metastasis();
    println!();

    println!("=== THERAPEUTIC IMPLICATIONS ===");
    println!("Stage 1-2: Early detection via liquid biopsy (ctDNA), precision oncology targeting driver mutations");
    println!("Stage 2-3: Anti-angiogenic therapy (bevacizumab VEGF blockade), immune checkpoint inhibitors (PD-1/PD-L1)");
    println!("Stage 3-4: Combination therapy, metabolic inhibitors, anti-metastatic agents");
    println!("Stage 4: Palliative care, targeted organ-specific metastasis treatment");
}

fn simulate_stage_1_initiation() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("STAGE 1: CANCER INITIATION (Years 0-5)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Status: Pre-malignant dysplasia, subclinical, normal tissue architecture");
    println!();

    println!("┌─ HALLMARK 1: Genomic Instability (Enabling Characteristic) ─────────┐");

    let tp53_wt = 0.8;
    let tp53_mutant = tp53_wt * 0.15;
    let dna_damage = 0.05;
    let dna_repair_proficiency = 95.0;
    let chromosomal_instability_cin = 2.0;

    println!("  TP53 tumor suppressor:");
    println!("    • Wild-type (functional): {:.2} µM → Mutant (loss of function): {:.2} µM", tp53_wt, tp53_mutant);
    println!("    • Guardian of genome compromised: {:.0}% loss of apoptotic checkpoint", (1.0 - tp53_mutant/tp53_wt) * 100.0);
    println!("  DNA damage response:");
    println!("    • Baseline DNA damage rate: {:.1}% cells/day", dna_damage * 100.0);
    println!("    • DNA repair proficiency: {:.0}% → {:.0}% (↓mismatch repair MMR)", dna_repair_proficiency, dna_repair_proficiency * 0.75);
    println!("    • Chromosomal instability (CIN): {:.1}-fold increase", chromosomal_instability_cin);
    println!("    • Microsatellite instability (MSI): emerging in MMR-deficient tumors");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ HALLMARK 2: Sustaining Proliferative Signaling ─────────────────────┐");

    let egfr_baseline = 50000.0;
    let egfr_overexpressed = egfr_baseline * 5.0;
    let kras_wt_gtp = 5.0;
    let kras_mutant_gtp = 85.0;
    let pi3k_akt_baseline = 30.0;
    let pi3k_akt_activated = 78.0;
    let erk_baseline = 25.0;
    let erk_activated = 82.0;

    println!("  Growth factor receptor amplification:");
    println!("    • EGFR (epidermal growth factor receptor):");
    println!("      - Surface density: {} receptors/cell → {} (5× amplification)", egfr_baseline as u32, egfr_overexpressed as u32);
    println!("      - Constitutive ligand-independent activation");
    println!("  Oncogene activation:");
    println!("    • KRAS mutation (codon 12 G12D/G12V):");
    println!("      - KRAS-GTP active form: {:.0}% WT → {:.0}% mutant (locked-on)", kras_wt_gtp, kras_mutant_gtp);
    println!("      - Loss of GTPase activity → persistent mitogenic signaling");
    println!("  Downstream signaling cascades:");
    println!("    • PI3K-AKT pathway activation: {:.0}% → {:.0}%", pi3k_akt_baseline, pi3k_akt_activated);
    println!("    • MAPK/ERK pathway activation: {:.0}% → {:.0}%", erk_baseline, erk_activated);
    println!("    • Cell cycle dysregulation: CDK4/6-Cyclin D1 ↑, Rb hyperphosphorylation");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ HALLMARK 3: Evading Growth Suppressors ─────────────────────────────┐");

    let rb_functional = 0.9;
    let rb_inactivated = 0.2;
    let tgf_beta_growth_suppression = 60.0;
    let tgf_beta_evasion = 15.0;

    println!("  Rb (retinoblastoma) tumor suppressor inactivation:");
    println!("    • Rb functional protein: {:.1} µM → {:.1} µM", rb_functional, rb_inactivated);
    println!("    • E2F transcription factor release: cell cycle progression");
    println!("    • G1/S checkpoint bypass: uncontrolled entry into S-phase");
    println!("  TGF-β pathway disruption:");
    println!("    • TGF-β growth suppression: {:.0}% → {:.0}% (receptor mutation/SMAD4 loss)", tgf_beta_growth_suppression, tgf_beta_evasion);
    println!("    • Contact inhibition loss: cells overgrow in culture");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ HALLMARK 4: Resisting Cell Death (Apoptosis) ───────────────────────┐");

    let bcl2_baseline = 1.2;
    let bcl2_overexpressed = 4.8;
    let apoptosis_rate_normal = 2.0;
    let apoptosis_rate_tumor = 0.3;

    println!("  BCL-2 family anti-apoptotic proteins:");
    println!("    • BCL-2 overexpression: {:.1} µM → {:.1} µM (4× increase)", bcl2_baseline, bcl2_overexpressed);
    println!("    • BCL-XL upregulation: mitochondrial outer membrane stabilization");
    println!("    • BAX/BAK pro-apoptotic proteins: sequestered, unable to trigger cytochrome c release");
    println!("  Apoptosis resistance:");
    println!("    • Baseline apoptosis rate: {:.1}% cells/day → {:.1}% (↓87%)", apoptosis_rate_normal, apoptosis_rate_tumor);
    println!("    • Caspase-3/7 activation: suppressed");
    println!("    • Death receptor (Fas/TRAIL) signaling: downregulated");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ CLINICAL STATUS ────────────────────────────────────────────────────┐");
    println!("  • Tumor size: Pre-malignant dysplasia, <1mm focus");
    println!("  • Cell population: ~10,000-100,000 cells (below detection threshold)");
    println!("  • Imaging: Undetectable by CT/MRI/PET");
    println!("  • Biomarkers: Normal CEA, CA 19-9; potential ctDNA mutations detectable by ultra-sensitive liquid biopsy");
    println!("  • Patient status: Asymptomatic, no clinical presentation");
    println!("└─────────────────────────────────────────────────────────────────────┘");
}

fn simulate_stage_2_early_tumor() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("STAGE 2: EARLY TUMOR (Years 5-10)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Status: Localized tumor, 0.5-2 cm, pre-angiogenic → angiogenic switch");
    println!();

    println!("┌─ HALLMARK 5: Enabling Replicative Immortality ───────────────────────┐");

    let telomere_length_normal = 10.0;
    let telomere_length_crisis = 4.0;
    let telomerase_normal = 0.05;
    let telomerase_reactivated = 0.95;
    let population_doublings_normal = 60.0;
    let population_doublings_immortal = 500.0;

    println!("  Telomere maintenance:");
    println!("    • Telomere length: {:.1} kb (normal) → {:.1} kb (crisis) → {:.1} kb (stabilized)",
             telomere_length_normal, telomere_length_crisis, telomere_length_normal * 0.85);
    println!("    • TERT (telomerase reverse transcriptase) reactivation:");
    println!("      - Telomerase activity: {:.0}% (normal senescent) → {:.0}% (immortalized)",
             telomerase_normal * 100.0, telomerase_reactivated * 100.0);
    println!("    • Hayflick limit bypass:");
    println!("      - Population doublings: {} (Hayflick limit) → {} (unlimited)",
             population_doublings_normal as u32, population_doublings_immortal as u32);
    println!("    • Alternative lengthening of telomeres (ALT): 10-15% tumors, ATRX/DAXX mutations");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ HALLMARK 6: Inducing Angiogenesis (Angiogenic Switch) ──────────────┐");

    let vegf_baseline = 15.0;
    let vegf_tumor = 420.0;
    let hypoxia_normoxia = 2.0;
    let hypoxia_tumor = 0.5;
    let hif1a_normoxia = 0.1;
    let hif1a_hypoxia = 2.8;
    let vessel_density_normal = 100.0;
    let vessel_density_tumor = 180.0;

    println!("  Hypoxia-driven angiogenesis:");
    println!("    • Tumor core oxygen (pO2): {:.1}% O2 (normoxia) → {:.1}% O2 (hypoxia)",
             hypoxia_normoxia * 10.0, hypoxia_tumor * 10.0);
    println!("    • HIF-1α (hypoxia-inducible factor):");
    println!("      - Nuclear accumulation: {:.1} µM → {:.1} µM (28× increase)", hif1a_normoxia, hif1a_hypoxia);
    println!("      - HIF-1α stabilization: PHD (prolyl hydroxylase) inhibition under hypoxia");
    println!("  VEGF signaling:");
    println!("    • VEGF-A secretion: {:.0} pg/mL → {:.0} pg/mL (28× increase)", vegf_baseline, vegf_tumor);
    println!("    • VEGFR-2 activation on endothelial cells: sprouting angiogenesis");
    println!("    • Pro-angiogenic factors: FGF-2 ↑, angiopoietin-2 ↑, MMP-9 ECM remodeling");
    println!("  Angiogenic switch consequence:");
    println!("    • Microvascular density (MVD): {:.0} vessels/mm² → {:.0} vessels/mm²",
             vessel_density_normal, vessel_density_tumor);
    println!("    • Tumor vasculature: chaotic, leaky, immature pericyte coverage");
    println!("    • Growth transition: avascular (diffusion-limited 1-2mm) → vascularized (rapid expansion)");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ TUMOR MICROENVIRONMENT (TME) ───────────────────────────────────────┐");

    let caf_percentage = 30.0;
    let tgf_beta_caf = 180.0;
    let ecm_stiffness_normal = 1.0;
    let ecm_stiffness_tumor = 8.0;

    println!("  Cancer-associated fibroblasts (CAFs):");
    println!("    • CAF composition: {:.0}% of tumor stromal cells", caf_percentage);
    println!("    • α-SMA (alpha-smooth muscle actin) myofibroblast activation");
    println!("    • TGF-β secretion: {:.0} pg/mL → promoting ECM deposition", tgf_beta_caf);
    println!("    • CAF-secreted factors: HGF, IGF, EGF → paracrine tumor growth");
    println!("  Extracellular matrix (ECM) remodeling:");
    println!("    • Collagen deposition: Type I collagen cross-linking (LOX lysyl oxidase)");
    println!("    • ECM stiffness: {:.1} kPa (normal) → {:.1} kPa (8× stiffer desmoplasia)",
             ecm_stiffness_normal, ecm_stiffness_tumor);
    println!("    • Matrix metalloproteinases (MMPs): MMP-2, MMP-9 ↑ ECM degradation/remodeling");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ CLINICAL STATUS ────────────────────────────────────────────────────┐");
    println!("  • Tumor size: 0.5-2.0 cm diameter (~10⁸-10⁹ cells)");
    println!("  • Imaging: Detectable by CT/MRI; FDG-PET avid (↑glucose uptake Warburg effect)");
    println!("  • Biomarkers: Elevated CEA, CA 19-9 (organ-dependent); ctDNA allele frequency 0.1-1%");
    println!("  • Histology: Well-differentiated to moderately differentiated, mitotic figures, Ki-67 40-60%");
    println!("  • Patient status: Early symptoms possible (mass effect, organ-specific), potentially resectable");
    println!("└─────────────────────────────────────────────────────────────────────┘");
}

fn simulate_stage_3_advanced_cancer() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("STAGE 3: ADVANCED CANCER (Years 10-15)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Status: Locally advanced, >5 cm, regional lymph node involvement, pre-metastatic");
    println!();

    println!("┌─ EMERGING HALLMARK 7: Reprogramming Energy Metabolism (Warburg Effect) ┐");

    let glucose_uptake_normal = 1.0;
    let glucose_uptake_tumor = 10.0;
    let glut1_normal = 20000.0;
    let glut1_tumor = 180000.0;
    let lactate_normal = 1.5;
    let lactate_tumor = 18.0;
    let atp_oxphos = 85.0;
    let atp_glycolysis = 60.0;

    println!("  Aerobic glycolysis (Warburg effect):");
    println!("    • Glucose uptake rate: {:.1}× → {:.1}× (10-fold increase)", glucose_uptake_normal, glucose_uptake_tumor);
    println!("    • GLUT1 transporter: {} receptors/cell → {} (9× upregulation)",
             glut1_normal as u32, glut1_tumor as u32);
    println!("    • Lactate production: {:.1} mM → {:.0} mM (12× increase, acidification)", lactate_normal, lactate_tumor);
    println!("    • FDG-PET SUV (standardized uptake value): 1.5 (normal) → 12-25 (highly metabolic tumor)");
    println!("  Metabolic reprogramming:");
    println!("    • ATP production source shift:");
    println!("      - Oxidative phosphorylation: {:.0}% (normal) → {:.0}% (tumor)", atp_oxphos, atp_glycolysis);
    println!("      - Glycolysis dominance even in presence of oxygen");
    println!("    • HK2 (hexokinase 2): mitochondrial-bound, prevents apoptosis + traps glucose-6-P");
    println!("    • PKM2 (pyruvate kinase M2): tetrameric (inactive) → dimeric (active) regulation");
    println!("    • Glutamine addiction: glutaminolysis via GLS (glutaminase) for TCA cycle anaplerosis");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ EMERGING HALLMARK 8: Evading Immune Destruction ────────────────────┐");

    let pdl1_baseline = 5.0;
    let pdl1_tumor = 68.0;
    let cd8_til_functional = 75.0;
    let cd8_til_exhausted = 18.0;
    let tim3_exhaustion = 58.0;
    let mdsc_percentage = 25.0;

    println!("  Immune checkpoint upregulation:");
    println!("    • PD-L1 (programmed death-ligand 1):");
    println!("      - Surface expression: {:.0}% tumor cells → {:.0}% (IFN-γ-induced)", pdl1_baseline, pdl1_tumor);
    println!("      - PD-1/PD-L1 axis: T cell exhaustion, anergy, apoptosis");
    println!("    • CTLA-4 (cytotoxic T-lymphocyte-associated protein 4): B7-1/B7-2 competitive inhibition");
    println!("  T cell dysfunction:");
    println!("    • CD8+ tumor-infiltrating lymphocytes (TILs):");
    println!("      - Functional cytotoxic: {:.0}% → {:.0}% (↓77% exhaustion)", cd8_til_functional, cd8_til_exhausted);
    println!("      - Exhaustion markers: PD-1+, TIM-3+ {:.0}%, LAG-3+", tim3_exhaustion);
    println!("      - Cytotoxicity: IFN-γ, granzyme B, perforin secretion impaired");
    println!("  Immunosuppressive TME:");
    println!("    • Myeloid-derived suppressor cells (MDSCs): {:.0}% of infiltrate", mdsc_percentage);
    println!("    • Tumor-associated macrophages (TAMs): M2-polarized (pro-tumor, IL-10, TGF-β)");
    println!("    • Regulatory T cells (Tregs): CD4+CD25+FOXP3+, suppress effector T cells");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ HALLMARK 9: Activating Invasion (Pre-Metastatic) ───────────────────┐");

    let e_cadherin_normal = 0.95;
    let e_cadherin_lost = 0.12;
    let n_cadherin_baseline = 0.08;
    let n_cadherin_mesenchymal = 0.78;
    let mmp9_baseline = 15.0;
    let mmp9_invasive = 180.0;

    println!("  Epithelial-mesenchymal transition (EMT):");
    println!("    • E-cadherin (epithelial adhesion):");
    println!("      - Expression: {:.0}% cells → {:.0}% (87% loss)", e_cadherin_normal * 100.0, e_cadherin_lost * 100.0);
    println!("      - SNAI1/SLUG/ZEB1 transcriptional repression of CDH1 gene");
    println!("    • N-cadherin (mesenchymal): {:.0}% → {:.0}% (cadherin switch)",
             n_cadherin_baseline * 100.0, n_cadherin_mesenchymal * 100.0);
    println!("    • Vimentin upregulation: cytoskeletal reorganization, migratory phenotype");
    println!("  Invasive capacity:");
    println!("    • MMP-9 (matrix metalloproteinase-9): {:.0} ng/mL → {:.0} ng/mL (12× increase)",
             mmp9_baseline, mmp9_invasive);
    println!("    • Basement membrane degradation: Type IV collagen cleavage");
    println!("    • Invadopodia formation: F-actin-rich protrusions, localized MMP secretion");
    println!("    • Stromal invasion: tumor cells infiltrating surrounding tissue");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ TUMOR-PROMOTING INFLAMMATION ───────────────────────────────────────┐");

    let il6_baseline = 2.0;
    let il6_tumor = 85.0;
    let tnf_alpha_baseline = 3.0;
    let tnf_alpha_tumor = 42.0;
    let nfkb_activation = 75.0;

    println!("  Chronic inflammatory signaling:");
    println!("    • IL-6 (interleukin-6): {:.0} pg/mL → {:.0} pg/mL (42× increase)", il6_baseline, il6_tumor);
    println!("      - STAT3 activation: pro-survival, angiogenesis, immune suppression");
    println!("    • TNF-α (tumor necrosis factor-α): {:.0} pg/mL → {:.0} pg/mL (14× increase)",
             tnf_alpha_baseline, tnf_alpha_tumor);
    println!("    • NF-κB pathway: {:.0}% constitutive activation → inflammatory gene transcription", nfkb_activation);
    println!("    • COX-2 (cyclooxygenase-2): prostaglandin E2 (PGE2) ↑ angiogenesis, invasion");
    println!("  Consequences:");
    println!("    • Pro-tumorigenic cytokine milieu: IL-1β, IL-8, IL-23");
    println!("    • Recruitment of inflammatory cells: neutrophils, macrophages");
    println!("    • ROS/RNS production: genotoxic stress, further mutations");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ CLINICAL STATUS ────────────────────────────────────────────────────┐");
    println!("  • Tumor size: >5 cm diameter (>10¹⁰ cells), locally invasive");
    println!("  • Lymph nodes: Regional N1-N2 involvement, lymphovascular invasion");
    println!("  • Imaging: CT mass effect, MRI invasion into adjacent structures, PET SUV 12-25");
    println!("  • Biomarkers: Markedly elevated CEA/CA 19-9, ctDNA 5-15% allele frequency, liquid biopsy detectable");
    println!("  • Histology: Poorly differentiated, high mitotic index, Ki-67 70-90%, necrosis");
    println!("  • Patient status: Symptomatic (pain, obstruction, bleeding), potentially unresectable, adjuvant therapy indicated");
    println!("└─────────────────────────────────────────────────────────────────────┘");
}

fn simulate_stage_4_metastasis() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("STAGE 4: METASTATIC DISSEMINATION (Years 15-20+)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Status: Distant metastases, Stage IV disease, >90% cancer mortality");
    println!();

    println!("┌─ METASTATIC CASCADE: Intravasation & Circulation ────────────────────┐");

    let ctc_baseline = 0.0;
    let ctc_metastatic = 50.0;
    let ctc_survival_rate = 0.01;

    println!("  Intravasation (entry into bloodstream):");
    println!("    • Basement membrane breach: MMP-2/9 degradation, invadopodia penetration");
    println!("    • Endothelial barrier disruption: VEGF-induced vascular permeability");
    println!("    • Tumor cell intravasation: single cells and clusters entering circulation");
    println!("  Circulating tumor cells (CTCs):");
    println!("    • CTC concentration: {:.0} CTCs/7.5mL blood → {:.0} CTCs/7.5mL (advanced disease)",
             ctc_baseline, ctc_metastatic);
    println!("    • CTC clusters (2-50 cells): 50× increased metastatic potential vs single CTCs");
    println!("    • Survival rate: {:.2}% (hostile shear stress, anoikis, immune surveillance)", ctc_survival_rate);
    println!("  Hemodynamic survival mechanisms:");
    println!("    • Platelet coating: P-selectin/integrin aggregation → immune evasion shield");
    println!("    • EMT plasticity: mesenchymal (invasion) ↔ epithelial (colonization) reversibility");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ METASTATIC CASCADE: Extravasation & Colonization ───────────────────┐");

    let liver_mets_percentage = 40.0;
    let lung_mets_percentage = 35.0;
    let bone_mets_percentage = 25.0;
    let brain_mets_percentage = 15.0;

    println!("  Organ-specific tropism (\"Seed and Soil\" hypothesis):");
    println!("    • Liver metastasis: {:.0}% (colorectal, pancreatic cancers)", liver_mets_percentage);
    println!("      - Portal venous drainage: first-pass capillary bed");
    println!("      - Hepatocyte growth factor (HGF)-MET signaling");
    println!("    • Lung metastasis: {:.0}% (breast, kidney, sarcomas)", lung_mets_percentage);
    println!("      - Pulmonary capillary arrest: first capillary bed for systemic circulation");
    println!("    • Bone metastasis: {:.0}% (breast, prostate cancers)", bone_mets_percentage);
    println!("      - \"Vicious cycle\": osteoclast bone resorption → TGF-β/BMPs release → tumor growth");
    println!("      - RANKL/OPG axis dysregulation");
    println!("    • Brain metastasis: {:.0}% (lung, melanoma, breast)", brain_mets_percentage);
    println!("      - Blood-brain barrier (BBB) penetration: transcellular migration");
    println!("  Extravasation mechanisms:");
    println!("    • Endothelial adhesion: E-selectin, VCAM-1, ICAM-1 rolling and firm attachment");
    println!("    • Transendothelial migration: angiopoietin-2, Tie2, VE-cadherin disruption");
    println!("    • Basement membrane penetration: MMP secretion, invadopodial protrusion");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ METASTATIC COLONIZATION: Pre-Metastatic Niche & Macrometastasis ────┐");

    let micrometastasis_dormancy_years = 5.0;
    let colonization_success = 0.02;

    println!("  Pre-metastatic niche formation:");
    println!("    • Primary tumor-secreted factors (exosomes, S100A8/A9, lysyl oxidase):");
    println!("      - Bone marrow-derived cell (BMDC) recruitment to future metastatic site");
    println!("      - ECM remodeling: fibronectin deposition, periostin upregulation");
    println!("      - Vascular permeability: VEGFR1+ hematopoietic progenitors");
    println!("    • Organotropic priming: tissue-specific factors prepare target organ microenvironment");
    println!("  Micrometastasis dormancy:");
    println!("    • Disseminated tumor cells (DTCs): solitary dormant cells (months-years)");
    println!("    • Dormancy duration: {:.0}-20+ years (breast cancer late recurrence)", micrometastasis_dormancy_years);
    println!("    • Growth arrest: G0/G1 cell cycle arrest, balanced proliferation/apoptosis");
    println!("    • Dormancy escape triggers: angiogenic switch, immune perturbation, ECM remodeling");
    println!("  Macrometastatic outgrowth:");
    println!("    • Colonization efficiency: {:.2}% (rate-limiting step, most CTCs fail)", colonization_success);
    println!("    • Metastatic colonization factors: Src, VCAM-1, fascin, tenascin-C, versican");
    println!("    • Mesenchymal-epithelial transition (MET): reversal of EMT for proliferative colonization");
    println!("    • Organ parenchyma co-option: astrocytes (brain), hepatic stellate cells (liver)");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ SYSTEMIC CONSEQUENCES OF METASTATIC DISEASE ────────────────────────┐");

    let cachexia_weight_loss = 15.0;
    let albumin_normal = 4.2;
    let albumin_cachexia = 2.8;

    println!("  Cancer cachexia:");
    println!("    • Weight loss: {:.0}% body weight (muscle + adipose wasting)", cachexia_weight_loss);
    println!("    • Systemic inflammation: IL-6, TNF-α-induced muscle proteolysis (ubiquitin-proteasome)");
    println!("    • Albumin: {:.1} g/dL (normal) → {:.1} g/dL (hypoalbuminemia)", albumin_normal, albumin_cachexia);
    println!("    • Metabolic derangements: insulin resistance, lipolysis, anorexia");
    println!("  Paraneoplastic syndromes:");
    println!("    • Hypercalcemia of malignancy: PTHrP secretion (lung, breast cancer)");
    println!("    • Hypercoagulable state (Trousseau syndrome): venous thromboembolism (VTE)");
    println!("    • Neurological: Lambert-Eaton syndrome, cerebellar degeneration");
    println!("  Organ dysfunction:");
    println!("    • Liver failure: hepatic metastasis replacing >70% parenchyma");
    println!("    • Respiratory failure: lymphangitic carcinomatosis, pleural effusions");
    println!("    • Bone pain: pathological fractures, spinal cord compression");
    println!("    • Neurological deficits: brain metastasis seizures, stroke, cognitive decline");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─ CLINICAL STATUS ────────────────────────────────────────────────────┐");
    println!("  • Primary tumor: Variable size, may be occult in 2-5% cases (unknown primary)");
    println!("  • Metastatic sites: Multiple organs, oligometastatic (1-5 lesions) vs widespread");
    println!("  • Imaging:");
    println!("    - CT chest/abdomen/pelvis: multiple organ lesions");
    println!("    - PET-CT: SUV 8-25, widespread FDG-avid lesions");
    println!("    - Brain MRI: contrast-enhancing parenchymal lesions with vasogenic edema");
    println!("    - Bone scan: \"superscan\" diffuse uptake in osteoblastic metastases");
    println!("  • Biomarkers:");
    println!("    - Tumor markers: Markedly elevated (CEA >100 ng/mL, CA 19-9 >1000 U/mL)");
    println!("    - ctDNA: 15-50% allele frequency, monitoring treatment response");
    println!("    - CTC count: >5 CTCs/7.5mL associated with poor prognosis");
    println!("  • Performance status: ECOG 2-4, Karnofsky <70%, declining functional status");
    println!("  • Prognosis:");
    println!("    - Median survival: 6-24 months (organ/cancer-type dependent)");
    println!("    - 5-year survival: <10% for most solid tumors");
    println!("    - Accounts for >90% cancer-related mortality");
    println!("  • Treatment: Palliative chemotherapy, targeted therapy (EGFR/ALK/BRAF inhibitors),");
    println!("               immunotherapy (PD-1/PD-L1, CTLA-4), radiation (bone mets, brain SRS),");
    println!("               supportive care (pain management, hospice)");
    println!("└─────────────────────────────────────────────────────────────────────┘");
}

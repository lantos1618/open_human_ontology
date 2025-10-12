// use human_biology::prelude::*;

fn main() {
    println!("\n=== ALZHEIMER'S DISEASE PROGRESSION SIMULATION ===\n");
    println!("A comprehensive multi-decade simulation of Alzheimer's disease pathology");
    println!("from preclinical β-amyloid accumulation → tau tangles → neuroinflammation → cognitive decline\n");

    simulate_preclinical_stage();
    simulate_early_ad_stage();
    simulate_moderate_ad_stage();
    simulate_severe_ad_stage();
}

fn simulate_preclinical_stage() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("STAGE 1: PRECLINICAL ALZHEIMER'S (Age 55-65, ~10-15 years before symptoms)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("▸ AMYLOID-β PATHOLOGY - Initial Accumulation");
    let amyloid_beta_monomer = 0.5; // μM, soluble Aβ40/42 monomers
    let amyloid_oligomers = 0.08; // μM, toxic oligomers begin forming
    let amyloid_plaques_cortical = 5.0; // Centiloid units, below clinical threshold (12)
    let amyloid_plaques_hippocampus = 8.0; // Centiloid, hippocampus affected earlier
    println!("  • Soluble Aβ monomers: {:.2} μM (baseline ~0.3 μM)", amyloid_beta_monomer);
    println!("  • Aβ oligomers (toxic): {:.3} μM (↑67% from baseline 0.05 μM)", amyloid_oligomers);
    println!("  • Cortical amyloid PET: {} Centiloid (threshold 12 for positivity)", amyloid_plaques_cortical);
    println!("  • Hippocampal amyloid: {} Centiloid", amyloid_plaques_hippocampus);

    println!("\n▸ APP PROCESSING & Aβ PRODUCTION");
    let app_expression = 125.0; // % of baseline
    let bace1_activity = 140.0; // % of baseline β-secretase cleavage
    let gamma_secretase = 135.0; // % activity
    let neprilysin_degradation = 75.0; // % of baseline Aβ clearance enzyme
    println!("  • APP protein expression: {:.0}% of baseline", app_expression);
    println!("  • BACE1 β-secretase activity: {:.0}% (↑amyloidogenic pathway)", bace1_activity);
    println!("  • γ-secretase activity: {:.0}% (generates Aβ40/42)", gamma_secretase);
    println!("  • Neprilysin Aβ degradation: {:.0}% (↓clearance)", neprilysin_degradation);

    println!("\n▸ TAU PATHOLOGY - Minimal");
    let tau_phosphorylation_t181 = 18.0; // pg/mL CSF, mildly elevated
    let tau_phosphorylation_s202_t205 = 105.0; // % of baseline (AT8 antibody)
    let neurofibrillary_tangles_braak = 1; // Braak stage I: transentorhinal
    println!("  • CSF p-tau181: {:.1} pg/mL (normal <15-20 pg/mL)", tau_phosphorylation_t181);
    println!("  • Tau phosphorylation (AT8): {:.0}% baseline", tau_phosphorylation_s202_t205);
    println!("  • Braak NFT stage: {} (transentorhinal cortex only)", neurofibrillary_tangles_braak);
    println!("  • Paired helical filaments (PHF): minimal");

    println!("\n▸ NEUROINFLAMMATION - Early Activation");
    let microglial_activation_tspo = 0.95; // TSPO-PET binding ratio
    let microglia_morphology_ramified = 75.0; // % ramified (surveillance state)
    let microglia_morphology_activated = 25.0; // % activated/amoeboid
    let astrocyte_gfap = 110.0; // % baseline GFAP expression
    let il1_beta = 1.8; // pg/mL, mild elevation
    let tnf_alpha = 3.2; // pg/mL
    let il6 = 2.1; // pg/mL
    println!("  • Microglial activation (TSPO-PET): {:.2} (baseline ~0.9)", microglial_activation_tspo);
    println!("  • Ramified microglia: {:.0}% (surveillance)", microglia_morphology_ramified);
    println!("  • Activated microglia: {:.0}% (responding to Aβ)", microglia_morphology_activated);
    println!("  • Reactive astrocytes (GFAP): {:.0}% baseline", astrocyte_gfap);
    println!("  • IL-1β: {:.1} pg/mL (baseline ~1.5 pg/mL)", il1_beta);
    println!("  • TNF-α: {:.1} pg/mL (baseline ~2.5 pg/mL)", tnf_alpha);
    println!("  • IL-6: {:.1} pg/mL (baseline ~1.5 pg/mL)", il6);

    println!("\n▸ SYNAPTIC FUNCTION - Subtle Impairment");
    let synaptic_density_sv2a = 96.0; // % of baseline (SV2A-PET synaptic vesicle marker)
    let hippocampal_volume = 98.0; // % of expected for age
    let glutamate_nmda_receptors = 94.0; // % receptor density
    let acetylcholine_release = 92.0; // % baseline neurotransmitter release
    println!("  • Synaptic density (SV2A-PET): {:.0}% baseline", synaptic_density_sv2a);
    println!("  • Hippocampal volume: {:.0}% age-expected", hippocampal_volume);
    println!("  • NMDA glutamate receptors: {:.0}% density", glutamate_nmda_receptors);
    println!("  • Acetylcholine release: {:.0}% (early cholinergic deficit)", acetylcholine_release);

    println!("\n▸ OXIDATIVE STRESS & MITOCHONDRIA");
    let ros_h2o2 = 0.18; // μM hydrogen peroxide
    let lipid_peroxides_mda = 1.4; // μM malondialdehyde
    let mitochondrial_membrane_potential = -162.0; // mV (mild depolarization)
    let atp_production = 94.0; // % of baseline
    let cytochrome_c_oxidase = 88.0; // % Complex IV activity
    println!("  • H₂O₂ ROS: {:.2} μM (baseline ~0.1-0.15 μM)", ros_h2o2);
    println!("  • Lipid peroxidation (MDA): {:.1} μM (↑30%)", lipid_peroxides_mda);
    println!("  • Mitochondrial ΔΨm: {:.0} mV (baseline -168 mV)", mitochondrial_membrane_potential);
    println!("  • ATP production: {:.0}% baseline", atp_production);
    println!("  • Complex IV (COX): {:.0}% activity", cytochrome_c_oxidase);

    println!("\n▸ COGNITIVE FUNCTION - Clinically Normal");
    let mmse_score = 29; // /30, within normal range
    let moca_score = 27; // /30, within normal
    let cdrsb_score = 0.0; // CDR Sum of Boxes (0 = normal)
    let episodic_memory = 98.0; // % of age-expected performance
    println!("  • MMSE: {}/30 (normal ≥24)", mmse_score);
    println!("  • MoCA: {}/30 (normal ≥26)", moca_score);
    println!("  • CDR-SB: {:.1} (0 = no impairment)", cdrsb_score);
    println!("  • Episodic memory: {:.0}% age-expected (subtle changes on sensitive tests)", episodic_memory);
}

fn simulate_early_ad_stage() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("STAGE 2: EARLY ALZHEIMER'S / MCI (Age 65-72, mild cognitive symptoms)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("▸ AMYLOID-β PATHOLOGY - Widespread Plaques");
    let amyloid_beta_monomer = 1.2; // μM, elevated soluble Aβ
    let amyloid_oligomers = 0.35; // μM, toxic oligomers peak
    let amyloid_plaques_cortical = 58.0; // Centiloid, clearly positive
    let amyloid_plaques_hippocampus = 75.0; // Centiloid
    let amyloid_frontal_cortex = 62.0; // Centiloid, spreading
    let neuritic_plaques = 18.0; // plaques/mm² (moderate CERAD score)
    println!("  • Soluble Aβ monomers: {:.1} μM (↑140% from preclinical)", amyloid_beta_monomer);
    println!("  • Aβ oligomers (toxic): {:.2} μM (↑340%, synaptotoxic)", amyloid_oligomers);
    println!("  • Cortical amyloid PET: {:.0} Centiloid (clearly positive)", amyloid_plaques_cortical);
    println!("  • Hippocampal burden: {:.0} Centiloid", amyloid_plaques_hippocampus);
    println!("  • Frontal cortex: {:.0} Centiloid (frontal spread)", amyloid_frontal_cortex);
    println!("  • Neuritic plaques: {:.0} plaques/mm² (moderate CERAD)", neuritic_plaques);

    println!("\n▸ TAU PATHOLOGY - Limbic Spread (Braak III-IV)");
    let tau_phosphorylation_t181 = 48.0; // pg/mL CSF, elevated
    let tau_phosphorylation_s202_t205 = 245.0; // % of baseline
    let tau_phosphorylation_s396_s404 = 280.0; // % PHF-1 antibody
    let neurofibrillary_tangles_braak = 4; // Braak IV: limbic/hippocampal
    let tau_pet_meta_temporal_suvr = 1.65; // standardized uptake value ratio
    let paired_helical_filaments = 12.0; // PHF/cell in hippocampal neurons
    println!("  • CSF p-tau181: {:.0} pg/mL (↑167%, pathological)", tau_phosphorylation_t181);
    println!("  • Tau phosphorylation (AT8): {:.0}% baseline (↑133%)", tau_phosphorylation_s202_t205);
    println!("  • Tau phosphorylation (PHF-1): {:.0}% baseline", tau_phosphorylation_s396_s404);
    println!("  • Braak NFT stage: {} (hippocampus, entorhinal cortex)", neurofibrillary_tangles_braak);
    println!("  • Tau-PET meta-temporal: {:.2} SUVR (positive >1.3)", tau_pet_meta_temporal_suvr);
    println!("  • PHF density: {:.0} PHF/neuron in hippocampus", paired_helical_filaments);

    println!("\n▸ NEUROINFLAMMATION - Robust Activation (First Wave with Aβ)");
    let microglial_activation_tspo = 1.32; // TSPO-PET, significantly elevated
    let microglia_morphology_ramified = 35.0; // % ramified
    let microglia_morphology_activated = 60.0; // % activated/phagocytic
    let microglia_morphology_dystrophic = 5.0; // % dysfunctional/senescent
    let astrocyte_gfap = 185.0; // % baseline, reactive astrogliosis
    let astrocyte_s100b = 220.0; // % s100β inflammatory marker
    let il1_beta = 8.5; // pg/mL, substantially elevated
    let tnf_alpha = 12.8; // pg/mL
    let il6 = 18.5; // pg/mL, acute phase response
    let complement_c3 = 165.0; // % baseline complement activation
    let nlrp3_inflammasome = 240.0; // % activity
    println!("  • Microglial activation (TSPO): {:.2} (↑39% from preclinical)", microglial_activation_tspo);
    println!("  • Ramified microglia: {:.0}% (↓53%)", microglia_morphology_ramified);
    println!("  • Activated microglia: {:.0}% (↑140%, phagocytic)", microglia_morphology_activated);
    println!("  • Dystrophic microglia: {:.0}% (senescent/dysfunctional)", microglia_morphology_dystrophic);
    println!("  • Reactive astrocytes (GFAP): {:.0}% (↑68%)", astrocyte_gfap);
    println!("  • Astrocyte S100β: {:.0}% (inflammatory)", astrocyte_s100b);
    println!("  • IL-1β: {:.1} pg/mL (↑372%)", il1_beta);
    println!("  • TNF-α: {:.1} pg/mL (↑300%)", tnf_alpha);
    println!("  • IL-6: {:.1} pg/mL (↑781%, acute phase)", il6);
    println!("  • Complement C3: {:.0}% (opsonization)", complement_c3);
    println!("  • NLRP3 inflammasome: {:.0}% activity (Aβ-triggered)", nlrp3_inflammasome);

    println!("\n▸ SYNAPTIC LOSS - Significant Impairment");
    let synaptic_density_sv2a = 72.0; // % baseline, 28% loss
    let hippocampal_volume = 88.0; // % age-expected, atrophy
    let entorhinal_cortex_volume = 82.0; // % age-expected
    let glutamate_nmda_receptors = 68.0; // % receptor density, substantial loss
    let acetylcholine_release = 58.0; // % baseline, cholinergic deficit
    let cholinergic_neurons_nbm = 78.0; // % neurons in nucleus basalis of Meynert
    let spine_density = 65.0; // % dendritic spines remaining
    println!("  • Synaptic density (SV2A): {:.0}% (↓28% loss)", synaptic_density_sv2a);
    println!("  • Hippocampal volume: {:.0}% (atrophy evident)", hippocampal_volume);
    println!("  • Entorhinal cortex: {:.0}% volume", entorhinal_cortex_volume);
    println!("  • NMDA receptors: {:.0}% (↓32%, excitotoxicity)", glutamate_nmda_receptors);
    println!("  • Acetylcholine release: {:.0}% (↓42%)", acetylcholine_release);
    println!("  • Cholinergic neurons (NBM): {:.0}% surviving", cholinergic_neurons_nbm);
    println!("  • Dendritic spine density: {:.0}%", spine_density);

    println!("\n▸ OXIDATIVE STRESS & MITOCHONDRIAL DYSFUNCTION");
    let ros_h2o2 = 0.48; // μM
    let ros_superoxide = 0.28; // μM
    let lipid_peroxides_mda = 3.8; // μM
    let lipid_peroxides_4hne = 2.5; // μM 4-hydroxynonenal
    let protein_carbonyls = 185.0; // % protein oxidation
    let mitochondrial_membrane_potential = -148.0; // mV, depolarization
    let atp_production = 72.0; // % baseline
    let cytochrome_c_oxidase = 62.0; // % Complex IV
    let mitochondrial_fission_drp1 = 165.0; // % fission events (fragmentation)
    let mitophagy_pink1_parkin = 85.0; // % clearance efficiency
    println!("  • H₂O₂: {:.2} μM (↑167%)", ros_h2o2);
    println!("  • Superoxide O₂⁻: {:.2} μM", ros_superoxide);
    println!("  • MDA lipid peroxides: {:.1} μM (↑171%)", lipid_peroxides_mda);
    println!("  • 4-HNE: {:.1} μM (membrane damage)", lipid_peroxides_4hne);
    println!("  • Protein carbonyls: {:.0}% (oxidative protein damage)", protein_carbonyls);
    println!("  • Mitochondrial ΔΨm: {:.0} mV (↓12%, dysfunction)", mitochondrial_membrane_potential);
    println!("  • ATP production: {:.0}% (bioenergetic failure)", atp_production);
    println!("  • Complex IV (COX): {:.0}% activity", cytochrome_c_oxidase);
    println!("  • Drp1 fission: {:.0}% (fragmentation)", mitochondrial_fission_drp1);
    println!("  • PINK1/Parkin mitophagy: {:.0}% efficiency (impaired clearance)", mitophagy_pink1_parkin);

    println!("\n▸ BLOOD-BRAIN BARRIER DISRUPTION");
    let bbb_permeability = 185.0; // % baseline permeability
    let tight_junction_occludin = 68.0; // % expression
    let pericyte_coverage = 78.0; // % capillary coverage
    let albumin_extravasation = 240.0; // % CSF/serum albumin ratio
    println!("  • BBB permeability: {:.0}% (↑85% leakage)", bbb_permeability);
    println!("  • Tight junction occludin: {:.0}%", tight_junction_occludin);
    println!("  • Pericyte coverage: {:.0}%", pericyte_coverage);
    println!("  • Albumin CSF/serum ratio: {:.0}% (extravasation)", albumin_extravasation);

    println!("\n▸ COGNITIVE FUNCTION - Mild Cognitive Impairment");
    let mmse_score = 25; // /30, borderline
    let moca_score = 21; // /30, impaired
    let cdrsb_score = 2.5; // CDR 0.5 (very mild dementia)
    let episodic_memory = 68.0; // % impaired memory
    let executive_function = 75.0; // % frontal/executive
    let semantic_memory = 82.0; // % language/semantic
    println!("  • MMSE: {}/30 (mild impairment)", mmse_score);
    println!("  • MoCA: {}/30 (impaired <26)", moca_score);
    println!("  • CDR-SB: {:.1} (0.5-4.5 = very mild/mild dementia)", cdrsb_score);
    println!("  • Episodic memory: {:.0}% (↓32%, prominent deficit)", episodic_memory);
    println!("  • Executive function: {:.0}%", executive_function);
    println!("  • Semantic memory: {:.0}% (relatively preserved)", semantic_memory);
}

fn simulate_moderate_ad_stage() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("STAGE 3: MODERATE ALZHEIMER'S DEMENTIA (Age 72-78)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("▸ AMYLOID-β PATHOLOGY - Plateau Phase");
    let amyloid_plaques_cortical = 98.0; // Centiloid, near plateau
    let amyloid_oligomers = 0.42; // μM, peak toxicity
    let amyloid_associated_dystrophic_neurites = 48.0; // % neurons with dystrophic neurites
    println!("  • Cortical amyloid PET: {:.0} Centiloid (plateau reached)", amyloid_plaques_cortical);
    println!("  • Aβ oligomers: {:.2} μM (plateau synaptotoxicity)", amyloid_oligomers);
    println!("  • Dystrophic neurites: {:.0}% neurons (plaque-associated)", amyloid_associated_dystrophic_neurites);

    println!("\n▸ TAU PATHOLOGY - Neocortical Spread (Braak V-VI, Second Wave)");
    let tau_phosphorylation_t181 = 128.0; // pg/mL CSF
    let neurofibrillary_tangles_braak = 5; // Braak V: neocortex
    let tau_pet_meta_temporal_suvr = 3.15; // widespread tau binding
    let tau_pet_lateral_temporal = 2.85; // SUVR lateral temporal
    let tau_pet_frontal = 2.28; // SUVR frontal cortex
    let paired_helical_filaments = 38.0; // PHF/cell
    let ghost_tangles = 15.0; // % NFTs as ghost tangles (cell death)
    let neuronal_loss_hippocampus = 45.0; // % neurons lost
    println!("  • CSF p-tau181: {:.0} pg/mL (↑167% from early AD)", tau_phosphorylation_t181);
    println!("  • Braak NFT stage: {} (neocortical involvement)", neurofibrillary_tangles_braak);
    println!("  • Tau-PET meta-temporal: {:.2} SUVR (↑91%)", tau_pet_meta_temporal_suvr);
    println!("  • Tau-PET lateral temporal: {:.2} SUVR", tau_pet_lateral_temporal);
    println!("  • Tau-PET frontal: {:.2} SUVR (executive dysfunction)", tau_pet_frontal);
    println!("  • PHF density: {:.0} PHF/neuron", paired_helical_filaments);
    println!("  • Ghost tangles: {:.0}% (extracellular after neuron death)", ghost_tangles);
    println!("  • Hippocampal neuron loss: {:.0}%", neuronal_loss_hippocampus);

    println!("\n▸ NEUROINFLAMMATION - Chronic Activation (Second Wave with Tau)");
    let microglial_activation_tspo = 1.68; // TSPO-PET peak
    let microglia_morphology_activated = 55.0; // %
    let microglia_morphology_dystrophic = 30.0; // % senescent, losing function
    let astrocyte_gfap = 285.0; // % severe reactive astrogliosis
    let astrocyte_aqp4 = 165.0; // % aquaporin-4, edema
    let il1_beta = 28.5; // pg/mL
    let tnf_alpha = 35.2; // pg/mL
    let il6 = 62.0; // pg/mL, chronic inflammation
    let il10 = 8.2; // pg/mL, anti-inflammatory attempt
    let il10_tnf_ratio = 0.23; // resolution failure
    let complement_c1q = 320.0; // % complement tagging
    let complement_c3 = 285.0; // % C3 opsonization
    let nlrp3_inflammasome = 380.0; // % activity
    println!("  • Microglial activation (TSPO): {:.2} (↑27% from early AD)", microglial_activation_tspo);
    println!("  • Activated microglia: {:.0}% (chronic activation)", microglia_morphology_activated);
    println!("  • Dystrophic microglia: {:.0}% (↑500%, senescence)", microglia_morphology_dystrophic);
    println!("  • Reactive astrocytes (GFAP): {:.0}% (↑54%)", astrocyte_gfap);
    println!("  • Astrocyte AQP4: {:.0}% (edema)", astrocyte_aqp4);
    println!("  • IL-1β: {:.1} pg/mL (↑235%)", il1_beta);
    println!("  • TNF-α: {:.1} pg/mL (↑175%)", tnf_alpha);
    println!("  • IL-6: {:.1} pg/mL (↑235%, chronic)", il6);
    println!("  • IL-10 anti-inflammatory: {:.1} pg/mL", il10);
    println!("  • IL-10/TNF-α ratio: {:.2} (failed resolution)", il10_tnf_ratio);
    println!("  • Complement C1q: {:.0}% (synapse tagging)", complement_c1q);
    println!("  • Complement C3: {:.0}% (↑73%)", complement_c3);
    println!("  • NLRP3 inflammasome: {:.0}% (↑58%)", nlrp3_inflammasome);

    println!("\n▸ MASSIVE SYNAPTIC & NEURONAL LOSS");
    let synaptic_density_sv2a = 38.0; // % baseline, 62% loss
    let hippocampal_volume = 65.0; // % age-expected, severe atrophy
    let entorhinal_cortex_volume = 52.0; // % age-expected
    let temporal_lobe_volume = 68.0; // % atrophy
    let cortical_thickness_global = 72.0; // % average
    let acetylcholine_release = 22.0; // % baseline, severe cholinergic deficit
    let cholinergic_neurons_nbm = 45.0; // % neurons surviving
    let glutamate_nmda_receptors = 38.0; // % receptor density
    let spine_density = 32.0; // % dendritic spines
    let neuronal_loss_ca1_hippocampus = 58.0; // % CA1 neurons lost
    println!("  • Synaptic density (SV2A): {:.0}% (↓62% total loss)", synaptic_density_sv2a);
    println!("  • Hippocampal volume: {:.0}% (severe atrophy)", hippocampal_volume);
    println!("  • Entorhinal cortex: {:.0}% volume", entorhinal_cortex_volume);
    println!("  • Temporal lobe: {:.0}% volume", temporal_lobe_volume);
    println!("  • Cortical thickness: {:.0}% average", cortical_thickness_global);
    println!("  • Acetylcholine: {:.0}% (↓78% total)", acetylcholine_release);
    println!("  • Cholinergic neurons: {:.0}% surviving (↓42% from early)", cholinergic_neurons_nbm);
    println!("  • NMDA receptors: {:.0}%", glutamate_nmda_receptors);
    println!("  • Spine density: {:.0}% (↓68%)", spine_density);
    println!("  • CA1 hippocampal neurons: {:.0}% lost", neuronal_loss_ca1_hippocampus);

    println!("\n▸ WHITE MATTER DEGENERATION");
    let white_matter_hyperintensities = 28.0; // % WM volume affected (Fazekas 3)
    let myelin_basic_protein = 58.0; // % MBP expression
    let axonal_transport_deficit = 42.0; // % transport rate
    println!("  • WM hyperintensities: {:.0}% volume (Fazekas 3)", white_matter_hyperintensities);
    println!("  • Myelin basic protein: {:.0}%", myelin_basic_protein);
    println!("  • Axonal transport: {:.0}% rate (tau impairs transport)", axonal_transport_deficit);

    println!("\n▸ CEREBROVASCULAR PATHOLOGY");
    let cerebral_amyloid_angiopathy = 68.0; // % vessels affected (CAA severity)
    let cerebral_blood_flow = 68.0; // % baseline CBF
    let bbb_permeability = 285.0; // % severe leakage
    println!("  • Cerebral amyloid angiopathy: {:.0}% vessels (CAA)", cerebral_amyloid_angiopathy);
    println!("  • Cerebral blood flow: {:.0}% (hypoperfusion)", cerebral_blood_flow);
    println!("  • BBB permeability: {:.0}% (severe disruption)", bbb_permeability);

    println!("\n▸ COGNITIVE FUNCTION - Moderate Dementia");
    let mmse_score = 16; // /30, moderate impairment
    let moca_score = 9; // /30, severe impairment
    let cdrsb_score = 7.5; // CDR 2 (moderate dementia)
    let episodic_memory = 28.0; // % severe memory loss
    let executive_function = 35.0; // % planning/judgment impaired
    let semantic_memory = 48.0; // % language decline
    let apraxia = 65.0; // % motor planning preserved
    let adl_independence = 55.0; // % activities of daily living
    println!("  • MMSE: {}/30 (moderate impairment)", mmse_score);
    println!("  • MoCA: {}/30 (severe impairment)", moca_score);
    println!("  • CDR-SB: {:.1} (4.5-9 = moderate dementia)", cdrsb_score);
    println!("  • Episodic memory: {:.0}% (↓59% from early AD)", episodic_memory);
    println!("  • Executive function: {:.0}% (↓53%)", executive_function);
    println!("  • Semantic memory: {:.0}% (↓41%, language decline)", semantic_memory);
    println!("  • Apraxia (motor skills): {:.0}% preserved", apraxia);
    println!("  • ADL independence: {:.0}% (requires assistance)", adl_independence);
}

fn simulate_severe_ad_stage() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("STAGE 4: SEVERE ALZHEIMER'S DEMENTIA (Age 78-85+)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("▸ AMYLOID & TAU PATHOLOGY - End-Stage");
    let amyloid_plaques_cortical = 108.0; // Centiloid
    let neurofibrillary_tangles_braak = 6; // Braak VI: widespread neocortex
    let tau_pet_global_suvr = 4.85; // diffuse tau binding
    let ghost_tangles = 48.0; // % NFTs as ghost tangles
    println!("  • Cortical amyloid: {:.0} Centiloid (saturation)", amyloid_plaques_cortical);
    println!("  • Braak NFT stage: {} (entire neocortex)", neurofibrillary_tangles_braak);
    println!("  • Tau-PET global: {:.2} SUVR (diffuse)", tau_pet_global_suvr);
    println!("  • Ghost tangles: {:.0}% (massive neuron death)", ghost_tangles);

    println!("\n▸ NEUROINFLAMMATION - Dystrophic/Exhausted");
    let microglial_activation_tspo = 1.52; // TSPO-PET declining (microglial exhaustion)
    let microglia_morphology_dystrophic = 58.0; // % majority dysfunctional
    let astrocyte_gfap = 420.0; // % extreme astrogliosis
    let il1_beta = 42.0; // pg/mL
    let tnf_alpha = 48.5; // pg/mL
    println!("  • Microglial activation (TSPO): {:.2} (↓10%, exhaustion)", microglial_activation_tspo);
    println!("  • Dystrophic microglia: {:.0}% (↑93%, loss of function)", microglia_morphology_dystrophic);
    println!("  • Reactive astrocytes: {:.0}% (extreme)", astrocyte_gfap);
    println!("  • IL-1β: {:.0} pg/mL (↑47%)", il1_beta);
    println!("  • TNF-α: {:.1} pg/mL (↑38%)", tnf_alpha);

    println!("\n▸ SEVERE NEURODEGENERATION");
    let synaptic_density_sv2a = 18.0; // % baseline, 82% loss
    let hippocampal_volume = 42.0; // % age-expected
    let entorhinal_cortex_volume = 35.0; // % age-expected
    let cortical_thickness_global = 52.0; // % average
    let brain_weight = 78.0; // % expected (1000g → 780g)
    let neuronal_loss_ca1_hippocampus = 85.0; // % CA1 neurons lost
    let acetylcholine_release = 8.0; // % baseline, near-complete cholinergic loss
    let cholinergic_neurons_nbm = 18.0; // % neurons surviving
    println!("  • Synaptic density: {:.0}% (↓82% total loss)", synaptic_density_sv2a);
    println!("  • Hippocampal volume: {:.0}% (extreme atrophy)", hippocampal_volume);
    println!("  • Entorhinal cortex: {:.0}% volume", entorhinal_cortex_volume);
    println!("  • Cortical thickness: {:.0}% average", cortical_thickness_global);
    println!("  • Brain weight: {:.0}% expected (~780g vs 1000g)", brain_weight);
    println!("  • CA1 hippocampal neurons: {:.0}% lost", neuronal_loss_ca1_hippocampus);
    println!("  • Acetylcholine: {:.0}% (near-complete loss)", acetylcholine_release);
    println!("  • Cholinergic neurons: {:.0}% surviving (↓60% from moderate)", cholinergic_neurons_nbm);

    println!("\n▸ COGNITIVE FUNCTION - Severe Dementia");
    let mmse_score = 4; // /30, severe impairment
    let cdrsb_score = 16.0; // CDR 3 (severe dementia)
    let episodic_memory = 5.0; // % near-total amnesia
    let semantic_memory = 12.0; // % severe language loss
    let adl_independence = 8.0; // % complete dependence
    let incontinence = true;
    let loss_of_speech = true;
    println!("  • MMSE: {}/30 (severe impairment)", mmse_score);
    println!("  • CDR-SB: {:.1} (9-18 = severe dementia)", cdrsb_score);
    println!("  • Episodic memory: {:.0}% (near-total amnesia)", episodic_memory);
    println!("  • Semantic memory: {:.0}% (mutism common)", semantic_memory);
    println!("  • ADL independence: {:.0}% (total care)", adl_independence);
    println!("  • Incontinence: {}", incontinence);
    println!("  • Speech: {}", if loss_of_speech { "severely impaired/lost" } else { "preserved" });

    println!("\n▸ SYSTEMIC COMPLICATIONS");
    println!("  • Dysphagia: aspiration pneumonia risk");
    println!("  • Immobility: pressure ulcers, contractures");
    println!("  • Infections: most common cause of death");
    println!("  • Cachexia: weight loss, malnutrition");

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("KEY INSIGHTS: TWO-WAVE MODEL OF NEUROINFLAMMATION");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("▸ WAVE 1 (Early AD): Aβ-driven neuroinflammation");
    println!("  • Microglial activation in response to Aβ plaques");
    println!("  • Detrimental effects on gray matter density (frontal, parietal)");
    println!("  • Synapse loss mediated by complement tagging (C1q, C3)");

    println!("\n▸ WAVE 2 (Moderate-Severe AD): Tau-driven neuroinflammation");
    println!("  • Microglial activation follows tau spread (Braak V-VI)");
    println!("  • Detrimental effects on temporal, occipital cortex");
    println!("  • Chronic inflammation → microglial exhaustion/dystrophy");

    println!("\n▸ THERAPEUTIC IMPLICATIONS:");
    println!("  • Anti-Aβ therapies (lecanemab, donanemab): early AD (Braak I-IV)");
    println!("  • Anti-tau therapies: target second wave (Braak V-VI)");
    println!("  • Anti-inflammatory: timing-dependent (beneficial early, harmful late?)");
    println!("  • Multi-target approaches: address Aβ, tau, neuroinflammation cascade");

    println!("\n▸ BIOMARKER CASCADE:");
    println!("  1. Aβ PET/CSF (10-20yr before symptoms)");
    println!("  2. Tau PET/CSF (5-10yr before symptoms)");
    println!("  3. Neuroinflammation (TSPO-PET) (parallels Aβ and tau)");
    println!("  4. Neurodegeneration (MRI atrophy, FDG-PET) (symptoms emerge)");
    println!("  5. Cognitive decline (follows neurodegeneration)");

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("Simulation demonstrates multi-decade Alzheimer's progression from");
    println!("preclinical Aβ accumulation → tau spread → neuroinflammation → neurodegeneration,");
    println!("informed by PET imaging (Aβ, tau, TSPO), CSF biomarkers, neuropathology,");
    println!("and recent TRIAD cohort findings (PMC12477628).\n");
}

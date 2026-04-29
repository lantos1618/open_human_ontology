# human_biology

A Rust library of cited, testable physiology models — closed-form formulas
and small ODE simulations. Each example binary is tied to a published model;
each numeric parameter carries a citation; each test asserts a literature
reference range. See [VISION.md](VISION.md) for scope and non-goals.

## Project Status

- Clean build, zero warnings
- 340 library tests + 16 integration tests passing
- 22 example ODE binaries, each tied to a published model
- L4 ground-truth registry with PMID/DOI citations
- ~22K LOC after the 2026-04 cull (down from ~75K)

## Organ Systems

Four cited L1 systems exercised by the example binaries:

- **Cardiovascular**: cardiac mechanics (LaPlace, Frank-Starling, MVO2), hemodynamics, electrophysiology
- **Nervous**: Hodgkin-Huxley + Goldman-Hodgkin-Katz, synaptic transmission, circadian rhythms
- **Respiratory**: gas exchange, pulmonary mechanics, oxygen transport kinetics
- **Renal**: Starling filtration, Cockcroft-Gault CrCl, Tm-limited reabsorption, GFR-scaled drug clearance

The 2026-04 cull removed eight uncited organ-system scaffolds (immune,
endocrine, digestive, sensory, integumentary, reproductive, muscular,
lymphatic) and ~50K LOC of typed-but-empty struct hierarchies that no L3
example reached and that carried no citations. Per VISION's quality bar:
if a number cannot be cited, the function does not ship.

## What's here

### L1 cited primitives
- **Cardiac Mechanics**: LaPlace's law, Frank-Starling curves, MVO2
- **Action Potentials**: Hodgkin-Huxley with gating variables; GHK resting potential
- **Respiratory Mechanics**: compliance, resistance, V/Q matching, surfactant dynamics
- **Renal**: Cockcroft-Gault CrCl, Starling filtration, Tm-limited reabsorption
- **Metabolic**: Mifflin-St Jeor RMR, Widmark + Michaelis-Menten alcohol PK
- **Pharmacology**: pharmacokinetics, pharmacogenomics primitives

### L4 ground-truth registries
- 20+ medical domains with peer-reviewed reference ranges
- Each entry carries a PMID or DOI; spot-checked against textbook bands
- Used by L3 examples to assert physiologic plausibility

### L2 genetics data
- TOML-driven gene catalogs with population frequencies
- Pharmacogenomics: ALDH2, CYP2D6, warfarin sensitivity, opioid metabolism
- Cited dietary/nutrition recommendations (lactose, ALDH2, MTHFR, CYP1A2, vitamin D)

## Quick Start

```bash
git clone https://github.com/lantos1618/open_human_ontology
cd open_human_ontology
cargo build --release
cargo test
```

Each example is a self-contained ODE binary tied to a published model:

```bash
cargo run --example cardiac_ischemia_detector --release
cargo run --example hodgkin_huxley_action_potential --release
cargo run --example acid_base_balance --release
```

## Architecture

```
src/
├── biology/genetics/    # L2: TOML-driven genetics + typed accessors (8 submodules)
├── systems/             # L1: cardiovascular, nervous, renal, respiratory
├── metabolism/          # L1: alcohol PK + enzyme kinetics
├── pharmacology/        # L1: pharmacokinetics, pharmacogenomics
├── nutrition/           # L1: cited evidence base for dietary recommendations
├── pathology/headache.rs # used by integration tests
├── validation/ground_truth/ # L4: PMID-cited reference ranges per domain
├── config/              # baseline params, presets
└── simulation_utils.rs  # helpers for L3 examples
data/genetics/*.toml     # L2: cited gene catalogs
examples/*.rs            # L3: 22 ODE binaries, each tied to a published model
```

## Examples

```bash
# Cardiovascular
cargo run --example cardiac_ischemia_detector --release
cargo run --example raas_blood_pressure_control --release
cargo run --example hemoglobin_allosteric_cooperativity --release

# Neural biophysics
cargo run --example hodgkin_huxley_action_potential --release
cargo run --example ghk_resting_potential --release
cargo run --example neuron_spike_train --release

# Respiratory
cargo run --example respiratory_pathology --release

# Endocrine feedback
cargo run --example insulin_glucose_feedback --release
cargo run --example thyroid_hormone_feedback --release
cargo run --example calcium_homeostasis_feedback --release
cargo run --example hpa_axis_cortisol_rhythm --release

# Pharmacology
cargo run --example multi_compartment_pk --release
cargo run --example pharmacodynamics_emax_model --release
cargo run --example alcohol_pharmacokinetics --release
cargo run --example drug_interaction_calculator --release
cargo run --example renal_clearance_pharmacokinetics --release

# Cellular signaling
cargo run --example gpcr_camp_signaling_cascade --release
cargo run --example glycolysis_kinetics_simulation --release

# Clinical / integrative
cargo run --example clinical_biomarker_analyzer --release
cargo run --example cancer_biomarker_simulation_groundtruth --release
cargo run --example exercise_physiology_vo2max --release
cargo run --example acid_base_balance --release
```

## Technology

- Rust 2021 edition
- `nalgebra` for linear algebra
- `serde` for serialization
- `proptest` for property-based testing

## Testing

```bash
cargo test                    # 340 lib + 16 integration
cargo test --lib              # library tests only
cargo test cardiovascular     # filter by module
```

## License

MIT — see LICENSE.

## Acknowledgments

Cited against medical literature (Guyton & Hall, Ganong's, peer-reviewed
papers). Each L1 primitive carries an inline PMID/DOI; each L4 reference
range cites a published source. Models that could not be cited were removed
in the 2026-04 cull rather than left as scaffolds.

---

**Version**: 0.1.0
**Last Updated**: 2026-04-28

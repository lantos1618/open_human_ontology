# human_biology

A Rust library of cited, testable physiology models — closed-form formulas
and small ODE simulations. Each example binary is tied to a published model;
each parameter carries a citation; each test asserts a literature reference
range. See [VISION.md](VISION.md) for scope and non-goals.

## Project Status

- Clean build, zero warnings
- 1255 library tests passing
- 22 example ODE binaries, each tied to a published model
- L4 ground-truth registry with PMID/DOI citations

## Organ Systems

Four systems with cited primitives, exercised by the example binaries:

- **Cardiovascular**: Cardiac mechanics (LaPlace, Frank-Starling, MVO2), hemodynamics, electrophysiology
- **Nervous**: Hodgkin-Huxley + Goldman-Hodgkin-Katz, synaptic transmission, circadian rhythms
- **Respiratory**: Gas exchange, pulmonary mechanics, oxygen transport kinetics
- **Renal**: Starling filtration, Cockcroft-Gault CrCl, Tm-limited reabsorption, GFR-scaled drug clearance

Eight uncited organ-system scaffolds (immune, endocrine, digestive, sensory,
integumentary, reproductive, muscular, lymphatic) were removed in the
2026-04 cull. They produced typed-but-empty struct hierarchies with no L3
example exercising them.

## What's here

### L1 cited primitives
- **Cardiac Mechanics**: LaPlace's law, Frank-Starling curves, MVO2
- **Action Potentials**: Hodgkin-Huxley with gating variables; GHK resting potential
- **Respiratory Mechanics**: Compliance, resistance, V/Q matching, surfactant dynamics
- **Renal**: Cockcroft-Gault CrCl, Starling filtration, Tm-limited reabsorption
- **Metabolic**: Mifflin-St Jeor RMR

### L4 ground-truth registries
- 20+ medical domains with peer-reviewed reference ranges
- Each entry carries a PMID or DOI; spot-checked against textbook bands
- Used by L3 examples to assert physiologic plausibility

### L2 genetics data
- TOML-driven gene catalogs with population frequencies
- Pharmacogenomics: ALDH2, CYP2D6, warfarin sensitivity, opioid metabolism

## Quick Start

Each example is a self-contained ODE binary tied to a published model:

```bash
cargo run --example cardiac_ischemia_detector --release
cargo run --example hodgkin_huxley_action_potential --release
cargo run --example acid_base_balance --release
```

## Installation

```bash
git clone https://github.com/lantos1618/open_human_ontology
cd open_human_ontology
cargo build --release
cargo test
cargo doc --open
```

## Architecture

```
src/
├── biology/          # Genetics (TOML-driven), molecular, cellular primitives
├── systems/          # 4 organ systems: cardiovascular, nervous, renal, respiratory
├── physiology/       # Time-series snapshots, stress/aging/inflammation primitives
├── chemistry/        # Reactions, equilibrium
├── physics/          # Mechanics, forces, thermodynamics
├── pharmacology/     # PK/PD primitives used by the example binaries
├── validation/       # L4 ground-truth registries with PMID/DOI citations
└── human.rs          # Slim Human aggregate (BiologicalSex, body metrics, 4-system state)
```

## Examples - Real Computational Models

Each example is a self-contained ODE binary tied to a published model.

```bash
# Cardiovascular
cargo run --example cardiac_ischemia_detector --release
cargo run --example raas_blood_pressure_control --release
cargo run --example hemoglobin_allosteric_cooperativity --release

# Nervous (biophysics)
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

# Cellular signaling
cargo run --example gpcr_camp_signaling_cascade --release
cargo run --example glycolysis_kinetics_simulation --release

# Clinical
cargo run --example clinical_biomarker_analyzer --release
cargo run --example exercise_physiology_vo2max --release
cargo run --example acid_base_balance --release
```

## Technology

- **Rust** 2021 edition
- **nalgebra** for linear algebra
- **serde** for serialization
- **rayon** for parallelization
- **proptest** for property-based testing

## Testing

```bash
cargo test                    # All tests (~1255 lib + 16 integration)
cargo test --lib              # Library tests only
cargo test cardiovascular     # Filter by module
```

## Documentation

- Full Rust docs: `cargo doc --open`
- Architecture: [`ARCHITECTURE.md`](ARCHITECTURE.md)
- Historical session logs: `agent/docs_archive/`

## Contributing

Contributions welcome! Focus areas:
- Expanding genetic databases
- Additional disease models
- Validation against clinical data
- Performance optimization
- Documentation improvements

## License

MIT License - see LICENSE file

## Acknowledgments

Cited against medical literature (Guyton & Hall, Ganong's, peer-reviewed
papers). Each L1 primitive carries an inline PMID/DOI; each L4 reference
range cites a published source. Models that could not be cited were removed
in the 2026-04 cull rather than left as scaffolds.

---

**Version**: 0.1.0
**Last Updated**: 2026-04-28

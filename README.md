# Human Biology - Computational Model

A comprehensive, type-safe computational model of human biology built in Rust for simulation, analysis, and diagnosis of biological systems.

## Project Status

- ✅ **Compilation**: Clean build
- ✅ **Tests**: 1536 passing
- ✅ **Systems**: 4 complete computational systems, 8 scaffolds
- 20 example ODE simulations, each tied to a published model

## Organ Systems

### ✅ Complete Computational Models
- **Cardiovascular**: Cardiac mechanics, hemodynamics, electrophysiology
- **Nervous**: Hodgkin-Huxley + Goldman-Hodgkin-Katz, synaptic transmission, circadian rhythms
- **Respiratory**: Gas exchange, pulmonary mechanics, oxygen transport kinetics
- **Renal**: Starling filtration, Cockcroft-Gault CrCl, Tm-limited reabsorption, GFR-scaled drug clearance

### 🚧 Scaffolds
- **Immune**: Cytokine networks, immune cells
- **Endocrine**: Hormones, feedback loops
- **Digestive**: GI tract, absorption
- **Sensory**: Vision, hearing
- **Integumentary**: Skin layers
- **Reproductive**: Reproductive anatomy
- **Muscular**: Muscle contraction
- **Lymphatic**: Lymph nodes, vessels

## What Actually Works

### Computational Models (Ready to Use)
- **Cardiac Mechanics**: LaPlace's law, Frank-Starling curves, MVO2, ischemia detection
- **Action Potentials**: Complete Hodgkin-Huxley equations with gating variables
- **Respiratory Mechanics**: Compliance, resistance, V/Q matching, surfactant dynamics
- **Mitochondrial Function**: ETC, OXPHOS, ROS production
- **ALDH2 Metabolism**: Acetaldehyde processing with genetic variants
- **Inflammation Markers**: Cytokine levels, acute phase response

### Clinical Validation (Ground-Truth Data)
- **20+ Medical Domains**: Cardiovascular, metabolic, renal, etc. with peer-reviewed references
- **Evidence-Based**: PMID/DOI citations for expected ranges
- **Biomarker Simulation**: Cancer markers, inflammation, Alzheimer's biomarkers

### Genetics & Ancestry
- **50+ Genes**: Externalized TOML data with population variants
- **Ancestry Models**: African, Asian, European genetic variants
- **Pharmacogenomics**: Drug metabolism (ALDH2, CYP2D6, etc.)

## Quick Start

```rust
use human_biology::{Human, BiologicalSex};

// Create a human model
let person = Human::new_adult_male(
    "john_doe",
    30.0,  // age
    175.0, // height (cm)
    75.0   // weight (kg)
);

// Get health metrics
let bmi = person.bmi();
let cardiac_output = person.cardiac_output_l_per_min();
let gfr = person.gfr_ml_per_min();
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
├── biology/          # Cellular, molecular, genetics, neural, immunology
├── systems/          # 13 organ systems (cardiovascular, nervous, etc.)
├── physiology/       # Stress, aging, inflammation, thermoregulation
├── simulation/       # Multi-system integration engine
├── chemistry/        # Reactions, equilibrium
├── physics/          # Mechanics, forces, thermodynamics
└── human.rs          # Main integrated model
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

## Development Status

### Completed Refactoring

**Phase 1: Honest Documentation**
- Corrected claims about system completeness
- Distinguished scaffolds from working implementations

**Phase 2: Remove Fake Simulations**
- Deleted hardcoded println-based examples
- Retained ground-truth validated simulations

**Phase 3: Externalize Data**
- Moved genetic data to TOML files
- Validated cancer risk data against peer-reviewed sources (NCI, BOADICEA, GeneReviews)

**Phase 4: Simplify Module Structure**
- Fixed all ambiguous glob reexports
- Removed warning suppressions

### Future Work
- Disease progression modeling
- Pharmacokinetics/pharmacodynamics
- Clinical validation framework

## Testing

```bash
cargo test                    # Run all 1536 tests
cargo test --lib             # Library tests only
cargo test cardiovascular    # System-specific tests
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

Built with references to medical literature (Guyton & Hall, Ganong's, peer-reviewed research). Validation status varies: some modules have ground-truth implementations (see `examples/*_groundtruth.rs`), while others are scaffolds awaiting proper validation.

---

**Version**: 0.1.0
**Last Updated**: April 25, 2026

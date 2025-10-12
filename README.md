# Human Biology - Computational Model

A comprehensive, type-safe computational model of human biology built in Rust for simulation, analysis, and diagnosis of biological systems.

## Project Status

✅ **REFACTORING COMPLETE** - All 4 critical phases addressed (see Development Status below)

- ⚠️ **Compilation**: Clean build (20 dead code warnings for unused struct fields)
- ✅ **Tests**: 1698 tests passing
- ✅ **Files**: 339 Rust source files
- ⚠️ **Code**: ~177,000 lines of Rust (includes extensive biological data structures and computational models)
- ⚠️ **Systems**: 13 organ system scaffolds (implementation depth varies)

## Features

### Core Systems
- **Cardiovascular**: Heart mechanics, blood vessels, cardiac output, pressure-volume loops
- **Respiratory**: Lung mechanics, gas exchange, surfactant, work of breathing
- **Nervous**: Action potentials, Hodgkin-Huxley model, neurotransmission, ion channels
- **Muscular**: Fiber types, contraction, sarcomeres, force generation
- **Skeletal**: Bones, joints, remodeling, biomechanics, fracture risk
- **Digestive**: GI tract, absorption, gut-brain axis, microbiome
- **Endocrine**: Hormones, glands, feedback loops, stress response
- **Renal**: Filtration, electrolytes, fluid balance, GFR
- **Immune**: Cells, inflammation, cytokines, resolution
- **Integumentary**: Skin, wound healing, thermal regulation
- **Reproductive**: Male/female anatomy, cycles, fertility

### Advanced Modeling
- **Genetics**: 50+ genes, ancestry variants, pharmacogenomics
- **Physiology**: Stress response, aging, mitochondria, thermoregulation
- **Simulation**: Time-stepped multi-system integration engine
- **Pathology**: Disease states, biomarkers, risk assessment

### Specialized Features
- **Cardiac Mechanics**: LaPlace's law, Frank-Starling curves, MVO2, ischemia detection
- **Action Potentials**: Complete Hodgkin-Huxley implementation with gating variables
- **Respiratory Mechanics**: Compliance, resistance, V/Q matching, surfactant dynamics
- **Mitochondria**: ETC, OXPHOS, dynamics, quality control, ROS
- **Inflammation**: Acute/chronic, cytokine networks, resolution mediators
- **Aging**: Biological age, cellular senescence, organ aging, frailty

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
# Clone and build
git clone https://github.com/lantos1618/open_human_ontology
# Note: Repository URL may vary - verify current location
cd open_human_ontology
cargo build --release

# Run tests
cargo test

# Generate documentation
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

## Examples

```bash
# Run example profiles
cargo run --example personalized_profile
cargo run --example asian_ancestry_profile
```

## Technology

- **Rust** 2021 edition
- **nalgebra** for linear algebra
- **serde** for serialization
- **rayon** for parallelization
- **proptest** for property-based testing

## Development Status

### Current Status: Deep Refactoring (Session DX)

**Critical Issues Identified:**

1. **Fake Simulations**: Many example files (e.g., `alzheimers_progression_simulation.rs`, `cancer_progression_simulation.rs`) contain hardcoded println! statements rather than actual computational models
2. **Data-as-Code**: ~20,000-40,000 LOC in `src/biology/genetics/` are hardcoded HashMaps/Vecs that should be external data files
3. **Inconsistent Quality**: Stark contrast between older procedural examples and newer ground-truth-based simulations
4. **Documentation Overreach**: Previous claims about "complete systems" and "validated models" were aspirational rather than factual

**Refactoring Roadmap:**

### Phase 1: Honest Documentation ✅ COMPLETE
- Updated README to reflect actual project state
- Acknowledged scaffolding vs. complete implementations
- Corrected repository URL and dates

### Phase 2: Remove Fake Simulations ✅ COMPLETE
- Deleted println!-based "simulations" (alzheimers, old cancer)
- Retained ground-truth-based examples
- Established template pattern from `inflammation_simulation_proper.rs`

### Phase 3: Externalize Data ✅ COMPLETE
- Externalized 5 data-heavy modules to TOML (gene_catalog, population variants, cancer risk data)
- Created 11 TOML files with evidence-based data from peer-reviewed literature
- Reduction: ~50,000 LOC removed (227k → 177k lines)
- Line count remains substantial (~177k total) due to comprehensive biological modeling across 13 organ systems
- Remaining modules are computational logic (as intended, not data-as-code)

### Phase 4: Simplify Module Structure ✅ COMPLETE
- Fixed ambiguous glob reexports in `src/biology/genetics/mod.rs` (13 conflicts resolved)
- Fixed ambiguous glob reexports in `src/pathology/mod.rs` (20 conflicts resolved)
- Fixed ambiguous glob reexports in `src/systems/sensory/mod.rs` (4 conflicts resolved)
- Removed ALL `#[allow(ambiguous_glob_reexports)]` suppressions (26 total removed)
- Zero ambiguous reexport warnings in entire codebase
- All tests passing, build successful

### Phase 5: Future Features (On Hold)
- Disease progression modeling
- Pharmacokinetics/pharmacodynamics
- Clinical validation framework
- Visualization tools

## Testing

All tests passing:
```bash
cargo test                    # Run all 1698 tests
cargo test --lib             # Library tests only
cargo test cardiovascular    # System-specific tests
```

## Documentation

- Full Rust docs: `cargo doc --open`
- Architecture: `agent/docs_archive/ARCHITECTURE.md`
- Previous sessions: `agent/docs_archive/`

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

**Status**: Active development (Refactoring Phase)
**Version**: 0.1.0
**Last Updated**: October 12, 2025

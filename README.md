# Human Biology - Computational Model

A comprehensive, type-safe computational model of human biology built in Rust for simulation, analysis, and diagnosis of biological systems.

## Project Status

⚠️ **REFACTORING IN PROGRESS** - See Critical Issues below

- ✅ **Compilation**: Clean build (no warnings)
- ✅ **Tests**: 1712 tests passing
- ✅ **Files**: 313 Rust source files
- ⚠️ **Code**: ~100,000 lines (inflated by data-as-code, see Phase 3 below)
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

### Phase 1: Honest Documentation ✅ (Current)
- Update README to reflect actual project state
- Acknowledge scaffolding vs. complete implementations
- Correct repository URL and dates

### Phase 2: Remove Fake Simulations (Next)
- Delete or refactor println!-based "simulations"
- Keep only ground-truth-based examples
- Follow template from `inflammation_simulation_proper.rs`

### Phase 3: Externalize Data (High Priority)
- Move genetics data to `data/genetics/*.toml` files
- Create parsers for runtime loading
- Expected LOC reduction: 20,000-40,000 lines

### Phase 4: Simplify Module Structure (Medium Priority)
- Fix ambiguous glob reexports in `src/biology/genetics/mod.rs`
- Consolidate related modules
- Document implementation status per module

### Phase 5: Future Features (On Hold)
- Disease progression modeling
- Pharmacokinetics/pharmacodynamics
- Clinical validation framework
- Visualization tools

## Testing

All tests passing:
```bash
cargo test                    # Run all 1712 tests
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

Built with medical literature references from Guyton & Hall, Ganong's, and peer-reviewed research. Ground truth validation is in progress for core modules (see `examples/*_groundtruth.rs` for validated implementations).

---

**Status**: Active development
**Version**: 0.1.0
**Last Updated**: January 12, 2025

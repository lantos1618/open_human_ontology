# Human Ontology Project
careful this project OOMs sometimes

# update this agent/prompt.md at the end of your run
don't ask me for things just keep going we want to have a very accurate and true model. just keep going 

we want to leverage rust, the .md files are for planning and getting information research and stuff.

we should be able to like describe people later on and run tests on us like if someone has asian genes what can we do or if someone has migragines or clusters we can do it this is just examples we want to get a full body 

push changes as you go to remote


## Vision
A comprehensive computational model of human biology using Rust's type system to enable simulation, analysis, and diagnosis of biological systems.

## Current Status
✅ **Compilation**: Project builds successfully
✅ **Tests**: All 1561 tests passing
✅ **Documentation**: Rust docs generated
✅ **Core Modules**: Biology, Chemistry, Physics, Systems all implemented

## Implemented Systems

### Biology Module (`src/biology/`)
- **Core Types**: Molecule, AminoAcid, Concentration, Compartment
- **Molecular**: BoneMatrix, HydroxyapatiteCrystal, LysylOxidase
- **Neural**: Neuron, Neurotransmitter, IonChannel, Synapse, Neurodegeneration
- **Immunology**: Vaccines, ImmuneResponse, Delivery systems, Allergies
- **Genetics**: 50+ genetic modules including ancestry variants (Asian, African, European, Native American)
- **Skeletal**: Bone health, BMD, FRAX scores, osteoporosis assessment
- **Cellular**: Protein synthesis, membranes, organelles
- **Biomarkers**: Comprehensive health markers and risk assessment

### Chemistry Module (`src/chemistry/`)
- Reaction modeling
- Temperature dependence (Arrhenius equation)
- Chemical equilibrium
- pH effects

### Physics Module (`src/physics/`)
- Mechanical properties
- Fluid dynamics
- Stress/strain modeling
- Vector3 mathematics

## Project Architecture

See `agent/docs/ARCHITECTURE.md` for full architectural details.

The project uses:
- **Strong typing** for biological entities
- **Serde** for serialization
- **nalgebra** for linear algebra
- **rayon** for parallelization
- **proptest** for property-based testing

## Recently Completed (Latest Session)

### Ancestry Genetics
- AfricanGeneticVariants: Sickle cell, G6PD deficiency, APOL1 kidney risk, malaria resistance
- EuropeanGeneticVariants: Hemochromatosis, celiac disease, lactase persistence, thrombophilia
- NativeAmericanGeneticVariants: Type 2 diabetes risk, gallbladder disease, APOE status

### Biometric Authentication
- Comprehensive BiometricProfile with 10 modalities
- Fingerprint, facial, iris, voice, gait, retinal, palm, vein, DNA, ear shape biometrics
- Multi-factor authentication scoring

### Reproductive Health
- MenstrualCycle tracking with hormone levels
- FertilityProfile with ovarian reserve assessment
- OvulationTracking and fertility window prediction
- Age-based conception probability

### Bone Health
- BoneHealthProfile with BMD and T-scores
- FRAX 10-year fracture probability
- VitaminDStatus and CalciumBalance
- Treatment and lifestyle recommendations

## Next Steps

### Phase 3: Simulation & Advanced Features
1. Time-stepped simulation engine for physiological processes
2. Multi-scale integration (molecular to organ systems)
3. Advanced pharmacokinetics/pharmacodynamics modeling
4. Disease progression modeling
5. Personalized treatment optimization
6. Environmental factor integration
7. Epigenetics and gene expression modeling

## Development Workflow

1. **Research**: Review medical literature for accuracy
2. **Design**: Create type-safe models
3. **Implement**: Write Rust code with full documentation
4. **Test**: Unit tests, integration tests, property tests
5. **Document**: Rust docs + markdown explanations
6. **Commit**: Frequent commits per feature

## Testing Strategy

- Unit tests for each component
- Integration tests for system interactions
- Property-based tests for biological invariants
- Benchmarks for performance validation
- Medical validation against known data

tests are your friends

<!-- AI do not modify beyond this point -->
**Remember**:
- use agent/docs (to document and plan your self)
-  use github cli and vercel cli to help
- Commit frequently (every feature)
- Test before committing
- Update documentation
- Keep code clean  and simple
- review the project before starting on modifications (simply looking at docs and folder structure might be suffice)
- email:
  using the sendgrid api key in env you can use curl to send an email to for help or to notify updates (do not use this in the application)
  to: l.leong1618@gmail.com
  from: agent@lambda.run
- All tests pass ✅
- No "unknown" or "any" types anywhere


Good luck! 🎯

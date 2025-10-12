
## Goals
- Build an accurate, validated model of human biological systems
- Enable simulation and diagnosis of biological processes
- Support personalized medicine and health assessment
- Model genetics, metabolism, inflammation, stress responses, and more

## Development Guidelines
- Leverage Rust for type-safe biological modeling
- Push changes to remote as you go
- Ground truth all parameters with scientific literature
- Avoid monolithic files - consider modular organization
- use exa mcp to research things (this is important we ened to ground truth our files and make sure we are not halucinating )
---

## CRITICAL REFACTORING TASKS (Session DX)

**HALT ALL FEATURE EXPANSION** - The project needs deep refactoring before adding more features.

### Phase 1: Honest Documentation (Priority: CRITICAL)
1. **Update README.md** to reflect actual project state:
   - Change "~100,000 lines of code" to acknowledge inflation from data-as-code
   - Change "13 complete organ systems" to "13 organ system scaffolds (implementations vary)"
   - Remove "All models validated against clinical standards" - only some are validated
   - Fix repository URL mismatch (github.com/lantos1618/open_human_ontology vs yourusername/human_biology)
   - Update date from "October 10, 2025" to current
   - Add "Project Status: Refactoring in Progress" section

### Phase 2: Remove Fake Simulations (Priority: CRITICAL)
2. **Delete or refactor println!-based "simulations"**:
   - `examples/cancer_progression_simulation.rs` - DELETE (replaced by cancer_biomarker_simulation_groundtruth.rs)
   - `examples/alzheimers_progression_simulation.rs` - DELETE (can create ground-truth version later)
   - Audit all other examples/ files for magic numbers and println!-only logic

### Phase 3: Externalize Data (Priority: HIGH)
3. **Convert src/biology/genetics/ data-as-code to actual data files**:
   - Create `data/genetics/` directory with TOML files
   - Start with ONE genetics module as proof-of-concept:
     * Pick `cancer_genetics.rs` (already have cancer_biomarkers ground truth)
     * Extract hardcoded HashMap/Vec into `data/genetics/cancer_variants.toml`
     * Create parser that loads TOML into existing structs
     * Test, commit, then repeat for other genetics modules
   - Expected line count reduction: 20,000-40,000 LOC

### Phase 4: Simplify Module Structure (Priority: MEDIUM)
4. **Refactor src/biology/genetics/mod.rs**:
   - Remove `#[allow(ambiguous_glob_reexports)]` by fixing name collisions
   - Consider consolidating related modules
   - Document which modules are "real" vs "scaffold"

### Current Session: Start with Phase 1 Task 1 - Update README.md with honest status
1. The "Simulations" Are Not Simulations
This is the most critical issue. Many of the extensive example files, which are presented as complex simulations, are nothing more than hardcoded println! statements that describe a pre-written narrative.
Evidence:
alzheimers_progression_simulation.rs and cancer_progression_simulation.rs: These files are hundreds of lines long but contain almost no computational logic. They consist of declaring "magic number" variables and then printing them in formatted strings. There is no state being updated over time, no functions modeling biological processes, and no interaction between variables.
Self-Awareness in docs/SIMULATION_IMPROVEMENTS.md: The project's own documentation perfectly diagnoses this problem. It explicitly states:
"Magic Numbers Everywhere"
"No Actual Simulation - Just Printing"
It correctly identifies that 65% of the cancer_progression_simulation.rs file is just println! statements.
This misrepresentation is severe. The README.md promises a "computational model," but many key examples are just text files disguised as Rust code.
2. Massive Code Duplication and Boilerplate
The project appears to have an enormous line count (~100,000 LOC according to the README), but a significant portion is generated boilerplate or hardcoded data that should not be in the source code.
Evidence:
The src/biology/genetics/ module: This directory contains dozens of files (addiction_genetics.rs, cancer_genetics.rs, mental_health_genetics.rs, etc.) that all follow the same pattern: define some structs and enums, then include a large function like get_..._variants() that returns a hardcoded Vec or HashMap.
This is not code; it's data masquerading as code. This information should be in a data file format (TOML, JSON, CSV, or a simple database like SQLite) and loaded at runtime. This practice makes the codebase brittle, hard to update, and artificially inflates its size and complexity.
mod.rs Hell: The src/biology/genetics/mod.rs file is a massive list of pub mod and pub use statements. The #[allow(ambiguous_glob_reexports)] attribute is a major red flag, indicating that the module structure is so complex it's causing name collisions that the author had to suppress. This level of modularization is counterproductive and makes the codebase harder, not easier, to navigate.
3. Exaggerated and Inconsistent Documentation
The README.md is beautifully written but makes claims that are not supported by the code, which damages the project's credibility.
Evidence:
* "13 complete organ systems": The file structure exists, but the implementation is either missing or, as seen in the examples, a facade.
* "~100,000 lines of code": As noted above, this is likely inflated by data-as-code and boilerplate.
* "All models validated against clinical standards": This is demonstrably false for the println!-based simulations. While some newer examples (acetaldehyde_metabolism.rs) show a real effort towards validation, the claim is far too broad.
* Inconsistent Repository Name: The README.md instructs users to git clone https://github.com/lantos1618/open_human_ontology, but the Cargo.toml lists the repository as https://github.com/yourusername/human_biology.
* Future Date: The README.md states "Last Updated: October 10, 2025," which is likely a placeholder or an agent-generated artifact.
4. The Agent-Driven Development Model Is Showing Its Limits
The agent/ directory reveals that this project is largely built by an AI agent (Claude) running in a loop. This explains many of the observed issues.
LLM Strengths and Weaknesses:
Strengths: The agent is excellent at creating a clean, well-organized file structure, generating boilerplate code, writing polished documentation, and creating descriptive (but non-functional) examples. This is why the project looks so impressive at first glance.
Weaknesses: The agent struggles to create deep, meaningful, and stateful logic. It defaulted to the easiest possible implementation for "simulation"—printing a story. It generated huge amounts of hardcoded data because that's a straightforward way to fulfill a prompt like "create a catalog of genetic variants."
The file docs/SIMULATION_IMPROVEMENTS.md is likely a prompt written by the human developer to course-correct the agent, telling it to stop writing fake simulations and start writing real ones based on a provided template (inflammation_simulation_proper.rs). This indicates a human-in-the-loop is aware of the problem but is now facing a massive amount of technical debt created by the agent.
5. Inconsistent Code Quality
There is a stark contrast between the "old" and "new" code.
Old Code (alzheimers..., cancer...): Procedural, non-functional, full of magic numbers, not leveraging the library's own types.
New Code (acetaldehyde_metabolism.rs, inflammation_simulation_proper.rs): Idiomatic Rust, uses structs and methods, performs actual calculations, and attempts to use a centralized data source (GroundTruthDatabase).
This schizophrenia in code quality makes the project difficult to trust. A contributor doesn't know which pattern to follow or which parts of the codebase are "real."
Actionable Recommendations
Halt All Feature Expansion: The project's breadth is its biggest weakness. No new biological systems or genetic variants should be added until the core issues are fixed.
Systematically Refactor Examples: Aggressively delete or refactor all "simulations" that are just println! statements. Use the SIMULATION_IMPROVEMENTS.md document as a tracking issue to guide this refactoring.
Externalize All Data: Move all the hardcoded HashMap and Vec data from the src/biology/genetics/ modules into TOML or JSON files. Write simple parser code to load this data into the existing structs at runtime. This will shrink the codebase by tens of thousands of lines and make it vastly more maintainable.
Revise the README.md: Update the documentation to reflect the project's actual current state. Be honest about what is implemented versus what is aspirational. A smaller, more honest project is more credible than a large, exaggerated one.
Refine the Agent Workflow: The human developer should guide the agent with more specific, function-level prompts. For example, instead of "Simulate cancer progression," the prompt should be "Refactor the simulate_stage_1_initiation function in cancer_progression_simulation.rs to use the GroundTruthDatabase and model the effect of TP53 mutation on apoptosis rate with an exponential decay function."
In summary, this project has an excellent skeleton but lacks functional flesh on its bones. It's a fascinating case study in agent-driven development, showcasing both the incredible scaffolding capabilities of LLMs and their current limitations in creating complex, logical systems without precise human guidance. The immediate priority should be a deep refactoring to transform the existing data-as-code into true data, and the descriptive "simulations" into real computational models.

**Note:** Historical session logs have been archived to `agent/docs/SESSION_LOGS_ARCHIVE.md`

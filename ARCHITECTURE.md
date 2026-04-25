# Architecture

The codebase has four layers. Each layer has its own programming model and its
own quality bar; mixing them is what produced the orchestration bloat that the
2026-04 cull removed.

## Layer 1 — Static domain types

`src/biology/`, `src/systems/`, `src/physiology/`, `src/pathology/`,
`src/anthropometry/`, `src/biometrics/`, `src/nutrition/`, `src/microbiome/`,
`src/pharmacology/`.

Plain Rust structs that model anatomy and biochemistry. They do not simulate
anything — they are a typed ontology. Methods compute textbook closed-form
formulas (BMI, GFR via Cockcroft-Gault, Frank-Starling stroke volume). Each
module ships unit tests asserting outputs land in physiological reference
ranges.

This is roughly 80% of the LOC.

## Layer 2 — Cited genetics data

`src/biology/genetics/` + `data/genetics/*.toml`.

Population variants, gene catalog, and risk multipliers live in TOML, loaded
behind a uniform `OnceLock + include_str!` accessor. Every value carries a
peer-reviewed source in the TOML so risk numbers are auditable rather than
hallucinated. Sources span Goldman 1943 through Gbadegesin 2024 NEJM.

## Layer 3 — ODE simulations

`examples/*.rs`.

Self-contained binaries, each tied to a named published model with Euler (or
closed-form) integration and output validated against literature reference
values. Pattern is uniform: a model struct, parameters from a citation,
`step(dt)`, printed results, and unit tests asserting physiological ranges.

The currently promoted systems with full L3 coverage:

- cardiovascular — `cardiac_ischemia_detector`, `raas_blood_pressure_control`,
  `hemoglobin_allosteric_cooperativity`
- nervous (biophysics) — `hodgkin_huxley_action_potential`,
  `ghk_resting_potential`, `neuron_spike_train`
- respiratory — `respiratory_pathology`
- endocrine feedback — `insulin_glucose_feedback`,
  `thyroid_hormone_feedback`, `calcium_homeostasis_feedback`,
  `hpa_axis_cortisol_rhythm`
- pharmacology — `multi_compartment_pk`, `pharmacodynamics_emax_model`,
  `alcohol_pharmacokinetics`, `drug_interaction_calculator`
- cellular signaling — `gpcr_camp_signaling_cascade`,
  `glycolysis_kinetics_simulation`
- renal — `renal_clearance_pharmacokinetics`
- acid-base — `acid_base_balance` (Henderson-Hasselbalch, Winters, Boston rules)
- clinical — `clinical_biomarker_analyzer`, `exercise_physiology_vo2max`

## Layer 4 — Validation registries

`src/validation/ground_truth/*.rs`.

Per-domain reference-range registries (oncology, cardiovascular, hepatic, …)
with PMID/DOI citations. About 3.5K LOC across 22 domains. The
`GroundTruthDatabase` interface is intended as the canonical source of
expected biomarker values for L3 examples; in practice only
`cancer_biomarker_simulation_groundtruth` currently calls it. Wiring more
L3 examples to L4 (so their tests assert "value within published clinical
range" rather than hand-coded thresholds) is a standing future task.

## How the layers interact

```
data/genetics/*.toml            (peer-reviewed values + citations)
        │  include_str! + OnceLock
        ▼
src/biology/genetics/*.rs       (typed accessors, lookups)
        │
        ▼
src/biology/*.rs ◀── src/systems/*.rs ◀── src/human.rs   (closed-form physiology)
                                                │
                                                ▼
                                        public top-level API
                                  (Human::new_adult_male, .bmi(),
                                   .cardiac_output_l_per_min(), .gfr_ml_per_min())

examples/*.rs   ── independent ODE binaries (Layer 3),
                   each citing a published model
```

## What is solid vs. what is still scaffold

**Solid (validated against literature):**

- 4 organ-system models with Layer-3 coverage: cardiovascular, nervous,
  respiratory, renal
- 19 example ODE binaries, each tied to a published model with unit tests
- Genetics TOML data for 5 population groups + 8-domain gene catalog +
  cancer risk

**Scaffold (struct trees, no real dynamics):**

- 8 organ systems: immune, endocrine, digestive, sensory, integumentary,
  reproductive, muscular, lymphatic. They have struct trees but methods
  mostly return defaults or compute simple aggregates.

The path to promoting a scaffold is the same every time: pick a clinically
important closed-form or ODE model from the literature, write a Layer-3
example with tests, then expose the model functions from the Layer-1 module.

## Conventions

- One canonical type per concept. Population-specific variants get aliased
  re-exports (`AfricanGenotype`, `EuropeanGenotype`). Avoid creating parallel
  hierarchies — that is what produced the orphan modules removed in the cull.
- Numeric parameters always carry a citation in a comment or in the TOML.
- Validation tests assert ranges, not exact values. Biological measurements
  vary; tests should accept any value a textbook would call physiologic.
- Examples print enough to be skimmable but never exceed one screen of
  intermediate output.

## Out of scope

- "Comprehensive" orchestration types that fan across all systems. Those
  produced the duplication and god-object problem the 2026-04 cull removed.
- Hallucinated parameter values. If a number is not in the literature, do
  not invent it; mark the function as a scaffold and move on.
- Session logs as documentation. Architecture docs describe the current
  state; session history belongs in commit messages and `agent/docs_archive/`.

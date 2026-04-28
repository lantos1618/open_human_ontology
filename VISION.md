# Vision

## What this project is

A Rust library of **cited, testable physiology models** — closed-form formulas
and small ODE simulations — each tied to a published paper, each asserted
against a literature reference range.

Three things, in priority order:

1. **L3 example binaries** — self-contained ODE simulations of named
   published models. Each binary owns its parameters, prints results,
   has unit tests asserting physiologic ranges.
2. **L4 ground-truth registry** — per-domain reference ranges with PMID/DOI
   citations, so an L3 example can call
   `db.get_dataset("renal").is_within_expected_range("gfr_ml_per_min_1_73m2", x)`
   and catch model drift the moment a parameter is mistuned.
3. **L1 cited primitives** — narrow, typed structs that compute textbook
   formulas (Cockcroft-Gault, BMI, Frank-Starling, BSA), with the citation
   inline. Used by L3 examples; not a fan-out ontology.

That's it. Three layers, each with one job.

## What this project is not

- **Not a "human ontology."** The name oversells. There is no global object
  graph of a person; the previous `human.rs` god-object pretended otherwise
  and produced uncited risk-scoring slop. Removed.
- **Not a clinical tool.** Nothing here makes diagnoses or recommendations
  for a real person. Risk-scoring functions that returned a multiplier
  with a string like "Female sex - 2.5x increased risk" were theater and
  have been deleted.
- **Not a pan-system simulator.** It does not simulate "the body." Each
  example is one named model. Cross-system orchestration types that fan
  across all systems are out of scope (the 2026-04 cull removed those;
  do not reintroduce them).
- **Not a competitor to OpenSim, COBRApy, libSBML, PhysiCell.** Those are
  research-grade. This is teaching-grade Rust with an unusual amount of
  citation discipline.

## Quality bar (non-negotiable)

Every numeric parameter carries one of:
- An inline citation comment with author, year, journal, PMID/DOI, OR
- A TOML entry whose schema requires a `source` field, OR
- A textbook formula widely taught (in which case still cite the formula).

If a number cannot be cited, the function does not ship. No "scaffold that
returns plausible defaults"; either it has a source or it does not exist.

Validation tests assert ranges, not exact values. Biological measurements
vary; tests should accept any value a textbook would call physiologic and
fail when a model drifts outside that band.

## What the layers contain (concretely)

```
data/genetics/*.toml                                     L2: cited TOML data
src/biology/genetics/*.rs                                L2: typed accessors
src/{cardiology,renal,respiratory,...}/*.rs              L1: cited primitives
examples/*.rs                                            L3: ODE models
src/validation/ground_truth/*.rs                         L4: registries
```

Note the renaming: `src/systems/cardiovascular/` → `src/cardiology/` etc.
Domain names match what clinicians and textbooks use. No "BodySystems"
wrapper enum. No "Human" struct. The library is a toolbox, not a person.

## What gets cut to get here

Everything that is "system scaffolding without a published model behind it":
- 8 of 11 organ-system trees in `src/systems/` (immune, endocrine,
  digestive, sensory, integumentary, reproductive, muscular, lymphatic
  scaffolds) — keep only the parts cited and used by an L3 example.
- `src/human.rs` (the god-object, ~870 lines after recent slop removal).
  Replace with: nothing. Each example builds the state it needs from
  L1 primitives directly.
- `src/diagnosis/population_analyzer.rs` — survives only if it counts
  cited inputs (sex, ancestry, carrier status). The "ancestry → conditions"
  list survives only if each entry has a PMID.
- ~71 alias re-exports in `src/biology/genetics/mod.rs` — collapse to the
  canonical types; no `AfricanGenotype`/`EuropeanGenotype` shadow names
  unless they wrap genuinely distinct data.

Estimated cut: another 8-12K LOC, leaving ~10-15K LOC of cited material.

## What stays

- All 22 L3 example binaries (already cited and tested).
- All L4 ground-truth registries (PMIDs spot-checked, real).
- L1 primitives that are exercised by an L3 example (Cardiac mechanics,
  GHK/HH ion channel math, Starling forces, Cockcroft-Gault, Henderson-
  Hasselbalch, Mifflin-St Jeor metabolic rate, Bergman/Sriram/etc. ODE
  parameter sets in their respective examples).
- The genetics TOML data + accessors — they're cited and don't pretend
  to be diagnostic; they're a typed lookup.

## How we get there

Two paths. Not both. Each has a clear cost.

**Path A — prune in place.** Keep `main`, delete the cuts above commit by
commit, tighten module names, run tests at each step. ~1-2 days. Preserves
git history of the cull. Risk: emotional inertia; some cruft survives
because deleting it would touch too many imports.

**Path B — redo on a new branch.** New branch `lean`, copy over only the
22 examples + the L4 registry + the genetics TOML + the cited L1 primitives
they actually use. Build `Cargo.toml` and `src/lib.rs` from scratch. Drop
everything else on the floor. ~2-3 days. Risk: duplicated work; integration
gaps until the new tree compiles.

**My take:** Path B. The existing tree's structure encodes the wrong
framing (per-system trees, god-object, comprehensive-everything). Pruning
fights that gravity. Starting from "examples + registry + cited primitives"
encodes the right framing from line one. Lose ~10 KLOC of typed-but-empty
struct hierarchies and never miss them.

## Naming

The crate name `human_biology` with the doc-comment "Human Ontology" is
inherited and oversells. Honest options:
- `cited_physiology` — ships the actual goal.
- `physiology_models` — boring, accurate.
- Keep `human_biology` but rewrite the doc-comment to "ODE physiology
  models with literature-cited validation."

Pick one before path A or B, because it shapes module names.

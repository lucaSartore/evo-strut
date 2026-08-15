# Master's Thesis — Genetic-Algorithm Support Structure Optimization for FDM 3D Printing

This directory contains Luca Sartore's Master's degree thesis (Artificial Intelligence
Systems, 2026), a LaTeX `book`-class document. The root file is
`sartore_luca_artificial_intelligence_systems_2026.tex`, which `\input`s the individual
chapter files (`abstract.tex`, `introduction.tex`, `previous_literature.tex`,
`motivations.tex`, `monte_carlo_stiffness_evaluation.tex`,
`support_optimization_algorithm.tex`, `results.tex`, `conclusions.tex`) plus
`front.tex` and the bibliography (`biblio.bib`).

## Core idea

3D printing (additive manufacturing) builds objects layer by layer, which fails on
"overhangs" — regions with nothing underneath — unless temporary **support
structures** are printed alongside the object and later discarded. Supports are pure
waste material, so minimizing how much support material is used is a real,
practical optimization problem.

Support-generation research so far has focused on industrial technologies (SLA/SLS),
which exert minimal force on the object while printing. **FDM (Fused Deposition
Modelling)** — the dominant *consumer*-grade 3D printing technology — is different:
the nozzle physically pushes on the part as it deposits material, so supports also
need enough **stiffness** to resist bending/detaching, not just enough presence to
catch a falling layer. Because of this, genetic-algorithm-based support optimizers
from the literature (Zhang et al. 2020, Vaissier et al. 2019), which use thin
tree-like support topologies optimized with no notion of stiffness, are not portable
to FDM and can literally cause failed prints (demonstrated by a real "z-wobble"
print-test failure on a plain tree topology).

## Thesis contribution

The thesis fills that gap: it designs a genetic-algorithm-based support structure
optimizer built specifically for FDM printing. Key pieces:

1. **Topology choice — rigid frame ("struct"), not a tree.** A case study comparing
   three topologies (tree / struct / cylinder) at equal height shows plain trees are
   the cheapest in material but structurally fail on FDM printers; the rigid frame
   topology (multiple ground connections, not just a single trunk) is chosen as the
   best material/stiffness tradeoff.
2. **Approximated Carlo stiffness evaluation** (major contribution). Exact stiffness/flexibility
   evaluation of a rigid frame is too slow to run the hundreds of thousands of times a
   genetic algorithm needs. A custom Approximated Carlo approximation is ~400x faster while
   sacrificing only ~4% accuracy, which is what makes the whole optimization
   computationally viable.
3. **Non-tree support structure optimization** (major contribution). A GA capable of
   optimizing rigid-frame topologies (not just trees, unlike prior work).
4. **Improved critical/floating region detection** — better propagation-aware
   detection of which regions need support vs. support + stiffness, validated on a
   "Dragon" print test that failed with stock state-of-the-art tools but succeeded
   with this method, unattended.
5. **Contact point grouping via CPPNs** (Compositional Pattern Producing Networks) —
   used for a divide-and-conquer speedup of the optimization; the GA itself runs in
   three stages: contact point selection, area grouping, and structure optimization.

## Headline result

~43% average reduction in support material consumption vs. OrcaSlicer's "Tree
Supports" (the best FDM-compatible baseline), i.e. roughly 10–30g saved per print,
validated with real physical print tests (all printed successfully). Direct
comparison to the SLA/SLS academic baselines (Zhang et al., Vaissier et al.) was not
possible — their code isn't public.

**Downsides:** higher upfront computation time, somewhat longer print times, and
slightly worse print quality at contact points than tree supports — all discussed as
either structural tradeoffs or fixable implementation limitations (60+ tunable
parameters, no slicer integration, contact-point topology quality) in the
Conclusions/Future Work.

## Research question

"How much material can be saved by applying genetic algorithms to support structure
optimization in FDM 3D printing?" — answered: ~43%.

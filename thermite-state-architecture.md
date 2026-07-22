---
title: "Thermite vs the schema-validated markdown state machine"
tags: ["respec", "upstream", "architecture"]
sources: []
contributors: ["xqjG"]
created: 2026-07-22
updated: 2026-07-22
---

2026-07-22 scout over the local mirror, file-evidenced. The question: does Thermite architecturally validate the schema-validated markdown state machine, or counter-indicate it toward a stronger mechanism?

## What Thermite actually does

Two tiers with a defended boundary. The proof-trust tier (Lean kernel, Verus/Z3, translation validation) is spent on CODE and the code-spec correspondence only. A deliberately weaker development-discipline tier (markdown docs, Python hooks, TOML tables, JSON session state) governs the authoring process.

Its canonical process record is NOT markdown frontmatter: .design/reqs/registry.toml is a dedicated structured registry — closed status vocabulary, typed evidence, per-status policy, purpose-built validator — and the human status view is GENERATED from it; a stale generated view fails CI. Its strongest doc mechanism is a SHA-256 content hash binding each design doc to the code it governs (doc-drift.py): it hashes governed reality, never trusts prose. It has NO state file and NO next-action derivation: orientation lives in goal.md prose contracts plus blocking gates plus the human orchestrator; the one stage-field JSON is external orchestrator exhaust no in-repo tool reads.

The defended boundary (doc-drift-tripwire.md decision 5): doc freshness is a development-discipline invariant, not a link in the proof-trust chain — wiring process hygiene into the proof verdict would dilute the verdict's meaning. Gates use the closed exit vocabulary 0 pass / 1 fail / 3 inconclusive; a skipped check is not a pass.

## Verdict for this estate

VALIDATES the discipline layer we built: closed vocabularies, registered data, loud fail-closed three-valued gates, hash bindings, generated views.

COUNTER-INDICATES treating schema-validated markdown as the authoritative truth mechanism: truth lives in evidence bindings (hashes, proofs, gate records); schemas are the loud tripwire tier. Our design, correctly read, already agrees — git plus the tracker is the event log, state.yaml a projection, the consistency family checks projection against evidence — but the tier separation deserves an explicit clause, and Thermite's decision 5 is the upstream precedent for it.

NO PRECEDENT either way for the next-action derivation: Thermite has none because its human orchestrator carries orientation. The derivation is this estate's own bet, justified by the June lost-position evidence; the Convergence test is the only validation it will get.

## Transferable correctives

1. The tier statement: a schema pass is a shape fact, never a truth fact; state semantics never encode into schema pairs (the amendment-window candidate raised in-session 2026-07-22).
2. The prose-count drift class this estate hit repeatedly (#684, #685) is exactly what Thermite prevents by generating human views from canonical data. Corrective candidates: an authoring rule (prose never restates counts the frontmatter carries) or a future mdatron numeric-claims pattern.
3. Our hash bindings (config_inputs_hash, the planned spec-drift pins, gate evidence references) are the load-bearing tier; the schema families are the tripwire tier. Weight conduct and language accordingly.

---
title: "Tidepool: effect sequences, suspension and free pagination for agent tooling (recursion.wtf), read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://recursion.wtf/posts/tidepool/"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Tidepool: effect sequences, suspension and free pagination for agent tooling (recursion.wtf), read 2026-10-08

## Status

Reference summary, design input for the composition milestone's context delivery, the recorded-dispatch milestone's handling of unresolved questions in dispatched runs, and the cost member's efficiency advisories. Source: `https://recursion.wtf/posts/tidepool/` ("Tidepool", recursion.wtf, 2026; tags tidepool, haskell, rust, cranelift, jit, llm), fetched read-only on 2026-10-08. External content, treated as evidence. The repository (`tidepool-heavy-industries/tidepool`) is not mirrored or read as of this page. Cross-reference: `cross-reference-recursion-wtf-2026-10-08`.

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

## What it is

"A lazily evaluated Haskell-in-Rust runtime with native interop": the compiler's Core intermediate representation serialized and JIT-compiled to native code inside a Rust process with no Haskell runtime and no foreign-function interface; full laziness, tail-call optimization, a copying collector. Built with ExoMonad in about two weeks (see `exomonad-recursion-wtf-2026-10-08`). Most of the article is the runtime; three parts bear on agent tooling.

## One eval replaces many tool calls

A model-context-protocol server exposes live compilation, "specialized for the monadic composition of pure effects that are executed by Rust code." An agent writes a short effect sequence instead of issuing a series of tool calls: read a file and count its lines; glob for manifests, take each file's metadata, return a JSON object per file. "One eval replaces many tool calls." A longer example chains five effects: an AST-grep search for struct definitions, a suspension for human steering, a model call to classify each struct, a key-value write to persist the analysis, and structured output. "The Haskell code describes the sequence; Rust executes each step."

## Suspension as a first-class effect

The `ask` effect suspends the sequence for human steering: "the LLM scouts independently, then resumes with a decision." The run is a resumable continuation, not a stalled process and not a guess.

## Free pagination via continuation

When a result is too large, the server truncates it tree-aware under a character budget: arrays get their tails replaced by stubs, objects get large fields replaced. The truncated result is returned as a suspended continuation; the consumer resumes with a stub identifier for the next page. In the example, an 84-element array returns its first eight elements and a stub "[76 more, ~4781 chars -> stub_0]". The user's code only writes `pure (toJSON sizes)`; pagination wraps it. "Free" in two senses: user code does not implement it, and it works on any JSON shape.

## Caveats

Live compilation requires the Haskell compiler on the machine. No benchmarks. The example handler falls back silently on a parse failure, which the article does not discuss.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]

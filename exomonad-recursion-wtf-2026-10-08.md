---
title: "ExoMonad: a tree of worktrees and guidance encoded as code (recursion.wtf), read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://recursion.wtf/posts/exomonad/"
    title: ""
    accessed_at: "2026-10-08"
  - url: "https://github.com/tidepool-heavy-industries/exomonad"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# ExoMonad: a tree of worktrees and guidance encoded as code (recursion.wtf), read 2026-10-08

## Status

Reference summary, design input for Slice 6 (dispatch and the review stage), the 2a primer, and the hooks-as-data doctrine. Source: `https://recursion.wtf/posts/exomonad/` ("ExoMonad", recursion.wtf, 2026; tags exomonad, tidepool, agent-orchestration, haskell, rust, llm), fetched read-only on 2026-10-08. External content, treated as evidence. The repository (`tidepool-heavy-industries/exomonad`) was mirrored read-only on 2026-10-08 at commit 98839fe (2026-08-31); its model, decision records and praxis are summarized in the last section. Cross-reference: `cross-reference-recursion-wtf-2026-10-08`.

## What it is

An orchestration engine that replaces "a swarm of agents ramming PRs into main" with a tree of worktrees. It attaches to the Claude Agent Teams message bus ("on-disk mailboxes and JSONL") so agents on other model architectures appear as native team members; it integrates with Copilot for pull-request review; it runs Claude, Gemini, Kimi, Letta Code and Copilot through their existing binaries and the user's existing subscriptions. It ships a default "devswarm" configuration for worktrees, coordination and iterating on pull requests, and is "radically reconfigurable": the author wrote a separate red-team configuration overnight (unpublished; "write your own"). It built itself over 700-plus pull requests, then built a Haskell runtime in Rust (Tidepool).

## Heterogeneous orchestration, by role and economics

"Opus is smart but expensive. Gemini is flaky but cheap and fast. Copilot is integrated with GitHub PR review and heavily subsidized." The workflow is "token arbitrage": each model for what it does best, shifted by which provider is subsidizing usage in a given week. Default roles: tech leads on Opus, "planners and project managers" who orchestrate, spawn children and review pull requests; devs on Gemini, who write code, file pull requests and iterate with Copilot on review feedback; workers, like devs but ephemeral, running in the spawning lead's directory.

## The tree of worktrees

Worktrees share a repository's object store. The author contrasts Gastown's model (one merge queue per repository, every worktree filing against main, contention solved by a dedicated queue-manager worker) with nested worktrees: a lead owns the root branch, splits a feature into subtasks, and spawns a subagent per subtask; small tasks go to devs as a single pull request, larger ones to leads who decompose recursively "with human steering as needed" until units are small enough to implement unsupervised. "A tree of worktrees, mirroring the structure of the project itself."

Four stages:

1. **Initial.** A lead writes scaffolding (types, tests, stub files, planning documents) as an initial commit. This becomes the base every subtask branch forks from.
2. **Unfold.** The lead decomposes and spawns waves of subagents, each in its own worktree off the scaffolding commit; a sub-lead spawns its own children the same way.
3. **Fold.** Subagents file pull requests against their parent branch, iterate with the automated reviewer, and notify the spawning lead; the lead merges completed pull requests and launches follow-up waves.
4. **Wave 2.** Independent subtasks are front-loaded into wave 1; the merged result becomes the scaffolding commit for wave 2. "Every agent in a wave forks from the same commit, so there is no rebase burden within a wave." Ancestors merge upward: leaf branches into their parent, parents into main.

## Guidance encoded as code

All orchestration logic runs in a shared server at the repository root; configuration, sockets and worktrees live in one directory. Early versions used YAML with custom expression languages; the author "kept re-implementing subsets of Haskell, mostly various forms of effect composition," and decided to use the language itself: routing, hooks, tool dispatch and event handling are Haskell effects executed in a WASM sandbox with predefined effects implemented by the Rust host. "A restricted subset", designed "for coding agents to modify, with humans in the loop for code review."

The worked example: Gemini kept stripping the dash from `#-}` in language pragmas. A pre-tool hook matches the replace tool on Haskell files and blocks a replacement whose old text contains `#-}`, whose new text lacks it, and whose new text contains `#}`:

```haskell
checkPragmaCorruption (Replace fp old new)
  | ".hs" `T.isSuffixOf` fp
  , "#-}" `T.isInfixOf` old
  , not ("#-}" `T.isInfixOf` new)
  , "#}" `T.isInfixOf` new
  = Just "BLOCKED: Your replacement corrupts Haskell LANGUAGE pragmas..."
checkPragmaCorruption _ = Nothing
```

"This avoids spending tokens prompting every Gemini agent, including those that never touch Haskell. The guidance is encoded as code." The author frames it as Kaizen, "a continuous process of finding opportunities to make 1% improvements"; fixes "become permanent infrastructure rather than a one-off prompt tweak," and "over time, these compound." A second example routes review events: a received review is injected as a message to the agent; an approval notifies the parent that the pull request is ready.

## Evidence of throughput

Tidepool, "a lazily evaluated Haskell-in-Rust runtime with native interop," built from scratch in about two weeks using less than half of two subscription plans: "over 32,000 lines of code across 61 PRs in just 4 days of active waves," then a week and a half of human-in-the-loop bug squashing, feature support and polish. The author contrasts this with a far costlier published compiler demo ("I don't have $20k to burn on tokens").

## Caveats

No benchmarks, error rates or cost comparisons; the evidence is the build story and output volume. "Human steering as needed" is not quantified. The hook example is one narrow heuristic. The red-team configuration is unpublished. The Haskell subset is described as restricted, and the author says readers need not know what a monad is.

## The repository (commit 98839fe, 2026-08-31, read 2026-10-08)

**The model, in the repository's own words.** "ExoMonad is a hylomorphism over context windows. The unfold is plan + scaffold + spawn; the fold is merge + integrate + surface-upward." Each node is an agent triad, worktree, context window and actor, "born, living, and dying together", one to one to one. Sub-leads are "compression boundaries": a root with three sub-leads each managing four leaves "sees O(3), not O(12)"; a four-level tree with branching factor three has 81 leaves "but no single context window ever reasons about more than 3 children." Waves are the rhythm: "the wave boundary is where understanding accumulates; the TL reads the merged diffs, learns what the children actually built, and uses that knowledge to write sharper specs for the next wave." Branch names are a coordinate system: dot-separated `{parent}.{name}` encodes the tree address, the parent of a branch is derived mechanically from its name, and every pull request targets its parent branch, never main.

**Spec commits (an accepted decision record).** Before spawning children, the parent commits a spec commit with three layers applied by depth: types (type signatures, trait definitions, function stubs; always); intent (decision-record-style markdown; shallow nodes only); acceptance (failing tests and property-test stubs; when feasible). "Children fork from this commit. The type stubs define module boundaries; each child owns specific files." The scaffold "need not be finished, globally green, or even compilable when a clear placeholder communicates the boundary better." A fresh child context "independently re-derives the implementation from those artifacts, which is part of the review architecture"; inheriting the parent's transcript is opt-in, "not the default reasoning channel." Consequences stated: the compiler catches interface mismatches between children; a child that needs a shared type changed sends a question to the parent.

**Tech lead praxis.** Delegate substantial independent work by default; handle small work, shared scaffolding, integration, conflicts and diagnostics directly when delegation costs more than it saves. More than about four independent leaves means interposing a sub-lead. Never poll: decompose, spec, spawn, then do useful work until pushed child events arrive. Spec quality, one shot: "objective and observable done criteria first, then mechanical paths, small read-first context, concise constraints, optional steps, exact verification, and handoff; repository-relative paths and concrete commands." Calibrated escalation: a leaf that fails repeatedly reports what it tried; the lead re-decomposes or escalates when authority or scope is missing.

**Model-gradient forking (accepted 2026-06-10, "decided in an interactive design review, two interview rounds, decisions by the user").** The tree's economics is an intelligence gradient: "expensive context decomposes, cheap leaves implement." The design forks a root's context window down-gradient so that cheaper sub-leads "inherit the full worked reasoning, the why, the rejected alternatives, the escalation tripwires" at the cheaper rate with a warm cache.

**Decision records.** Seventeen files under `docs/decisions/`, each with a status line (Accepted, Implemented with the commit, Proposed, Superseded with the date and reason) and, on the later ones, review provenance ("reviewed adversarially, mechanics compile-proven in a scratch crate"; "design interview"). One records the authoring-DSL design sentence: "A role definition is pure data: a tool roster, two hook pipelines, an observer list, and a session-start function. The framework owns the folds and the JSON erasure; the domain contributes typed tools and typed stages." Roles are data; hooks are pipelines of typed stages.

**An incident of its own.** A post-wave audit document (2026-04-16) consolidates the automated reviewer's feedback across fifteen merged pull requests after "a poller state-machine limitation" treated reviews with inline comments as no review, "leading to several actionable suggestions being bypassed during the wave." Each item carries a verdict (action needed, or not). The control that was authored was not exercised; the audit is the repair.

**Also present.** A notes file summarizing another practitioner's agentic-engineering patterns (red-green test-driven development, run the tests first, agentic manual testing, the compound-engineering loop of documenting what works per project); plans with dated handoffs; a one-command containerized trial that mounts the user's existing credentials read-only.

---
title: "ExoMonad: a tree of worktrees and guidance encoded as code (recursion.wtf), read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://recursion.wtf/posts/exomonad/"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

Reference summary, design input for Slice 6 (dispatch and the review stage), the 2a primer, and the hooks-as-data doctrine. Source: `https://recursion.wtf/posts/exomonad/` ("ExoMonad", recursion.wtf, 2026; tags exomonad, tidepool, agent-orchestration, haskell, rust, llm), fetched read-only on 2026-10-08. External content, treated as evidence. The repository (`tidepool-heavy-industries/exomonad`) is not mirrored or read as of this page. Cross-reference: `cross-reference-recursion-wtf-2026-10-08`.

### what it is

An orchestration engine that replaces "a swarm of agents ramming PRs into main" with a tree of worktrees. It attaches to the Claude Agent Teams message bus ("on-disk mailboxes and JSONL") so agents on other model architectures appear as native team members; it integrates with Copilot for pull-request review; it runs Claude, Gemini, Kimi, Letta Code and Copilot through their existing binaries and the user's existing subscriptions. It ships a default "devswarm" configuration for worktrees, coordination and iterating on pull requests, and is "radically reconfigurable": the author wrote a separate red-team configuration overnight (unpublished; "write your own"). It built itself over 700-plus pull requests, then built a Haskell runtime in Rust (Tidepool).

### heterogeneous orchestration, by role and economics

"Opus is smart but expensive. Gemini is flaky but cheap and fast. Copilot is integrated with GitHub PR review and heavily subsidized." The workflow is "token arbitrage": each model for what it does best, shifted by which provider is subsidizing usage in a given week. Default roles: tech leads on Opus, "planners and project managers" who orchestrate, spawn children and review pull requests; devs on Gemini, who write code, file pull requests and iterate with Copilot on review feedback; workers, like devs but ephemeral, running in the spawning lead's directory.

### the tree of worktrees

Worktrees share a repository's object store. The author contrasts Gastown's model (one merge queue per repository, every worktree filing against main, contention solved by a dedicated queue-manager worker) with nested worktrees: a lead owns the root branch, splits a feature into subtasks, and spawns a subagent per subtask; small tasks go to devs as a single pull request, larger ones to leads who decompose recursively "with human steering as needed" until units are small enough to implement unsupervised. "A tree of worktrees, mirroring the structure of the project itself."

Four stages:

1. **Initial.** A lead writes scaffolding (types, tests, stub files, planning documents) as an initial commit. This becomes the base every subtask branch forks from.
2. **Unfold.** The lead decomposes and spawns waves of subagents, each in its own worktree off the scaffolding commit; a sub-lead spawns its own children the same way.
3. **Fold.** Subagents file pull requests against their parent branch, iterate with the automated reviewer, and notify the spawning lead; the lead merges completed pull requests and launches follow-up waves.
4. **Wave 2.** Independent subtasks are front-loaded into wave 1; the merged result becomes the scaffolding commit for wave 2. "Every agent in a wave forks from the same commit, so there is no rebase burden within a wave." Ancestors merge upward: leaf branches into their parent, parents into main.

### guidance encoded as code

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

### evidence of throughput

Tidepool, "a lazily evaluated Haskell-in-Rust runtime with native interop," built from scratch in about two weeks using less than half of two subscription plans: "over 32,000 lines of code across 61 PRs in just 4 days of active waves," then a week and a half of human-in-the-loop bug squashing, feature support and polish. The author contrasts this with a far costlier published compiler demo ("I don't have $20k to burn on tokens").

### caveats

No benchmarks, error rates or cost comparisons; the evidence is the build story and output volume. "Human steering as needed" is not quantified. The hook example is one narrow heuristic. The red-team configuration is unpublished. The Haskell subset is described as restricted, and the author says readers need not know what a monad is.


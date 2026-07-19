---
title: "Domain value scorecard (five-dataset empirical review)"
tags: ["respec", "review-methodology", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-07-19
updated: 2026-07-19
---


## Design Specification

### verdicts by tier

**Load-bearing on code (all datasets agree):** software-engineer (best substance ratio; atomic-write, symlink-reject, panic-risk catches), quality-engineer (mutation testing is the single most reliable value source — four surviving-mutation catches in issue-tracker alone; killed a tautological test in the cycles), security (value arrives in cold-batch bursts: SIGPIPE panic, u64::MAX overflow, duplicate-ID load found in one round after five warm rounds passed them), solution-architect (caught the E0001/E0050 reserved-code contract break), platform-engineer (CI, deny.toml, toolchain pinning, lint floors — verifiably present in trees today).

**Load-bearing on prose artifacts:** technical-writer (largest finding-owner on the suite corpus, 19/51; owns the audit-trail format) with documentation-reviewer as its cold-reader pair — externally validated: a genuine cold reader hit exactly the implicit-knowledge defects DR exists to catch and TW-the-author could not see. AI-engineer earned load-bearing status through suite dogfooding (13/51 forward findings; born from a 1.2M-token round hitting the daily limit).

**Situational:** red-team (real value only where a genuine attack surface exists — lint evasion; zero findings against prose; noise leader on a no-network CLI: ~3 substantive vs ~34 hallucinated), ux (thin on headless CLIs), data-engineer (near-total overlap with security on these project shapes).

**Zero-yield on non-matching shapes (confirmed 4x):** performance, privacy, accessibility, localization — 23 empty "no findings" issues in the cycles are what running them anyway costs.

### structural lessons

1. Domain value is a function of artifact shape, not domain identity (TW/DR invert between code and prose).
2. Pairs are generator-discriminator structures (Security-RedTeam, TW-DR; sanity-check as validator of last resort) — never fold a pair; enforce separation with tool-restricted roles. Codified in suite Review 77.
3. The operator's manual test is the best single defect-finder on record: the ID-reuse invariant bug was caught by the director's checklist after 11 cold reviews missed it.
4. 50/51 suite-corpus findings were director-raised; external cold readers punch far above internal cold sessions on prose. Spend cold review on code; spend operator attention and external readers on prose.
5. Small early spec-stage cold review (3 domains, 13 findings, one day after authoring) changed design direction; 18-domain late cycles yielded ~20% substantive. Review breadth added late is where hallucinations come from: rounds past the stop signal manufactured 7-of-8 hallucinated findings.
6. Hallucinated findings cite code not in HEAD — evidence-gated filing (citation verified against HEAD, or a failing test) removes the class.


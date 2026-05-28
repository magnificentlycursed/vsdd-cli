---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-05-27
phase: phase-3
scope: Tier 1 spec set (methodology.md + README + DESIGN-METHODOLOGY + DESIGN-SCHEMA + DESIGN-OBSERVABILITY + DESIGN-VERIFICATION + 10 phase primers + 18 domain prompts) — Communication-cluster review (Technical Writer + AI Engineer + Security lenses)
lens: Communication cluster — TW + AIE + Security lenses applied to Tier 1 spec artifacts; prose-surface coherence + AI-runtime-cost discipline + credential-handling + supply-chain + threat-model dimensions
source: domain-raised
session_note: cold-context — first Phase 3 IAR round on the spec set; Communication cluster composed via 3-domain skill-mode aggregation in cold-context Agent spawn; adversarial-pair separation invariant honored (Security ↔ Red Team on different clusters; TW ↔ DR on different clusters); cluster-shape declared per Phase 3 primer's 4-cluster default. Dispatcher's actual Communication cluster composition (TW + AIE + Security) diverges from the Phase 3 primer's declared composition (Security + TW + Accessibility + Privacy + Localization); divergence captured as Finding 9 below.
model: claude-opus-4-7
execution_method: Agent-tool subagent spawn (cold-context approximation; main session orchestrator dispatched per cluster-batching shape with --no-memory equivalent via Agent tool isolation)
sycophancy_compensation: I share substrate with the authoring instance; the bias is to read the spec set as "internally coherent because I would have written it the same way." The compensation pressure: enumerate cross-doc contradictions the author normalized + name what the spec promises but doesn't deliver mechanically. Resisted accepting "auth_method splits into operator_local + ci subfields" as sufficient credential-handling discipline when key-rotation paths, AuthMethodChanged event-variant status (retired vs reserved-for-v1+), and the "operational runbook" referenced for compromised-credential procedure don't actually exist as committed artifacts. Resisted accepting "methodology.md is the canonical governing spec" when it lacks the abbreviation reference table that the README has + omits CHANGELOG discipline + omits MCP server + omits capture-source enum — all referenced as load-bearing in DESIGN docs but absent from the standalone document deployed to adopting projects via `vsdd init`.
---

# Communication Cluster Review 1 — 2026-05-27

**Phase 3 cluster:** Communication (TW + AIE + Security composed via 3-domain aggregation)
**Cold-session shape:** Agent-tool subagent spawn (operator-dispatched from main session; cluster prompt + primer + supplements + project tree readable; no operator-memory mount)
**Adversarial-pair separation:** preserved — Security ↔ Red Team on different clusters (Red Team is in the Adversarial cluster per Phase 3 primer); TW ↔ DR on different clusters (DR is in the Adversarial cluster).

## Scope + method

Read in primer-mandated order: Phase 3 primer + 3 domain prompts + methodology.md + README + 4 DESIGN docs + supplements (markdown / claude-code-cli / github-actions) + vocabulary.yaml + anonymization-patterns.yaml registries + Phase 5 round 1 prior review entries (SA + QE + Security) for context on already-routed findings.

Three-lens enumeration:

- **TW lens** — first-use expansion discipline, vocabulary registry coverage, cross-document staleness, quantitative-claim discipline, audience-altitude, Mentor voice, CHANGELOG cooperation.
- **AIE lens** — capture-source provenance completeness, cost-band cataloging, prompt-cache discipline, cluster-batching coherence with practice, sub-agent scope-down, model-tier right-sizing, auth × cost-model coordination.
- **Security lens** — threat-model specificity, trust-boundary placement, credential handling, auth + authz, input validation, supply-chain integrity, forensic-trail integrity, bypass-marker discipline.

Findings 1-12 below. Per primer 3 + Source field discipline, each carries finding_id + domain + dim + classification + routing.

## Findings

### Finding 1 — methodology.md lacks first-use abbreviation expansion (Dim 1: first-use expansion) — Open

`finding_id: 1-f1` · `domain: technical-writer` · `dim: 1` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: methodology.md, target_section: opening-scope }`

`methodology.md` is declared the "canonical governing spec" deployed standalone to adopting projects via `vsdd init` (DESIGN-METHODOLOGY § Methodology spec section list, `methodology.md:57`). Domain abbreviations SO / SA / SE / QE / TW / DR / PE / UX first appear in the Phase taxonomy table at `methodology.md:82` and the Phase-domain composition table at `methodology.md:111-119` **without any first-use expansion** in the document body. The README has the abbreviation reference table (`README.md:47`). The methodology spec does not.

An operator opening `methodology.md` deployed to their adopting project sees `SO`, `SA`, `QE`, `TW`, `DR`, `PE`, `UX`, `IAR`, `MVR` in tables with no inline expansion and no glossary section. The `check-naming-discipline.py` hook (DESIGN-VERIFICATION:181, rule "first-use expansion") would fire `VSDD-W0001` against the canonical methodology spec the toolkit ships.

**Why this matters:** the toolkit dogfoods on itself; if `methodology.md` violates the first-use expansion discipline the toolkit enforces, the discipline isn't load-bearing in the canonical document — it's aspirational. Two-audience principle is undermined: a human cold-reader can't decode the spec without the README.

**Routing:** Phase 4 → Phase 1a (methodology.md revision). Add an opening-scope subsection equivalent to README's "Conventions used in this doc" + expand SO / SA / SE / QE / TW / DR / PE / UX / AIE on first use OR add explicit abbreviation table immediately after the opening scope section. Coordinates with Finding 2 (vocabulary registry coverage gap).

### Finding 2 — Vocabulary registry omits the domain-abbreviation set (Dim 2: vocabulary registry conformance) — Open

`finding_id: 1-f2` · `domain: technical-writer` · `dim: 2` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: templates/registry/vocabulary.yaml, target_section: managed-section }`

`templates/registry/vocabulary.yaml` (deployed to `.vsdd/registry/vocabulary.yaml`) is the canonical methodology terms registry. It carries entries for VSDD / MVR / Exit Signal / Exacting Mentor / IAR / Red Gate / capture-source / Raise to SO / phase-domain composition / always-on baseline. It does **not** carry entries for SO / SA / SE / QE / TW / DR / PE / UX / AIE / PerfE / OTel / OTLP / MCP / SDK / SARIF / LSP / FinOps — the 17 most-frequently-used abbreviations in the spec set (31 occurrences in methodology.md alone per grep).

The `check-naming-discipline.py` hook's "vocabulary registry compliance" rule (DESIGN-VERIFICATION:181) scans for deprecated aliases via the registry; an unregistered abbreviation is per DESIGN-METHODOLOGY:806 a "novel-term-without-registry-entry" candidate-code surface. The registry's coverage gap means the hook can't fire on the most-used terms — they're below the floor of detectability.

**Why this matters:** the registry's purpose is canonical-term enforcement. If the 17 most-frequent abbreviations aren't in the registry, the registry is decorative — a checkbox declaration with no operational teeth.

**Routing:** Phase 4 → Phase 1a (vocabulary.yaml extension). Add entries for the 17 abbreviations with their canonical expansions + domain_scope. Composes with Finding 1 (methodology.md first-use expansion section).

### Finding 3 — AuthMethodChanged + AuthFailureObserved event variants in cross-doc contradiction (Dim 3: cross-document staleness detection) — Open

`finding_id: 1-f3` · `domain: technical-writer` · `dim: 3` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: README.md+DESIGN-METHODOLOGY.md+DESIGN-OBSERVABILITY.md, target_section: auth-method }`

`AuthMethodChanged` + `AuthFailureObserved` event-variant status is contradicted across docs:

- `README.md:265` — `AuthMethodChanged` exists; fires on "rotation, scale-shift, plan-credit-exhaustion fallback."
- `README.md:285` — "AuthMethodDeclared / AuthMethodChanged / AuthFailureObserved event variants provide forensic record" (all three exist).
- `README.md:288` — "emit AuthFailureObserved" on compromised credential (variant exists).
- `README.md:340` — `AuthMethodChanged, AuthFailureObserved` "are reserved for v1+ adoption pending operator-rotation or rate-limit recurrence evidence."
- `DESIGN-METHODOLOGY.md:308` — "Operator declares at `vsdd init` time + may change per `AuthMethodChanged` event" (exists).
- `DESIGN-METHODOLOGY.md:333` — "`AuthMethodChanged` + `AuthFailureObserved` retired/consolidated per the variant-proliferation governance audit; rate-limit + invalid-credential events covered by Agent SDK OTel signals natively (no methodology-specific variant needed). Auth-rotation events route through `OperatorDirectiveApplied{directive: auth-method-rotation}` consolidated variant."
- `DESIGN-SCHEMA.md` 18-variant payload list (~lines 306-323) and `DESIGN-OBSERVABILITY.md` § Auth + identity variants table — list only `AuthMethodDeclared`; `AuthMethodChanged` + `AuthFailureObserved` are absent.

Three contradictory states coexist in the spec set: (a) variants exist and fire (`README:265`, `README:285`, `README:288`, `DESIGN-METHODOLOGY:308`); (b) variants are "reserved for v1+" (`README:340`); (c) variants are "retired/consolidated" with routing through `OperatorDirectiveApplied{directive: auth-method-rotation}` (`DESIGN-METHODOLOGY:333`). The schema set (DESIGN-SCHEMA) and the OTel emission tables (DESIGN-OBSERVABILITY) honor (c) — variants do not exist. README's text honors (a) in three places.

**Why this matters:** the Security narrative around auth-rotation in README (line 285 + line 288) tells the operator to expect `AuthFailureObserved` in the audit trail; the actual schema set + emission tables don't define the variant. An operator following README's compromised-credential procedure (line 288) would emit an undefined event that no validator + no collector would accept. This is exactly the document-staleness recurrence the suite-development reviews surfaced — `VSDD-W0030` candidate territory.

**Routing:** Phase 4 → Phase 1a (README + DESIGN-METHODOLOGY auth-method section coordinated revision). Pick the canonical state: either (a) define both variants in DESIGN-SCHEMA + DESIGN-OBSERVABILITY + retract the "reserved for v1+" language in README; OR (c) update README:265 / 285 / 288 to route auth-rotation + compromise through `OperatorDirectiveApplied{directive: auth-method-rotation | credential-rotation}` consistently with DESIGN-METHODOLOGY:333. Recommend (c) — preserves variant-proliferation governance discipline; matches DESIGN-SCHEMA's existing 18-variant set.

### Finding 4 — methodology.md omits CHANGELOG discipline (Dim 8: CHANGELOG cooperation discipline + Dim 3: cross-document staleness) — Open

`finding_id: 1-f4` · `domain: technical-writer` · `dim: 8` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: methodology.md+DESIGN-METHODOLOGY.md, target_section: methodology-spec-section-list }`

`methodology.md` has **zero mentions of CHANGELOG** across its 415 lines. DESIGN-METHODOLOGY § Methodology spec section list (`DESIGN-METHODOLOGY.md:59-78`) — the table of required sections for the canonical methodology spec — does not include a CHANGELOG-discipline section. But:

- DESIGN-METHODOLOGY §§ "CHANGELOG discipline (design)" (lines 698-742) spends ~50 lines on CHANGELOG as the 15th — now 13th — artifact class.
- DESIGN-SCHEMA defines CHANGELOG as the only structural-mode validation class with 10-rule discipline (lines 579-631).
- DESIGN-VERIFICATION declares `check-changelog-discipline.py` as hook 16 (line 193) with 5 accepted + 5 candidate codes (`VSDD-W0190` through `VSDD-L0050`).
- README has a "CHANGELOG discipline" section (~50 lines at lines 733-801).

CHANGELOG is the 15th first-class artifact class spanning ~150 lines of DESIGN content + 10 error codes + a dedicated hook — and the canonical methodology spec deployed to adopting projects omits it entirely. Additionally, the toolkit's own `vsdd-cli` repo has **no committed CHANGELOG.md** at any path; the toolkit dogfoods its own discipline (`README.md:799-801`) but the file doesn't exist yet, so `VSDD-E0240: changelog-deleted` cannot fire (no last-known-good baseline) and the dogfood claim is unverifiable.

**Why this matters:** adopting projects reading methodology.md as the canonical governing spec won't see CHANGELOG declared as a methodology discipline — the hook will fire on their CHANGELOG.md with no spec referent the operator can point to. The methodology floor and the toolkit's enforced floor are non-coincident.

**Routing:** Phase 4 → Phase 1a (methodology.md + DESIGN-METHODOLOGY revision). Two coordinated actions: (a) add a "CHANGELOG cooperation discipline" section to methodology.md's section list (~15-20 lines); (b) extend DESIGN-METHODOLOGY § Methodology spec section list table with the section + author the actual section in methodology.md. Composes with Finding 12 (toolkit dogfood gap — no CHANGELOG committed).

### Finding 5 — methodology.md omits MCP server tool surface (Dim 3: cross-document staleness; AIE Dim 2: cost-band cataloging) — Open

`finding_id: 1-f5` · `domain: technical-writer+ai-engineer` · `dim: 3` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: methodology.md, target_section: methodology-spec-section-list }`

`methodology.md` mentions MCP server only twice — both in closing cross-references (lines 410 + 412) pointing to DESIGN-METHODOLOGY + DESIGN-OBSERVABILITY for "MCP server tool surface" and "MCP server architecture." There is **no methodology-spec section** describing the MCP server's role in the methodology (the 4 tools agents leverage in every Claude Code session).

DESIGN-METHODOLOGY § Methodology spec section list does not include an MCP-server-tool-surface section. But:

- DESIGN-METHODOLOGY §§ "MCP server tool surface" (lines 453-481) declares 4 tools + cache strategy + acceptance criteria.
- DESIGN-OBSERVABILITY §§ "MCP server" (lines 416-462) declares per-tool cost bands (methodology.lookup 1-5k tokens; substrate-docs-search 5-20k tokens) + cache TTL strategy.
- `claude-code-cli.md` supplement § Software Engineer extensions calls out MCP integration as a substrate primitive.

The methodology's own spec doesn't name the MCP server as a load-bearing methodology surface. Per AIE Dim 2 (cost-band cataloging per operation): the cost bands declared in DESIGN-OBSERVABILITY don't trace to a methodology-spec section that operators reading the canonical spec see — so cost discipline is invisible at the level adopting projects consume.

**Why this matters:** the MCP server is one of the toolkit's 9 "center of gravity" deliverables (README:117). Methodology.md is supposed to capture every architectural decision in the DESIGN docs in at least one section (methodology.md:31, "The `check-methodology-semantics.py` hook mechanically validates these invariants"). MCP-server-tool-surface is an architectural decision; methodology.md has no section reflecting it. The check-methodology-semantics.py hook should fire — and if it doesn't, the hook is under-specified.

**Routing:** Phase 4 → Phase 1a (methodology.md + DESIGN-METHODOLOGY revision). Add an MCP-server-as-methodology-surface section to methodology.md (~15-20 lines) covering the 4 tools + cost bands + cache strategy at the spec altitude. Composes with Finding 6 (capture-source enum not enumerated in methodology.md).

### Finding 6 — capture-source enum referenced but not enumerated in methodology.md (Dim 3: cross-document staleness; AIE Dim 1: capture-source provenance) — Open

`finding_id: 1-f6` · `domain: ai-engineer` · `dim: 1` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: methodology.md, target_section: two-cooperating-audit-trail-layers }`

`methodology.md:297` references `capture_source` as a field on the `EventEnvelope` shape ("Both layers carry the same `EventEnvelope` shape (`agent_id`, `agent_seq`, `timestamp`, `signed_by`, `signature`, `capture_source`)") and the vocabulary.yaml registry has `capture-source` as a registered term — but the methodology spec **does not enumerate the 7 values** of the enum. DESIGN-METHODOLOGY:300 declares them; DESIGN-OBSERVABILITY § Capture-source provenance (lines 290-305) declares them; DESIGN-SCHEMA event-variant common envelope (lines 291-302) declares them. methodology.md does not.

Per AIE Dim 1 (capture-source provenance — "Every cost-relevant event carries the enum — but does the spec actually enumerate every cost-relevant event?"): the spec the toolkit deploys to adopting projects names the field but doesn't bind it to specific values. An operator reading methodology.md cannot know what `capture_source: vsdd-custom-event` means without reading the DESIGN docs. The 7-value enum is the canonical FinOps + observability discipline; it belongs in the canonical spec.

**Why this matters:** cost-discipline depends on every cost-relevant event carrying provenance. If the methodology spec doesn't enumerate the provenance values, the discipline isn't load-bearing at the spec level — it's an implementation detail. Per AIE failure mode "Token cost estimate from client-side SDK conflated with authoritative billing" — the capture-source enum's `sdk-result-message` vs `usage-api-reconciled` distinction IS the discipline; it must surface at spec altitude.

**Routing:** Phase 4 → Phase 1a (methodology.md § Two cooperating audit-trail layers extension). Enumerate the 7 capture-source values + their meaning in 1-2 sentences each (~10 lines). Composes with Finding 5 (MCP server cost bands also missing).

### Finding 7 — Threat model referenced but never authored (Dim 1: threat model specificity) — Open

`finding_id: 1-f7` · `domain: security` · `dim: 1` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: (new) THREAT-MODEL.md OR DESIGN-METHODOLOGY § Threat model section, target_section: new-section }`

The Security domain prompt's Dim 1 declares: "The project's threat model names attackers, motivations, capabilities, attack surfaces — specific to this project, not generic." Across all 6 canonical docs, the only mention of "threat model" is `DESIGN-VERIFICATION.md:638` — a single line inside the dependency-approval section ("Security notes — CVE history; license; threat model considerations"). There is **no project-specific threat model authored anywhere** in the toolkit spec.

Specific threats the toolkit must defend but doesn't name in a single threat-model artifact:

- **Schema injection** — partially addressed at DESIGN-VERIFICATION § Canonical-schema-path discipline (lines 54-62: "PR-submitted schema files ... are NOT consumed at validation time"). Defense named without the attack class explicitly modeled.
- **Bypass-marker circumvention** — partially addressed at DESIGN-VERIFICATION § Bypass-marker enforcement (lines 365-396: "Label-applier must differ from PR-author"). Defense named; threat class not modeled as part of a coherent model.
- **Supply-chain insertion via new dependencies** — partially addressed at DESIGN-VERIFICATION § Dependency approval (lines 619-674). Discipline declared; attacker-capability-model absent.
- **OTel collector forwarding to malicious endpoint** — partially addressed at DESIGN-OBSERVABILITY:146 ("External-backend operator-confirmation"). Defense named; attacker-capability-model absent.
- **Pre-built binary supply-chain** — partially addressed at DESIGN-VERIFICATION § Pre-built binaries (lines 799-815: cosign + SLSA + reproducible builds). Defense named; attacker-capability-model absent.
- **Methodology drift attack** — un-addressed: a malicious PR drifts the project's `methodology.md` relative to toolkit-canonical; `check-methodology-version-drift.py` fires `VSDD-W0200` (warning, not error). No model of what an attacker gains by methodology drift.
- **OTEL_LOG_RAW_API_BODIES enabling** — partially addressed at DESIGN-OBSERVABILITY:170 ("stays default-off"). Recommendation; no model of attacker who flips it.

Per Security failure-mode "Threat modeling as a checklist exercise — STRIDE/PASTA mechanically walked without project-specific threats named": the present state is the inverse — defenses ship without a model. Both fail the discipline differently.

**Why this matters:** the toolkit makes structural claims about defending the audit trail, credentials, and supply chain. Without a threat model declaring attackers, capabilities, and motivations, each defense is justified ad-hoc. New defenses lack a coherent referent for "does this match the model?"

**Routing:** Phase 4 → Phase 1a (operator-directive: author a THREAT-MODEL.md OR add a "Threat model" section to DESIGN-METHODOLOGY). Three options: (a) standalone THREAT-MODEL.md as the 6th canonical doc; (b) Threat model section within DESIGN-METHODOLOGY § Security disciplines; (c) Per-DESIGN-doc § Threat model sub-sections each defending the doc's surface. Recommend (a) — single authoritative model crossing all subsystems.

### Finding 8 — Credential-rotation procedure declared but the "operational runbook" doesn't exist (Dim 3: credential handling + Dim 7: forensic-trail integrity) — Open

`finding_id: 1-f8` · `domain: security` · `dim: 3` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: (new) runbooks/credential-rotation.md OR DESIGN-METHODOLOGY § Credential disciplines, target_section: new-section }`

`README.md:288` and `DESIGN-METHODOLOGY.md:336` both declare: "Compromised credential procedure: ... lives in operational runbook." The operational runbook is not authored — there is no `runbooks/` directory and no per-procedure document. Cross-document staleness compounds with Finding 3: the procedure references `AuthFailureObserved` which (per Finding 3) the schema set does not define.

`README.md:286` declares: "key rotation procedure documented (monthly cadence recommended; ad-hoc on compromise)." The procedure is not documented — only the recommendation that it should be.

**Why this matters:** credential-rotation is the highest-stakes Security operation in the toolkit's lifecycle. The methodology declares the procedure lives in an unauthored runbook + emits an undefined event. Per Security failure-mode "Credential redaction that runs after the credential value has already touched a log surface": the rotation flow includes "audit event log via Agent SDK OTel signals for cycle activity post-compromise + pre-revocation" — but the procedure for retrieving + auditing those signals isn't specified. An operator under compromise pressure cannot execute a procedure that exists only as a 2-line summary.

**Routing:** Phase 4 → Phase 1a (operator-directive: author runbooks/credential-rotation.md OR extend DESIGN-METHODOLOGY § Security disciplines with concrete procedure). The runbook content the spec implies: (a) revoke at Anthropic console / GitHub Secrets / etc.; (b) `vsdd observe metrics --since <pre-revocation-timestamp>` audit query (subcommand exists? — verify); (c) emit `OperatorDirectiveApplied{directive: credential-rotation}` event; (d) reissue + update `.vsdd/config.yaml` env-var-name reference; (e) `vsdd verify check --hook check-anonymization` regression-check across full repo history. Each step needs concrete commands.

### Finding 9 — Cluster-batching shape diverges between Phase 3 primer and dispatcher (Dim 4: cluster-batching shape + Dim 5: methodology-evolution coherence) — Open

`finding_id: 1-f9` · `domain: ai-engineer+vsdd-methodology` · `dim: 4` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: DESIGN-METHODOLOGY+Phase3primer, target_section: cluster-batching-shape }`

The Phase 3 primer (`.claude/commands/vsdd-phase-3.md:18-20`) and DESIGN-METHODOLOGY § Cluster-batching shape (`DESIGN-METHODOLOGY.md:382-392`) both declare:

- Implementation cluster — SE + QE + Performance Engineer
- Architecture cluster — SA + Platform Engineer + Data Engineer
- Communication cluster — **Security + TW + Accessibility + Privacy + Localization**
- Adversarial cluster — Red Team + DR + UX + **AI Engineer** + Solution Owner + VSDD Methodology + Sanity Check

The dispatcher prompt that spawned THIS review declared the Communication cluster as "**Technical Writer + AI Engineer + Security**." AI Engineer is in the **Adversarial** cluster per the canonical spec, not the Communication cluster. The dispatcher's composition for this round diverges from the methodology's declared shape.

Per Phase 3 primer's pre-cycle methodology check ("every Phase 3 cycle declares its shape before execution begins"): the dispatcher's cluster-shape declaration was implicit (no `PhaseCompositionDeclared` event emitted) and divergent. Per AIE Dim 4: "this very cycle is running cluster-batched; is the methodology coherent with practice?" — the answer surfaced by this finding is **no, it isn't coherent for this round**.

Multiple interpretations are valid:
- (a) Operator chose a smaller cluster set for spec-stage review (5 domains in Comm cluster overkill when only 3 dimensions actually drive); divergence-with-rationale.
- (b) AI Engineer logically fits Communication cluster's prose + ai-runtime-cost focus better than Adversarial cluster's structured-finding focus.
- (c) Adversarial cluster ballooned to 7 domains (Red Team + DR + UX + AIE + SO + VSDD-Methodology + Sanity Check) per the methodology spec — too many for one agent's context budget; moving AIE to Comm balances.

**Why this matters:** the cluster shape is declared load-bearing in 4 places (Phase 3 primer + DESIGN-METHODOLOGY + methodology.md cross-ref + cold-session budget table). When practice diverges from declaration without explicit `PhaseCompositionDeclared` + bypass-marker, the discipline isn't operational — it's aspirational. Methodology-evolution coherence (VSDD Methodology meta-domain dim 4) fires here.

**Routing:** Phase 4 → operator-directive. Three options: (a) update DESIGN-METHODOLOGY + Phase 3 primer to move AIE to Communication cluster (matches Adversarial cluster's 7→6 size reduction); (b) update DESIGN-METHODOLOGY to permit cluster-shape variation per round with a "PhaseCompositionDeclared" requirement enforced on the dispatcher; (c) note the divergence as a one-off + reassert the canonical shape for future rounds. Suggest (a) or (b); (a) is mechanically simpler.

### Finding 10 — `OTEL_LOG_RAW_API_BODIES` default-off discipline lacks mechanical enforcement (Dim 3: credential handling + Dim 5: input validation completeness) — Open

`finding_id: 1-f10` · `domain: security` · `dim: 3` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: DESIGN-OBSERVABILITY+DESIGN-VERIFICATION, target_section: redaction-processor+anonymization-hook }`

`DESIGN-OBSERVABILITY.md:170` declares: "`OTEL_LOG_RAW_API_BODIES` stays default-off — enabling it would route full API request bodies (including credential-shaped values in headers) through the OTel pipeline, undermining the redaction discipline. Operator can enable for debugging in private contexts but the methodology recommends keeping it off."

The discipline is a **recommendation**. There is no:
- Schema validator that rejects `.vsdd/config.yaml` declaring `OTEL_LOG_RAW_API_BODIES: 1`
- Hook that scans `.vsdd/env-vars` or shell-history for the env var
- Validation in `vsdd init` that warns if the operator has the env var set in their environment
- Pre-commit hook that detects raw-API-body shapes in `.vsdd/events.jsonl` (would catch the env var having been flipped at some point in history)

Per Security Dim 3 (credential handling discipline): "Schema validators reject credential-shaped fields structurally." The redaction processor (DESIGN-OBSERVABILITY:160-170) handles the value-level redaction IF the credential makes it into the pipeline. With `OTEL_LOG_RAW_API_BODIES=1` enabled, full request bodies stream through — the redaction processor runs but a smart attacker can structure prompts that defeat regex-based redaction (e.g., encoding the key in non-base64 or splitting across log entries).

**Why this matters:** the toolkit's structural credential exclusion is declared at the event-variant schema level (Schema validator rejects credential-shaped fields per DESIGN-SCHEMA:325-340). With raw-API-bodies enabled the SDK's OTel log events contain the request body as a string field that's not subject to credential-shape exclusion (the field name is body / content / prompt — not credential-shaped). Per the Security failure mode "Input validation that trusts the trust boundary — validator inside the boundary protecting against attacks from outside it": the redaction processor is inside the boundary; the env var enabling raw-bodies is outside it.

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION new hook + DESIGN-OBSERVABILITY enforcement upgrade). Three coordinated actions: (a) add a candidate error code `VSDD-W0XXX: otel-log-raw-api-bodies-enabled` that fires when the env var is set in `.vsdd/env-vars` or detected in the current process environment; (b) extend the redaction processor to scan log-event body fields (not just attribute names) for credential-shaped substrings; (c) document the structural attack surface in the threat model (Finding 7).

### Finding 11 — Multi-machine operator identity continuity deferred to v1+ but breaks Phase 6 attestation today (Dim 4: authentication + authorization + Dim 7: forensic-trail integrity) — Deferred (with note)

`finding_id: 1-f11` · `domain: security+ai-engineer` · `dim: 4` · `classification: deferred` · `source: regression-replay` (re-surfaces Security F5 from `review-log/2026-05-27-security.md`) · `routing: { target_phase: deferred-to-v1+, target_artifact: DESIGN-METHODOLOGY § Open decisions deferred, target_section: line-999 }`

DESIGN-METHODOLOGY:999 declares: "Multi-machine operator identity continuity ... Deferred to v1+ pending earned-by-recurrence evidence (Phase 5 round 1 Security F5; single-operator-single-machine projects do not hit this case in v1 evaluation cycles)." The deferral was Security-cluster-raised; the Communication cluster re-surfaces it because Phase 6 Exit Signal attestation depends on `signed_by` (SSH key fingerprint per DESIGN-SCHEMA Event-envelope) — and the toolkit's own development cycles use multiple machines.

The `signed_by` field on every event envelope is an SSH key fingerprint. The Exit Signal record's `attested_by` carries "operator identity (SSH key fingerprint OR github handle)" (DESIGN-SCHEMA:502-516). The DESIGN-METHODOLOGY:999 deferral notes the canonical case (single-operator-single-machine) but doesn't address the toolkit's own meta-case: the toolkit dogfoods on itself; the operator may author commits from laptop + desktop; the Exit Signal attestation when the toolkit hits v1.0 will need a consistent identity.

**Why this matters:** the deferral is reasonable for adopting projects but is sycophancy-compensable for the toolkit's own development. Per Security failure-mode "Input validation that trusts the trust boundary": the trust boundary at attestation-signing is the SSH key fingerprint set; if the methodology doesn't define which keys are valid for which operator-identity, the attestation has trust-boundary ambiguity by construction.

**Routing:** Deferred to v1+ per DESIGN-METHODOLOGY:999 — but with the added constraint that the toolkit's own v1.0 Exit Signal attestation cycle should retire the deferral OR explicitly declare the toolkit operator-identity-set (e.g., "operator identities for the toolkit's v1.0 attestation are SSH key fingerprints X + Y, both registered to magnificentlycursed"). The deferral should not silently pass into v1.0 release.

### Finding 12 — Toolkit CHANGELOG.md absent at spec-stage; "vsdd dogfoods its own discipline" claim is unverifiable (Dim 8: CHANGELOG cooperation + Dim 3: cross-document staleness) — Open

`finding_id: 1-f12` · `domain: technical-writer` · `dim: 8` · `classification: resolved-pending` · `source: domain-raised` · `routing: { target_phase: phase-1a, target_artifact: (new) CHANGELOG.md, target_section: bootstrap }`

`README.md:799-801` declares: "vsdd's own CHANGELOG / The `vsdd-cli` repo's own CHANGELOG.md follows this pattern verbatim. The toolkit dogfoods its own discipline." `ls /Users/.../vsdd-cli/CHANGELOG.md` returns "No such file or directory." The dogfood claim is current-state-false.

Per Finding 4 (methodology.md omits CHANGELOG discipline) + Finding 12 here: the toolkit spec promises CHANGELOG discipline + claims to dogfood + the file doesn't exist. The `check-changelog-discipline.py` hook's `VSDD-E0240: changelog-deleted` rule (file integrity per DESIGN-SCHEMA:621) requires a last-known-good snapshot at toolkit-init — the snapshot would be empty / non-existent at present, so the rule cannot fire and the toolkit's own initial CHANGELOG bootstrap path isn't validated.

**Why this matters:** Per TW failure-mode "Stale claim ('the system handles up to N concurrent requests') with no measurement — quantitative claim without evidence": "The toolkit dogfoods its own discipline" is a quantitative + falsifiable claim with no evidence at present. The fix is to author the CHANGELOG. Spec-stage is the right phase to do it — the methodology amendment + first-PR-discipline + first-dependency-approval entries that have already landed (per recent commits) are exactly the changelog-relevant material the toolkit is supposed to record.

**Routing:** Phase 4 → Phase 1a (operator-directive: bootstrap CHANGELOG.md). Use the Keep-a-Changelog template from README:775-794. Backfill entries for the visible commits (spec-stage authoring; Phase 5 round 1 routing; dependency-approval directive; methodology-version-pin discipline; cluster-batching reconciliation). Confirms the dogfood claim + provides the last-known-good snapshot the `VSDD-E0240` rule needs to operate.

## Summary

**12 findings — 1 Deferred (with v1.0 attestation-cycle constraint), 11 Open (Resolved-pending status).**

Classification universe used: resolved-pending (Open + routed) / deferred. No Dismissed, no Hallucinated, no Accepted-rationale. The classification universe per primer 3 is `resolved | deferred | dismissed | hallucinated | accepted`; this cluster's MVR-readiness assessment: **not at MVR** — 11 substantive Open findings.

**Per-domain breakdown:**
- TW (5 findings): F1 (first-use expansion in methodology.md) · F2 (vocabulary registry coverage gap) · F3 (AuthMethodChanged/Observed contradictions) · F4 (CHANGELOG discipline absent from methodology.md) · F12 (CHANGELOG.md not committed)
- AIE (3 findings): F5 (MCP server cost bands not in methodology.md; shared with TW) · F6 (capture-source enum not enumerated in methodology.md) · F9 (cluster-batching divergence; shared with VSDD Methodology meta-domain)
- Security (4 findings): F7 (threat model never authored) · F8 (credential-rotation runbook absent) · F10 (`OTEL_LOG_RAW_API_BODIES` mechanical enforcement gap) · F11 (multi-machine identity deferral with v1.0 attestation note)

**Cross-document staleness clusters:**
- methodology.md (the canonical deployed spec) is missing 4 substantial sections (CHANGELOG, MCP server, capture-source enum, abbreviation table) that DESIGN docs treat as load-bearing. The check-methodology-semantics.py hook (methodology.md:31) is under-specified — it should fire on these gaps + does not.
- README ↔ DESIGN-METHODOLOGY ↔ DESIGN-SCHEMA contradiction on `AuthMethodChanged` + `AuthFailureObserved` (Finding 3) — 3 contradictory states across 6 references.

**Mechanical-enforcement gaps:**
- `OTEL_LOG_RAW_API_BODIES` env-var discipline is recommendation-only (Finding 10).
- Operational runbook for credential rotation is named but unauthored (Finding 8).
- Threat model is referenced but unauthored (Finding 7).
- CHANGELOG.md is dogfooded but unauthored (Finding 12).

Per the cluster's MVR closure: the Communication-cluster lens surfaced 11 substantive findings on a spec set that has already passed Phase 5 round 1 review (SA + QE + Security) for 13 mostly-mechanical-fixable findings. Phase 3 Communication cluster's surface is qualitatively different: cross-doc coherence + load-bearing-section-completeness + threat-model-as-coherent-artifact. None of the 11 findings is mechanical drift; each requires a methodology-spec amendment + operator-directive routing.

## Coordination notes

**Architecture cluster overlap:** Findings 9 (cluster-batching divergence) + 11 (multi-machine identity) touch architecture-cluster territory (SA's lens on cluster-shape coherence + identity-discipline). Recommend the Architecture cluster confirm the canonical cluster-shape decision in their own review-log entry; this cluster routes F9 to operator-directive.

**Implementation cluster overlap:** Findings 4 + 12 (CHANGELOG absent + methodology.md doesn't reference CHANGELOG) touch the QE-PerfE-SE lens because CHANGELOG bootstrap requires authoring the file + backfilling entries — implementation work. Recommend Implementation cluster takes the CHANGELOG bootstrap as a single coordinated commit; this cluster routes F4 + F12 jointly.

**Adversarial cluster overlap:** Findings 7 (threat model) + 10 (OTEL_LOG_RAW_API_BODIES) are exactly the surface Red Team's adversarial-mindset lens hits hardest. The Communication cluster's Security lens surfaced the defense-without-model pattern; Red Team's lens will surface the attack-against-model pattern. Recommend the Adversarial cluster's Red Team domain re-read F7 + F10 as their first-finding seed.

**Cross-cluster routing burden:**
- 2 findings require operator-directive (F7 threat model authoring; F9 cluster-shape decision).
- 4 findings require methodology-spec amendments (F1, F2, F4, F5, F6 — methodology.md revisions to add abbreviations + vocabulary + CHANGELOG + MCP + capture-source enum).
- 2 findings require coordinated cross-doc revisions (F3 AuthMethod variant contradiction; F8 credential-rotation runbook).
- 1 finding requires new validator hook (F10 OTEL_LOG_RAW_API_BODIES).
- 1 finding requires file bootstrap (F12 CHANGELOG.md).
- 1 finding is deferred with v1.0-attestation-cycle constraint (F11 multi-machine identity).

This is a heavy round for Phase 4 routing. Recommend operator bundles per cluster:
- (a) methodology.md amendment commit: F1 + F2 + F4 + F5 + F6 (single canonical-spec revision covering 5 omissions).
- (b) AuthMethod variant resolution commit: F3 (cross-doc reconciliation).
- (c) Threat model + runbook commit: F7 + F8 (Security disciplines bundle).
- (d) `OTEL_LOG_RAW_API_BODIES` hook commit: F10 (new candidate code).
- (e) CHANGELOG bootstrap commit: F4 + F12 (composes with bundle a).
- (f) Cluster-shape coherence commit: F9 (DESIGN-METHODOLOGY + Phase 3 primer revision).
- (g) Defer F11 with v1.0 attestation-cycle note.

7 PR boundaries. Per operator memory "One PR at a time — no stacked PRs": bundle a (5 methodology.md amendments) is the single largest unit + naturally first; e (CHANGELOG bootstrap) composes into a's PR. Subsequent PRs sequenced after a's merge.

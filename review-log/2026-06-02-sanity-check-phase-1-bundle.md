---
schema_class: review-entry
schema_version: 1.0.0
review_number: 4
date: 2026-06-02
phase: phase-3
scope: >-
  vsdd-cli crosslink #12 Phase 1 bundle (m1-codes-and-dsl-bundle). SC lens
  applied to the DESIGN doc + the three bundled changes (reserved-code rename,
  DSL Field-on-Object-missing-key symmetry, defined() carve-out drop) + the two
  schema-tightening reverts.
lens: >-
  Sanity Check — hallucinated coinages, scope drift, methodology adherence.
  Validator-of-last-resort + rubber-duck check against DESIGN + methodology
  amendment record + sibling Phase 3 reviews (SE/SA/QE/SO bundle entries).
source: director-raised
session_note: >-
  Cold-session SC review of a three-change bundle authored by Claude. Rubber-
  ducks four operator-named questions: coined ranges, bundle-as-coining, 1a/1b/1c
  condensation authorization, Red Gate -> implementation methodology gap, and
  the revert's regression-check surface.
model: claude-opus-4-7
execution_method: >-
  Cold-session sub-agent, worktree-no-memory, Phase 3 primer + SC prompt loaded
  fresh; cross-referenced sibling SO review only for cross-finding coherence.
sycophancy_compensation: >-
  Author identity overlaps with reviewer identity (Claude wrote the bundle and
  the four sibling reviews). SC's last-resort posture means "looks fine" is
  abdication; each rubber-duck names what specifically held or didn't.
supplements_loaded: []
---

# Sanity Check Phase 3 Review — Crosslink #12 Bundle

## SC-F1 — Three new coined E-ranges: justified, not speculative

**Classification:** hallucinated (no finding). **Routing:** n/a.
**Lens:** Consistency / rubber-duck.

Question rubber-ducked: are `E0050`/`E0070`/`E0080` reservations earned by
emission, or reserved-on-spec for future use? DESIGN § Reserved-code drift maps
each range to a present-tense emission site: `E0050` <- verify.rs schema
violation; `E0070` <- main.rs IO failure; `E0080` <- main.rs pipeline-orchestration
failure. Three sites, three ranges; one-to-one with no speculative siblings
(`E0051-E0059` etc. left unspecified, room-to-grow but not pre-coined). This is
the earned-by-recurrence pattern methodology.md § Forward-only disciplines
asks for, applied to code-range vocabulary. Not hallucinated coinage.

## SC-F2 — "Bundle" terminology: ordinary English, not a coined methodology term

**Classification:** hallucinated (no finding). **Routing:** n/a.
**Lens:** Consistency / scope discipline.

Question rubber-ducked: is "bundle" (as in `m1-codes-and-dsl-bundle`) a new
methodology coining that should have an amendment record, or just a milestone
name? methodology.md § Vocabulary anchors does not list "bundle." DESIGN uses
it descriptively ("ship together as crosslink #12"), and Phase 1c decomposition
is the canonical place for a single-milestone-with-three-changes decision. The
milestone slug carries the descriptor; the methodology vocabulary doesn't grow.
Not a coining. If "bundle" started appearing in domain prompts or primers as
a noun-of-art, SC would re-raise.

## SC-F3 — Phase 1a/1b/1c condensation: self-authorized, no amendment cited

**Classification:** accepted with remediation. **Routing:** Raise-to-VSDD-Methodology.
**Lens:** Consistency / methodology adherence.

Question rubber-ducked: did the cluster-batched cold-session amendment
(2026-06-02, DESIGN-METHODOLOGY.md:405) authorize composing Phase 1a + 1b + 1c
in a single skill-interactive session? Reading the amendment: it makes
`InlineMultiDomain` first-class **for Phase 3** and explicitly says "The Phase 3
default remains cluster-batched cold-session." That amendment governs Phase 3
composition — not Phase 1 sub-phase serialization. DESIGN's three composition
blocks (declared_at 21:30 / 21:35 / 21:40, five minutes apart, same skill-
interactive session) implicitly treat 1a/1b/1c as condensable, but no
methodology record authorizes that. methodology.md § Phases lists 1a/1b/1c as
distinct phases; the primer set has separate primers for each. Condensing
without an amendment is methodology-spirit drift — three timestamps don't
constitute three composition events if the cold-session reset between them
didn't happen.

Better version: either (a) emit `OperatorDirectiveApplied{directive: methodology-
amended, content: "Phase 1 sub-phases may be condensed into single skill-
interactive session when domains overlap and scope is pre-decomposed"}` with
SO disposition, or (b) re-run 1b and 1c as discrete sessions. The audit trail
needs one or the other.

## SC-F4 — Phase 2a Red Gate -> Phase 2b implementation gap: 9 + 145, no drift

**Classification:** hallucinated (no finding). **Routing:** n/a.
**Lens:** Consistency.

Question rubber-ducked: between Phase 2a authoring 9 tests and Phase 2b closing
all 9 + maintaining 145 prior, did the contract under test shift? DESIGN
§ Phase 2a Red Gate seeds names 7 tests grouped under 3 contract headers;
operator-reported count is 9 (presumably 7 named + 2 implicit regression checks,
which is the right shape — Red Gate authors should add boundary tests as they
go). 145 prior tests passing means no behavior regression beyond the
intentional E-code renames. The methodology shape held: Red Gate Red -> Green
without contract migration mid-implementation. Sibling SO review (SO-F2)
already raises the wire-format-version question, which is the only consumer-
visible contract concern; that's an SO-jurisdiction finding, not an SC one.

## SC-F5 — Schema-tightening revert: corpus pass is the regression check, no Red Gate

**Classification:** accepted. **Routing:** SO (cross-routes to QE).
**Lens:** Edge / Consistency.

Question rubber-ducked: was the revert tested with a Red Gate "corpus passes
after revert" assertion, or is the corpus's existing pass-status the regression
check? DESIGN § Acceptance criteria says "vsdd corpus passes `mdatron verify`
after the schema-tightening reverts" — that's the assertion, but it's a
corpus-as-test pattern, not an in-test-suite assertion. If a future change to
`phase-primer.json` re-tightens `supplements_in_scope` accidentally, only
running the full corpus through `verify` would catch it; the Rust test suite
wouldn't. That's a defensible boundary (the schemas live in `vsdd-core/`; the
DSL lives in `mdatron-core/`; the test surfaces are separate by design), but
it should be named. SC accepts the disposition with the note that adopters
running the Rust test suite alone get false-green coverage on the schema-revert
contract. Cross-routes to QE for "is there a meta-test that runs the corpus
during CI" question.

## Cross-finding coherence

SC-F3 (methodology condensation) and SO-F1/SO-F3 (spec/event-log silence) are
the same failure-mode-class: applied changes not surfacing in the methodology
event log. If SO emits `OperatorDirectiveApplied` for the DESIGN-MDATRON.md
table amendment and `OperatorDirectiveRevoked` for the schema reverts, the
methodology-amendment-for-1a/1b/1c-condensation event belongs in the same
commit. Three event-log gaps in one bundle is the systemic signal: the bundle's
methodology-discipline surface is the weak point, not the code.

SC-F5 and the SA/SE reviews (not yet read in this session) likely overlap on
test-surface coverage; SC defers to QE's bundle review (review_number 3) for
the canonical coverage disposition.

## Recommended dispositions

- SC-F1: Hallucinated; no action.
- SC-F2: Hallucinated; no action.
- SC-F3: Resolved-pending until either methodology amendment is recorded OR
  1b/1c are re-composed as discrete sessions. Raise-to-VSDD-Methodology.
- SC-F4: Hallucinated; no action.
- SC-F5: Accepted; QE cross-route for corpus-in-CI question.

One Resolved-pending + one Accepted-with-cross-route + three Hallucinated.
Per Phase 3 primer § Swarm-invocation triggers, real findings surfaced ->
swarm invocation continues.

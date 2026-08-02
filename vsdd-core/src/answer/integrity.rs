//! The pure snapshot-scoped integrity checks — the snapshot-schema
//! audit's members whose inputs are materialized fields. The
//! degraded-kind check is the derivation's own enforced property; the
//! three shell-side checks (refs, installed-artifact integrity, unsigned events) join the
//! report in `integrity_shell`.
//!
//! The emitted kinds mirror the audit block's check ids in
//! `templates/registry/snapshot-schema.md` — a declared mirror like the
//! loader's recovery constant, pinned by the convergence corpus.

use crate::snapshot::{AcquisitionOutcome, Snapshot};
use crate::state::State;

/// Run the snapshot-scoped checks; returns finding kinds, deduplicated,
/// order stable. Pure; the derivation calls it only when the snapshot
/// was acquired.
pub fn snapshot_integrity(state: &State, snapshot: &Snapshot) -> Vec<String> {
    let mut kinds: Vec<String> = Vec::new();
    let push = |k: &str, kinds: &mut Vec<String>| {
        if !kinds.iter().any(|have| have == k) {
            kinds.push(k.to_string());
        }
    };

    // round-parity: a manifest's declared count reconciles with its
    // round's tracked children.
    for manifest in &snapshot.round_manifests {
        let tracked = snapshot
            .round_children
            .iter()
            .find(|c| c.handle == manifest.handle)
            .map(|c| c.child_count)
            .unwrap_or(0);
        if tracked != manifest.declared_finding_count {
            push("round-parity", &mut kinds);
        }
    }

    // unresolvable handles cited in result comments.
    if snapshot.comment_handles.iter().any(|h| !h.resolves) {
        push("unresolvable-handles-in-result-comments", &mut kinds);
    }

    // an open finding without an owner or a validator (the contract's
    // lifecycle falsifier). Field-readiness gate (vsdd-cli #820, REQ-7): this
    // check reads the lifecycle-role fields, which the Slice-1 spine-only join
    // does not acquire — it runs only when those fields were acquired, so a live
    // spine-only snapshot does not mis-fire it (the convergence fixtures declare
    // full acquisition and run it as before).
    if snapshot.finding_fields_acquired.lifecycle_roles
        && snapshot
            .findings
            .iter()
            .any(|f| f.status == "open" && (f.owner.is_none() || f.validator.is_none()))
    {
        push("findings-missing-owner-or-validator", &mut kinds);
    }

    // a closed finding with neither an evidence reference nor a recorded
    // disposition (disposition closures close lawfully without evidence).
    if snapshot.finding_fields_acquired.evidence
        && snapshot
            .findings
            .iter()
            .any(|f| f.status == "closed" && !f.evidence_reference_present && f.disposition.is_none())
    {
        push("closed-findings-missing-evidence", &mut kinds);
    }

    // the unrouted-findings query (contract: Status — the process-integrity
    // query the re-sequence-enforcement-spine amendment placed at Layer 2;
    // vsdd-cli #810/#811): a finding closed by FIX — closed, and not an
    // exempt disposition closure — that carries no filed routing. The
    // forward-only universe (REQ-5) excludes findings closed before the
    // routing amendment's ratification boundary, carried on the record's
    // `closed_before_ratification` datum. An integrity finding that never
    // degrades the answer, exactly like its siblings.
    if !unrouted_findings(snapshot).is_empty() {
        push("unrouted-findings", &mut kinds);
    }

    // the phase pointer against milestone state: active milestones exist
    // and the pointer's milestone is not among them.
    let actives: Vec<&str> = snapshot
        .milestones
        .iter()
        .filter(|m| m.is_active)
        .map(|m| m.name.as_str())
        .collect();
    if !actives.is_empty() && !actives.contains(&state.open_findings_pointer.milestone.as_str()) {
        push("phase-pointer-against-milestone-state", &mut kinds);
    }

    kinds
}

/// The unrouted-findings query as a standalone predicate (vsdd-cli #820): the
/// handles of findings closed by FIX — closed, and not an exempt disposition
/// closure — that sit in the forward-only universe (REQ-5, not closed before the
/// ratification boundary) and carry no filed routing. Empty when the spine field
/// group was not acquired (the field-readiness gate). `snapshot_integrity` emits
/// the `unrouted-findings` kind iff this is non-empty, and the `vsdd gate`
/// guardrail blocks on it — one predicate, so the report and the block never
/// diverge.
pub fn unrouted_findings(snapshot: &Snapshot) -> Vec<String> {
    if !snapshot.finding_fields_acquired.spine {
        return Vec::new();
    }
    snapshot
        .findings
        .iter()
        .filter(|f| {
            f.status == "closed"
                && f.disposition.is_none()
                && !f.closed_before_ratification
                && !f.routing_present
        })
        .map(|f| f.handle.clone())
        .collect()
}

/// The routing-before-fix guardrail's verdict over an acquired snapshot
/// (vsdd-cli #820). Fail-closed: an unverifiable acquisition blocks, never
/// passes vacuously.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateVerdict {
    /// No unrouted findings — the gate passes.
    Pass,
    /// Findings closed by fix without routing — the handles that need routing.
    Block(Vec<String>),
    /// The acquisition could not be verified (tracker absent or unusable); the
    /// gate blocks rather than pass on an unchecked run.
    Unverifiable(String),
}

/// Compute the guardrail verdict (vsdd-cli #820): Acquired → Pass or Block on the
/// unrouted-findings query; Absent or Unusable → Unverifiable (fail-closed).
///
/// The third fail-closed arm (vsdd-cli #818 Fix 2, guardrail REQ-4): Acquired
/// with the spine field group UNACQUIRED — the failed finding leg's record —
/// is Unverifiable with its own distinct message. The tracker was present but
/// routing could not be read, so passing on the empty findings would be the
/// vacuous pass the requirement forbids. The arm keys on the SAME field
/// `unrouted_findings` gates its readiness on, so the query's dormancy and
/// the gate's fail-closed arm can never diverge.
pub fn gate_verdict(snapshot: &Snapshot) -> GateVerdict {
    match snapshot.acquisition_outcome {
        AcquisitionOutcome::Acquired if !snapshot.finding_fields_acquired.spine => {
            GateVerdict::Unverifiable(
                "the finding query failed with the tracker present — routing cannot be verified"
                    .to_string(),
            )
        }
        AcquisitionOutcome::Acquired => {
            let unrouted = unrouted_findings(snapshot);
            if unrouted.is_empty() {
                GateVerdict::Pass
            } else {
                GateVerdict::Block(unrouted)
            }
        }
        AcquisitionOutcome::Absent => GateVerdict::Unverifiable(
            "crosslink tracker absent — routing cannot be verified".to_string(),
        ),
        AcquisitionOutcome::Unusable => GateVerdict::Unverifiable(
            "crosslink tracker unusable — routing cannot be verified".to_string(),
        ),
    }
}

/// The finding-reading checks that did NOT run over this snapshot, each with
/// why (vsdd-cli #818 Fix 2 — the dormant-vs-clean distinction): the three
/// finding-reading members of the snapshot-schema audit block gate on
/// `finding_fields_acquired`, and a gated-off check must SURFACE as
/// dormant or could-not-check rather than let its silence read as
/// checked-clean. The reason rule: an unacquired spine under an Acquired
/// tracker is only ever the failed finding leg (the live join never defers
/// the spine by scope), and a failed leg acquired no finding record at all —
/// so every finding-reading check is could-not-check; a deferred group with
/// the spine acquired is dormant by scope (the Slice-5 deferral). Pure;
/// empty exactly when every finding-reading check ran.
pub fn finding_checks_not_run(snapshot: &Snapshot) -> Vec<super::CheckNotRun> {
    use super::{CheckNotRun, CheckNotRunReason};
    let acquired = &snapshot.finding_fields_acquired;
    let entry = |check: &str, reason: CheckNotRunReason| CheckNotRun {
        check: check.to_string(),
        reason,
    };
    if !acquired.spine {
        return [
            "unrouted-findings",
            "findings-missing-owner-or-validator",
            "closed-findings-missing-evidence",
        ]
        .into_iter()
        .map(|check| entry(check, CheckNotRunReason::CouldNotCheck))
        .collect();
    }
    let mut out = Vec::new();
    if !acquired.lifecycle_roles {
        out.push(entry(
            "findings-missing-owner-or-validator",
            CheckNotRunReason::Dormant,
        ));
    }
    if !acquired.evidence {
        out.push(entry(
            "closed-findings-missing-evidence",
            CheckNotRunReason::Dormant,
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        finding_checks_not_run, gate_verdict, snapshot_integrity, unrouted_findings, GateVerdict,
    };
    use crate::answer::CheckNotRunReason;
    use crate::snapshot::{AcquisitionOutcome, FindingFieldsAcquired, FindingRecord, Snapshot};
    use crate::state::schema::{ActiveComposition, CompositionMode, OpenFindingsPointer, State};

    fn minimal_state() -> State {
        State {
            schema_version: "0.1.0".to_string(),
            current_phase: Some("phase-4".to_string()),
            current_layer: Some(2),
            open_findings_pointer: OpenFindingsPointer {
                milestone: "layer 2".to_string(),
            },
            last_gate_result: None,
            active_composition: ActiveComposition {
                scope: "test".to_string(),
                domains: Vec::new(),
                mode: CompositionMode::SkillInteractive,
                config_inputs_hash: "sha256:test".to_string(),
            },
            published: None,
        }
    }

    fn unowned_open_finding() -> FindingRecord {
        FindingRecord {
            handle: "the unowned open finding".to_string(),
            status: "open".to_string(),
            owner: None,
            validator: None,
            evidence_reference_present: false,
            disposition: None,
            routing_present: false,
            closed_before_ratification: false,
        }
    }

    fn snapshot_with(findings: Vec<FindingRecord>, acquired: FindingFieldsAcquired) -> Snapshot {
        Snapshot {
            acquisition_outcome: AcquisitionOutcome::Acquired,
            // Empty milestones so the phase-pointer check cannot interfere
            // (its guard needs at least one active milestone).
            milestones: Vec::new(),
            findings,
            round_manifests: Vec::new(),
            round_children: Vec::new(),
            comment_handles: Vec::new(),
            display_repo_name: "test".to_string(),
            display_session: "test".to_string(),
            display_work_item: "test".to_string(),
            display_active_milestone: "test".to_string(),
            finding_fields_acquired: acquired,
            finding_acquisition_note: None,
        }
    }

    #[test]
    fn spine_only_acquisition_does_not_mis_fire_the_lifecycle_role_check() {
        // Field-readiness gate (vsdd-cli #820, REQ-7): a live spine-only join
        // leaves owner/validator unacquired, so the check must stay dormant —
        // an unowned open finding is NOT flagged when its inputs were not read.
        let snap = snapshot_with(vec![unowned_open_finding()], FindingFieldsAcquired::SPINE_ONLY);
        let kinds = snapshot_integrity(&minimal_state(), &snap);
        assert!(
            !kinds.iter().any(|k| k == "findings-missing-owner-or-validator"),
            "lifecycle-role check stays dormant when its fields were not acquired"
        );
    }

    #[test]
    fn full_acquisition_fires_the_lifecycle_role_check() {
        // The gate is a gate, not an always-off: the same unowned finding IS
        // flagged when the lifecycle-role group was acquired (the fixtures'
        // path, here pinned as the in-code contrast).
        let snap = snapshot_with(
            vec![unowned_open_finding()],
            FindingFieldsAcquired::default(),
        );
        let kinds = snapshot_integrity(&minimal_state(), &snap);
        assert!(
            kinds.iter().any(|k| k == "findings-missing-owner-or-validator"),
            "lifecycle-role check fires when its fields were acquired"
        );
    }

    // ── The routing-before-fix guardrail (Slice 1, vsdd-cli #820) ────────────

    fn closed_unrouted_finding(handle: &str) -> FindingRecord {
        FindingRecord {
            handle: handle.to_string(),
            status: "closed".to_string(),
            owner: None,
            validator: None,
            evidence_reference_present: false,
            disposition: None,
            routing_present: false,
            closed_before_ratification: false,
        }
    }

    #[test]
    fn unrouted_findings_lists_closed_fix_closes_without_routing() {
        let snap = snapshot_with(
            vec![closed_unrouted_finding("#77")],
            FindingFieldsAcquired::SPINE_ONLY,
        );
        assert_eq!(unrouted_findings(&snap), vec!["#77".to_string()]);
        // AC-5: snapshot_integrity emits the kind iff the query is non-empty —
        // the report and the gate share one predicate, so they never diverge.
        assert!(snapshot_integrity(&minimal_state(), &snap)
            .iter()
            .any(|k| k == "unrouted-findings"));
    }

    #[test]
    fn unrouted_findings_excludes_routed_disposition_and_out_of_universe() {
        let mut routed = closed_unrouted_finding("#1");
        routed.routing_present = true;
        let mut disposed = closed_unrouted_finding("#2");
        disposed.disposition = Some("dismissed".to_string());
        let mut pre_boundary = closed_unrouted_finding("#3");
        pre_boundary.closed_before_ratification = true;
        let snap = snapshot_with(
            vec![routed, disposed, pre_boundary],
            FindingFieldsAcquired::SPINE_ONLY,
        );
        assert!(unrouted_findings(&snap).is_empty());
        assert!(!snapshot_integrity(&minimal_state(), &snap)
            .iter()
            .any(|k| k == "unrouted-findings"));
    }

    #[test]
    fn gate_blocks_unrouted_passes_clean_and_fails_closed_on_degraded() {
        let unrouted = snapshot_with(
            vec![closed_unrouted_finding("#77")],
            FindingFieldsAcquired::SPINE_ONLY,
        );
        assert_eq!(
            gate_verdict(&unrouted),
            GateVerdict::Block(vec!["#77".to_string()]),
            "an unrouted fix-close blocks, naming the handle"
        );

        let clean = snapshot_with(Vec::new(), FindingFieldsAcquired::SPINE_ONLY);
        assert_eq!(gate_verdict(&clean), GateVerdict::Pass);

        // Fail-closed: an unverifiable acquisition blocks, never passes.
        let mut absent = snapshot_with(Vec::new(), FindingFieldsAcquired::SPINE_ONLY);
        absent.acquisition_outcome = AcquisitionOutcome::Absent;
        assert!(matches!(gate_verdict(&absent), GateVerdict::Unverifiable(_)));
        let mut unusable = snapshot_with(Vec::new(), FindingFieldsAcquired::SPINE_ONLY);
        unusable.acquisition_outcome = AcquisitionOutcome::Unusable;
        assert!(matches!(
            gate_verdict(&unusable),
            GateVerdict::Unverifiable(_)
        ));
    }

    // ── The failed-finding-leg arm (vsdd-cli #818 Fix 2, guardrail REQ-4) ────

    #[test]
    fn an_acquired_snapshot_with_the_spine_unacquired_fails_the_gate_closed() {
        // The third arm: Acquired with the spine group unacquired — the failed
        // finding leg's record — is Unverifiable with its own distinct
        // message, never a vacuous Pass on the empty findings.
        let failed = snapshot_with(Vec::new(), FindingFieldsAcquired::NONE);
        match gate_verdict(&failed) {
            GateVerdict::Unverifiable(reason) => assert!(
                reason.contains("finding query failed"),
                "the message names the failed leg, distinct from the \
                 absent/unusable wordings: {reason}"
            ),
            other => panic!("a failed-leg record must fail closed, got {other:?}"),
        }
        // The contrast pin: the SAME empty findings with the spine acquired is
        // the genuinely clean tracker — Pass. The arm keys on the record of
        // what was read, not on the emptiness.
        let clean = snapshot_with(Vec::new(), FindingFieldsAcquired::SPINE_ONLY);
        assert_eq!(gate_verdict(&clean), GateVerdict::Pass);
    }

    #[test]
    fn checks_not_run_distinguishes_clean_dormant_and_could_not_check() {
        // Full acquisition: every finding-reading check ran — empty, so
        // checked-clean stays a real claim (the mis-map negative).
        let full = snapshot_with(Vec::new(), FindingFieldsAcquired::default());
        assert!(finding_checks_not_run(&full).is_empty());

        // Spine-only: the two deferred-group checks are DORMANT by scope —
        // named, not silent, and not confused with an error.
        let spine_only = snapshot_with(Vec::new(), FindingFieldsAcquired::SPINE_ONLY);
        let dormant = finding_checks_not_run(&spine_only);
        let named: Vec<&str> = dormant.iter().map(|c| c.check.as_str()).collect();
        assert_eq!(
            named,
            vec![
                "findings-missing-owner-or-validator",
                "closed-findings-missing-evidence"
            ]
        );
        assert!(
            dormant.iter().all(|c| c.reason == CheckNotRunReason::Dormant),
            "deferred-by-scope groups read dormant, never could-not-check"
        );

        // The failed leg: no finding record was acquired at all, so all three
        // finding-reading checks are COULD-NOT-CHECK — distinguishable from
        // dormant-by-scope on the reason member.
        let failed = snapshot_with(Vec::new(), FindingFieldsAcquired::NONE);
        let unavailable = finding_checks_not_run(&failed);
        assert_eq!(unavailable.len(), 3);
        assert!(
            unavailable
                .iter()
                .all(|c| c.reason == CheckNotRunReason::CouldNotCheck),
            "a failed leg reads could-not-check on every finding-reading check"
        );
        assert!(
            unavailable.iter().any(|c| c.check == "unrouted-findings"),
            "the gate's own query is named among the unavailable checks"
        );
    }
}

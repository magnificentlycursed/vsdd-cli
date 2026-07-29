//! The pure snapshot-scoped integrity checks — the snapshot-schema
//! audit's members whose inputs are materialized fields. The
//! degraded-kind check is the derivation's own enforced property; the
//! three shell-side checks (refs, substrate, unsigned events) join the
//! report in `integrity_shell`.
//!
//! The emitted kinds mirror the audit block's check ids in
//! `templates/registry/snapshot-schema.md` — a declared mirror like the
//! loader's recovery constant, pinned by the convergence corpus.

use crate::snapshot::Snapshot;
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
    if snapshot.finding_fields_acquired.spine
        && snapshot.findings.iter().any(|f| {
            f.status == "closed"
                && f.disposition.is_none()
                && !f.closed_before_ratification
                && !f.routing_present
        })
    {
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

#[cfg(test)]
mod tests {
    use super::snapshot_integrity;
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
}

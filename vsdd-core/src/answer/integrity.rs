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
    // lifecycle falsifier).
    if snapshot
        .findings
        .iter()
        .any(|f| f.status == "open" && (f.owner.is_none() || f.validator.is_none()))
    {
        push("findings-missing-owner-or-validator", &mut kinds);
    }

    // a closed finding with neither an evidence reference nor a recorded
    // disposition (disposition closures close lawfully without evidence).
    if snapshot
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
    if snapshot.findings.iter().any(|f| {
        f.status == "closed"
            && f.disposition.is_none()
            && !f.closed_before_ratification
            && !f.routing_present
    }) {
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

//! The deviations gate leg over `.vsdd/registry/deviation-registry.yaml`
//! (build-plan Phase 1, the deviations gate leg; ratified remediation design
//! `.design/decomposition-topology-remediation.md` REQ-4/REQ-6, fixture
//! family AC-5).
//!
//! Shape: pure over parsed data — `deviations_verdict` takes the parsed
//! registry, a caller-supplied `today` (no clock in vsdd-core), the gate
//! mode, and an injectable issue-state oracle, and returns a verdict plus
//! the warn-grade surface (a warning list). `load_deviation_registry` is
//! the I/O boundary; `deviations_gate` composes the two, mapping every
//! load failure to `Unverifiable` — deleting the registry must never pass.
//!
//! The gate compares only in-entry dates (REQ-4: re-arm rewrites the
//! expiry in the entry; the gate never parses tracker prose). The local
//! gate treats `disposition_ref` as an unverified claim — ref-presence
//! plus in-entry date recency, warn grade; disposition verification over
//! server-synced state is the CI leg's #815-bound future.

use std::path::Path;

use serde::Deserialize;

use super::integrity::GateVerdict;
use crate::bounded_read::{read_bounded, MAX_ARTIFACT_BYTES};

/// The 30-day default expiry window, computed from `entry_date` (REQ-4).
const DEFAULT_EXPIRY_DAYS: i64 = 30;

/// The parsed deviation registry (remediation design REQ-4: the file
/// carries `schema_version` plus the entry list).
///
/// The self-governance instance additionally carries a founding-population
/// record (`initial_population_sweep`); the gate reads entries only, so
/// the top level tolerates unknown keys while entries are strict.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DeviationRegistry {
    /// The registry schema version.
    pub schema_version: u32,
    /// The deviation entries, `standing` and `resolved` alike; the gate
    /// enumerates `standing` entries (and premature resolutions treated
    /// as still standing).
    pub entries: Vec<DeviationEntry>,
}

/// One registry entry (REQ-4's field roster).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeviationEntry {
    /// Plain-name identifier (no coined letter-clusters).
    pub id: String,
    /// What rides off-affordance or persists as a fallback.
    pub deviation: String,
    /// The act-to-affordance map `act` key departed from; null with a
    /// stated class for non-affordance deviations.
    pub deviates_from: Option<String>,
    /// The stated class when `deviates_from` is null (e.g. toolchain-pin).
    #[serde(default)]
    pub deviation_class: Option<String>,
    /// Why the deviation stands.
    pub stated_reason: String,
    /// Exactly one machine-decidable retest predicate; absent on resolved
    /// entries.
    #[serde(default)]
    pub retest_trigger: Option<RetestTrigger>,
    /// Optional prose context for the trigger (held context, not predicate).
    #[serde(default)]
    pub trigger_context: Option<String>,
    /// Entry date (ISO date); the 30-day default expiry computes from it.
    pub entry_date: String,
    /// Expiry date; re-arm rewrites it in the entry, so the gate compares
    /// only in-entry dates. Absent on resolved entries.
    #[serde(default)]
    pub expiry: Option<String>,
    /// The tracker issue owning the deviation.
    pub owning_issue: String,
    /// `standing` or `resolved` (terminal; regression requires a new entry).
    pub status: DeviationStatus,
    /// Machine shape of the Solution Owner decision: issue plus comment
    /// timestamp. The local gate treats it as an unverified claim
    /// (ref-presence plus in-entry date recency, warn grade); the CI leg
    /// verifies it over server-synced state (a #815-bound future).
    #[serde(default)]
    pub disposition_ref: Option<DispositionRef>,
}

/// Entry lifecycle status (REQ-4: `standing` → `resolved`, or `standing` →
/// re-armed `standing` with a new expiry; no parked third state).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviationStatus {
    /// The deviation stands; the gate enumerates and expiry-checks it.
    Standing,
    /// Resolved under a Solution Owner disposition; terminal. A resolved
    /// entry WITHOUT its own `disposition_ref` is a premature resolution,
    /// treated by the gate as still standing.
    Resolved,
}

/// The retest trigger: exactly one machine-decidable predicate, typed by
/// the schema-fixed grammar (REQ-4).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetestTrigger {
    /// The predicate's grammar class.
    #[serde(rename = "type")]
    pub trigger_type: TriggerType,
    /// The predicate text; a fully-machine boolean compound within one
    /// class counts as one predicate.
    pub predicate: String,
}

/// The schema-fixed trigger grammar (REQ-4): `date | issue-state |
/// version-compare | artifact-presence`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TriggerType {
    /// Fires when `today` reaches the predicate date (in-entry comparison).
    Date,
    /// Fires on an upstream issue reaching the predicated state; decidable
    /// only through an issue-state oracle.
    IssueState,
    /// Fires on a version comparison against a named artifact. No
    /// evaluator in this build: undecidable (local warn-pass, CI
    /// inconclusive per resolved Q3).
    VersionCompare,
    /// Fires on a grep-decidable artifact-presence predicate. No evaluator
    /// in this build: undecidable (local warn-pass, CI inconclusive per
    /// resolved Q3).
    ArtifactPresence,
}

/// Machine shape of a Solution Owner disposition reference (REQ-4).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DispositionRef {
    /// The tracker issue carrying the decision comment.
    pub issue: String,
    /// The decision comment's timestamp (in-entry date material for the
    /// recency comparison).
    pub comment_timestamp: String,
}

/// The gate's strictness mode (REQ-6's mode seam): the difference is
/// confined to undecidable-trigger handling and (future) disposition
/// verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateMode {
    /// The developer surface: undecidable triggers pass with a recorded
    /// warning; `disposition_ref` is an unverified claim (warn grade).
    Local,
    /// The enforcement surface: undecidable is inconclusive — exit-2 class
    /// `Unverifiable`, never a silent pass; dispositions verify over
    /// server-synced state (#815-bound).
    Ci,
}

/// The state an issue-state oracle reports for an upstream issue reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueState {
    /// The referenced issue is open.
    Open,
    /// The referenced issue is closed.
    Closed,
}

/// The deviations check's result: a three-valued verdict (the existing
/// 0/1/2 exit-class currency) plus the warn-grade surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviationsOutcome {
    /// Pass / Block(offending entry ids with reasons) / Unverifiable(reason).
    pub verdict: GateVerdict,
    /// Recorded warnings (undecidable triggers on the local gate, abusive
    /// overrides carrying the applied default, unverified-claim notes).
    pub warnings: Vec<String>,
}

/// Registry load/shape failure — every variant maps to `Unverifiable`
/// (fail-closed: an absent, malformed, or shape-invalid registry must
/// never pass the gate).
#[derive(Debug, thiserror::Error)]
pub enum DeviationRegistryError {
    /// The registry file is absent at the expected path; restore it (or
    /// re-run `vsdd init` to redeploy the scaffold) — deletion never passes.
    #[error("deviation registry absent at {path} — deleting the registry never passes the gate")]
    Absent {
        /// The path probed.
        path: String,
    },
    /// The file exists but could not be read.
    #[error("deviation registry unreadable at {path}: {detail}")]
    Unreadable {
        /// The path probed.
        path: String,
        /// What the read failed on.
        detail: String,
    },
    /// The file read but is not valid YAML or misses the registry shape
    /// (missing `schema_version`, an entry missing a required field, a
    /// date field or machine predicate that does not parse).
    #[error("deviation registry shape-invalid at {path}: {detail}")]
    ShapeInvalid {
        /// The path probed.
        path: String,
        /// What the shape check failed on.
        detail: String,
    },
}

// ── Pure date arithmetic (proleptic Gregorian; no clock, no dependency) ──────

/// Days-from-civil (Howard Hinnant's algorithm): `(y, m, d)` → days since
/// 1970-01-01.
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = y - i64::from(m <= 2);
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (i64::from(m) + 9) % 12;
    let doy = (153 * mp + 2) / 5 + i64::from(d) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Civil-from-days: days since 1970-01-01 → `(y, m, d)`.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (y + i64::from(m <= 2), m, d)
}

/// Parse a strict `YYYY-MM-DD` ISO date into a day number (days since
/// 1970-01-01), or `None` when the string is not a plausible ISO date.
fn parse_iso_date(s: &str) -> Option<i64> {
    let b = s.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    if !b.iter().enumerate().all(|(i, c)| {
        if i == 4 || i == 7 {
            *c == b'-'
        } else {
            c.is_ascii_digit()
        }
    }) {
        return None;
    }
    let y: i64 = s[0..4].parse().ok()?;
    let m: u32 = s[5..7].parse().ok()?;
    let d: u32 = s[8..10].parse().ok()?;
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    Some(days_from_civil(y, m, d))
}

/// Format the ISO date for a day number since 1970-01-01 — the CLI
/// layer's `today` helper (the clock stays at the CLI; this is pure
/// arithmetic over a caller-supplied day count).
pub fn iso_date_from_unix_days(days: i64) -> String {
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Parse an issue-state predicate into `(issue_ref, target_state)`
/// disjuncts: `<owner/repo#N> state == <open|closed>`, joined by ` or `
/// (a fully-machine boolean compound within one class counts as one
/// predicate — REQ-4).
fn parse_issue_predicate(predicate: &str) -> Option<Vec<(String, IssueState)>> {
    let mut out = Vec::new();
    for disjunct in predicate.split(" or ") {
        let (issue_ref, state) = disjunct.trim().rsplit_once(" state == ")?;
        let target = match state.trim() {
            "open" => IssueState::Open,
            "closed" => IssueState::Closed,
            _ => return None,
        };
        let issue_ref = issue_ref.trim();
        if issue_ref.is_empty() {
            return None;
        }
        out.push((issue_ref.to_string(), target));
    }
    Some(out)
}

// ── Load (the I/O boundary) ──────────────────────────────────────────────────

/// Read and shape-validate the deviation registry at `path` — the I/O
/// boundary in front of the pure verdict. Bounded read; strict entry
/// shape (`deny_unknown_fields`); date fields and machine predicates
/// (date, issue-state) must parse. Every failure is fail-closed at the
/// gate (`Unverifiable`).
pub fn load_deviation_registry(
    path: &Path,
) -> Result<DeviationRegistry, DeviationRegistryError> {
    let display = path.display().to_string();
    let read = match read_bounded(path) {
        Ok(r) => r,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(DeviationRegistryError::Absent { path: display })
        }
        Err(e) => {
            return Err(DeviationRegistryError::Unreadable {
                path: display,
                detail: e.to_string(),
            })
        }
    };
    if read.oversize {
        return Err(DeviationRegistryError::ShapeInvalid {
            path: display,
            detail: format!("exceeds the {MAX_ARTIFACT_BYTES}-byte artifact bound"),
        });
    }
    let registry: DeviationRegistry = serde_yaml_ng::from_slice(&read.bytes).map_err(|e| {
        DeviationRegistryError::ShapeInvalid {
            path: display.clone(),
            detail: e.to_string(),
        }
    })?;
    if registry.schema_version != 1 {
        return Err(DeviationRegistryError::ShapeInvalid {
            path: display,
            detail: format!(
                "unsupported schema_version {} (this binary reads schema_version 1)",
                registry.schema_version
            ),
        });
    }
    for entry in &registry.entries {
        let defect = |detail: String| DeviationRegistryError::ShapeInvalid {
            path: display.clone(),
            detail,
        };
        if parse_iso_date(&entry.entry_date).is_none() {
            return Err(defect(format!(
                "entry '{}': entry_date {:?} is not an ISO date",
                entry.id, entry.entry_date
            )));
        }
        if let Some(expiry) = &entry.expiry {
            if parse_iso_date(expiry).is_none() {
                return Err(defect(format!(
                    "entry '{}': expiry {:?} is not an ISO date",
                    entry.id, expiry
                )));
            }
        }
        if let Some(trigger) = &entry.retest_trigger {
            match trigger.trigger_type {
                TriggerType::Date => {
                    if parse_iso_date(trigger.predicate.trim()).is_none() {
                        return Err(defect(format!(
                            "entry '{}': date trigger predicate {:?} is not an ISO date",
                            entry.id, trigger.predicate
                        )));
                    }
                }
                TriggerType::IssueState => {
                    if parse_issue_predicate(&trigger.predicate).is_none() {
                        return Err(defect(format!(
                            "entry '{}': issue-state predicate {:?} does not match \
                             '<owner/repo#N> state == <open|closed>' (or-joined)",
                            entry.id, trigger.predicate
                        )));
                    }
                }
                // version-compare and artifact-presence predicates carry no
                // machine grammar in this build; they evaluate as
                // undecidable, never silently passing in CI.
                TriggerType::VersionCompare | TriggerType::ArtifactPresence => {}
            }
        }
    }
    Ok(registry)
}

// ── The pure verdict ─────────────────────────────────────────────────────────

/// The pure deviations verdict over a parsed registry (REQ-6): enumerate
/// `standing` entries (premature resolutions included); fail loud on a
/// lapsed expiry or a fired machine-decidable retest trigger without a
/// newer Solution Owner `disposition_ref`, comparing only in-entry dates
/// against the caller-supplied `today` (ISO date; no clock in vsdd-core).
/// `issue_oracle` resolves an upstream issue reference to its state, or
/// `None` when no oracle is available (the undecidable direction: local
/// warn-pass, CI inconclusive — resolved Q3).
pub fn deviations_verdict(
    registry: &DeviationRegistry,
    today: &str,
    mode: GateMode,
    issue_oracle: &dyn Fn(&str) -> Option<IssueState>,
) -> DeviationsOutcome {
    let mut warnings: Vec<String> = Vec::new();
    let mut blocks: Vec<String> = Vec::new();
    let mut inconclusive: Vec<String> = Vec::new();

    let Some(today_days) = parse_iso_date(today) else {
        return DeviationsOutcome {
            verdict: GateVerdict::Unverifiable(format!(
                "caller-supplied today {today:?} is not an ISO date"
            )),
            warnings,
        };
    };

    // Defensive shape net for hand-constructed registries: the loader
    // validates these, but the pure function must not panic on any input.
    let shape_defect = |detail: String| DeviationsOutcome {
        verdict: GateVerdict::Unverifiable(format!("deviation registry shape defect: {detail}")),
        warnings: Vec::new(),
    };

    for entry in &registry.entries {
        // Enumerate standing entries. `resolved` is terminal only under its
        // own Solution Owner disposition_ref; a premature resolution is
        // treated as still standing (AC-5).
        let premature =
            entry.status == DeviationStatus::Resolved && entry.disposition_ref.is_none();
        if entry.status == DeviationStatus::Resolved && !premature {
            continue;
        }
        if premature {
            warnings.push(format!(
                "deviation entry '{}': resolved without a Solution Owner disposition_ref — \
                 premature resolution, treated as still standing",
                entry.id
            ));
        } else if entry.disposition_ref.is_none() {
            // Creation at `standing` is Solution-Owner-gated; the local
            // gate checks ref-presence only (warn grade, REQ-4).
            warnings.push(format!(
                "deviation entry '{}': standing without a Solution Owner disposition_ref \
                 (creation is Solution-Owner-gated; unverified)",
                entry.id
            ));
        }

        let Some(entry_days) = parse_iso_date(&entry.entry_date) else {
            return shape_defect(format!(
                "entry '{}': entry_date {:?} is not an ISO date",
                entry.id, entry.entry_date
            ));
        };
        let default_days = entry_days + DEFAULT_EXPIRY_DAYS;

        // Effective expiry: the 30-day default from entry_date; a stated
        // expiry within the default applies; a beyond-default override
        // applies only when a disposition_ref covers it — an uncovered
        // override is an entry-level defect carrying the default, warned
        // (REQ-4, resolved Q2).
        let effective_expiry_days = match &entry.expiry {
            None => default_days,
            Some(expiry) => {
                let Some(expiry_days) = parse_iso_date(expiry) else {
                    return shape_defect(format!(
                        "entry '{}': expiry {:?} is not an ISO date",
                        entry.id, expiry
                    ));
                };
                if expiry_days > default_days && entry.disposition_ref.is_none() {
                    warnings.push(format!(
                        "deviation entry '{}': expiry {} beyond the 30-day default without a \
                         Solution Owner disposition — inoperative override, the default {} applies",
                        entry.id,
                        expiry,
                        iso_date_from_unix_days(default_days)
                    ));
                    default_days
                } else {
                    expiry_days
                }
            }
        };

        if effective_expiry_days < today_days {
            blocks.push(format!(
                "{}: expiry {} lapsed (today {today}) without a Solution Owner re-arm",
                entry.id,
                iso_date_from_unix_days(effective_expiry_days)
            ));
            // A lapsed entry is already loud; its trigger adds nothing.
            continue;
        }

        let Some(trigger) = &entry.retest_trigger else {
            continue;
        };
        let mut undecidable = |detail: String| match mode {
            GateMode::Local => warnings.push(format!(
                "deviation entry '{}': retest trigger undecidable ({detail}) — \
                 passing warn-grade on the local gate",
                entry.id
            )),
            GateMode::Ci => inconclusive.push(format!(
                "deviation entry '{}': retest trigger undecidable ({detail})",
                entry.id
            )),
        };
        match trigger.trigger_type {
            TriggerType::Date => {
                let predicate = trigger.predicate.trim();
                let Some(predicate_days) = parse_iso_date(predicate) else {
                    return shape_defect(format!(
                        "entry '{}': date trigger predicate {:?} is not an ISO date",
                        entry.id, trigger.predicate
                    ));
                };
                if predicate_days <= today_days {
                    // Fired. Covered only by a disposition whose in-entry
                    // timestamp postdates the trigger date (lexicographic
                    // over the shared ISO prefix; never tracker prose).
                    let covered = entry
                        .disposition_ref
                        .as_ref()
                        .is_some_and(|d| d.comment_timestamp.trim() > predicate);
                    if !covered {
                        blocks.push(format!(
                            "{}: date trigger {predicate} fired without a newer \
                             Solution Owner disposition",
                            entry.id
                        ));
                    }
                }
            }
            TriggerType::IssueState => match parse_issue_predicate(&trigger.predicate) {
                None => undecidable(format!(
                    "issue-state predicate {:?} unparseable",
                    trigger.predicate
                )),
                Some(disjuncts) => {
                    let mut fired = false;
                    let mut unknown = false;
                    for (issue_ref, target) in &disjuncts {
                        match issue_oracle(issue_ref) {
                            Some(state) if state == *target => {
                                fired = true;
                                break;
                            }
                            Some(_) => {}
                            None => unknown = true,
                        }
                    }
                    if fired {
                        // A fired issue-state trigger has no in-entry firing
                        // date to compare a disposition against: it stays
                        // loud until the Solution Owner re-arms the entry
                        // with a rewritten predicate/expiry or resolves it
                        // (REQ-4's re-arm rewrites the entry).
                        blocks.push(format!(
                            "{}: issue-state trigger fired ({}) — retest owed; \
                             re-arm requires a Solution Owner disposition rewriting the entry",
                            entry.id,
                            trigger.predicate.trim()
                        ));
                    } else if unknown {
                        undecidable("issue state unavailable to this gate run".to_string());
                    }
                }
            },
            TriggerType::VersionCompare => {
                undecidable("no version-compare evaluator in this build".to_string());
            }
            TriggerType::ArtifactPresence => {
                undecidable("no artifact-presence evaluator in this build".to_string());
            }
        }
    }

    // 0/1/2 ordering: inconclusive (the exit-2 class) outranks Block;
    // definite blocks ride the reason string so nothing hides behind the
    // inconclusive verdict.
    let verdict = if !inconclusive.is_empty() {
        let mut reasons = inconclusive;
        reasons.extend(blocks);
        GateVerdict::Unverifiable(format!(
            "deviations inconclusive on the CI leg: {}",
            reasons.join("; ")
        ))
    } else if !blocks.is_empty() {
        GateVerdict::Block(blocks)
    } else {
        GateVerdict::Pass
    };
    DeviationsOutcome { verdict, warnings }
}

/// The composed deviations gate leg: load the registry at `registry_path`
/// and evaluate it; every load failure maps to `Unverifiable` in BOTH
/// modes (the fail-closed keystone).
pub fn deviations_gate(
    registry_path: &Path,
    today: &str,
    mode: GateMode,
    issue_oracle: &dyn Fn(&str) -> Option<IssueState>,
) -> DeviationsOutcome {
    match load_deviation_registry(registry_path) {
        Ok(registry) => deviations_verdict(&registry, today, mode, issue_oracle),
        Err(err) => DeviationsOutcome {
            verdict: GateVerdict::Unverifiable(err.to_string()),
            warnings: Vec::new(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{iso_date_from_unix_days, parse_iso_date, parse_issue_predicate, IssueState};

    #[test]
    fn civil_date_round_trip_including_leap_days() {
        for iso in ["1970-01-01", "2024-02-29", "2026-08-01", "2099-12-31"] {
            let days = parse_iso_date(iso).expect("valid ISO date parses");
            assert_eq!(iso_date_from_unix_days(days), iso, "round-trip for {iso}");
        }
        assert_eq!(parse_iso_date("1970-01-01"), Some(0), "the epoch is day 0");
    }

    #[test]
    fn parse_iso_date_rejects_non_dates() {
        for bad in ["2026-8-01", "2026/08/01", "2026-13-01", "2026-08-+1", "today", ""] {
            assert_eq!(parse_iso_date(bad), None, "{bad:?} is rejected");
        }
    }

    #[test]
    fn issue_predicate_parses_single_and_compound_forms() {
        let single = parse_issue_predicate("owner/repo#9 state == closed").unwrap();
        assert_eq!(
            single,
            vec![("owner/repo#9".to_string(), IssueState::Closed)]
        );
        let compound =
            parse_issue_predicate("o/r#9 state == closed or o/r#10 state == closed").unwrap();
        assert_eq!(compound.len(), 2);
        assert_eq!(parse_issue_predicate("no grammar here"), None);
    }
}

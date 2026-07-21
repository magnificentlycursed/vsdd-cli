# Layer 1 manual-test checklist — state artifact and versioned data

Operator-adopted 2026-07-21 (decision on vsdd-cli #716; drafted as agent
proposals, adopted per The operator authors the oracle). These are the
director tests the automated suite cannot grade: human judgment of
diagnostic quality, hand-performed corruption, and the assistive read.
Surfaces: each item drives the layer's fixture harness or the smallest
reader entry point the layer lands; the operator may adjust surfaces at
execution and records outcomes on the layer's tracker trail.

1. **Malformed state file.** Drop a colon from a line of `.vsdd/state.yaml`
   and invoke the reader. Expect: a rustc-shaped diagnostic naming the
   file, the parse location (line and column), and the restore-content
   recovery text; no panic, no backtrace. Judge: does the message read as
   a complete instruction a tired human can follow?
2. **Absent state file.** Remove `.vsdd/state.yaml` and invoke the reader.
   Expect: a worded diagnostic naming the path, the restore recovery, a
   nonzero exit; no panic.
3. **Permission failure.** `chmod 000 .vsdd/state.yaml` and invoke the
   reader. Expect: the permission-or-input/output diagnostic with the
   branched recovery text (file permissions, or the disk or mount fault).
4. **Published-marker immutability.** Given a state whose `published`
   block is written, hand-edit its version and invoke the reader. Expect:
   the next read or self-validation flags the change naming the field.
5. **Registry resilience.** Corrupt one registry artifact's frontmatter
   (any of the nine in `templates/registry/`). Expect: the loader's
   diagnostic names that file and location; the other eight load.
6. **Future schema version.** Set `schema_version: 9.9.9` in the state
   file. Expect: refusal naming the version seen and the versions
   supported; no partial read.
7. **Color-strip and assistive read.** Run items 1–3 in a no-color
   terminal: no information lost. Read the diagnostics with the named
   screen reader on the operator's checklist slot (verification
   architecture: the accessible-equivalent designation is discharged by a
   real assistive-technology pass): the text reads as complete sentences.
8. **Token fidelity.** For one read-failure kind, compare the emitted
   machine token and recovery action verbatim against
   `templates/registry/statusline-data.md` — the code's vocabulary is the
   loaded data set's, never a hardcoded copy.

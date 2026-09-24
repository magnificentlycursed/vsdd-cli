# Slice 3 manual-test checklist — install, static half (`vsdd init` drift handling and the deployed templates)

PROPOSED (agent-drafted 2026-09-24 under vsdd-cli#874's session; adoption is the
operator's act per The operator authors the oracle — record the decision on the
tracker and replace this line with the adoption record). Director tests for the
part of Install that is built: the per-file classification, drift refusal, the
`--check` / `--dry-run` / `--update` / `--force` / `--no-prompt` / `--ci-mode`
conduct, and the deployed template set (its count lives in the install manifest and the AC-11 assertion of `vsdd-core/tests/install_slice_red_gate.rs`, never restated here). The statusline-offer conduct is
pinned by the install-offer fixtures and is not repeated here beyond item 7. The
generated members (skills and domain prompts) are Slice 2's and are out of scope.
Surfaces: `vsdd init` in a scratch clone of this repo. Outcomes are recorded on
the slice's tracker trail.

1. **Second run is a no-op that says so.** In a fresh clone, run `vsdd init`
   twice. Expect: the second run writes nothing and reports every managed file
   as unchanged by name — never a silent exit 0.
2. **A hand-edited managed file is refused with a rustc-shaped diagnostic.** Edit
   one deployed registry file (add a line to `.vsdd/registry/economics-data.md`),
   run `vsdd init`. Expect: refusal naming the file, the classification
   ("Conflict: operator-edited"), the recorded-versus-found state, and the two
   exits — `--force` to overwrite, or keep the edit. Judge: is the choice
   explained well enough that the operator would not guess?
3. **`--dry-run` predicts exactly what a real run does.** With the edit from item 2
   in place, run `vsdd init --dry-run`, note the per-file plan, then run
   `vsdd init --force`. Expect: the real run's actions equal the plan, file for
   file.
4. **`--update` touches only unedited files whose template changed.** Simulate a
   toolkit upgrade (run a build with one template's bytes changed) and run
   `vsdd init --update` on a tree with one operator-edited managed file.
   Expect: the changed-template file updates, the operator-edited file is left
   alone and named as skipped, and nothing else moves.
5. **Non-interactive conduct never prompts and never overwrites.** Run
   `vsdd init --no-prompt` and `vsdd init --ci-mode` against the Conflict tree
   from item 2, with stdin closed. Expect: no hang, the Conflict file skipped and
   named, exit status documented in the output. Then `--ci-mode` alone: the
   statusline offer is not made and the manual invocation is printed.
6. **`--check` deploys nothing.** Run `vsdd init --check` in a fresh clone.
   Expect: the environment probe is reported (crosslink initialized? runtime
   settings present?), no file is written — verify with `git status`.
7. **The statusline offer discloses its write.** In an interactive run with no
   statusline entry in the runtime settings, decline the offer. Expect: the
   prompt named the file it would write, the default, and the decline path; after
   declining, the settings file is byte-identical and `vsdd status --statusline`
   still works by hand.
8. **Every deployed template resolves in the install manifest.** After a clean
   init, compare `.vsdd/registry/installed-artifact-manifest.md` against the tree:
   every listed artifact exists; every deployed file is listed; `vsdd status`
   reports no installed-artifact-integrity finding. Then delete one deployed
   file and re-run `vsdd status`: the dangling reference is a loud finding.
9. **Hollow-shell loudness.** Delete a hook payload that a tracked wiring
   references and open a session in the clone. Expect: the wired hook exits
   nonzero in the runtime harness — a visible failure, never a guarded no-op.
10. **Color-strip and assistive read.** Run items 2 and 5 in a no-color terminal
    and read them with the named screen reader: no information lost; the
    diagnostics read as complete sentences.

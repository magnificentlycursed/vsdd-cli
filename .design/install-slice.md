# Feature: Slice 3 — Install (init drift-handling + template deployment)

## Summary

`vsdd init` (in `vsdd-core/src/init.rs`) today classifies each managed file two
ways — skip-if-equal, silently overwrite on toolkit upgrade, and hard-refuse on
operator drift — and it does not deploy any of the `templates/*` artifacts. This
slice replaces that with a three-way classification (manifest hash × current-disk
hash × new-template hash, mirroring crosslink's `classify_update`), a
`--force`/`--update`/`--no-prompt`/`--dry-run` flag surface with an interactive
Conflict prompt, a breaking `ManifestEntry.template_version_at_deploy` manifest
field with a sha-first migration of pre-field manifests, extends the deployment
plan to cover 15 of the 16 `templates/*` files (each tagged with a per-artifact
**management class** — `managed`, `scaffold`, or `section-managed`), **generates**
the adopter's `installed-artifact-manifest.md` from the artifacts it actually
deploys rather than deploying the vsdd-cli-specific one as a template, and
**repoints the runtime registry loader**
(`vsdd-core/src/registry/mod.rs::load_set`/`load_all`) to read deployed registry
sets from `.vsdd/registry/`.

This is a phase-1a design authored the vsdd way (a `.design/` doc via the design
affordance) for crosslink issue #834, folding the operator-resolved init and
template content from the now-closed #14 (sections B and C, decisions Q2/Q3/Q4).
The mdatron subprocess client (#14 section A) is defunct/#835 and out of scope;
the `vsdd-core` removal + workspace-shape contract touch is a later slice.

## Requirements

- REQ-1: **Three-way classification replaces the two-way check.** Each managed
  file is classified from the triple (recorded manifest hash, current-disk hash,
  new-template hash) into exactly one of: **Unchanged** (all three equal → skip),
  **ToolkitUpgrade** (disk == manifest ≠ template → update), **Conflict** (disk ≠
  manifest, the operator-edited case → do not silently overwrite), **Missing**
  (destination absent → deploy). This mirrors crosslink's `classify_update` and
  replaces the current `init.rs` logic where a `dest.exists()` file that matches
  the prior manifest but differs from source is *silently overwritten*
  (`init.rs` line ~132 "No drift but content differs from source = toolkit
  upgrade. Overwrite.") and a disk hash differing from the manifest raises the
  hard `InitError::ManagedFileDrifted`. (B1)

- REQ-2: **`--dry-run` prints the plan and writes nothing.** With `--dry-run`,
  init prints the per-file classification (REQ-1) and the action it *would* take,
  writes no file, writes no manifest, appends no event, and exits 0. It composes
  with the other flags (e.g. `--force --dry-run` prints what a forced run would
  overwrite, still writing nothing). (B2)

- REQ-3: **`--force` overwrites Conflict files.** `--force` overwrites a Conflict
  (operator-edited) file with the new template and records its new hash +
  `template_version_at_deploy`. This is the explicit override that replaces
  today's unconditional `ManagedFileDrifted` refuse-on-drift. (B3)

- REQ-4: **`--update` applies ToolkitUpgrade only.** `--update` writes
  ToolkitUpgrade files (disk == manifest ≠ template) and leaves Conflict files
  untouched unless `--force` is also given. (B4)

- REQ-5: **`--no-prompt` is non-interactive and CI-safe; `ci_mode` implies it.**
  With `--no-prompt`, Conflict files are skipped (never overwritten) unless
  `--force`; no prompt is issued even on a TTY. The existing
  `InitOptions.ci_mode` field (`init.rs` line ~34, currently consumed only by
  `let _ = options;`) implies `--no-prompt`. (B5)

- REQ-6: **Interactive Conflict prompt on a TTY by default.** When stdout/stdin
  is a TTY and `--no-prompt` is not set, each Conflict file prompts the operator
  per file with the choices: keep the operator edit, accept the new template, or
  show the diff. The chosen action is applied to that file only. (B6)

- REQ-7: **`ManifestEntry` gains `template_version_at_deploy` (breaking manifest
  change) with a sha-first migration.** The `ManifestEntry` struct in `init.rs`
  (line ~285, currently `{ sha256: String }` only) gains a
  `template_version_at_deploy: String` field recording the toolkit version whose
  template produced the deployed content, so three-way classification survives
  across upgrades. This is a breaking `init-manifest.json` format change that
  must land before v0.1.0 publishes. A pre-field manifest (an entry with no
  `template_version_at_deploy`) is migrated **sha-first** (Q4): if disk ==
  recorded sha256, adopt/upgrade and backfill the field; if disk ≠ recorded
  sha256, classify as Conflict (operator review) — never a silent overwrite. (B7,
  Q4)

- REQ-8: **Idempotence is preserved.** An all-Unchanged run writes no file, does
  not rewrite the manifest, emits no `ProjectInitialized` event, and exits 0 —
  the current behavior (`init.rs` skips manifest write when the serialized
  content is byte-identical, and emits the event on first init only). (B8)

- REQ-9: **Error handling is typed and conservative.** A non-git directory is
  refused (`InitError::NotGitRepository`, unchanged); a filesystem failure is a
  typed `InitError::Io { path, error }` (unchanged); a corrupt/unparseable
  manifest is treated as first-init (as `load_manifest` does today, returning
  `Ok(None)` on a serde error), never a false drift. A template whose destination
  path collides with a pre-existing non-managed file (a file present on disk with
  no manifest entry) is classified Conflict, not silently overwritten. (B9)

- REQ-10: **Re-run-to-converge atomicity; the manifest is written last.** Init is
  idempotent with no transactional atomicity (#14 Q3, matching mdatron's v0.1 init
  posture). The `init-manifest.json` is written after all file deployments
  (as `init.rs` already sequences: deploy loop → `.vsdd/` skeleton → manifest →
  event), so a partial/interrupted run re-runs cleanly: missing files are
  re-classified Missing and re-deployed, then the manifest is rewritten. (B10,
  #14 Q3)

- REQ-11: **`vsdd init` deploys 15 of the 16 `templates/*` files; the 16th is
  generated (REQ-15).** The deployment plan (`build_deployment_plan` in
  `init.rs`) is extended from the current 47 artifacts (4 schemas + 1 pattern +
  10 phase primers + 18 domain prompts + 14 supplements) to also include 15 files
  under `templates/` — the artifacts `init.rs` line ~14 explicitly defers
  ("Templates deployment … is deferred to a follow-up iteration"). The verified
  deployed set is: 2 CI workflows (`vsdd-verify.yml`, `vsdd-observe-pr-body.yml`),
  1 DESIGN template (`DESIGN.md.vsdd-template`), 1 statusline script
  (`vsdd-statusline.sh`), and 11 `templates/registry/*` data sets (8 `.md`:
  `act-to-affordance-map`, `composition-scope-and-actions`, `dispatch-data`,
  `economics-data`, `gate-data`, `snapshot-schema`, `state-schema`,
  `statusline-data`; 3 `.yaml`: `anonymization-patterns`, `canonical-patterns`,
  `vocabulary`). The 16th file, `templates/registry/installed-artifact-manifest.md`,
  is **not** deployed as a template — it is instance-specific to vsdd-cli and is
  generated per adopter by REQ-15. (C1, D2 — set re-verified against the current
  tree: 16 files, 15 deployed + 1 generated.)

- REQ-12: **Deployed templates are drift-tracked under REQ-1 according to their
  management class (REQ-16).** Each deployed template file is recorded in
  `init-manifest.json` (hashed, with `template_version_at_deploy`) and is subject
  to the three-way classification (REQ-1) and flag surface (REQ-2..6) to the
  extent its management class prescribes: a `managed` file is classified
  whole-file; a `section-managed` file is classified only over its tool-owned
  section(s) (`managed_section_anchors`), leaving operator-extension regions
  untouched; a `scaffold` file is deploy-if-absent and never enters
  ToolkitUpgrade/Conflict overwrite once present. The statusline script's own
  header ("This file is a managed artifact: vsdd init refuses to overwrite a
  hand-modified copy", `templates/statusline/vsdd-statusline.sh` lines ~12–13)
  states the `managed` posture that REQ-1 upgrades from refuse-on-drift to the
  Conflict prompt. (C2, D3)

- REQ-13: **Template destinations are the deployed adopter paths.** The
  destinations are: `templates/.github/workflows/vsdd-verify.yml` →
  `.github/workflows/vsdd-verify.yml`; `templates/.github/workflows/vsdd-observe-pr-body.yml`
  → `.github/workflows/vsdd-observe-pr-body.yml` (both confirmed by the
  workflows' own header comments); `templates/DESIGN.md.vsdd-template` →
  `DESIGN.md` (**scaffold** class per REQ-16 — the template header states it
  "deploys this to adopting projects that don't already have a DESIGN.md", so the
  operator owns it thereafter and it is exempt from the ToolkitUpgrade overwrite
  of REQ-1); `templates/statusline/vsdd-statusline.sh` →
  `.vsdd/statusline/vsdd-statusline.sh`; and each `templates/registry/<file>` →
  `.vsdd/registry/<file>` (the destination the registry sets' own header comments
  name — e.g. `vocabulary.yaml` line 2, "Deployed by `vsdd init` … at
  .vsdd/registry/vocabulary.yaml"). These `.vsdd/registry/` destinations are the
  path the runtime loader is repointed to read (REQ-14/D1), so the adopter's
  `vsdd status` finds the registry it just installed. No template requires
  init-time adopter-name substitution: the DESIGN template uses operator-fill
  placeholders (e.g. the `<Project Name>` heading the operator populates), and
  the workflows use generic `${{ secrets.* }}` references. (C3, D1, D2)

- REQ-14: **The runtime registry loader is repointed to `.vsdd/registry/`.**
  Today `vsdd-core/src/registry/mod.rs::load_set` builds its path as
  `repo_root.join("templates/registry").join("{class}.md")` (lines ~139–141) and
  `load_all` (line ~305) walks every set the same way, while the binary passes
  `std::env::current_dir()` as `repo_root` (`vsdd/src/main.rs` lines ~80/87/95;
  also `vsdd/src/status/human.rs`). As part of this slice the loader is repointed
  to read deployed registry sets from `<repo_root>/.vsdd/registry/` (the adopter
  config home and the REQ-13 deploy destination), falling back to
  `<repo_root>/templates/registry/` when `.vsdd/registry/` is absent so the
  vsdd-cli source repo continues to load from its own tree. Both `load_set` and
  `load_all` — and the read of the generated `installed-artifact-manifest.md`
  (REQ-15) — use the same repointed resolution. (D1)

- REQ-15: **`vsdd init` generates the adopter's installed-artifact manifest.**
  `templates/registry/installed-artifact-manifest.md` is instance-specific (its
  frontmatter enumerates vsdd-cli's own hook wiring, plugin set, git hooks, and
  MCP servers) and is therefore **not** deployed as a template (REQ-11). Instead
  init generates `.vsdd/registry/installed-artifact-manifest.md` enumerating the
  artifacts it actually deployed into the adopter, conforming to the
  `installed-artifact-manifest` schema. The generated content is deterministic
  over the deployed set, so a converged re-run produces byte-identical content and
  rewrites nothing (composing with REQ-8 idempotence). It is a generated instance
  artifact — not a deployed template — carries no management class (there is no
  canonical template to drift against), and is regenerated (not drift-classified)
  whenever the deployed set changes. (D2)

- REQ-16: **Each deployed template carries a management class.** Every entry in
  the extended deployment plan (REQ-11) is tagged with one of three management
  classes that determines how REQ-1's classification applies. **`managed`** —
  drift-tracked whole-file under the three-way classification (the 2 workflows,
  the statusline script, and the 8 `.md` registry data sets). **`scaffold`** —
  deploy-if-absent, never overwritten once present, exempt from
  ToolkitUpgrade/Conflict (`DESIGN.md`). **`section-managed`** — only the
  tool-owned section(s) delimited by `managed_section_anchors` are drift-tracked,
  while operator-extension regions are left untouched (the 3 `.yaml` registry
  files `vocabulary.yaml`, `canonical-patterns.yaml`, `anonymization-patterns.yaml`,
  each of which marks its managed region with `# === vsdd managed ===` … `# === End
  vsdd managed ===` above an `operator_extensions:` region). The class is a
  per-artifact attribute on the deployment plan; the section anchors are recorded
  on the manifest entry via the `managed_section_anchors` field `init.rs` line
  ~280 already anticipates. (D3)

## Acceptance Criteria

- [ ] AC-1: A test harness constructs each of the four states — all-equal;
  disk==manifest≠template; disk≠manifest; destination absent — and asserts the
  classification is Unchanged / ToolkitUpgrade / Conflict / Missing respectively,
  and that a disk==manifest≠template file is *updated* (not refused) while a
  disk≠manifest file is *not* silently overwritten. (REQ-1)
- [ ] AC-2: `vsdd init --dry-run` on a repo with at least one Missing and one
  Conflict file prints both classifications and their planned actions, and after
  the run no destination file, no `init-manifest.json`, and no
  `.vsdd/events.jsonl` line has changed on disk; exit code is 0. `--force
  --dry-run` likewise writes nothing. (REQ-2)
- [ ] AC-3: `vsdd init --force` on a Conflict file overwrites it with the current
  template content and the resulting manifest entry's sha256 matches the template
  and `template_version_at_deploy` equals the current toolkit version. (REQ-3)
- [ ] AC-4: `vsdd init --update` writes ToolkitUpgrade files and leaves a Conflict
  file byte-unchanged; `vsdd init --update --force` also overwrites the Conflict
  file. (REQ-4)
- [ ] AC-5: `vsdd init --no-prompt` (and, separately, `InitOptions.ci_mode = true`)
  skips a Conflict file with no prompt and exit 0; adding `--force` overwrites it.
  A `--no-prompt` run issues no prompt even when run against a TTY. (REQ-5)
- [ ] AC-6: With a simulated TTY and no `--no-prompt`, a Conflict file triggers a
  per-file prompt offering keep / accept / diff; selecting "accept" overwrites
  that file and selecting "keep" leaves it unchanged, and the choice applies to
  that file alone (a second Conflict file is prompted independently). (REQ-6)
- [ ] AC-7: `init-manifest.json` written by this slice contains
  `template_version_at_deploy` on every entry; loading a pre-field manifest whose
  recorded sha256 matches disk backfills the field and does not overwrite the
  file, while a pre-field manifest whose recorded sha256 differs from disk yields
  a Conflict (not a silent overwrite). (REQ-7)
- [ ] AC-8: Running `vsdd init` twice with no interposed edits leaves the manifest
  byte-identical, writes no file on the second run, appends no second
  `ProjectInitialized` event, and exits 0 both times. (REQ-8)
- [ ] AC-9: init in a non-git directory returns `InitError::NotGitRepository`; a
  read/write failure returns `InitError::Io` naming the path; a corrupt
  `init-manifest.json` is treated as first-init (no false `ManagedFileDrifted`);
  and a template destination occupied by a pre-existing unmanaged file is
  classified Conflict. (REQ-9)
- [ ] AC-10: Deleting a subset of already-deployed files and re-running (without
  `--dry-run`) re-deploys exactly the deleted files (Missing), rewrites the
  manifest last, and converges to the same manifest a clean first init would
  produce. (REQ-10)
- [ ] AC-11: After `vsdd init` on a fresh git repo, all 15 deployed `templates/*`
  artifacts are present at their REQ-13 destinations and each appears as an entry
  in `init-manifest.json`; the generated
  `.vsdd/registry/installed-artifact-manifest.md` (REQ-15) is present but is not a
  deployed-template entry; and the deployed-artifact count in the
  `ProjectInitialized` event equals the prior 47 plus the 15 deployed templates
  (= 62). (REQ-11)
- [ ] AC-12: A `managed`-class deployed template edited by the operator and
  re-`init`-ed is classified Conflict (not silently overwritten); a `managed`
  template unchanged since deploy but bumped in the toolkit is classified
  ToolkitUpgrade. (REQ-12)
- [ ] AC-13: Both workflow files land at `.github/workflows/`, the statusline
  script at `.vsdd/statusline/vsdd-statusline.sh`, and every deployed
  `templates/registry/<file>` at `.vsdd/registry/<file>`; the DESIGN template
  lands at `DESIGN.md` only when absent (an existing `DESIGN.md` is left untouched
  and not entered as a drift Conflict); and no deployed file contains an
  unsubstituted init-time token (the operator-fill placeholders in the DESIGN
  template are deployed verbatim by design). (REQ-13)
- [ ] AC-14: After `vsdd init` deploys the registry sets to `.vsdd/registry/`,
  `vsdd status` in the adopter repo loads those sets via the repointed
  `load_set`/`load_all` (reading `.vsdd/registry/`, not `templates/registry/`);
  and in the vsdd-cli source repo, where `.vsdd/registry/` is absent, the loader
  falls back to `templates/registry/` and still loads. (REQ-14)
- [ ] AC-15: `vsdd init` on a fresh repo writes
  `.vsdd/registry/installed-artifact-manifest.md` whose `entries` enumerate the
  artifacts actually deployed (not vsdd-cli's own environment), it validates
  against the `installed-artifact-manifest` schema, and a second `init` with no
  interposed change leaves it byte-identical and rewrites nothing. (REQ-15)
- [ ] AC-16: Each deployed artifact's plan entry carries its management class; a
  `scaffold` file (`DESIGN.md`) present on disk is left untouched under `--update`
  (no ToolkitUpgrade); a `section-managed` file (`vocabulary.yaml`) with an
  operator entry appended below `# === End vsdd managed ===` plus a toolkit change
  inside the managed section classifies the managed section as ToolkitUpgrade
  while preserving the operator-extension region byte-for-byte; and a `managed`
  file follows whole-file REQ-1 classification. (REQ-16)

## Architecture

All init behavior lives in `vsdd-core/src/init.rs`. The current shape:
`init(project_root, options)` loads the prior `.vsdd/init-manifest.json`
(`load_manifest`), builds a `Vec<(String, Vec<u8>)>` plan via
`build_deployment_plan`, walks it (drift check → deploy-or-skip), ensures the
`.vsdd/` skeleton (`events.jsonl`, `config.yaml`), writes the manifest, and emits
the first-init event. The serde shapes are `Manifest { vsdd_version, files:
BTreeMap<String, ManifestEntry> }` and `ManifestEntry { sha256 }`.

**Two→three-way change.** The inline `dest.exists()` block (`init.rs` ~lines
107–141) is replaced by an explicit classifier over the triple (prior manifest
sha, current-disk sha, source/template sha). The current code has only two
observable outcomes for an existing file — skip (equal), or the branch comment
"No drift but content differs from source = toolkit upgrade. Overwrite." plus the
`ManagedFileDrifted` early-return. The new classifier yields Unchanged /
ToolkitUpgrade / Conflict / Missing, and the per-classification action is gated by
the flag surface (a new field set on `InitOptions`, which today carries only
`ci_mode` and is otherwise ignored via `let _ = options;`). The interactive
prompt (REQ-6) is TTY-gated; `--no-prompt`/`ci_mode` (REQ-5) takes the
non-interactive skip path. `InitError::ManagedFileDrifted` stops being an
unconditional early-return and becomes the Conflict-without-`--force`-and-without-TTY
outcome (or is retired in favor of a skipped-Conflict report field on
`InitReport`).

**Manifest-format migration.** `ManifestEntry` gains `template_version_at_deploy:
String`. Note the naming: `init.rs` line ~280's own doc comment already
anticipates a future `vsdd_version_at_deploy` field; the operator-resolved name
for this slice is `template_version_at_deploy` (Q4/#14), and today the two are the
same value (`VSDD_VERSION`, `CARGO_PKG_VERSION`, stamped at deploy) — the field
records the toolkit version whose template produced the content. `load_manifest`
deserializes with a `#[serde(default)]` on the new field so a pre-field manifest
parses; the classifier then applies the sha-first migration (REQ-7): recorded
sha == disk → backfill + adopt; recorded sha ≠ disk → Conflict. **Disambiguation:**
this is `init.rs`'s `ManifestEntry`, *not* `vsdd-core/src/registry/sets.rs`'s
`ManifestEntry` (the data model for the `installed-artifact-manifest.md` registry
set) — two unrelated types sharing a name.

**Deployment path.** `build_deployment_plan` currently pulls bundled artifacts
compiled in via `include_str!` from `vsdd-core/src/lib.rs` (`schemas::*`,
`patterns::CROSS_REFERENCES`, `artifacts::PHASE_PRIMERS`,
`artifacts::DOMAIN_PROMPTS`, `artifacts::SUPPLEMENTS`) and maps them to `.mdatron/schemas/`,
`.mdatron/patterns/`, `.claude/commands/`, and `supplements/`. This slice adds 15
`templates/*` files to the plan, each tagged with a management class (D3/REQ-16);
the 16th, `installed-artifact-manifest.md`, is excluded and generated per adopter
(D2/REQ-15). The templates are currently *runtime-read*
from the repo tree, not bundled: `vsdd-core/src/registry/mod.rs` `load_set` /
`load_all` read `repo_root.join("templates/registry/<class>.md")`, and the binary
(`vsdd/src/main.rs`) passes `std::env::current_dir()` as `repo_root`. So the
running vsdd reads registry sets from `<cwd>/templates/registry/` (this slice
repoints that to `.vsdd/registry/` — see the "Registry loader repoint" note). The
9 `.md` sets are consumed by `load_all` (in an adopter, the
`installed-artifact-manifest` set is the one generated by REQ-15 and read from the
generated `.vsdd/registry/` copy); the 3 `.yaml` (`vocabulary`,
`canonical-patterns`, `anonymization-patterns`) are data files consumed elsewhere
(e.g. the register / naming discipline). To deploy them, the slice either bundles them via
`include_str!` (matching the existing artifact pattern) or reads them from the
source `templates/` tree at build/deploy time; the `include_str!` approach is
consistent with the other 47 artifacts and is the recommended path.

**Registry loader repoint (D1/REQ-14).** The registry sets' header comments say
init deploys them to `.vsdd/registry/`, but the runtime loader reads them from
`<cwd>/templates/registry/`. This slice reconciles the two by repointing the
loader: `vsdd-core/src/registry/mod.rs::load_set` (lines ~139–141, which today
build `repo_root.join("templates/registry").join("{class}.md")`) and `load_all`
(line ~305) resolve to `<repo_root>/.vsdd/registry/` first, falling back to
`<repo_root>/templates/registry/` when the former is absent so the vsdd-cli source
repo still loads from its own tree. The binary call sites are unchanged — they
already pass `std::env::current_dir()` as `repo_root` (`vsdd/src/main.rs` lines
~80/87/95; `vsdd/src/status/human.rs`) — so an adopter's `vsdd status` finds the
registry it just installed. This is a small runtime change pulled into the slice.

**Generated installed-artifact manifest (D2/REQ-15).**
`installed-artifact-manifest.md` is *instance-specific to vsdd-cli* (its
frontmatter enumerates this repo's own crosslink hooks, plugin set, git hooks, and
MCP servers), so it is not in the deployed set. Instead init generates the
adopter's `.vsdd/registry/installed-artifact-manifest.md` from the artifacts it
actually deployed, using the `installed-artifact-manifest` data model in
`vsdd-core/src/registry/sets.rs`. Generation is deterministic over the deployed
set and reuses init.rs's existing "skip the write when the serialized content is
byte-identical" behavior, so a converged re-run rewrites nothing (REQ-8).

**Management class (D3/REQ-16).** Each deployed-plan entry carries a `managed` /
`scaffold` / `section-managed` tag, added alongside the `(dest, bytes)` the plan
already holds. `managed` runs REQ-1 whole-file; `scaffold` is deploy-if-absent;
`section-managed` runs REQ-1 only over the tool-owned region delimited by
`# === vsdd managed ===` … `# === End vsdd managed ===` (verified present in
`templates/registry/vocabulary.yaml`, `canonical-patterns.yaml`, and
`anonymization-patterns.yaml`), recording those anchors via the
`managed_section_anchors` field `init.rs` line ~280's doc comment already
anticipates. No `.md` registry set carries these markers, so all 8 deployed `.md`
sets are `managed`.

## Resolved Decisions

The three questions this design opened were resolved by operator ratification and
are recorded here; every REQ / AC / Architecture line above reflects them.

### D1: Registry deploy destination and the loader repoint (was Q1)

**Decision.** Adopter registry sets deploy to `.vsdd/registry/` (the adopter
config home), and the runtime registry loader is **repointed** to read from
`.vsdd/registry/`. Today `vsdd-core/src/registry/mod.rs::load_set` reads
`<repo_root>/templates/registry/<class>.md` (lines ~139–141) and `load_all`
(line ~305) does the same, with the binary passing `std::env::current_dir()` as
`repo_root` (`vsdd/src/main.rs` lines ~80/87/95; `vsdd/src/status/human.rs`).
This slice repoints both to `<repo_root>/.vsdd/registry/`, falling back to
`templates/registry/` when `.vsdd/registry/` is absent so the vsdd-cli source repo
still loads from its own tree (option (a) of the original question). Captured as
REQ-14 (AC-14) and the "Registry loader repoint" Architecture note. The statusline
script shares this `.vsdd/`-layout decision and deploys to
`.vsdd/statusline/vsdd-statusline.sh` (REQ-13).

### D2: `installed-artifact-manifest.md` is generated, not deployed (was Q2)

**Decision.** `templates/registry/installed-artifact-manifest.md` is
instance-specific to vsdd-cli (its frontmatter describes this repo's own hook
wiring, plugin set, git hooks, and MCP servers) and is therefore **not** deployed
as a template. Instead `vsdd init` **generates** the adopter's
`.vsdd/registry/installed-artifact-manifest.md` from the artifacts it actually
deploys. The deployed-template set is therefore **15 deployed + 1 generated**,
superseding #14's captured "deploy all 16": REQ-11 lists the 15 deployed files and
REQ-15 (AC-15) covers the generated manifest. The other 11 `templates/registry/*`
sets are toolkit-canonical and deployed as-is under their management class (D3).

### D3: Per-artifact management class (was Q3)

**Decision.** Each deployed artifact carries a **management class** that
determines how REQ-1's three-way classification applies:
- **`managed`** — drift-tracked whole-file (the 2 workflows, the statusline
  script, and the 8 `.md` registry data sets).
- **`scaffold`** — deploy-if-absent, never overwritten once present (`DESIGN.md`).
- **`section-managed`** — only the tool-owned section(s) delimited by
  `managed_section_anchors` are drift-tracked; operator-extension regions are left
  untouched. Grounding shows all three `.yaml` registry files
  (`vocabulary.yaml`, `canonical-patterns.yaml`, `anonymization-patterns.yaml`),
  not `vocabulary.yaml` alone, mark their managed region with
  `# === vsdd managed ===` … `# === End vsdd managed ===` above an
  `operator_extensions:` region, so all three are section-managed.

Captured as REQ-16 (AC-16) and the "Management class" Architecture note; `init.rs`
line ~280 already anticipates the `managed_section_anchors` field the
section-managed class records.

## Out of Scope

- **The mdatron subprocess client** (#14 section A) — defunct, closed as #835;
  vsdd does not wrap mdatron. `vsdd-core/src/subprocess.rs` (the bounded runner)
  is referenced only for background; nothing in this slice invokes mdatron as a
  subprocess.
- **The `vsdd-core` removal / workspace re-org and its Architecture workspace-shape
  contract touch** — a later slice; this slice keeps the two-crate workspace.
- **mdatron's own `init` / `config.yaml`** — mdatron's own bootstrap, already
  shipped upstream; not vsdd's concern here.
- **Statusline wiring into the adopter's Claude Code settings** (the settings
  `statusLine` entry / reference surface) — Layer 4 wiring, separate from
  depositing the script file; this slice deploys the file, not the settings edit.
- **crates.io publish + CI install-hint migration** — a later slice.

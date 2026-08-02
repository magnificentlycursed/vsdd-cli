# vsdd

**vsdd** is a Rust toolkit that binds an AI agent's development process to
**Verified Spec-Driven Development (VSDD)** — a software methodology in which the
specification is authored first, made verifiable, and defended by adversarial
cold-session review before any code is accepted. vsdd is a **methodology
harness**: an installed environment in which following the methodology is the
path of least resistance, and deviation produces immediate, compiler-shaped
feedback.

vsdd does not run agents and does not track work itself. It is the methodology
layer that composes against tools that do:

- **[crosslink](https://github.com/dollspace-gay/crosslink)** — the tracker and
  orchestration layer: issues, dispatch (swarm), worktrees, gates, sessions, and
  knowledge. The project's audit trail lives here.
- **Claude Code** — the **runtime harness**: the agent runtime that executes the
  agent and provides the hooks and tool restrictions the methodology's guardrails
  ride on.
- **[mdatron](https://github.com/magnificentlycursed/mdatron)** — the
  **conformance engine**, consumed as a separate binary. It is a
  methodology-agnostic typed-markdown validator (Schematron-derived, with
  rustc-shaped diagnostics) that checks every methodology artifact. vsdd authors
  the VSDD-specific schemas and patterns; mdatron enforces them.

vsdd itself supplies the methodology layer installed into crosslink's seams: the
process spec, the phase state, deterministic domain composition, domain prompts,
generated agent context, hooks, and the cost layer. It is **self-hosted** — it
applies its own methodology to its own development, so this repository is the
worked example.

> VSDD extends **VDD** (the adversarial-review predecessor methodology) with a
> spec-first pipeline. Both methodologies, crosslink, and the Thermite harness
> (the outer agent harness) are authored by
> **[dollspace](https://github.com/dollspace-gay)**; see
> [Credits](#credits). vsdd-cli is an independent Rust implementation of that
> methodology, not the methodology itself.

## Project status

vsdd is under active bootstrap and respecification. The pieces described below
are labelled **built** or **planned** throughout, and the two are kept strictly
separate — an unbuilt capability is never described here as if it ships. The
current built core is the **engine** — the shared core every capability builds
on (phase state, the status answer, the versioned registry data, and
terminal-output safety) — plus the first vertical slice (the routing-before-fix
guardrail). The conformance-and-efficiency subsystem, the golden-path
dispatcher, and the efficiency insight engine are designed and queued, not yet
built.

## Install and build

vsdd is a Cargo workspace with two crates: `vsdd` (the binary) and `vsdd-core`
(the pure library). It requires a recent Rust toolchain (see
[`rust-toolchain.toml`](./rust-toolchain.toml)).

```sh
# Build from a checkout
cargo build

# Install the vsdd binary from source
cargo install --path vsdd
```

The **conformance engine ([mdatron](https://github.com/magnificentlycursed/mdatron))
is a separate binary** that vsdd consumes over a tool-to-tool boundary — vsdd
never links it as a library. Install it alongside vsdd:

```sh
cargo install mdatron --locked   # published on crates.io (0.5.0+)
```

crosslink and mdatron must be present before `vsdd init` deploys a toolkit into a
project. `vsdd init --check` runs a pre-flight environment probe and reports what
is missing before writing anything.

## Command surface

vsdd ships as a single `vsdd` binary that dispatches subcommands, matching the
`cargo` / `rustup` / `git` convention.

### Built

| Command | What it does |
|---|---|
| `vsdd init` | Drift-aware toolkit deployment into a crosslink-initialized repo. Deploys managed files and registry data, records per-file state in a manifest, and refuses to clobber operator edits — on a conflict it prompts to keep the edit, accept the new template, or show a diff. Flags: `--check` (pre-flight probe only), `--dry-run`, `--update` (upgrade unedited files whose template changed), `--force`, and `--ci-mode` / `--no-prompt` (non-interactive). |
| `vsdd status` | Answers "what phase are we in?" from the phase-state artifact and the tracker, in one command. Three forms from one computation: a human terminal form (default), a machine form (`--machine`, JSON), and a one-line statusline segment (`--statusline`, with `--repo-set <path>` for a multi-repo display). It also reports process-integrity drift unprompted (phase-pointer/milestone mismatch, unrouted findings, dangling installed-artifact references) and stays honest under a broken state — every surface still speaks. |
| `vsdd gate` | The routing-before-fix guardrail (the first vertical slice): blocks when a finding closed by a fix carries no filed routing. Exit codes are `0` pass, `1` blocked, `2` unverifiable (fail-closed). `--machine` renders the verdict as JSON. |

### Planned

These are specified in the **Verifiable conformance and efficiency** behavioral contract of
[`.design/agent-first-vsdd-toolkit.md`](./.design/agent-first-vsdd-toolkit.md)
(design rationale preserved as the `verifiable-conformance-and-efficiency` knowledge page)
and are **not yet built**:

| Command | What it will do |
|---|---|
| `vsdd gate conformance` | The conformance verifier: read the harness-produced run record (the transcript the checked agent cannot author), recompute the composition the dispatch *should* have loaded, and gate on `was ⊇ should` — proving the methodology's disciplines actually fired for each dispatched agent. |
| `vsdd dispatch` | The golden-path composer: assemble a dispatch's composed context and inject it by construction, so the correct, methodology-conformant dispatch is the easy one to run. |
| `vsdd insight` | The efficiency insight engine: a reader over the run records that surfaces right-sizing (model/effort provisioning, prompt-cache reuse, targeted reads), with a provenance tag on every figure — recorded, measured, judgment, or could-not-check. |

Verification of markdown artifacts is **not** a vsdd command — that is mdatron's
job (`mdatron verify`). Earlier telemetry-style commands (`vsdd observe`,
`vsdd cost`) and a documentation MCP (Model Context Protocol) server
(`vsdd mcp-serve`) are retired or cut: the
cost layer is re-scoped to a build-time static price plus the records-based
`vsdd insight` reader, and knowledge surfaces through crosslink's existing
viewers rather than a vsdd-built one.

## How the methodology works at a glance

**The phase pipeline.** VSDD runs a project through ten canonical phases. A phase
closes only when its exit gate passes, and "what phase are we in?" has exactly
one answer any cold agent can derive from the repo and tracker.

| Phase | Name | In one line |
|---|---|---|
| 1a | Behavioral Specification | Author the milestone's behavioral contracts |
| 1b | Verification Architecture | Author the test strategy and purity boundaries |
| 1c | Spec Review Gate (Decomposition) | Decompose the spec into milestones; validate entry |
| 2a | Test Suite Generation (Red Gate) | Author failing tests against the spec |
| 2b | Minimal Implementation | Write the minimal code that turns the Red Gate green |
| 2c | Refactor | Re-shape the implementation with the tests staying green |
| 3 | Adversarial Refinement (the VDD Roast) | Cold-session multi-domain review; classify + route |
| 4 | Feedback Integration Loop | Route findings to the earliest phase that fixes them |
| 5 | Formal Hardening | Mutation, fuzz, property, and security hardening |
| 6 | Convergence (the Exit Signal) | Terminal attestation that spec, tests, code, and verification agree |

**Enforcement-spine-first vertical slices.** Rather than building the whole
apparatus layer by layer, vsdd is decomposed into vertical slices, each carrying
its own guardrail so the discipline it enforces is live as soon as the slice
lands. Guardrails are named by their honest **enforcement grade** — *detection*
(reads and reports), *friction* (a bypassable local block that records the
bypass), or *CI-backed block* (a server-side check that cannot be skipped) — and
a control is never described at a stronger grade than it has.

**Domains and composition.** Reviews are performed by named review *domains*
(Solution Owner, Software Engineer, Quality Engineer, Security, and the rest —
sixteen role domains plus two meta-domains). Given the phase, the project's
declared surfaces, and the review config, the active domains and their dispatch
shape are computed by a deterministic function: identical inputs produce
identical compositions. An author-side domain and its cold-reader validator
never share a reviewer session.

**The conformance subsystem.** The center of gravity is *verifiable
conformance*: proving from the run record that the harness's own disciplines
fired — that each dispatched agent loaded its composed context and ran
right-sized. The guiding principle is that **availability is not activation**:
authoring a discipline as a file nobody loads is not the same as running it, and
the subsystem mechanizes that check rather than trusting it.

## Where the spec lives

- **[`.design/agent-first-vsdd-toolkit.md`](./.design/agent-first-vsdd-toolkit.md)**
  is the authoritative, live specification — the contract this toolkit ships
  against. Its behavioral contracts, requirements, and acceptance criteria are
  the source of truth; when this README and the contract disagree, the contract
  wins.
- The **Verifiable conformance and efficiency** behavioral contract in
  [`.design/agent-first-vsdd-toolkit.md`](./.design/agent-first-vsdd-toolkit.md)
  governs the conformance-and-efficiency subsystem (the planned commands above);
  its design rationale is preserved as the `verifiable-conformance-and-efficiency`
  knowledge page.
- All design work lives in [`.design/`](./.design/), authored through the
  `/design` skill and the crosslink pipeline. The root `DESIGN-*.md` documents
  (including `DESIGN-VERIFICATION.md`) were **removed** under #826 — superseded
  by the contract above; git history is the archive.

## Continuous integration

Every pull request runs three gates:

- **`mdatron verify`** — corpus conformance over the methodology's markdown
  estate.
- **vsdd-cli test** — the toolkit's own test suite and clippy.
- **routing gate** — the routing-before-fix guardrail (`vsdd gate`) against the
  live tracker. Promoting it to a hard block is the operator's to arrange via
  branch-protection settings (the CI-backed-block grade).

A local pre-commit hook ([`.githooks/pre-commit`](./.githooks/pre-commit),
enabled per clone with `git config core.hooksPath .githooks`) runs
`mdatron verify` on staged markdown, schema, and pattern files, and fails closed
if mdatron is not installed.

## Credits

The VSDD and VDD methodologies, crosslink, and the Thermite harness are authored
by **[dollspace](https://github.com/dollspace-gay)**. This repository is an
independent Rust implementation of that methodology; the methodology itself is
dollspace's.

- [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00)
  — the canonical methodology.
- [VDD whitepaper](https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25)
  — the predecessor that introduced the adversarial-review discipline VSDD
  extends.
- [crosslink](https://github.com/dollspace-gay/crosslink) — the tracker and
  orchestration layer vsdd composes against.

## License

MIT. See [`Cargo.toml`](./Cargo.toml).

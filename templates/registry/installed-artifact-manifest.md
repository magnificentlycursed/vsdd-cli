---
schema_class: installed-artifact-manifest
schema_version: 0.3.1
status: draft-proposal
reference_surfaces:
  - {id: project-settings, path: .claude/settings.json, scope: repo}
  - {id: project-settings-local, path: .claude/settings.local.json, scope: repo}
  - {id: user-settings, path: ~/.claude/settings.json, scope: host}
  - {id: plugin-listing, path: "the settings enabledPlugins map plus the user plugin install directory (~/.claude/plugins)", scope: repo-or-host}
  - {id: project-server-config, path: .mcp.json, scope: repo}
  - {id: user-server-config, path: ~/.claude.json mcp section, scope: host}
  - {id: command-listing, path: .claude/commands/, scope: repo}
  - {id: statusline-command-path, path: settings statusLine entry, scope: repo-or-host}
  - {id: git-config, path: .git/config core.hooksPath, scope: per-clone}
entries:
  - id: settings-hook-wiring
    path: .claude/settings.json
    class: hook-wiring
    source: crosslink-init
    lifetime: tracked-wiring
    referenced_by: []
    pairs_with: [hook-session-start, hook-work-check, hook-post-edit-check, hook-prompt-guard, hook-pre-web-check, hook-heartbeat]
    resolution: exists
    fail_mode: fail-closed
    note: rewired fail-closed 2026-07-20 (operator ruling, vsdd-cli #658); a future crosslink init settings merge may clobber this — dollspace-gay/crosslink#15 — and this manifest's check is the catcher
  - {id: hook-session-start, path: .claude/hooks/session-start.py, class: hook-payload, source: crosslink-init, lifetime: per-clone-payload, referenced_by: [project-settings], pairs_with: [settings-hook-wiring], resolution: exists-and-referenced, fail_mode: fail-closed}
  - {id: hook-work-check, path: .claude/hooks/work-check.py, class: hook-payload, source: crosslink-init, lifetime: per-clone-payload, referenced_by: [project-settings], pairs_with: [settings-hook-wiring], resolution: exists-and-referenced, fail_mode: fail-closed}
  - {id: hook-post-edit-check, path: .claude/hooks/post-edit-check.py, class: hook-payload, source: crosslink-init, lifetime: per-clone-payload, referenced_by: [project-settings], pairs_with: [settings-hook-wiring], resolution: exists-and-referenced, fail_mode: fail-closed}
  - {id: hook-prompt-guard, path: .claude/hooks/prompt-guard.py, class: hook-payload, source: crosslink-init, lifetime: per-clone-payload, referenced_by: [project-settings], pairs_with: [settings-hook-wiring], resolution: exists-and-referenced, fail_mode: fail-closed}
  - {id: hook-pre-web-check, path: .claude/hooks/pre-web-check.py, class: hook-payload, source: crosslink-init, lifetime: per-clone-payload, referenced_by: [project-settings], pairs_with: [settings-hook-wiring], resolution: exists-and-referenced, fail_mode: fail-closed}
  - {id: hook-heartbeat, path: .claude/hooks/heartbeat.py, class: hook-payload, source: crosslink-init, lifetime: per-clone-payload, referenced_by: [project-settings], pairs_with: [settings-hook-wiring], resolution: exists-and-referenced, fail_mode: fail-closed}
  - id: hook-crosslink-config
    path: .claude/hooks/crosslink_config.py
    class: hook-support
    source: crosslink-init
    lifetime: per-clone-payload
    referenced_by: []
    pairs_with: []
    resolution: exists
    fail_mode: undefined
    note: imported by hook payloads, not wired directly
  - id: mcp-wiring
    path: .mcp.json
    class: server-wiring
    source: crosslink-init
    lifetime: tracked-wiring
    referenced_by: []
    pairs_with: [mcp-agent-prompt, mcp-knowledge, mcp-safe-fetch]
    resolution: exists
    fail_mode: undefined
    note: Claude Code reports a failed server load, but silent non-load in headless runs is the named degradation the preflight data binds a check to
  - {id: mcp-agent-prompt, path: .claude/mcp/agent-prompt-server.py, class: server-payload, source: crosslink-init, lifetime: per-clone-payload, referenced_by: [project-server-config], pairs_with: [mcp-wiring, mcp-enablement], resolution: exists-and-referenced, fail_mode: undefined}
  - {id: mcp-knowledge, path: .claude/mcp/knowledge-server.py, class: server-payload, source: crosslink-init, lifetime: per-clone-payload, referenced_by: [project-server-config], pairs_with: [mcp-wiring, mcp-enablement], resolution: exists-and-referenced, fail_mode: undefined}
  - {id: mcp-safe-fetch, path: .claude/mcp/safe-fetch-server.py, class: server-payload, source: crosslink-init, lifetime: per-clone-payload, referenced_by: [project-server-config], pairs_with: [mcp-wiring, mcp-enablement], resolution: exists-and-referenced, fail_mode: undefined}
  - id: mcp-enablement
    path: .claude/settings.local.json
    class: server-enablement
    source: operator
    lifetime: per-clone-wiring
    referenced_by: []
    pairs_with: [mcp-agent-prompt, mcp-knowledge, mcp-safe-fetch]
    resolution: exists
    fail_mode: undefined
    note: per-developer overlay enabling the three project servers
  - id: commands-crosslink
    path: .claude/commands/
    class: command-listing
    source: crosslink-init
    lifetime: per-clone-payload
    referenced_by: [command-listing]
    pairs_with: []
    resolution: exists
    fail_mode: undefined
    note: "crosslink-deployed members: audit check commit crosslink-guide design dev-release featree feature kickoff maintain preflight qa review workflow"
  - id: plugin-set
    path: "user settings enabledPlugins — rust-analyzer-lsp@claude-plugins-official 1.0.0, user scope"
    class: plugin-listing
    source: operator
    lifetime: host-wiring
    referenced_by: [plugin-listing]
    pairs_with: []
    resolution: exists
    fail_mode: undefined
    note: "the wired plugin set the session-substrate check verifies present — a wired-but-absent plugin is exactly this entry's catch (vsdd-cli #686); enablement lives at user scope — host-wiring, the axis value added for it (vsdd-cli #703): every clone on this host inherits it, a fresh host diverges silently, and the check is the compensating control"
  - id: commands-vsdd
    path: .claude/commands/vsdd-*.md
    class: command-listing
    source: vsdd-source
    lifetime: tracked-payload
    referenced_by: [command-listing]
    pairs_with: []
    resolution: exists
    fail_mode: undefined
    note: "this repo is the source of these artifacts — tracked by the .gitignore carve-outs (operator ruling 2026-07-20); 28 members, 18 domain prompts + 10 phase primers (counts verified against disk 2026-07-21, vsdd-cli #684)"
  - {id: chassis-hook-config, path: .crosslink/hook-config.json, class: chassis-config, source: crosslink-init, lifetime: tracked-wiring, referenced_by: [], pairs_with: [], resolution: exists, fail_mode: fail-closed}
  - {id: chassis-rules, path: .crosslink/rules/, class: rules, source: crosslink-init, lifetime: tracked-payload, referenced_by: [], pairs_with: [], resolution: exists, fail_mode: undefined}
  - id: project-rules
    path: .crosslink/rules/project.md
    class: rules
    source: operator
    lifetime: tracked-payload
    referenced_by: []
    pairs_with: []
    resolution: exists
    fail_mode: undefined
    note: the crosslink-injected conduct guards, operator-approved 2026-07-20
  - id: githook-wiring
    path: .git/config core.hooksPath=.githooks
    class: git-hook-wiring
    source: operator
    lifetime: per-clone-wiring
    referenced_by: [git-config]
    pairs_with: [githook-pre-commit]
    resolution: exists
    fail_mode: fail-open-guarded
    note: git silently runs no hooks when hooksPath is unset in a fresh clone — git's own semantics, not repairable in-repo; the session-substrate check over this entry is the compensating control
  - id: githook-pre-commit
    path: .githooks/pre-commit
    class: git-hook-payload
    source: operator
    lifetime: tracked-payload
    referenced_by: [git-config]
    pairs_with: [githook-wiring]
    resolution: exists-and-referenced
    fail_mode: fail-closed
    note: blocks the commit when mdatron is absent (operator ruling 2026-07-20, vsdd-cli #658)
  - {id: identity-agent, path: .crosslink/agent.json, class: identity-state, source: chassis-runtime, lifetime: per-clone-payload, referenced_by: [], pairs_with: [identity-keys], resolution: exists, fail_mode: undefined}
  - id: identity-keys
    path: .crosslink/keys/
    class: identity-state
    source: chassis-runtime
    lifetime: per-clone-payload
    referenced_by: []
    pairs_with: [identity-agent]
    resolution: exists
    fail_mode: undefined
    note: agent keypair directory referenced by agent.json's ssh_key_path; path-only entry — no key material, comments, or fingerprints belong in this manifest (vsdd-cli #662)
  - id: identity-driver-key
    path: .crosslink/driver-key.pub
    class: identity-state
    source: chassis-runtime
    lifetime: per-clone-payload
    referenced_by: []
    pairs_with: []
    resolution: exists
    fail_mode: undefined
    note: lifetime records the intended state; the file is currently git-tracked — remediation parked as vsdd-cli #656 (upstream dollspace-gay/crosslink#28)
  - id: statusline-wiring
    path: no project-level statusLine entry
    class: statusline-wiring
    source: operator
    lifetime: host-wiring
    referenced_by: [statusline-command-path]
    pairs_with: []
    resolution: worded-absence
    fail_mode: undefined
    note: "wiring lives at user level today — host-wiring, reclassified in the #703 axis sweep this entry originally escaped (vsdd-cli #709): every clone on this host inherits it, a fresh host loses it silently; a project-level entry arrives with the Install requirement's offer (Layer 4)"
---

# Installed-artifact manifest — vsdd-cli instance

The installed environment is a closed world (contract: Conformance at action
time, the chassis-affordance closure, ratified 2026-07-20). The frontmatter
above is the versioned data: every artifact the environment expects, each
with its source, its lifetime, its pairing, and its observed fail mode.
The lifetime axis is tracked-or-per-clone crossed with wiring-or-payload
— four values, plus host-wiring for wiring living in host-scoped config
that every clone on the host inherits (added for the plugin surface,
vsdd-cli #703). It is the tracked/per-clone divergence that produced the
estate's 2026-07-20 incident (the audit on the trail of "Spec amendment:
the dispatch posture split (attended/autonomous), from mdatron's kickoff
live fire", vsdd-cli #622). Git hooks carry the inverted pairing:
per-clone wiring in git config over a tracked payload.

Vocabulary gloss (vsdd-cli #715; verification split stated per
vsdd-cli #763): `resolution` states the entry's CLAIM — `exists` (the
artifact is present), `exists-and-referenced` (present and named by its
reference surface), `worded-absence` (a deliberate absence recorded in
words and checked as such). What the substrate check VERIFIES today is
narrower for the middle member: it verifies presence and reports the
referenced-by half inconclusive — never a silent pass — until the
reference-surface check lands with its consumer (vsdd-cli #746). `fail_mode` records observed failure behavior:
`fail-closed` (absence blocks loudly), `fail-open-guarded` (absence
degrades to a no-op with a named compensating control), and `undefined`
(not yet characterized — a fact about the record, not a safety claim).

The user-settings surface carries no local overlay by design of the
vehicle: Claude Code defines settings local overlays at project scope
only (.claude/settings.local.json), so the contract's settings-with-
their-local-overlays expectation is satisfied by project-settings-local
alone — the omission is this stated fact, not an oversight (vsdd-cli
#686). The plugin-listing surface and the plugin-set entry land with the
same ruling: the enabled-plugin map is a reference surface like any
other, and the one enabled plugin is manifest-listed.

The session-substrate check consumes these entries (three-valued,
fail-closed; surfaced in Status per the contract). mdatron validates this
file against `.mdatron/schemas/installed-artifact-manifest.json` — the
schema pair rule: a data set and its schema land together, always.

Declared cross-field constraints (schema description carries them; executed
by the session-substrate check at read, and by mdatron's cross-file family
when it lands — vsdd-cli #661): entry ids unique; every `pairs_with` id
resolves to an entry; every `referenced_by` id resolves to a reference
surface.

Additions — the operator's included — enter by the recorded pair: artifact
plus manifest entry together. An artifact absent from this manifest is
unlisted and flagged; an entry whose artifact is absent is flagged.

Authored under phase-1c data authoring (vsdd-cli #598, set issue #658;
format ruling: markdown-with-frontmatter, operator-adopted 2026-07-20 on
#660). Draft vocabulary under the maturity lifecycle until first publish.

Member adoptions recorded on the set issue do not advance this
artifact's status: the status field advances by the phase-exit
adoption act, then first publish (vsdd-cli #715, executing the #697
item-5 standing disposition at the cold pass's finding).

#!/usr/bin/env python3
"""check-no-letter-clusters — ban letter-only labels in user-facing prose.

Cleanroom Python implementation of vsdd-suite's hook of the same name at
`github.com/magnificentlycursed/guild-portfolio @ 5789ad4 vsdd-suite/hooks/
check-no-letter-clusters.py`. Implemented from spec; not copied.

The discipline: thematic groupings (clusters of findings, verification surfaces,
SO-decision options, multi-path choices) carry a descriptive name as their
primary identifier. Letter labels (A/B/C/D) and Roman-numeral labels
(I/II/III) require a lookup that descriptive names avoid; future readers
land in the audit-trail without the lookup table loaded.

vsdd-cli specialization vs vsdd-suite: Roman-numeral cluster variant
(`Cluster I-VIII`) added per session defect class 1 retrospective
(`review-log/2026-05-28-vsdd-methodology.md` Defect 1) — vsdd-suite's
pattern set is letter-only, missing Roman numerals.

Patterns flagged:
1. `Cluster [A-Z]` (+ optional digit suffix)
2. `Cluster [IVX]+` (Roman numerals; vsdd-cli specialization)
3. `Surface [A-Z]` (+ optional .digit)
4. `Path [A-Z]`
5. `Option [A-Z]`

Allowed (carries meaning at point of use):
- Descriptive names: `Mutation Testing`, `Purity Boundary Audit`
- Established abbreviations: `Dim N`, `Layer N`, `Round N`, `Phase N`,
  `Finding N`, `R8 F1`, domain slugs (`solution-architect`, `red-team`)

Scope: markdown files in DESIGN docs, README, methodology spec, review-log,
domain prompts, supplements, templates.

Out-of-scope:
- Code files (.py, .rs, .toml, .yml) where Surface / Path might be legit identifiers
- Fenced code blocks (preserved historical references)
- Lines starting with `> ` (quoted blocks)

Bypass: `<!-- hook-bypass[check-no-letter-clusters]: <rationale> -->` in first 5
lines of file. Bypasses are themselves findings for the next registry-walk review.

Forward-only: enforces at commit time on new/changed files; pre-existing
audit-trail references in historical files are not migrated by this hook.

Exit codes: 0 if clean, 1 if violations.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

HOOK_ID = "check-no-letter-clusters"

PATTERNS: list[tuple[re.Pattern[str], str]] = [
    (
        re.compile(r"\bCluster [A-Z]\d*\b"),
        "letter-label `Cluster X`. Rename to a descriptive identifier carrying "
        "meaning at point of use.",
    ),
    (
        re.compile(r"\bCluster (?:I{1,3}|IV|V|VI{1,3}|IX|X)\b"),
        "Roman-numeral `Cluster N` (vsdd-cli ext). Rename to a descriptive "
        "identifier; Roman numerals share the lookup-cost problem with letters.",
    ),
    (
        re.compile(r"\bSurface [A-Z](?:\.\d)?\b"),
        "letter-label `Surface X`. Use the Phase 5 surface's descriptive name "
        "(`Mutation Testing`, `Fuzz Testing`, `Proof Execution`, "
        "`Purity Boundary Audit`, `Security Hardening`).",
    ),
    (
        re.compile(r"\bPath [A-Z]\b"),
        "letter-label `Path X` (typically AskUserQuestion options). Rename to "
        "a descriptive identifier naming the substantive choice.",
    ),
    (
        re.compile(r"\bOption [A-Z]\b"),
        "letter-label `Option X`. Rename per the same rule as `Path X`.",
    ),
]

BYPASS_RE = re.compile(rf"<!--\s*hook-bypass\[{HOOK_ID}\]:\s*.+?\s*-->")

IN_SCOPE_NAME_PREFIXES = (
    "DESIGN", "README", "TODO", "PROCESS", "CHANGELOG", "COMPATIBILITY",
    "methodology", "FINDINGS-INDEX",
)
IN_SCOPE_PATH_FRAGMENTS = (
    "review-log/", "manual-tests/", "supplements/",
    ".claude/commands/", "templates/", "hooks/",
)


def in_scope(path: Path) -> bool:
    if path.suffix != ".md":
        return False
    s = str(path)
    # archive/ holds historical / superseded artifacts; .crosslink/ is the
    # crosslink substrate (not vsdd-cli content); neither is scanned.
    if s.startswith("archive/") or "/archive/" in s:
        return False
    if s.startswith(".crosslink/") or "/.crosslink/" in s:
        return False
    if any(frag in s for frag in IN_SCOPE_PATH_FRAGMENTS):
        return True
    name = path.name
    return any(name.startswith(p) for p in IN_SCOPE_NAME_PREFIXES)


def has_bypass(text: str) -> bool:
    head = "\n".join(text.splitlines()[:5])
    return bool(BYPASS_RE.search(head))


def scan_file(path: Path) -> list[str]:
    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return []
    if has_bypass(text):
        return []
    findings: list[str] = []
    in_fence = False
    for lineno, line in enumerate(text.splitlines(), start=1):
        stripped = line.lstrip()
        if stripped.startswith("```"):
            in_fence = not in_fence
            continue
        if in_fence or stripped.startswith("> "):
            continue
        for pat, rationale in PATTERNS:
            m = pat.search(line)
            if m:
                findings.append(f"{path}:{lineno}: `{m.group(0)}` — {rationale}")
    return findings


def main(argv: list[str]) -> int:
    args = argv[1:]
    if args:
        paths = [Path(a) for a in args]
    else:
        paths = sorted(p for p in Path(".").rglob("*.md") if in_scope(p))
    all_findings: list[str] = []
    for p in paths:
        if not in_scope(p):
            continue
        all_findings.extend(scan_file(p))
    if all_findings:
        print(f"{HOOK_ID}: letter-label anti-pattern detected", file=sys.stderr)
        for line in all_findings:
            print(f"  {line}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))

#!/usr/bin/env python3
"""check-document-staleness — flag mechanical staleness signals in markdown.

Cleanroom Python implementation of vsdd-suite's hook of the same name at
`github.com/magnificentlycursed/guild-portfolio @ 5789ad4 vsdd-suite/hooks/
check-document-staleness.py`. Implemented from spec; not copied.

The discipline: post-Round-1 artifact-state drifts from pre-Round-1
artifact-state (impl + spec evolve); forward-facing docs lag behind; Round 2+
review catches the drift minutes-to-days later. The lag-window is operator-
visible cost. This hook catches obvious staleness markers at commit time.

Conservative scope: high-precision patterns, low false-positive rate. Missing
detections covered by domain-prompt amendments at next review round.

Detection patterns:
1. In-flight phrases (`in flight`, `in progress`, `to be authored`,
   `scoped but not built`, `active in PR #N`, `Round N in progress`)
2. Forward-facing claims with stale-triggers (`currently`, `as of`,
   `pending`) — softer signal, advisory only when paired with date/PR/Round

Scope: all markdown files in the repository.

Bypass: `<!-- hook-bypass[check-document-staleness]: <rationale> -->` in first
5 lines. Primarily for files that intentionally quote historical in-flight
state per the forward-only narrative-preservation discipline.

Exit codes: 0 if clean, 1 if violations.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

HOOK_ID = "check-document-staleness"

PATTERNS: list[tuple[re.Pattern[str], str]] = [
    (
        re.compile(r"\bin flight\b", re.IGNORECASE),
        "in-flight phrase. Verify the cited work has not completed post-merge.",
    ),
    (
        re.compile(r"\bin progress\b", re.IGNORECASE),
        "in-progress phrase. Verify the cited work has not completed.",
    ),
    (
        re.compile(r"\bto be authored\b", re.IGNORECASE),
        "to-be-authored phrase. Verify the cited artifact has not been authored "
        "since.",
    ),
    (
        re.compile(r"\bscoped but not built\b", re.IGNORECASE),
        "scoped-but-not-built phrase. Verify the cited surface remains unbuilt.",
    ),
    (
        re.compile(r"\bactive in PR\s*#\d+\b", re.IGNORECASE),
        "active-in-PR phrase. Verify the cited PR remains open.",
    ),
    (
        re.compile(r"\bRound \d+ in progress\b", re.IGNORECASE),
        "Round-in-progress phrase. Verify a later Round has not advanced past "
        "the cited one.",
    ),
    (
        re.compile(r"\bnot yet (?:authored|implemented|built|deployed)\b", re.IGNORECASE),
        "not-yet-X phrase. Verify the cited surface has not since been completed.",
    ),
    (
        re.compile(r"\bTBD\b"),
        "TBD marker. Resolve before commit or convert to a tracked issue + cite "
        "the tracker ID.",
    ),
]

BYPASS_RE = re.compile(rf"<!--\s*hook-bypass\[{HOOK_ID}\]:\s*.+?\s*-->")


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
    return True


def main(argv: list[str]) -> int:
    args = argv[1:]
    if args:
        paths = [Path(a) for a in args]
    else:
        paths = sorted(Path(".").rglob("*.md"))
    all_findings: list[str] = []
    for p in paths:
        if not in_scope(p):
            continue
        all_findings.extend(scan_file(p))
    if all_findings:
        print(f"{HOOK_ID}: staleness markers detected", file=sys.stderr)
        for line in all_findings:
            print(f"  {line}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))

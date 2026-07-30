#!/bin/sh
# vsdd statusline wiring (Layer 3; installed by vsdd init at Layer 4).
#
# One segment line per configured repo, current repo first. The repo
# set is explicit adopter configuration at the registered location —
# never discovered — and the aggregate budget is denominated repo
# count times per_repo_budget_ms, both read from the same file.
#
# The runtime harness's session JSON arrives on stdin and passes through
# UNCONSUMED: the segment reads none of it (the stdin-count conduct).
#
# This file is a managed artifact: vsdd init refuses to overwrite a
# hand-modified copy (the drift-refusal conduct).
#
# Composing with an existing statusline? Do not wrap this script — add
# exactly this one line to your own statusline command; segments
# compose by concatenation:
#
#     vsdd status --statusline
#
# (Append `--repo-set "$HOME/.config/vsdd/statusline.yaml"` to that
# line if you use the multi-repo display.)

CONFIG="${VSDD_STATUSLINE_CONFIG:-$HOME/.config/vsdd/statusline.yaml}"

# An EXPLICITLY named config that does not exist is loud, never a
# silent single-repo degrade (the never-silent principle); the default
# location's absence is the lawful single-repo shape.
if [ -n "$VSDD_STATUSLINE_CONFIG" ] && [ ! -f "$VSDD_STATUSLINE_CONFIG" ]; then
    echo "vsdd statusline: the configured repo set does not exist: $VSDD_STATUSLINE_CONFIG" >&2
    exec vsdd status --statusline
fi

if [ -f "$CONFIG" ]; then
    exec vsdd status --statusline --repo-set "$CONFIG"
fi
exec vsdd status --statusline

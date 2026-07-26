#!/bin/sh
# vsdd statusline wiring (Layer 3; installed by vsdd init at Layer 4).
#
# One segment line per configured repo, current repo first. The repo
# set is explicit adopter configuration at the registered location —
# never discovered — and the aggregate budget is denominated repo
# count times per_repo_budget_ms, both read from the same file.
#
# The substrate's session JSON arrives on stdin and passes through
# UNCONSUMED: the segment reads none of it (the stdin-count conduct).
#
# This file is a managed artifact: vsdd init refuses to overwrite a
# hand-modified copy (the drift-refusal conduct). Composing with an
# existing statusline? Do not wrap this script — add the invocation
# line below to your own command; segments compose by concatenation.

CONFIG="${VSDD_STATUSLINE_CONFIG:-$HOME/.config/vsdd/statusline.yaml}"

if [ -f "$CONFIG" ]; then
    exec vsdd status --statusline --repo-set "$CONFIG"
fi
exec vsdd status --statusline

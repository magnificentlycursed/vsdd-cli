# Layer 3 manual tests — the three renderings and the wiring script

Adopted from the entry draft (operator adoption 2026-07-25, recorded on
vsdd-cli #772); the operator may amend at the run. Each item is a
keyboard act with an observed outcome; run against the real estate,
not fixtures. Record the run as a result comment on the layer's trail.

1. **Live segment.** Wire `vsdd status --statusline` into this host's
   real statusLine (via the wiring script) and glance-read it during a
   working session. Pass: the four fields read at a glance; nothing
   requires a second look to parse; the milestone count is the number
   you expect.
2. **Real degraded rendering.** Take the tracker offline (rename the
   `.crosslink` store temporarily) and watch the segment refresh.
   Pass: the marker word appears as a plain word; `vsdd status` names
   the kind and its next step in the words the data set carries;
   nothing panics; restore and watch it clear.
3. **Color in the real terminal.** View the human form in the usual
   terminal theme, then in a monochrome view (`NO_COLOR` or a pipe).
   Pass: nothing meaningful disappears with color.
4. **The wiring script on the real estate.** Configure the repo set
   with vsdd-cli and mdatron; run the script. Pass: two lines, this
   repo first, each line naming its repo; the aggregate feels
   instant — within the count-times-budget denomination.
5. **The composition instruction by hand.** Against an existing
   statusline wrapper, follow the printed non-wrapper instruction
   verbatim. Pass: a working composed segment with no doc lookup
   between the refusal and the composition.
6. **The #713 re-wire.** Wire the segment through the script on this
   host (the user-level statusLine entry that vanished 2026-07-20),
   then rewrite the runtime-harness config once (any unrelated settings
   change). Pass: the segment survives the rewrite.

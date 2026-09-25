<!-- # crosslink:custom — tracked vsdd-cli policy rules; crosslink ships no bundled rules since upstream 62e637ab7, so every tracked rule is custom by definition (vsdd-cli PR #42) -->
## Crosslink (Available)

Crosslink issue tracking is available but not required. Use it when it helps.

```bash
crosslink quick "title" -p <priority> -l <label>   # Create + label + work
crosslink issue list -s open                              # See open issues
crosslink issue close <id>                                # Close issue
crosslink session work <id>                         # Mark focus
crosslink session end --notes "..."                 # Save handoff
```

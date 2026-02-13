# Resume: gitterm-v2

**Last checkpoint:** 2026-02-12 evening

## You Were Just Working On
Implemented the **Attention System** (Phase 2) and tested it interactively. All features working — ready to commit.

**Just did:** Fixed Ctrl+1-9 and Ctrl+` writing stray characters into the terminal by adding modifier tracking and write suppression in the BackendCall handler.

**Immediate next step:** Commit all changes (attention system, Noop binding, write suppression, color dedup, CLAUDECODE env fix). Then move on to Phase 3 (Tab Overflow) or Phase 4 (Console Panel) from `design/WORKSPACE_DESIGN.md`.

## Completed This Session
- **Attention system (Phase 2):** Full implementation — `needs_attention` field on TabState, detection via terminal title `*` prefix, auto-clear on user input
- **Pulsing animation:** 500ms subscription timer, `attention_pulse_bright` toggle on App, conditional subscription (only active when attention exists)
- **Tab bar indicators:** Pulsing amber `●` icon, stripped `*` prefix from title, amber-tinted background with border for attention tabs
- **Workspace bar indicators:** Pulsing amber dot for workspaces with attention, amber badge showing attention count, red `!` badge for console errors
- **Spine indicators:** Larger dots (6x6) for workspaces with attention/error, pulsing amber or red color
- **Ctrl+` shortcut:** Round-robin jump to next attention tab across all workspaces, with slide animation
- **iced_term `Noop` binding:** Added new `BindingAction::Noop` variant to iced_term_fork to suppress terminal character output for app shortcuts
- **Write suppression:** Modifier tracking via `ModifiersChanged` events + BackendCall filtering to prevent Ctrl+1-9 and Ctrl+` from typing into terminal
- **Workspace color dedup:** `WorkspaceColor::next_available()` picks first unused color instead of `from_index(len)`
- **CLAUDECODE env fix:** Clear `CLAUDECODE` and `CLAUDE_CODE_ENTRYPOINT` env vars in PTY setup so Claude Code can launch inside GitTerm terminals

## Key Files
- `src/main.rs` — All app changes (attention fields, events, detection, UI indicators, write suppression)
- `../iced_term_fork/src/bindings.rs` — Added `Noop` variant to `BindingAction` enum
- `../iced_term_fork/src/view.rs` — Added `Noop` handler (returns None, no character written)
- `design/WORKSPACE_DESIGN.md` — Full design spec for all phases

## Blockers/Issues
- None — all features working and tested
- Pre-existing warnings only: `tab_id` dead code in log_server.rs, `_viewport` in iced_term view.rs

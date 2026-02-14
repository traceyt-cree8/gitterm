# Resume: gitterm-v2

**Last checkpoint:** 2026-02-14 18:10

## You Were Just Working On
Bug fixes and UI polish — collapsible sidebar and git status auto-refresh.

**Just did:** Pushed both commits to origin/master (`78f6c78` collapsible sidebar, `6c64e87` git refresh fix).

**Immediate next step:** Continue with remaining workspace features from `design/WORKSPACE_DESIGN.md` or address new UI polish requests.

## Completed This Session
- Implemented Claude Sidebar Tree View — collapsible sections showing Claude Code config (skills, plugins, MCP servers, hooks, settings) with click-to-view-file
- Fixed markdown WebView overlay covering console — removed inline WebView, switched to Iced-native renderer
- Added native markdown table rendering — header rows, alternating row backgrounds, cell borders
- Added inline formatting via `iced::advanced::text::Span` + `Rich::with_spans()` — bold, italic, inline code (with highlight), links
- Fixed text wrapping — switched from Row-based to rich_text Spans for proper word-level wrapping
- Added collapsible sidebar with Cmd+B toggle — collapse chevron in tab bar, icon rail when collapsed with mode buttons + expand chevron
- Fixed git status auto-refresh — removed condition that only polled when viewing diffs, now polls every 5s regardless
- Committed 4 times: `bea5bf4`, `71582e6`, `78f6c78`, `6c64e87` — all pushed to origin/master

## Key Files
- `src/main.rs` — All app logic (~5900 lines). Key sections:
  - `Event::Tick` handler (line ~2573) — git status polling
  - `view_sidebar_rail()` — collapsed sidebar icon rail
  - `view_claude_sidebar()` — Claude config tree view
  - `parse_inline_markdown()` / `view_markdown_table()` — native markdown rendering
- `src/markdown.rs` — HTML template for "View in Browser"
- `src/webview.rs` — WebView module (inline use removed, "View in Browser" still works)

## Blockers/Issues
- Mermaid diagrams no longer render inline (acceptable — "View in Browser" works)
- Untracked files: `test-refresh.txt` (test artifact, can delete), design reference files in `design/`

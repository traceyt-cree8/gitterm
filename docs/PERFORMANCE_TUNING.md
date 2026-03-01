# Performance and Rendering Tuning

This document captures the recent performance/rendering work in GitTerm and the knobs you can tune after real usage.

## What Changed

### 1) File viewer rendering pipeline

- Markdown (`.md`) now prefers inline WebView rendering (including Mermaid) instead of the old plain fallback.
- HTML (`.html`) renders inline in the WebView instead of showing raw HTML text.
- Excalidraw (`.excalidraw`) renders inline via a dedicated viewer template.
- Excalidraw viewer CSS/UI was tightened to remove extra chrome/padding and avoid top spacing artifacts.

Code:
- `src/main.rs`
- `src/markdown.rs`
- `src/excalidraw.rs`
- `assets/excalidraw/viewer.html`

### 2) Loading-state UX cleanup

- Added delayed loading indicators so fast operations do not flash spinners/messages.
- Applied to file loads, diff loads, and syntax-first render flow.
- File view now waits for the first syntax-highlight batch before showing text (avoids plain->colored flash).

Code:
- `src/main.rs` (`LOADING_INDICATOR_DELAY_MS` and file/diff view logic)

### 3) Syntax highlighting overhaul (Syntect)

- Replaced ad-hoc line coloring with Syntect-based highlighting.
- Added extension/token/name fallbacks so `ts/tsx/mts/cts/js/jsx/mjs/cjs` map correctly.
- Added startup warmup to reduce first-open latency.
- Added async syntax generation to keep UI responsive.
- Added lazy syntax expansion on scroll:
  - initial small batch
  - additional batches requested as you scroll into unprocessed regions
- Added syntax-highlight cache and eviction.

Code:
- `src/main.rs`
- `src/services.rs`

### 4) Large-file and render safeguards

- Hard caps added for:
  - max rendered lines in file view
  - max lines/bytes/segments for syntax highlighting
  - max rendered diff lines and diff syntax work
- Large text files use preview mode (partial read + explicit notice).

Code:
- `src/main.rs`
- `src/services.rs`

### 5) Git panel refresh behavior

- Git polling moved to adaptive cadence using change streaks and whether Git is in focus.
- Loading banner in Git list is now shown only for initial fetch, not every background poll (reduces flashing).
- Uses hash-based change detection to avoid unnecessary fast polling.

Code:
- `src/main.rs`

### 6) File-load de-dupe and stale reload reduction

- Suppresses duplicate file-load requests (same path, very short interval).
- Skips reloading unchanged file if signature (mtime + len) matches.

Code:
- `src/main.rs`

### 7) Log server control

- Log server default changed to off in config.
- Added runtime toggle in app UI and menu.

Code:
- `src/main.rs`

## Runtime Profiling

Run with:

```bash
GITTERM_PERF=1 cargo run --features excalidraw
```

Useful log groups:

- `git_status`, `git_poll`: git cadence/latency
- `file_tree`: directory listing cost
- `file_load`: file open cost and mode chosen
- `syntax_load`: syntax batch timing/size
- `syntect ...`: syntax engine details/cache hits
- `diff ...`: diff generation timing
- `webview ...`: inline webview create/reuse + payload size
- `mem ...`: rough memory posture by category

## Tuning Knobs

All of these are in:
- `src/main.rs`

### Syntax and file-view knobs

- `FILE_SYNTAX_INITIAL_LINES` (current: `120`)
  - First syntax batch before file is shown.
  - Increase for more initially colored lines; decrease for faster first paint.

- `FILE_SYNTAX_SCROLL_PREFETCH_LINES` (current: `220`)
  - Extra lines requested when scrolling.
  - Increase for smoother scrolling at cost of more background CPU.

- `MAX_FILE_VIEW_RENDER_LINES` (current: `1200`)
  - Max lines rendered without syntax data.

- `MAX_FILE_VIEW_RENDER_LINES_WITH_SYNTAX` (current: `1200`)
  - Max lines rendered when syntax highlighting is active.

- `MAX_SYNTAX_HIGHLIGHT_LINES` (current: `1200`)
- `MAX_SYNTAX_HIGHLIGHT_BYTES` (current: `96 * 1024`)
- `MAX_SYNTAX_HIGHLIGHT_SEGMENTS` (current: `8000`)
  - Upper bounds for syntax work to prevent UI stalls.

- `SYNTAX_HIGHLIGHT_CACHE_MAX_ENTRIES` (current: `64`)
  - LRU size for file syntax cache.

### Diff knobs

- `MAX_DIFF_VIEW_RENDER_LINES` (current: `1200`)
- `MAX_DIFF_SYNTAX_HIGHLIGHT_LINES` (current: `900`)
- `MAX_DIFF_SYNTAX_HIGHLIGHT_BYTES` (current: `768 * 1024`)
- `MAX_DIFF_SYNTAX_SEGMENTS` (current: `9000`)
- `DIFF_SYNTAX_CACHE_MAX_ENTRIES` (current: `64`)

### Large-file knobs

- `MAX_FULL_TEXT_LOAD_BYTES` (current: `1_000_000`)
  - Above this, load preview instead of full file text.

- `LARGE_TEXT_PREVIEW_BYTES` (current: `256 * 1024`)
- `LARGE_TEXT_PREVIEW_LINES` (current: `2000`)
  - Preview cutoffs for large files.

- `MAX_INLINE_WEBVIEW_BYTES` (current: `1_500_000`)
  - Max inline size for markdown/html/excalidraw web previews.

### Loading message knob

- `LOADING_INDICATOR_DELAY_MS` (current: `120`)
  - Delay before “Loading...” / “Highlighting syntax...” is shown.
  - Helps avoid flash for fast operations.

### Git polling knobs

- `GIT_POLL_FAST_INTERVAL_MS` (current: `5000`)
- `GIT_POLL_MEDIUM_INTERVAL_MS` (current: `10000`)
- `GIT_POLL_SLOW_INTERVAL_MS` (current: `15000`)
- `GIT_POLL_IDLE_INTERVAL_MS` (current: `30000`)
- `GIT_POLL_NON_REPO_INTERVAL_MS` (current: `20000`)
  - Adaptive cadence used by `next_git_poll_interval_ms(...)`.

## Suggested Tuning Order (after real usage)

1. Tune `FILE_SYNTAX_INITIAL_LINES` and `FILE_SYNTAX_SCROLL_PREFETCH_LINES`.
2. If memory pressure is high, reduce cache sizes and render line caps.
3. If Git panel still feels noisy, increase medium/slow/idle poll intervals.
4. If users complain about loading flashes, increase `LOADING_INDICATOR_DELAY_MS` slightly.

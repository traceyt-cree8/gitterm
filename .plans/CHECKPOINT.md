# Checkpoint: gitterm-v2

**Last checkpoint:** Friday, March 6, 2026 at 06:36 AM PST

## You Were Just Working On
Performance optimization of git status polling — reduced from 3 sequential git process spawns to 1 by switching to `--porcelain=v2 --branch` format and adding `--no-optional-locks`.

**Just did:** Rewrote `collect_git_status` in `src/services.rs` to use a single `git status --porcelain=v2 --branch --no-renames --no-optional-locks` command instead of 3 separate git commands.
**Immediate next step:** Test the git status performance improvement on the `producer-fresh` repo — verify times drop from 200-400ms to under 100ms, and confirm branch name + file status parsing works correctly with the v2 porcelain format.

## Completed This Session
- Removed dot-file/dot-folder filtering from file explorer — files starting with `.` now always shown (`src/main.rs` and `src/services.rs`)
- Removed the "Show .*" / "Hide .*" toggle button from file explorer sidebar UI (`src/main.rs`)
- Optimized `collect_git_status` in `src/services.rs`: consolidated 3 git process spawns into 1 using `--porcelain=v2 --branch`
- Added `--no-optional-locks` flag to reduce lock contention with concurrent git operations (e.g., Claude Code)
- Made repo root discovery (`rev-parse --show-toplevel`) conditional — only runs when `.git` dir not found at repo_path (self-heal case)
- Both changes compile cleanly

## Active Plan
No active plan file found. Work was driven by user-reported issues: missing dot folders in file explorer and slow git status polling (200-400ms per poll on producer-fresh repo).

## Key Files
- `src/services.rs` — Contains `collect_git_status()` and `collect_file_tree()` — the two functions modified this session
- `src/main.rs` — Main app file (~3300 lines), removed dot-file filter in `fetch_file_tree()` and removed toggle button UI
- `src/config.rs` — Config persistence, still has `show_hidden` field (now unused but harmless for backward compat)

## Blockers/Issues
- The `show_hidden` field/plumbing still exists in config, state, and events (e.g., `Event::ToggleHidden`, `self.show_hidden`) — it's dead code now but not cleaned up. Low priority.
- Git status performance improvement is untested on the actual slow repo (`producer-fresh`) — need to verify the v2 porcelain format parsing handles all edge cases (renames, unmerged files, detached HEAD).

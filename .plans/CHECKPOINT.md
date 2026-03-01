# Checkpoint: gitterm-v2

**Last checkpoint:** 2026-03-01 09:20 AM PST

## You Were Just Working On
Building an agent-capture system for GitTerm — a pi extension that silently captures structured metadata on every git commit, as the foundation for Entire.io-style agent-aware version control features.

**Just did:** Committed docs (Entire.io research + performance tuning) and set up pi-config repo under source control
**Immediate next step:** Design the experience layer for viewing captured agent session data — either in GitTerm's UI or a separate tool. The capture pipeline is complete and running; now decide how to surface the data.

## Completed This Session
- Researched Entire.io (GitHub wrapper with Trails, Checkpoints, agent attribution) and documented findings in `docs/ENTIRE-IO-RESEARCH.md`
- Identified GitTerm's competitive advantage: it owns the terminal where agents run, enabling real-time capture vs post-hoc reconstruction
- Designed a two-layer data shape: Layer 1 (mechanical, instant) captured at commit time; Layer 2 (LLM-generated insights) deferred for later
- Built `agent-capture.ts` pi extension that hooks `tool_execution_end`, detects git commits, and writes structured JSONL
- Iterated on the capture shape across multiple test cycles:
  - v1: basic commit/branch/repo/models/files/tokens/cost/duration
  - v2: split duration into `wall_clock_seconds` + `agent_active_seconds` (tracks turn durations)
  - v3: added `user_prompts` (count + actual text) and `errors` (count + recovered)
  - v4: added `billing` field ("sub" | "api") by discovering `modelRegistry.isUsingOAuth()`, renamed `cost_usd` to `estimated_api_cost_usd`
- Decided capture range tracks between commits (not per-prompt), so multi-prompt work toward a single commit is captured together
- Created `~/GitRepo/pi-config/` repo to put all pi customizations under source control
- Built `install.sh` that symlinks extensions, prompts, themes, skills, settings, presets, and memory from the repo into `~/.pi/agent/`
- Verified symlinks work — extensions load and fire through them
- Committed all pi config: 7 extensions, 4 prompts, 6 themes, settings, presets, memory
- Reviewed project-local skills (port-ui, coordinator) — decided they stay project-local, not worth generalizing yet

## Active Plan
No active plan file. Working towards an agent-aware version control system for GitTerm:
1. ✅ **Capture pipeline** — complete, running silently on every commit
2. ⬜ **Experience layer** — how to view/query the captured data (GitTerm sidebar? separate tool? CLI?)
3. ⬜ **Layer 2 insights** — LLM-generated summaries, intent extraction, shareable artifacts
4. ⬜ **Trails** — enhanced issue/PR workflow (future phase)
5. ⬜ **Review workflow** — inline reviews, approvals (future phase)

## Key Files
- `~/.pi/agent/extensions/agent-capture.ts` → `~/GitRepo/pi-config/extensions/agent-capture.ts` — The capture extension, hooks git commits and writes JSONL
- `~/.config/gitterm/captures/{repo}/log.jsonl` — Where captured data accumulates (per-repo)
- `docs/ENTIRE-IO-RESEARCH.md` — Full research doc with feature mapping and competitive analysis
- `~/GitRepo/pi-config/install.sh` — Pi config installer (symlinks everything)
- `~/GitRepo/pi-config/extensions/` — All 7 global pi extensions under source control

## Blockers/Issues
- File tracking in captures only records pi tool usage (read/edit/write), not files modified via bash commands — acceptable since git diff covers that
- Capture uses `ctx.cwd` for repo slug, so commits in other repos via `cd` in bash get filed under the wrong repo — edge case, acceptable for now
- Layer 2 (LLM-generated insights) not yet designed — needs decisions on when/how to generate summaries and whether to use local models or the active agent

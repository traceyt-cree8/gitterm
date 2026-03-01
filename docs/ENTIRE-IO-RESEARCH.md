# Entire.io Research & GitTerm Integration Opportunity

> Research conducted 2026-03-01

## What Entire.io Is

A **GitHub wrapper/alternative UI** (React SPA, TanStack Router) that adds two
core concepts on top of Git:

### Trails (Enhanced Issues + PRs)

A single concept merging issues and pull requests:

- **Statuses:** `draft → open → in_progress → in_review → done → merged → closed`
- **Priority:** urgent, high, medium, low, none
- **Type:** bug, feature, chore, docs, refactor
- **Labels:** custom, color-coded
- **Assignees & Reviewers:** with approval workflow (approved / changes requested)
- **Comments:** threaded discussion
- **Branch tracking:** base branch → feature branch (like PRs)
- **Grouping/filtering:** by status, assignee, type, priority

### Checkpoints (Enhanced Commits)

Per-branch commit history with richer metadata:

- Standard commit data (message, files changed, lines added/removed)
- **Agent attribution:** tracks which AI agent made each change
  - Supported agents: claude, gemini, amp, codex, opencode, copilot
- **Transcripts:** full agent conversation that produced the code
- **Diff viewer:** inline diff with file-level breakdown

### Dashboard

- User activity feed showing recent checkpoints across repos
- Stats: commits, files changed, lines added/removed
- Agent contribution breakdown (pie chart / bars)

### Their Vision (Not Yet Built)

From their `/vision` page:

1. **"Version Control for Agents"** — git-compatible database layer capturing
   intent, constraints, and agent context as first-class data
2. **"Semantic Reasoning Layer"** — persistent shared memory for agent-to-agent
   collaboration, tied to version control history
3. **"AI-Native SDLC"** — UI for reviewing/approving/deploying hundreds of
   agent-produced changes per day

### Technical Stack

- React SPA with TanStack Router
- GitHub OAuth for authentication
- API proxies GitHub: `/api/v1/cache/{org}/{repo}/...`
- PostHog analytics
- Sentry error tracking
- Standard REST API patterns

### Routes

```
/                               — Landing/marketing
/login                          — GitHub OAuth
/overview                       — User dashboard
/{slug}                         — Org/user profile
/gh/{org}/{repo}/               — Repo overview
/gh/{org}/{repo}/trails         — Trail list (issues+PRs)
/gh/{org}/{repo}/trails/{id}    — Trail detail
/gh/{org}/{repo}/checkpoints/{branch}        — Checkpoint list
/gh/{org}/{repo}/checkpoints/{branch}/{id}   — Checkpoint detail
```

---

## GitTerm Integration Opportunity

### Why GitTerm Has a Natural Advantage

Entire.io is a **post-hoc viewer** — it looks at what already happened. GitTerm
is **where the work actually happens**. The agents run in GitTerm's terminal.
The commits are made from GitTerm's git integration. This means GitTerm can:

1. **Capture agent context as it's created** (not reconstructed after the fact)
2. **Link sessions to commits in real-time** (not guessed from metadata)
3. **Provide the review workflow where the code is written** (not in a separate
   browser tab)

### Feature Mapping

| Entire.io Feature    | GitTerm Equivalent                                              | Difficulty |
| -------------------- | --------------------------------------------------------------- | ---------- |
| Trails (issues+PRs)  | Sidebar panel with enhanced status workflow                     | Medium     |
| Trail creation       | Create from terminal context (branch, recent work)              | Medium     |
| Checkpoints          | Enhanced commit view with agent detection                       | Easy       |
| Agent attribution    | Detect agent from commit author/message/session                 | Easy       |
| Agent transcripts    | Capture terminal session → link to commits                      | Medium     |
| Diff review          | Already have word-level diff highlighting ✅                    | Done       |
| Dashboard/stats      | Agent contribution stats per repo                               | Easy       |
| Branch management    | Already have branch context ✅                                  | Done       |
| Reviewer workflow    | Approve/request changes on trails from within GitTerm           | Medium     |
| Semantic context     | Pi memory/compaction summaries linked to commits                | Hard       |

### What GitTerm Already Has

- ✅ Git status, staging, diffs with word-level highlighting
- ✅ Terminal where agents actually run (pi, claude code, codex, etc.)
- ✅ Tab/workspace system per repo
- ✅ File explorer with syntax highlighting
- ✅ Branch detection and auto-switching
- ✅ Native macOS app (fast, not a browser tab)

### Proposed V1 Features

#### Phase 1: Agent-Aware Commits (Easy Wins)

- Detect agent authorship in commits (parse commit messages for pi/claude/codex
  signatures, check author fields)
- Show agent attribution badges in the commit/diff view
- Agent contribution stats in a sidebar widget

#### Phase 2: Session Capture

- Capture terminal output during agent sessions
- Link captured sessions to the commits they produce
- Store as lightweight transcripts (compressed text, not full terminal state)
- View session transcript alongside diff

#### Phase 3: Trails (Enhanced Workflow)

- Trail panel in sidebar (list of trails for current repo)
- Create trails from current branch context
- Status workflow: draft → open → in_progress → in_review → done → merged
- Link trails to branches (auto-detect)
- Trail detail view with comments

#### Phase 4: Review Workflow

- Inline review comments on diffs
- Approve / request changes
- Review checklist integration
- Multi-agent change review (batch review of agent PRs)

### Key Design Decisions to Make

1. **Backend:** Local-only (SQLite/git notes) vs. hosted service vs. GitHub API
   wrapper?
2. **Collaboration:** Single-user first, or multi-user from the start?
3. **Agent detection:** Convention-based (commit message parsing) vs. explicit
   (agent writes metadata)?
4. **Transcript storage:** Git notes? Separate DB? Files in `.gitterm/`?
5. **Trail storage:** GitHub Issues/PRs underneath? Or independent?

### Competitive Position

| Aspect              | Entire.io                  | GitTerm + Extensions       |
| ------------------- | -------------------------- | -------------------------- |
| Where work happens  | Separate browser tab       | Right where you code       |
| Agent capture       | After-the-fact attribution | Real-time session capture  |
| Git integration     | API wrapper over GitHub    | Direct libgit2 access      |
| Performance         | Web app                    | Native Rust/Iced           |
| Offline support     | None (needs API)           | Full (local git)           |
| Multi-provider      | GitHub only (currently)    | Any git repo               |
| Terminal            | None                       | Full PTY terminal          |
| Extensibility       | None                       | Open source                |

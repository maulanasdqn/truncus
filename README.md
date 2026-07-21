# truncus

Unified AI memory cluster on Cloudflare. Every Claude Code session is captured on exit, distilled by Workers AI, embedded into Cloudflare Vectorize, and recalled automatically when the next session starts — plus searchable on demand through MCP tools. It also **learns**: durable lessons are reflected out of each session, reinforced as they recur, and injected into future sessions.

```
Claude Code ── SessionEnd hook ──► truncus-hook ──► truncus-worker (Rust → WASM)
Claude Code ◄─ SessionStart hook ◄─ truncus-hook ◄─ summaries + chunks
Claude Code ◄─ MCP (stdio) ──────► truncus-mcp  ──► semantic search
                                                    │
                                    Workers AI (bge-m3 + llama) · Vectorize · D1 · R2
```

## Crates

| Crate | Role |
|---|---|
| `truncus-core` | DTOs, transcript parser, chunker, config, API client |
| `truncus-worker` | Cloudflare Worker: ingest, pipeline, search, context |
| `truncus-hook` | Claude Code SessionEnd / SessionStart hook adapter |
| `truncus-mcp` | stdio MCP server: `memory_search`, `recent_sessions`, `get_session` |
| `truncus-cli` | `truncus` binary: search, sessions, reprocess, install |
| `truncus-web` | Browser dashboard (React + TanStack Router + shadcn/ui) for reading memory |

## Setup

Requirements: Rust (with `wasm32-unknown-unknown` target), Node.js (for `npx wrangler`), a Cloudflare account.

```bash
npx wrangler login
./scripts/setup.sh        # creates D1, R2, Vectorize (+ metadata indexes), applies migrations, sets secrets, deploys
```

Install the local binaries and wire up Claude Code:

```bash
for crate in truncus-hook truncus-mcp truncus-cli; do cargo install --path $crate; done
truncus install           # writes ~/.config/truncus/config.toml, registers hooks + MCP server
```

## Linking another device

Memory is shared through the single Worker: every device that talks to the same URL with the same token reads and writes the same cluster (sessions are tagged with the device hostname). On each new machine:

```bash
curl -fsSL https://raw.githubusercontent.com/maulanasdqn/truncus/main/scripts/install.sh | bash -s -- \
  --url https://truncus.stynx.app --token <TRUNCUS_API_TOKEN>
```

That builds the three binaries from this repo, writes the client config, registers the SessionEnd/SessionStart hooks, and adds the MCP server — from then on every new Claude Code session on that device saves to and recalls from the shared memory.

## Usage

Sessions are saved and recalled automatically: a Stop hook live-captures the session as you work (throttled to once per 5 minutes — override with `TRUNCUS_CAPTURE_INTERVAL_SECS`), and SessionEnd always captures the final state. Re-ingesting is cheap: the pipeline only re-embeds chunks whose content changed. Manual access:

```bash
truncus search "how did i fix the login bug"
truncus sessions --project truncus
truncus session <session-id>
truncus reprocess <session-id>
truncus delete <session-id>
truncus lessons --project truncus     # what has been learned, most reinforced first
truncus reflect --project truncus     # (re)distill lessons from recent sessions
truncus vault sync --project truncus ~/Obsidian/MyVault   # feed knowledge from a vault
truncus knowledge "how does auth work" --project truncus  # search that knowledge
```

Inside Claude Code, ask things like *"what did I work on last week?"* — the `truncus` MCP tools handle recall.

## Dashboard

A browser dashboard for reading your memory lives in [`truncus-web/`](truncus-web) and is deployed at **https://truncus-ui.stynx.app** — overview stats, a searchable session list with per-session detail, and semantic search. Sign in by pasting your `TRUNCUS_API_TOKEN` (kept in the browser's `localStorage`).

```bash
cd truncus-web
bun install
bun run dev           # http://localhost:5180
bun run deploy        # vite build + wrangler deploy
```

`vite dev` proxies `/v1/*` to the Worker, so no CORS setup is needed locally.

## Self-improvement loop

Truncus turns episodic memory into continual learning. After each session is summarized, a Workers-AI **reflection** pass extracts durable *lessons* — pitfalls, fixes, preferences, conventions, effective workflows — as structured records. A lesson that recurs is **reinforced** (its confidence rises, evidence accrues) rather than duplicated. At the next SessionStart the top lessons for the project are **injected** into Claude's context, so past experience shapes future work:

```
session → reflect (Workers AI) → lessons (D1) → reinforce on repeat
                                      │
   SessionStart ◄── inject top lessons ◄──┘
```

Reflection runs automatically in the ingest pipeline; `truncus reflect` (or `POST /v1/lessons/reflect`) backfills or re-distills on demand. Browse and curate lessons in the dashboard (`/lessons`), the CLI (`truncus lessons`), or the `lessons` MCP tool.

## Knowledge base (Obsidian vault)

Feed a project reference knowledge from a local Obsidian vault so an agent has it available without pasting notes into context:

```bash
truncus vault sync --project <name> /path/to/vault
```

Each markdown note is chunked, embedded, and stored project-scoped (separate from session memory). At SessionStart the agent is told a knowledge base exists and retrieves only the relevant notes on demand via the `knowledge_search` MCP tool — token-efficient, since matching chunks are pulled instead of the whole vault. Re-running `vault sync` is incremental (unchanged notes skipped by content hash, deleted notes pruned). Browse and search it in the dashboard (`/knowledge`).

## API

All endpoints require `Authorization: Bearer $TRUNCUS_API_TOKEN`. Responses carry permissive CORS headers (and answer `OPTIONS` preflight) so the browser dashboard can call the API cross-origin.

| Endpoint | Purpose |
|---|---|
| `POST /v1/sessions` | ingest a session (returns 202, processes async) |
| `POST /v1/sessions/:id/process` | re-run the pipeline for one session |
| `DELETE /v1/sessions/:id` | remove a session from D1, R2, and Vectorize |
| `GET /v1/search?q&project&kind&limit` | semantic search over summaries + chunks |
| `GET /v1/context?project` | recall bundle for session start |
| `GET /v1/sessions?project&limit&offset` | list sessions (paginated; returns `total`, `limit`, `offset`) |
| `GET /v1/sessions/:id` | one session with summary |
| `GET /v1/lessons?project&limit` | list distilled lessons, most reinforced first |
| `POST /v1/lessons/reflect?project&session&limit` | reflect over sessions to extract + reinforce lessons |
| `DELETE /v1/lessons/:id` | remove a lesson |
| `GET/POST/DELETE /v1/notes?project` | list / sync / clear vault notes for a project |
| `POST /v1/notes/prune` · `GET /v1/notes/projects` | reconcile removed notes · list projects with a knowledge base |
| `GET /v1/knowledge?q&project&limit` | semantic search over vault notes |

## Notes

- Vectorize index: 1024 dims (bge-m3), cosine metric, metadata indexes on `project`, `kind`, `ts` — created before any insert, as Vectorize requires.
- Vectorize mutations take 5–10 s to become queryable; fresh sessions appear in search shortly after ingest.
- The Worker calls the Vectorize v2 REST API (no native Rust binding yet), so it needs a `CF_API_TOKEN` secret scoped to Vectorize Edit.

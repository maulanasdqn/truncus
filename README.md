# truncus

Unified AI memory cluster on Cloudflare. Every Claude Code session is captured on exit, distilled by Workers AI, embedded into Cloudflare Vectorize, and recalled automatically when the next session starts — plus searchable on demand through MCP tools.

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

Sessions are saved and recalled automatically. Manual access:

```bash
truncus search "how did i fix the login bug"
truncus sessions --project truncus
truncus session <session-id>
truncus reprocess <session-id>
truncus delete <session-id>
```

Inside Claude Code, ask things like *"what did I work on last week?"* — the `truncus` MCP tools handle recall.

## API

All endpoints require `Authorization: Bearer $TRUNCUS_API_TOKEN`.

| Endpoint | Purpose |
|---|---|
| `POST /v1/sessions` | ingest a session (returns 202, processes async) |
| `POST /v1/sessions/:id/process` | re-run the pipeline for one session |
| `DELETE /v1/sessions/:id` | remove a session from D1, R2, and Vectorize |
| `GET /v1/search?q&project&kind&limit` | semantic search over summaries + chunks |
| `GET /v1/context?project` | recall bundle for session start |
| `GET /v1/sessions?project&limit` | list sessions |
| `GET /v1/sessions/:id` | one session with summary |

## Notes

- Vectorize index: 1024 dims (bge-m3), cosine metric, metadata indexes on `project`, `kind`, `ts` — created before any insert, as Vectorize requires.
- Vectorize mutations take 5–10 s to become queryable; fresh sessions appear in search shortly after ingest.
- The Worker calls the Vectorize v2 REST API (no native Rust binding yet), so it needs a `CF_API_TOKEN` secret scoped to Vectorize Edit.

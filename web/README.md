# truncus-web

Dashboard for browsing your Truncus memory — sessions, summaries, and semantic
search — talking to the `truncus-worker` API.

Built with React 19, TanStack Router (file-based) + Query, Tailwind v4, and a
vendored shadcn/ui component set. Structure follows the KYA `apps/*/web`
convention: route-group folders (`_public`, `_authenticated`) with co-located
`_components`, `_hooks`, and `_constants`.

## Run it

```bash
bun install
bun run dev            # http://localhost:5180
```

Open the app, paste your `TRUNCUS_API_TOKEN`, and connect. The token is stored
in this browser's `localStorage` and sent only to the Worker.

`vite dev` proxies `/v1/*` to `https://truncus.stynx.app`, so the browser stays
same-origin and no CORS setup is needed. Point the proxy at a local
`wrangler dev` instead:

```bash
TRUNCUS_API_URL=http://localhost:8787 bun run dev
```

## Pages

| Route | What it shows |
|---|---|
| `/overview` | Totals, status breakdown, top projects, recent sessions |
| `/sessions` | All sessions, filterable by project + free text |
| `/sessions/$id` | One session: metadata, summary, delete |
| `/search` | Semantic search over summaries + chunks |

## Scripts

```bash
bun run dev         # dev server + API proxy
bun run build       # production build to dist/
bun run preview     # serve the built dist/
bun run typecheck   # tsc --noEmit
```

## Deploying as static files

The build output in `dist/` is a static SPA. If you host it and call the Worker
directly (setting `VITE_API_URL=https://truncus.stynx.app`), the browser hits a
different origin, so the Worker must send CORS headers. The zero-config path is
to keep same-origin: serve `dist/` behind a proxy that forwards `/v1/*` to the
Worker (Cloudflare Pages Functions, nginx, Caddy, etc.).

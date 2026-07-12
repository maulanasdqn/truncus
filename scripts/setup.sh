#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../truncus-worker"

WRANGLER="npx --yes wrangler"
INDEX="truncus-memory"

echo "==> checking Cloudflare auth"
ACCOUNT_ID=$($WRANGLER whoami 2>/dev/null | grep -oE '[0-9a-f]{32}' | head -1)
if [ -z "$ACCOUNT_ID" ]; then
  echo "not logged in — run: npx wrangler login" >&2
  exit 1
fi
echo "account: $ACCOUNT_ID"

echo "==> creating D1 database"
$WRANGLER d1 create truncus-db 2>/dev/null || echo "truncus-db already exists"
DB_ID=$($WRANGLER d1 list --json | python3 -c "import json,sys; print(next(d['uuid'] for d in json.load(sys.stdin) if d['name']=='truncus-db'))")
echo "d1 id: $DB_ID"

echo "==> creating R2 bucket"
$WRANGLER r2 bucket create truncus-raw 2>/dev/null || echo "truncus-raw already exists"

echo "==> creating Vectorize index + metadata indexes"
$WRANGLER vectorize create "$INDEX" --dimensions=1024 --metric=cosine 2>/dev/null || echo "$INDEX already exists"
$WRANGLER vectorize create-metadata-index "$INDEX" --property-name=project --type=string 2>/dev/null || true
$WRANGLER vectorize create-metadata-index "$INDEX" --property-name=kind --type=string 2>/dev/null || true
$WRANGLER vectorize create-metadata-index "$INDEX" --property-name=ts --type=number 2>/dev/null || true

echo "==> patching wrangler.jsonc"
python3 - "$ACCOUNT_ID" "$DB_ID" <<'EOF'
import re, sys
account_id, db_id = sys.argv[1], sys.argv[2]
path = "wrangler.jsonc"
raw = open(path).read()
raw = raw.replace("REPLACE_WITH_ACCOUNT_ID", account_id)
raw = re.sub(r'"database_id": "[^"]*"', f'"database_id": "{db_id}"', raw)
open(path, "w").write(raw)
print("patched", path)
EOF

echo "==> applying D1 migrations"
$WRANGLER d1 migrations apply truncus-db --remote

echo "==> secrets"
if [ -n "${TRUNCUS_API_TOKEN:-}" ]; then
  echo "$TRUNCUS_API_TOKEN" | $WRANGLER secret put TRUNCUS_API_TOKEN
else
  echo "generate one with: openssl rand -hex 32"
  $WRANGLER secret put TRUNCUS_API_TOKEN
fi
if [ -n "${CF_API_TOKEN:-}" ]; then
  echo "$CF_API_TOKEN" | $WRANGLER secret put CF_API_TOKEN
else
  echo "create an API token with Vectorize Edit permission at https://dash.cloudflare.com/profile/api-tokens"
  $WRANGLER secret put CF_API_TOKEN
fi

echo "==> deploying"
$WRANGLER deploy

echo "==> done — next: for crate in truncus-hook truncus-mcp truncus-cli; do cargo install --path \$crate; done && truncus install"

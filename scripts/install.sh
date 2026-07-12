#!/usr/bin/env bash
set -euo pipefail

REPO="https://github.com/maulanasdqn/truncus"
URL="${TRUNCUS_URL:-}"
TOKEN="${TRUNCUS_TOKEN:-}"

while [ $# -gt 0 ]; do
  case "$1" in
    --url) URL="$2"; shift 2 ;;
    --token) TOKEN="$2"; shift 2 ;;
    *) echo "usage: install.sh [--url WORKER_URL] [--token API_TOKEN]" >&2; exit 1 ;;
  esac
done

if ! command -v cargo >/dev/null 2>&1; then
  echo "Rust is required — install it first: https://rustup.rs" >&2
  exit 1
fi

echo "==> installing truncus binaries from $REPO"
for crate in truncus-hook truncus-mcp truncus-cli; do
  cargo install --git "$REPO" "$crate" --locked
done

echo "==> wiring up Claude Code"
if [ -n "$URL" ] && [ -n "$TOKEN" ]; then
  truncus install --url "$URL" --token "$TOKEN"
else
  truncus install
fi

echo "==> done — open a new Claude Code session; your memory is linked"

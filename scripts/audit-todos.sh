#!/usr/bin/env bash
# Compare inline // TODO(ID): comments in src/ and firmware/ against docs/TODO.md.
# Read-only — always exits 0; prints a human-readable drift report.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TODO_DOC="${ROOT}/docs/TODO.md"
SEARCH_DIRS=("${ROOT}/src" "${ROOT}/firmware")

if [[ ! -f "$TODO_DOC" ]]; then
  echo "error: missing ${TODO_DOC}" >&2
  exit 1
fi

echo "==> Pi Smart Clock TODO audit"
echo "    Tracker: docs/TODO.md"
echo ""

mapfile -t DOC_OPEN_IDS < <(
  grep -E '^\| [A-Z]+-[0-9]+ \| open \|' "$TODO_DOC" \
    | sed -E 's/^\| ([A-Z]+-[0-9]+) \|.*/\1/' \
    | sort -u
)

mapfile -t DOC_DONE_IDS < <(
  awk '/^## Completed/,0' "$TODO_DOC" \
    | grep -E '^\| [A-Z]+-[0-9]+ \|' \
    | grep -v '^| ID |' \
    | grep -v '^| ----' \
    | sed -E 's/^\| ([A-Z]+-[0-9]+) \|.*/\1/' \
    | sort -u
)

TMP_INLINE="$(mktemp)"
TMP_IDS="$(mktemp)"
trap 'rm -f "$TMP_INLINE" "$TMP_IDS"' EXIT

for dir in "${SEARCH_DIRS[@]}"; do
  [[ -d "$dir" ]] || continue
  grep -RInE '// TODO\([A-Z]+-[0-9]+\):' "$dir" --include='*.rs' 2>/dev/null >> "$TMP_INLINE" || true
done

if [[ -s "$TMP_INLINE" ]]; then
  sed -E 's/.*\/\/ TODO\(([A-Z]+-[0-9]+)\):.*/\1/' "$TMP_INLINE" | sort -u > "$TMP_IDS"
else
  : > "$TMP_IDS"
fi

echo "==> Inline TODO(ID): comments ($(wc -l < "$TMP_INLINE" | tr -d ' ') lines)"
if [[ -s "$TMP_INLINE" ]]; then
  while IFS= read -r line; do
    echo "    $line"
  done < "$TMP_INLINE"
else
  echo "    (none — add // TODO(ID): when starting work)"
fi
echo ""

echo "==> Open IDs in docs/TODO.md (${#DOC_OPEN_IDS[@]})"
printf '    %s\n' "${DOC_OPEN_IDS[@]}"
echo ""

echo "==> Drift checks"
DRIFT=0

while IFS= read -r id; do
  [[ -n "$id" ]] || continue
  if printf '%s\n' "${DOC_OPEN_IDS[@]}" | grep -qx "$id"; then
    continue
  fi
  if printf '%s\n' "${DOC_DONE_IDS[@]}" | grep -qx "$id"; then
    echo "    warn: inline TODO($id) still in code but marked done in docs/TODO.md"
    DRIFT=1
    continue
  fi
  echo "    warn: inline TODO($id) in code but no matching open/done row in docs/TODO.md"
  DRIFT=1
done < "$TMP_IDS"

MISSING_INLINE=0
for id in "${DOC_OPEN_IDS[@]}"; do
  if ! grep -qx "$id" "$TMP_IDS"; then
    if [[ $MISSING_INLINE -eq 0 ]]; then
      echo "    info: open doc IDs without inline // TODO(ID): (optional)"
    fi
    echo "          $id"
    MISSING_INLINE=1
  fi
done

if [[ $DRIFT -eq 0 && $MISSING_INLINE -eq 0 ]]; then
  echo "    ok: no drift detected"
elif [[ $DRIFT -eq 0 ]]; then
  echo "    ok: no errors (informational lines above)"
fi

echo ""
echo "==> Untagged // TODO: (no ID — consider adding one)"
UNTAGGED=0
for dir in "${SEARCH_DIRS[@]}"; do
  [[ -d "$dir" ]] || continue
  while IFS= read -r line; do
    echo "    $line"
    UNTAGGED=1
  done < <(grep -RInE '// TODO:' "$dir" --include='*.rs' 2>/dev/null | grep -vE '// TODO\([A-Z]+-[0-9]+\):' || true)
done
if [[ $UNTAGGED -eq 0 ]]; then
  echo "    (none)"
fi

echo ""
echo "Done. Update docs/TODO.md when closing items."

#!/usr/bin/env bash
# Kontrolas, ke la fonta tavolo enhavas neniun ĉinan tekston (AGENTS.md).
set -euo pipefail
cd "$(dirname "$0")/.."

mapfile -d '' dosieroj < <(
    find src examples tools -type f -print0 2>/dev/null
    printf 'justfile\0Cargo.toml\0deny.toml\0'
)

if [ "${#dosieroj[@]}" -eq 0 ]; then
    echo "sen-cina: neniuj dosieroj por kontroli" >&2
    exit 1
fi

trovita=$(perl -CSD -ne \
    'print "$ARGV:$.:$_\n" if /[\x{3400}-\x{4DBF}\x{4E00}-\x{9FFF}\x{F900}-\x{FAFF}]/' \
    "${dosieroj[@]}")

if [ -n "$trovita" ]; then
    echo "sen-cina: ĉina teksto trovita en la fonta tavolo:" >&2
    echo "$trovita" >&2
    exit 1
fi

echo "sen-cina: bone"

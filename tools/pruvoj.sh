#!/usr/bin/env bash
# Komparas la katalogon `docs/PRUVOJ.md` kun la `#[kani::proof]`-atributoj en
# `src/` (Q05).
#
# Uzo:
#   bash tools/pruvoj.sh --kontroli
#   bash tools/pruvoj.sh --listi
set -euo pipefail
cd "$(dirname "$0")/.."

katalogo="docs/PRUVOJ.md"

nunaj() {
    # Solver aŭ aliaj atributoj rajtas stari inter `#[kani::proof]` kaj `fn`.
    grep -rhA8 --include='*.rs' '#\[kani::proof\]' src 2>/dev/null \
        | sed -n 's/.*fn \([A-Za-z0-9_]*\).*/\1/p' \
        | sort -u
}

dokumentitaj() {
    sed -n 's/^| `\([A-Za-z0-9_]*\)` |.*/\1/p' "$katalogo" | sort -u
}

modo="${1:---kontroli}"
case "$modo" in
    --listi)
        nunaj
        ;;
    --kontroli)
        atributoj=$(grep -rho '#\[kani::proof\]' src | wc -l | tr -d ' ')
        nuntempaj=$(nunaj)
        if [ -z "$nuntempaj" ]; then
            echo "pruvoj: neniu #[kani::proof] trovita en src/" >&2
            exit 1
        fi
        nombro=$(printf '%s\n' "$nuntempaj" | wc -l | tr -d ' ')
        if [ "$atributoj" -ne "$nombro" ]; then
            echo "pruvoj: duoblaj aŭ nelegeblaj atributoj ($atributoj kontraŭ $nombro)" >&2
            exit 1
        fi
        if [ "$nuntempaj" != "$(dokumentitaj)" ]; then
            echo "pruvoj: la katalogo $katalogo malkongruas kun src/:" >&2
            diff <(dokumentitaj) <(printf '%s\n' "$nuntempaj") >&2 || true
            exit 1
        fi
        echo "pruvoj: $nombro (bone)"
        ;;
    *)
        echo "uzo: bash tools/pruvoj.sh [--kontroli|--listi]" >&2
        exit 2
        ;;
esac

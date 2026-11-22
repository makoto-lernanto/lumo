#!/usr/bin/env bash
# Reproduktas la orajn fiksojn kaj komparas ilin kun `docs/FIKSAĴOJ.md`
# (D03, D07). La haketo kovras nur la videblajn pikselojn en la kanala ordo
# ARGB.
#
# Uzo:
#   bash tools/haketi.sh --kontroli
#   bash tools/haketi.sh --skribi
set -euo pipefail
cd "$(dirname "$0")/.."

dosiero="docs/FIKSAĴOJ.md"
fiksoj=(
    "plena-ortangulo"
    "klipita-ortangulo"
    "kadro-malhela-normala"
    "kadro-malhela-ŝveba"
    "kadro-malhela-premata"
    "kadro-hela-normala"
    "kadro-hela-ŝveba"
    "kadro-hela-premata"
)
modo="${1:---kontroli}"
hodiau=$(date +%F)

kalkuli() {
    cargo run --quiet --example haketi -- "$1" | sha256sum | cut -d' ' -f1
}

dokumentita_hash() {
    awk -v n="$1" -F'|' '
        $2 ~ ("`" n "`") { gsub(/[ `\t]/, "", $4); print $4; exit }
    ' "$dosiero"
}

case "$modo" in
    --kontroli)
        fiasko=0
        for nomo in "${fiksoj[@]}"; do
            hash=$(kalkuli "$nomo")
            dokumentita=$(dokumentita_hash "$nomo")
            if [ -z "$dokumentita" ]; then
                echo "haketi: mankas linio por $nomo en $dosiero" >&2
                fiasko=1
            elif [ "$hash" = "$dokumentita" ]; then
                echo "haketi: $nomo (bone)"
            else
                echo "haketi: $nomo malkongruas (kalkulita $hash, dokumentita $dokumentita)" >&2
                fiasko=1
            fi
        done
        exit "$fiasko"
        ;;
    --skribi)
        for nomo in "${fiksoj[@]}"; do
            hash=$(kalkuli "$nomo")
            if ! awk -v n="$nomo" -v d="$hodiau" -v h="$hash" '
                $0 ~ ("^\\| `" n "` \\|") {
                    print "| `" n "` | " d " | `" h "` |"
                    trovita = 1
                    next
                }
                { print }
                END { if (!trovita) exit 3 }
            ' "$dosiero" > "$dosiero.nova"; then
                rm -f "$dosiero.nova"
                echo "haketi: mankas linio por $nomo en $dosiero" >&2
                exit 1
            fi
            mv "$dosiero.nova" "$dosiero"
            echo "haketi: skribita $nomo $hash"
        done
        ;;
    *)
        echo "uzo: bash tools/haketi.sh [--kontroli|--skribi]" >&2
        exit 2
        ;;
esac

#!/usr/bin/env bash
# Kontrolas aŭ skribas la version de Lumo en Cargo.toml, README.md kaj
# docs/LUMO.md.
#
# Uzo:
#   bash tools/versio.sh --kontroli
#   bash tools/versio.sh --skribi 0.2.0
set -euo pipefail
cd "$(dirname "$0")/.."

erari() {
    echo "versio: $*" >&2
    exit 1
}

eltiri_cargo() {
    sed -n 's/^version = "\([^"]*\)"$/\1/p' Cargo.toml | head -n1
}

eltiri_readme() {
    sed -n 's/^\(> \)\?\*\*Versio:\*\* \([0-9][0-9.]*\).*/\2/p' README.md | head -n1
}

eltiri_lumo() {
    sed -n 's/^\(> \)\?\*\*Versio:\*\* \([0-9][0-9.]*\).*/\2/p' docs/LUMO.md | head -n1
}

kontroli() {
    local cargo readme lumo
    cargo=$(eltiri_cargo)
    readme=$(eltiri_readme)
    lumo=$(eltiri_lumo)
    [ -n "$cargo" ] || erari "mankas version en Cargo.toml"
    [ -n "$readme" ] || erari "mankas '**Versio:**' en README.md"
    [ -n "$lumo" ] || erari "mankas '**Versio:**' en docs/LUMO.md"
    if [ "$cargo" = "$readme" ] && [ "$cargo" = "$lumo" ]; then
        echo "versio: $cargo (bone)"
    else
        erari "malkongruo: Cargo.toml=$cargo README.md=$readme docs/LUMO.md=$lumo"
    fi
}

modo="${1:---kontroli}"
case "$modo" in
    --kontroli)
        kontroli
        ;;
    --skribi)
        nova="${2:-}"
        case "$nova" in
            [0-9]*.[0-9]*.[0-9]*) ;;
            *) erari "uzo: bash tools/versio.sh --skribi 0.2.0" ;;
        esac
        perl -0pi -e 's/^version = "[^"]*"/version = "'"$nova"'"/m' Cargo.toml
        README_VERSIO="$nova" perl -pi -e \
            's/^(> )?(\*\*Versio:\*\* )[0-9][0-9.]*/$1$2$ENV{README_VERSIO}/' README.md
        LUMO_VERSIO="$nova" perl -pi -e \
            's/^(> )?(\*\*Versio:\*\* )[0-9][0-9.]*/$1$2$ENV{LUMO_VERSIO}/' docs/LUMO.md
        kontroli
        ;;
    *)
        erari "nekonata modo: $modo"
        ;;
esac

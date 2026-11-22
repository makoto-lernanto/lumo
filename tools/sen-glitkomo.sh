#!/usr/bin/env bash
# Kontrolas, ke `src/` enhavas neniun glitkoman tipon (`f32`, `f64`) (L04).
set -euo pipefail
cd "$(dirname "$0")/.."

if trovita=$(grep -rnE '\b(f32|f64)\b' src); then
    echo "sen-glitkomo: glitkoma tipo trovita en src/:" >&2
    echo "$trovita" >&2
    exit 1
fi

echo "sen-glitkomo: bone"

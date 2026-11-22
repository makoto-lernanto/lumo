# Pruvokatalogo

Ĉiu `#[kani::proof]` havas unu linion en ĉi tiu tabelo. `bash
tools/pruvoj.sh --kontroli` komparas la katalogon kun la atributoj en `src/`
kaj malsukcesas ĉe manko aŭ troo (Q05). Kani ruliĝas nur en DI (Q02), unu
pruvo po tasko (Q06).

| Pruvo | Dosiero | Invarianto |
| --- | --- | --- |
| `miksi_neniam_troas` | `src/kani_pruvoj.rs` | R07: kun premultiplika fonto la kunfandita alfa kaj koloro neniam troas 255. |
| `klipo_restas_en_surfaco` | `src/kani_pruvoj.rs` | L15: la videbla regiono kaj la plej granda indekso restas ene de `larĝo × alto` kaj de la bufro. |

`klipo_restas_en_surfaco` pruvas L15 por `larĝo, alto, paŝo ≤ u16::MAX` kaj
`paŝo ≥ larĝo` (ĉi tion jam garantias `Surfaco::nova`). Sub tiu limo la
indeksoj restas sub 2^32, do la u32-kalkulo de la pruvo kongruas kun la
u64-kalkulo de `plenigi`. Senlima pruvo de la 64-bita produto superis la
600-sekundan templimon de Q06.

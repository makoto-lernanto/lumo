# Pruvokatalogo

Ĉiu `#[kani::proof]` havas unu linion en ĉi tiu tabelo. `bash
tools/pruvoj.sh --kontroli` komparas la katalogon kun la atributoj en `src/`
kaj malsukcesas ĉe manko aŭ troo (Q05). Kani ruliĝas nur en DI (Q02), unu
pruvo po tasko (Q06).

| Pruvo | Dosiero | Invarianto |
| --- | --- | --- |
| `miksi_neniam_troas` | `src/kani_pruvoj.rs` | R07: kun premultiplika fonto la kunfandita alfa kaj koloro neniam troas 255. |
| `klipo_restas_en_surfaco` | `src/kani_pruvoj.rs` | L15: la videbla regiono kaj la plej granda indekso restas ene de `larĝo × alto` kaj de la bufro. |

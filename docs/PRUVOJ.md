# Pruvokatalogo

Ĉiu `#[kani::proof]` havas unu linion en ĉi tiu tabelo. `bash
tools/pruvoj.sh --kontroli` komparas la katalogon kun la atributoj en `src/`
kaj malsukcesas ĉe manko aŭ troo (Q05). Kani ruliĝas nur en DI (Q02), unu
pruvo po tasko (Q06).

| Pruvo | Dosiero | Invarianto |
| --- | --- | --- |
| `miksi_neniam_troas` | `src/kani_pruvoj.rs` | R07: kun premultiplika fonto la kunfandita alfa kaj koloro neniam troas 255. |
| `klipo_restas_en_surfaco` | `src/kani_pruvoj.rs` | L15: la videbla regiono restas ene de `larĝo × alto`, kaj la kolumna indekso restas sub `paŝo`; `plenigi` gardas la vican komencon per `get_mut`, do ĝi ne skribas ekster la bufro. |
| `rondigo_kovrado_en_limoj` | `src/kani_pruvoj.rs` | R04–R06: la kovrado de la 4 × 4-superprovado restas ≤ 255, ĉar ĝi venas el al plej 16 specimenoj. |

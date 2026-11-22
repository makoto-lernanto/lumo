# Lumo

Pura Rust-rastrumilo por la grafika interfaco FluoU de MiKerno kaj Horizonto.

> Stato: 0.2.0 — Fazo 2 (rondoj, maskoj, translokigo) preta; la deviga normo
> estas [docs/LUMO.md](docs/LUMO.md).
>
> **Versio:** 0.2.0

## Kio estas Lumo

Lumo transformas ordonaron (sendoliston) en pikselojn sur surfaco, kiun
provizas la alvokanto. Ĝi estas biblioteko sen sistemvokoj, sen fadenoj kaj
sen propra memoro.

- `no_std`, sen `alloc`, sen `unsafe` (`#![forbid(unsafe_code)]`).
- Neniu C-kodo, neniu `bindgen`, neniu C-ABI-dependo.
- Nur entjera fiksa komo: la samaj ordonoj donas la samajn bajtojn sur
  `x86_64` kaj `arm64`.

## Kial propra rastrumilo

La konataj pure-Rustaj rastrumiloj ne taŭgas: `tiny-skia` dependas de `f32`
kaj `alloc`; `vello` postulas GPU. FluoU (A06/P17) postulas pikselan
paritatecon inter la du arkoj, kiun oni plej facile pruvas per entjera
aritmetiko.

## Strukturo

| Vojo | Enhavo |
| --- | --- |
| `docs/LUMO.md` | la deviga normo, la kvalita konstitucio kaj la fazoj |
| `docs/PRUVOJ.md` | la pruvokatalogo (`#[kani::proof]`) |
| `docs/FIKSAĴOJ.md` | la oraj haketoj kaj ilia historio |
| `docs/RENDIMENTO.md` | la mezuroj de la kanonika vojo |
| `src/` | `Surfaco`, `Koloro`, `Rektangulo`, `Ordono` kaj `desegnu` |
| `justfile`, `deny.toml`, `tools/` | la loka kvalita pordego |
| `.github/workflows/di.yml` | la DI-matrico (Kani po unu pruvo) |
| `Cargo.toml` | pako kaj eldono |

Lumo estas aparta projekto: ĝi ne dependas de MiKerno, FluoU nek Horizonto.
La kompozitoro de Horizonto dependos de ĝi, ne inverse.

## Kvalita Konstitucio

La neintertraktebla konstitucio estas
[docs/LUMO.md §8](docs/LUMO.md#8-kvalita-konstitucio-neintertraktebla). Ŝi
spegulas la pordegon de MiKerno (`../mikerno/docs/MEJLOSTONOJ.md` §9.1–§9.3
kaj §4.7–§4.8): loka `just kontroli` sen Kani, matrica `kani-kontroli` en DI
(unu pruvo po tasko, templimoj, artefaktoj), Miri, `cargo deny`, malpermeso
de senbaza `#[allow]` kaj piksela paritateco inter `x86_64` kaj `arm64`.

Loke oni neniam rulas `cargo kani` (WSL2 elĉerpiĝas je memoro).

## Fazoj

| Fazo | Versio | Celo |
| --- | --- | --- |
| 1 | 0.1.0 | fundamento: surfaco, koloro, ortangula plenigo, klipo, kunfandado |
| 2 | 0.2.0 | rondigita ortangulo, maskoj, translokigo |
| 3 | 0.3.0 | bitmAPA teksto |
| 4 | 0.4.0 | damaĝo kaj saturiĝo |
| 5 | 0.5.0 | paritateco kaj plena pordego |
| 6 | 0.6.0 | kovrado 8 × 8 kaj optimumigo |
| 7 | 0.7.0 | Mika-malklarigo |
| 8 | 0.8.0 | eldonkandidato: dokumentaro, fuzado, buĝetoj |
| 9 | 1.0.0 | frostigo kaj unua stabila eldono |

Ĉiu Fazo havas detalajn enhavon, evitindajn erarojn kaj akceptkriteriojn en
[docs/LUMO.md §10](docs/LUMO.md#10-detalaj-fazoj).

## Konstruado kaj testado

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
```

La loka pordego estas unu komando:

```sh
just kontroli
```

Ĝi kontrolas la version, la foreston de glitkomo kaj ĉina teksto, la
pruvokatalogon, la orajn haketojn, la formaton, `clippy` sur ambaŭ arkoj, la
testojn, la publikan dokumentaron kaj la eldonan konstruon. Kani mem ruliĝas
nur en DI (`just miri` kaj `just permesiloj` restas apartaj).

## Licenco

MIT. Vidu [LICENSE](LICENSE).

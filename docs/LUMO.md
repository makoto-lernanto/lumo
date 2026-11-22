# Lumo — normiga gvidilo pri la rastrumilo de FluoU

> Stato: 2026-11-22 · Fazo 2 preta · celo 1.0 · MIT
>
> **Versio:** 0.2.0
>
> Ĉi tiu dokumento estas la deviga normo de Lumo, la sola rastrumilo
> permesata por FluoU. Ĝi difinas la limojn, la objektojn, la rastrumadon,
> la determinismon, la kvalitan pordegon kaj la fazan planon. Kodo,
> identigiloj kaj dokumentoj estas Esperantaj.
>
> La formo de la fazoj, la Kvalita Konstitucio kaj la akceptkriterioj sekvas
> la mejloŝtonan dokumenton de MiKerno
> (`../mikerno/docs/MEJLOSTONOJ.md`, precipe §4.7–§4.8 kaj §9.1–§9.3) kaj
> `../mikerno/AGENTS.md`. Se MiKerno ŝanĝas sian pordegon, Lumo sekvas ĝin
> en la sekva Fazo.

---

## 0. Normaj vortoj

| Vorto | Signifo |
| --- | --- |
| `DEVI` | absoluta postulo; malobservo rifuzas la ŝanĝon |
| `DEVI NE`, `MALPERMESITE` | absoluta malpermeso |
| `DEVUS` | forta rekomendo; deflankiĝo postulas skriban pravigon |
| `POVAS` | permeso |
| `KONFORMA` | plenumas ĉiujn `DEVI`-rulojn de ĉi tiu dokumento |

Ruloj havas identigilojn (`L`, `R`, `T`, `K`, `D`, `P`, `Q`). Oni ne
renumeru ilin sen ĝisdatigi la konformkontrolon.

---

## 1. Celaro

Lumo transformas **ordonaron** (sendoliston) en pikselojn sur **surfaco**, kiun
provizas la alvokanto. Ĝi estas biblioteko: neniu sistemvoko, neniu fadeno,
neniu propra memoro, neniu dosieraliro.

La celoj estas, en ordo:

1. **Pureco** — neniu C-kodo, neniu C-ABI, neniu `unsafe`.
2. **Determinismo** — la samaj ordonoj produktas la samajn bajtojn sur
   `x86_64` kaj `arm64` (FluoU A06 kaj P17).
3. **Malpezeco** — `no_std`, neniom da dinamika memoro.
4. **Flateco** — la rastrumilo restu malgranda; la komplikeco apartenas al
   FluoU, ne al Lumo.

### 1.1. Kial propra rastrumilo

`tiny-skia` estas pura Rust, sed dependas de `f32` kaj `alloc`; `vello`
postulas GPU. Nek taŭgas por la postulo de piksela paritateco inter la du
arkoj, kaj la unua ankaŭ ne por `no_std`. Lumo anstataŭe uzas entjeran fiksan
komon kaj strikte difinitan aritmetikon, kiun oni povas pruvi kaj haketi.

### 1.2. Rilato al la pordego de MiKerno

MiKerno restas la fonto de la kvalita regularo: la sen-Kani-a loka pordego,
la matrica Kani-pordego, Miri, `cargo deny`, la malpermeso de `#[allow]` kaj
la regulo «unu pruvo, unu tasko» devenas de tie. Lumo ne kopias la
mejloŝtonan enhavon de MiKerno; ĝi nur sekvas la saman formon, por ke ambaŭ
projektoj restu kompareblaj.

---

## 2. Limoj kaj principoj

- **L01.** La fina artefakto `DEVI` esti pura Rust: neniu C-kodo, neniu
  `bindgen`, neniu C-ABI-dependo. Gastigaj atestiloj `POVAS` uzi purajn
  Rust-kratojn.
- **L02.** La publikigita kratro `DEVI` esti `no_std` kaj `DEVI NE` uzi
  `alloc`, `Vec`, `Box` aŭ `String`. Ĉiuj bufroj venas de la alvokanto. La
  testa konstruo `POVAS` uzi `std` per `cfg(test)`.
- **L03.** `#![forbid(unsafe_code)]` `DEVI` resti en la rado. La rapidon oni
  atingas per tranĉaĵoj kaj limigitaj indeksoj, ne per krudaj montriloj.
- **L04.** Neniu glitkoma aritmetiko en la desegna vojo. Ĉiu geometrio uzas
  entjeran fiksan komon; ĉiu divido rondigas duon-alte (`+127` antaŭ `/255`).
- **L05.** Lumo `DEVI NE` alvoki sistemvokojn, legi dosierojn aŭ posedi
  fadenojn. Ĝi estas pura funkcio de (surfaco, ordonaro).
- **L06.** Ĉiuj identigiloj, komentoj, mesaĝoj kaj dokumentoj `DEVI` esti
  Esperantaj.
- **L07.** La publika API `DEVI` havi dokumentitan version; rompa ŝanĝo
  postulas pliigon de la ĉefversio.
- **L08.** Manko de loko `DEVI` konduki al tranĉita (silente ne desegnita)
  eligo, ne al paniko nek al skribo ekster la surfaco.
- **L09.** Ĉiu aritmetiko `DEVI` esti sen troo: kontrolitaj operacioj aŭ
  pruvita limo.
- **L10.** Lumo laboras nur kun **aparat-pikseloj**. La konvertiĝo el DIP
  apartenas al FluoU aŭ al la kompozitoro, ne al Lumo.
- **L11.** Lumo `DEVI NE` koni ĵetonojn, temojn, materialojn nek movon; ĝi
  ricevas nur kolorojn, geometrion kaj glifojn.
- **L12.** La rastrumilo `DEVI NE` posedi kaŝmemoron trans la voko; stato
  vivas nur en la alvokanto.
- **L13.** La API `DEVI` esti sama sur `x86_64` kaj `arm64`; platformaj
  `cfg`-branĉoj en la desegna vojo estas `MALPERMESITAJ` en 1.0.

---

## 3. Objektoj

### 3.1. Surfaco

`Surfaco` estas muta tranĉaĵo da `u32` plus geometrio:

| Kampo | Signifo |
| --- | --- |
| `pikseloj` | `&mut [u32]`, unu `u32` po pikselo |
| `larĝo` | videbla larĝo en pikseloj |
| `alto` | videbla alto en pikseloj |
| `paŝo` | distanco inter vicoj en **pikseloj** (ne bajtoj) |

La origino estas supre-maldekstre; la vicoj kreskas malsupren. La bajta ordo
estas hosta (`u32` en memoro), la kanala ordo estas kanonike ARGB8888.

- **L14.** `pikseloj.len()` `DEVI` esti almenaŭ `paŝo × alto`; la kontrolo
  kalkuliĝas per `u64`.
- **L15.** Lumo `DEVI NE` skribi ekster la ortangulo `larĝo × alto`, ankaŭ
  kiam `paŝo > larĝo`.

### 3.2. Koloro

`Koloro` estas **antaŭmultiplika** `(a, r, g, b)`, ĉiu kanalo `u8`, kun la
invarianto `r, g, b ≤ a`.

- **L16.** Antaŭmultiplika formo estas deviga; senpremultiplika koloro
  `DEVI NE` eniri la rastrumilon.
- **L17.** Travideblo `a = 0` signifas «neniu efiko»; surskribo de tia koloro
  `DEVI` lasi la celon senŝanĝa.
- **L18.** Konstruilo `DEVI` rifuzi koloron, kiu malobservas la invarianton
  (`Option`), ne «ripari» ĝin silente.

### 3.3. Ordonaro

`Ordono` estas la sola enigo de la rastrumilo. La ordonaro estas fiksa
tranĉaĵo, kiun provizas la alvokanto.

| Limo | Valoro | Ekde |
| --- | --- | --- |
| Maksimuma nombro da ordonoj | 256 | 0.1 |
| Maksimuma profundo de klipo | 8 | 0.1 |
| Maksimuma profundo de translokigo | 8 | 0.2 |
| Maksimuma nombro da damaĝaj ortanguloj | 16 | 0.4 |
| Maksimuma malklariga radiuso | 64 | 0.7 |

| Ordono | Signifo | Ekde |
| --- | --- | --- |
| `Plenigi { rektangulo, koloro }` | plenigu ortangulon | 0.1 |
| `Klipo { rektangulo }` | premu la klipon (komunaĵo kun la nuna) | 0.1 |
| `Malŝlipo` | restarigu la antaŭan klipon | 0.1 |
| `RondaRekto { rektangulo, radiuso, koloro }` | plenigu rondigitan ortangulon | 0.2 |
| `Masko { origino, amplekso, koloro, kovradoj }` | apliku 8-bitan kovradan maskon | 0.2 |
| `Translokigo { dx, dy }` | premu translokigon | 0.2 |
| `Maltranslokigo` | restarigu la antaŭan translokigon | 0.2 |
| `Glifoj { origino, bazo, koloro, tiparo, tekstero }` | desegnu bitmape teksteton | 0.3 |
| `Malklarigi { rektangulo, radiuso }` | malklarigu la nunan surfacon | 0.7 |

`Ordono` `DEVI` esti `#[non_exhaustive]`, por ke novaj variantoj ne rompu
eksterajn `match`-ojn.

- **L19.** Klipo kaj translokigo estas **stakoj**; `Malŝlipo` kaj
  `Maltranslokigo` restarigas ekzakte la antaŭan staton, ne «nuligas» ĝin.
- **L20.** Ordonoj plenumiĝas en la donita ordo; ĉiu pikselo estas
  anstataŭigata per la kunfandita rezulto de ĉiu ordono laŭvice.
- **L21.** Troa profundo aŭ mankanta malŝlipo estas alvokanta eraro; en testa
  konstruo `debug_assert!` rajtas ĝin montri, en eldona konstruo la ordono
  estas ignorata sen paniko.
- **L22.** `kovradoj` de `Masko` `DEVI` havi almenaŭ `amplekso.0 × amplekso.1`
  bajtojn; tro mallonga tranĉaĵo estas ignorata (L08), ne legata eksteren.
  La masko estas densa: vico post vico, sen paŝo.

### 3.4. Geometrio

Ortanguloj estas entjeraj, en aparat-pikseloj. Radiuso estas ne-negativa
entjero; valoro pli granda ol duono de la pli mallonga flanko estas limigata
al tiu duono.

- **L23.** Koordinatoj `DEVI` esti traktataj kiel 16.16 fiksa komo interne;
  la publika API akceptas entjerojn.
- **L24.** Ordonoj tute ekster la klipo aŭ la surfaco `DEVI` esti ignorataj
  sen efiko kaj sen eraro.
- **L25.** Ordonoj post la 256-a en la tranĉaĵo `DEVI` esti ignorataj (L08);
  la alvokanto respondecas pri la limo.

---

## 4. Rastrumado

### 4.1. Kovrado

Ĉiu pikselo ricevas **kovradon** `c` en `0..=255`, aparte de la koloro.

- **R01.** `c = 0` ne ŝanĝas la celon; `c = 255` aplikas la koloron plene.
- **R02.** Kovrado estas kalkulata per entjera aritmetiko; neniuj glitkomaj
  funkcioj (`sin`, `cos`, `sqrt`) rajtas aperi en la desegna vojo.
- **R03.** Por `Masko` la kovrado estas la maska bajto mem; por `Glifoj` ĝi
  estas `0` aŭ `255`.

### 4.2. Rondigita ortangulo

La randoj de `RondaRekto` estas kovritaj per **superprovado**:

| Versio | Specimenoj | Formulo |
| --- | --- | --- |
| 0.2–0.5 | 4 × 4 | `c = (trafoj × 255 + 8) / 16` |
| ekde 0.6 | 8 × 8 | `c = (trafoj × 255 + 32) / 64` |

- **R04.** La sama superprovado estas deviga sur ambaŭ arkoj; alie la
  paritateco de A06 rompiĝas.
- **R05.** Analiza kovrado `POVAS` anstataŭi la superprovadon nur se ĝi
  produktas identajn bajtojn por ĉiuj oraj fiksaĵoj de §7.
- **R06.** Superprovadaj specimenoj situas en la centroj de siaj subpikseloj
  (`(i × 2 + 1) / (2 × n)`); la geometria aparteno estas duonfermita, por ke
  najbaraj formoj ne kovru la saman specimenon dufoje.

### 4.3. Kunfandado

Por antaŭmultiplika fonto `(As, Cs)` kaj celo `(Ad, Cd)`, kun kovrado `c`:

```text
As' = (As × c + 127) / 255
Cs' = (Cs × c + 127) / 255
el  = Cs' + (Cd × (255 − As') + 127) / 255
```

Ĉiuj operacioj estas en `u32`; la rezulto estas garantiata `≤ 255` per la
invarianto `Cs ≤ As`.

- **R07.** La formulo estas normiga; platformaj variaĵoj (SIMD, alia rondigo)
  `DEVI NE` ŝanĝi eĉ unu bajton.
- **R08.** Post la rondigo oni `DEVI` teni `Cs' ≤ As'` (ekzemple per
  `min`); alie la invarianto de L16 rompiĝas.
- **R09.** Travideblaj surfacoj kaj fenestroj estas kunfandataj per la sama
  formulo; aparta «miksa» reĝimo ne ekzistas en 1.0.

### 4.4. Klipado

La klipo estas la komunaĵo de la surfaca ortangulo kaj ĉiuj `Klipo`-ordonoj
en la stako. Malplena klipo eligas neniun skribon.

- **R10.** Klipado `DEVI` okazi antaŭ kovrado kaj kunfandado; oni ne rajtas
  unue desegni kaj poste tranĉi.
- **R11.** La komunaĵo kalkuliĝas per `i64`, por ke `i32::MIN` ne troigu.

### 4.5. Translokigo

`Translokigo` ŝanĝas la originon de ĉiuj sekvaj geometrioj ĝis la responda
`Maltranslokigo`. Klipo restas en aparat-piksela spaco kaj estas aplikata
**post** la translokigo.

- **R12.** Nombrado de translokigoj estas entjera kaj sen troo; koordinatoj
  estas konvertataj al fiksa komo nur post la aldono.

---

## 5. Teksto

Lumo 0.1–1.0 subtenas nur **bitmapajn tiparojn**: 1-bita glifoj, kovrado
`0` aŭ `255`, sen kontraŭaliasingo.

- **T01.** La glifformato `DEVI` esti tiu de la kerna bitmAPA tiparo
  (`../mikerno/mikerno/src/ekrano/fonto.rs`): larĝo, alto, baza linio,
  antaŭeniro kaj kruda bitmAPO po glifo. La tiparaj datumoj estas
  alvokanto-provizitaj; Lumo ne ŝarĝas nek kaŝmemorigas ilin.
- **T02.** La tiparo `DEVI` kovri ĉiujn Esperantajn literojn (Ĉ Ĝ Ĥ Ĵ Ŝ Ŭ,
  minuskle kaj majuskle, kune kun la bazaj latinaj signoj). Mankanta glifo
  estas konforma eraro.
- **T03.** Mankanta glifo en nekonforma tiparo `DEVI` avanci sen desegni, ne
  paniki.
- **T04.** 1.0 `DEVI NE` subteni kernadon, kunmiksadon de skribsistemoj nek
  plur-direktan aranĝon. Tiuj apartenas al estonta tipara tavolo.
- **T05.** Teksto `DEVI` esti desegnata tra la sama kunfandada formulo (R07)
  kiel ĉiu alia koloro.
- **T06.** `bazo` estas la y-koordinato de la baza linio; la supro de la
  glifa ĉelo estas `bazo − baza_linio`. Antaŭeniro estas la glifa larĝo, sen
  interlitera spaco en 1.0.
- **T07.** Skalaj tiparoj (TrueType, OpenType) ne estas parto de 1.0. Estonta
  aparta tavolo produktos kovradajn maskojn; Lumo restos rastrumilo de
  maskoj kaj glifaj ĉeloj.

---

## 6. Kompozicio

La piktela vojo estas ĉi tiu, kaj nur ĉi tiu:

```text
Aplikaĵo → FluoU-kompona arbo → ordonaro → dukto → kompozitoro → Lumo → ekrano
```

- **K01.** Aplikaĵo `DEVI NE` aliri la frambufon rekte (FluoU P18). Ĝi nur
  konstruas FluoU-komponojn kaj lasas FluoU produkti la ordonaron.
- **K02.** La kompozitoro estas aparta servo; ĝi posedas la surfacojn kaj la
  frambufon. Lumo estas biblioteko ene de ĝi, ne servo.
- **K03.** En 1.0 la ordonaro transiras la dukton kiel ordinaraj mesaĝoj; la
  komuna memoro ne estas postulata. Se la mesaĝa limo malhelpas, oni unue
  malgrandigu la ordonaron per damaĝo, ne aldonu novan memoran vojon.
- **K04.** La kompozitoro kalkulas damaĝon kiel la union de la ortangulaj
  limoj de la ordonoj, kiuj efektive skribis almenaŭ unu pikselon; la kopio al
  la frambufo okazas unufoje po kadro.
- **K05.** Konvertiĝo al la frambufa kanala ordo (ekzemple XRGB8888) apartenas
  al la kompozitoro, ne al Lumo.
- **K06.** Malklarigo (`Malklarigi`) efikas nur al la nuna surfaco kaj nur
  interne de la klipo; ĝi ne legas ekster la surfacon.

### 6.1. Rendimento

- **P01.** La kanonika vojo estas skalaraj strioj, sen dinamika memoro; la
  celo estas 1024 × 768 sur `ramfb`.
- **P02.** SIMD (SSE2, NEON) `POVAS` veni post 1.0, nur se ĝi estas
  bit-ekzakta kun la skalara vojo kaj pruvita per D03.
- **P03.** GPU-malantaŭo ne ekzistas en 1.0. Abstrakta malantaŭaĵa trajto
  aperu nur kiam ekzistas dua malantaŭaĵo; antaŭe ĝi estus nura ŝarĝo.
- **P04.** La kompozitoro `DEVUS` desegni en RAM-surfacon kaj kopii al la
  frambufo unufoje po kadro; ripetaj MMIO-scriboj estas malpermesitaj pro
  rendimento.

---

## 7. Determinismo kaj pruvaro

- **D01.** La sama ordonaro sur la sama surfaco `DEVI` produkti identan
  bajtaron sur `x86_64` kaj `arm64`. Neniu dokumentita escepto ekzistas en
  1.0.
- **D02.** La oraj fiksaĵoj estas almenaŭ: FluoU-titolstrio (32 DIP) kun la
  tri regiloj kaj la titolo, en la normala, ŝveba kaj premata statoj, en
  malhela kaj hela temoj, je 1 DIP = 1 pikselo.
- **D03.** Ĉiu fiksaĵo havas SHA-256 en `docs/FIKSAĴOJ.md`; la gastiga
  rultempo kaj ambaŭ QEMU-arkoj `DEVI` doni la saman haketon.
- **D04.** Kani-pruvoj `DEVI` kovri almenaŭ: indeksan aritmetikon de klipo,
  la limon `el ≤ 255` de R07, la limojn de malklariga sumo, kaj la foreston
  de skribo ekster `larĝo × alto`.
- **D05.** Pruvoj `DEVI NE` esti malfortigataj per `#[allow]`, «assume» aŭ
  malŝaltitaj asertoj por igi ilin verdaj.
- **D06.** Testoj ne rajtas uzi glitkomajn referencojn; la referenco estas la
  normiga entjera formulo mem.
- **D07.** Haketa ŝanĝo `DEVI` havi linion en `docs/FIKSAĴOJ.md` kun dato,
  kaŭzo kaj bilda diferenca raporto; sen tio la ŝanĝo estas malakceptata.

La ora haketo estas la sola akceptkriterio por bildaj ŝanĝoj: ŝanĝo de
haketo sen dokumentita kaŭzo estas malakceptata.

---

## 8. Kvalita Konstitucio (neintertraktebla)

Ĉi tiu sekcio estas la «Kvalita Konstitucio» de Lumo. Ĝi estas deviga por
ĉiu Fazo, ankaŭ por dokumentaj kaj ilaj ŝanĝoj, kiuj tuŝas la pordegon.

### 8.1. Kongruo kun MiKerno

La reguloj de MiKerno estas la fonto; ili validas ankaŭ ĉi tie, kun la
adaptoj de ĉi tiu sekcio.

| MiKerno | En Lumo |
| --- | --- |
| `MEJLOSTONOJ.md` §9.1 (tri neintertrakteblaj reguloj) | Q09, Q10 |
| `MEJLOSTONOJ.md` §9.2 (pordego, Miri) | Q01–Q03, Q07 |
| `MEJLOSTONOJ.md` §4.7 (nuba DI, neniu loka Kani) | Q02 |
| `MEJLOSTONOJ.md` §4.8 (matrica Kani, templimoj, artefaktoj) | Q05, Q06 |
| `MEJLOSTONOJ.md` M18 (permesiloj) kaj `AGENTS.md` §DI | Q04, Q08 |
| `AGENTS.md` §Laborfluo | Q10, Q11 |
| FluoU A06/P17 (piksela paritateco) | Q12 |

Se MiKerno ŝanĝas regulon, Lumo sekvas en la sekva Fazo; malkongruo estas
konforma eraro.

### 8.2. Lokaj kontroloj

- **Q01.** Antaŭ ĉiu komito `just kontroli` `DEVI` esti verda. Ĝi enhavas:
  - `bash tools/versio.sh --kontroli` (Cargo.toml, LUMO.md, README kongruas);
  - `bash tools/sen-glitkomo.sh --kontroli` (neniu `f32`/`f64` en `src/`);
  - `bash tools/sen-cina.sh --kontroli` (neniu ĉina teksto en la fonta
    tavolo);
  - `cargo fmt --all -- --check`;
  - `cargo clippy --all-targets -- -D warnings`;
  - `cargo clippy --target aarch64-unknown-none -- -D warnings`;
  - `cargo test`;
  - `cargo build --release`.
- **Q02.** Loke oni `DEVI NE` ruli `cargo kani`: WSL2 elĉerpiĝas je memoro.
  La formala pordego ruliĝas nur en DI.
- **Q03.** La loka pordego `DEVI` resti ruliĝebla sen Kani kaj sen reto; la
  publikigata kratro restas sen `alloc`.
- **Q04.** Ekzistas `deny.toml` kun la sama permeslisto kiel MiKerno: nur
  MIT, Apache-2.0, BSD-3-Clause kaj Unicode-DFS-2016. GPL, AGPL kaj LGPL
  estas rifuzataj. Loke: `cargo deny check` (`just permesiloj`).
- **Q05.** `docs/PRUVOJ.md` listigas ĉiun `#[kani::proof]` unu linie.
  `bash tools/pruvoj.sh --kontroli` komparas la katalogon kun la atributoj
  en `src/` kaj malsukcesas ĉe manko aŭ troo.
- **Q06.** Kani `DEVI` esti matrica: unu tasko por ĉiu pruvo, kun
  `timeout -k 10 600`, `timeout-minutes: 30`, `fail-fast: false` kaj
  protokola artefakto ankaŭ ĉe malsukceso. Kani kaj Rust estas fiksitaj
  (Rust 1.100.0, oficiala Kani-ago).
- **Q07.** `cargo +nightly miri test` `DEVI` esti verda por ĉiuj hostaj
  testoj; Miri kaptas nedifinitan konduton, kiun la ordinara rultimo ne
  montras.
- **Q08.** `cargo deny check` `DEVI` esti verda; nova dependo bezonas
  permeson de la uzanto kaj linion en la vortaro.
- **Q09.** Oni `DEVI NE` ŝanĝi kontrolon por ke ĝi pasu: neniu `#[allow]`,
  neniu `kani::assume` por mallarĝigi la enigon, neniu forigo aŭ komentado de
  malsukcesa testo aŭ pruvo, neniu malaltigo de `-D warnings`. Se escepto
  estas vere necesa, ĝi havas justigan komenton en Esperanto, estas listigita
  aparte en la raporto kaj atendas la aprobon de la uzanto.
- **Q10.** Ĉiu Fazo finiĝas per la «Difino de preta» (§8.4); nur tiam oni
  raportas kaj atendas pluajn instrukciojn.
- **Q11.** Sen klara permeso de la uzanto oni `DEVI NE` ruli `git commit`
  nek `git push`. Nur-dokumenta komito `POVAS` uzi `[skip ci]`, kun noto ke
  la kodo ne ŝanĝiĝis.
- **Q12.** La paritateca kontrolo de §8.3 `DEVI` esti verda ekde Fazo 5; ĝi
  komparas la samajn fiksaĵojn sur ambaŭ arkoj kaj malsukcesas ĉe iu ajn
  diferenca bajto.

### 8.3. Paritateco

- La fiksaĵoj produktiĝas per `bash tools/haketi.sh` sur `x86_64` kaj, per
  `qemu-aarch64` aŭ la MiKerno-QEMU-harness, sur `arm64`.
- Oni haketas nur la videblajn pikselojn en difinita kanala ordo, neniam
  internaĵon, padron aŭ testan eligon.
- Neniu platforma escepto ekzistas en 1.0; «dokumentita escepto» ne estas
  permesata por paritateco.

### 8.4. Difino de preta

Fazo estas preta nur kiam ĉiuj jenaj estas veraj:

1. `just kontroli` verda loke.
2. DI verda: `kontroli-kaj-konstrui`, `permesiloj-kontroli`,
   `miri-kontroli`, `kani-pretigi` kaj la tuta `kani-kontroli`-matrico;
   ekde Fazo 5 ankaŭ `paritateco`.
3. La oraj haketoj estas aŭ senŝanĝaj, aŭ ŝanĝitaj kun linio en
   `docs/FIKSAĴOJ.md` laŭ D07.
4. `docs/LUMO.md` ĝisdatigita: historia tabelo, stato de la Fazo, novaj
   API-oj kaj novaj pruvoj.
5. La versio pliigita laŭ §9 kaj `bash tools/versio.sh --kontroli` verda.
6. Neniu nova `unsafe`, `alloc`, glitkomo aŭ malfortigita pruvo.
7. La liveraĵo enhavas la veran eligon de `just kontroli` kaj la konkludon de
   la Kani-matrico (kun la protokoloj de malsukcesintaj pruvoj). Sen eligo
   estas sen plenumo.
8. La rezulto raportita al la uzanto; oni haltas antaŭ la sekva Fazo.

### 8.5. Skemo de DI

`.github/workflows/di.yml` spegulas MiKernon:

| Tasko | Enhavo |
| --- | --- |
| `kontroli-kaj-konstrui` | `just kontroli` sen Kani, plus `aarch64-unknown-none`-lintado |
| `permesiloj-kontroli` | `cargo deny check` laŭ `deny.toml` |
| `miri-kontroli` | `cargo +nightly miri test` |
| `kani-pretigi` | eltiras la pruvliston el `docs/PRUVOJ.md` |
| `kani-kontroli` | matrico: unu pruvo po tasko, `--exact --harness`, templimoj de Q06 |
| `paritateco` | haketoj de ambaŭ arkoj (ekde Fazo 5) |

Flava aŭ ruĝa pordego blokas la Fazon; oni riparas la kaŭzon, ne
malŝaltas la kontrolon.

### 8.6. Laborfluo

1. Efektivigu nur la Fazon indikitan de la uzanto.
2. Rulu la lokan pordegon kaj riparu ĉion antaŭ ol raporti.
3. Ĝisdatigu la version, `docs/LUMO.md` kaj la akceptan protokolon.
4. Raportu la rezulton kaj haltu; atendu pluajn instrukciojn.

---

## 9. Fazoj — superrigardo

La versio de Lumo estas sendependa de la versio de MiKerno. Ĉiu Fazo pliigas
la malĉefan numeron; la lasta estas `1.0.0`.

| Fazo | Versio | Celo | Elirejo |
| --- | --- | --- | --- |
| 1 | 0.1.0 | fundamento: surfaco, koloro, ortangula plenigo, klipo, kunfandado | unuaj pruvoj kaj oraj haketoj |
| 2 | 0.2.0 | rondigita ortangulo, maskoj, translokigo | fenestra kadro kaj regiloj |
| 3 | 0.3.0 | bitmAPA teksto | titolstrio kun titolo |
| 4 | 0.4.0 | damaĝo kaj saturiĝo | damaĝo egala al la efektiva ŝanĝo |
| 5 | 0.5.0 | paritateco kaj plena pordego | identaj haketoj sur ambaŭ arkoj |
| 6 | 0.6.0 | kovrado 8 × 8 kaj optimumigo | pli glataj randoj sen malbona rendimento |
| 7 | 0.7.0 | Mika-malklarigo | entjera kesta malklarigo |
| 8 | 0.8.0 | eldonkandidato | dokumentaro, fuzado, buĝetoj; neniuj novaj funkcioj |
| 9 | 1.0.0 | frostigo kaj unua stabila eldono | tag `v1.0.0`, frostigita API |

La detaloj, la evitindaj eraroj kaj la akceptkriterioj estas en §10.

**Stato de la efektivigo (2026-11-22).** Fazo 1 (`0.1.0`) kaj Fazo 2
(`0.2.0`) estas pretaj; la ceteraj fazoj restas planitaj.

---

## 10. Detalaj fazoj

### 10.1. Fazo 1 — 0.1.0 «Fundamento»

**Celo.** Starigi la pordan infrastrukturon kaj la plej malgrandan
rastrumilon, kiu povas esti pruvita: ortangula plenigo kun klipo kaj la
normiga kunfandado.

**Enhavo.**

1. `Surfaco::nova(pikseloj, larĝo, alto, paŝo) -> Option<Surfaco>` kun
   kontrolo per `u64` (L14).
2. `Koloro::nova(a, r, g, b) -> Option<Koloro>` kun la invarianto de L16.
3. `Ordono` (`#[non_exhaustive]`) kun `Plenigi`, `Klipo`, `Malŝlipo`.
4. `desegnu(surfaco: &mut Surfaco, ordonoj: &[Ordono])`; klipa stako 8;
   ordo de L20; kunfandado de §4.3.
5. `justfile`, `deny.toml`, `.github/workflows/di.yml` laŭ §8.5;
   `tools/versio.sh`, `tools/sen-glitkomo.sh`, `tools/sen-cina.sh`,
   `tools/haketi.sh`, `tools/pruvoj.sh`; `docs/PRUVOJ.md`,
   `docs/FIKSAĴOJ.md`.
6. Du Kani-pruvoj: `miksi_neniam_troas` (R07) kaj
   `klipo_restas_en_surfaco` (L15).

**Evitindaj eraroj.**

- Konfuzi `paŝo` (pikseloj) kun bajtoj; tio kaŝe skribas eksteren.
- Multipliki `larĝo × alto` en `u32`; uzu `u64` kaj `checked_*`.
- Konverti negativajn `i32`-koordinatojn al `usize` antaŭ klipado; klipu
  unue, en `i64`.
- Fidi la rondigon de `+127` por konservi `Cs ≤ As`; aldonu `min` (R08).
- Uzi `debug_assert!` kiel la solan limkontrolon; eldona konstruo ankaŭ
  `DEVI` klipi.
- Uzi glitkomon aŭ hazardan iteracian ordon en testoj (D06).
- Forgesi `#[non_exhaustive]`; poste aldono de `RondaRekto` rompus uzantojn.
- Meti `std` en la bibliotekan vojon; nur `cfg(test)` rajtas ĝin uzi.

**Akcepto.**

1. `just kontroli` verda (Q01), inkluzive de la `no_std`-lintado por
   `aarch64-unknown-none`.
2. DI: `kontroli-kaj-konstrui`, `permesiloj-kontroli`, `miri-kontroli`
   verdaj; `kani-pretigi` kaj la matrico kun la du pruvoj verdaj.
3. Ora haketo por «plena ortangulo» kaj por «klipita ortangulo» registrita en
   `docs/FIKSAĴOJ.md` kaj reproduktita de `tools/haketi.sh --kontroli`.
4. `tools/sen-glitkomo.sh` verda; neniu `f32`/`f64` en `src/`.
5. Ĉiuj publikaj eroj havas dokumentaron en Esperanto;
   `RUSTDOCFLAGS='-D missing-docs' cargo doc --no-deps` verda.
6. `git diff --check` sen blankaj eraroj; versio `0.1.0` kongrua en
   `Cargo.toml`, `README.md` kaj ĉi tiu dokumento.

**Stato (2026-11-22).** Efektivigitaj: `Surfaco::nova` (`u64`-kontrolo, L14,
L15), `Koloro::nova` kun la premultiplika invarianto (L16, L18), `Rektangulo`,
`Ordono` (`#[non_exhaustive]`; `Plenigi`, `Klipo`, `Malŝlipo`) kaj `desegnu`
(klipa stako 8, ordo de L20, normiga kunfandado de §4.3). La internaj klipaj
limoj estas en 16.16 fiksa komo kaj kalkuliĝas per `i64` (L23, R11). La loka
pordego (`justfile`, `deny.toml`, `tools/versio.sh`, `tools/sen-glitkomo.sh`,
`tools/sen-cina.sh`, `tools/haketi.sh`, `tools/pruvoj.sh`) kaj la DI-matrico
estas en la deponejo. La du pruvoj estas `miksi_neniam_troas` (R07) kaj
`klipo_restas_en_surfaco` (L15) en `docs/PRUVOJ.md`; la oraj haketoj
`plena-ortangulo` kaj `klipita-ortangulo` estas registritaj en
`docs/FIKSAĴOJ.md` kaj reproduktataj de `tools/haketi.sh --kontroli`.

### 10.2. Fazo 2 — 0.2.0 «Rondoj, maskoj, translokigo»

**Celo.** Desegni la FluoU-fenestran kadron kaj la tri regilojn sen teksto:
rondigitaj anguloj, ikonoj per maskoj kaj translokigita geometrio.

**Enhavo.**

1. `RondaRekto` kun 4 × 4 superprovado (R04–R06), limigo de radiuso al duono
   de la pli mallonga flanko kaj `radiuso = 0` egala al `Plenigi`.
2. `Masko` kun 8-bita kovrado (R03, L22); la maskoj de la FluoU-ikonoj estas
   datumoj de la alvokanto, ne de Lumo.
3. `Translokigo`/`Maltranslokigo` kun stako 8 (R12) kaj la ordo «geometrio,
   poste klipo».
4. Kadraj oraj fiksaĵoj (sen titolo): regila grupo en normala, ŝveba kaj
   premata statoj, malhela kaj hela temoj.
5. Komunranda testo: du apudaj formoj produktas la samajn pikselojn kiel unu
   forma unio, sen duobla kovrado ĉe la komuna rando.
6. Kani-pruvo `rondigo_kovrado_en_limoj` (kovrado ≤ 255).

**Evitindaj eraroj.**

- Meti specimenajn centrojn sur la randon; specimeno `DEVI` aparteni al
  ekzakte unu pikselo (R06).
- Rondigi la kovradon per aliula konstanto anstataŭ la normigaj `+8` (4 × 4)
  kaj `+32` (8 × 8); tio ŝanĝas ĉiujn haketojn.
- Forgesi limigi radiuson; granda radiuso povas legi ekster la ortangulo.
- Apliki translokigon al la klipo mem; klipo restas en aparat-piksela spaco
  (R12).
- Legi maskon per `paŝo` de la surfaco; la masko estas densa, sen paŝo.
- Superprovadi la tutan surfacon anstataŭ nur la ortangulan limon.
- Uzi malsamajn specimenajn formulojn en tekstoj kaj bildoj; unu formulo por
  ĉiuj (R04).

**Akcepto.**

1. `just kontroli` kaj la tuta DI verda; la nova pruvo en la matrico.
2. Ses kadraj haketoj registritaj; la komunranda testo verda por ĉiuj paroj
   de najbaraj formoj.
3. `radiuso = 0` donas bajte identan rezulton al `Plenigi` por ĉiuj
   fiksaĵaj ortanguloj.
4. Masko kun tro mallonga `kovradoj` estas ignorata sen paniko kaj sen
   skribo (L08, L22).
5. Translokiga stako: troa profundo kaj mankanta `Maltranslokigo` kondutas
   laŭ L21 (testa `debug_assert!`, eldona ignorado).
6. Rendimenta mezuro registrita en `docs/RENDIMENTO.md` (plen-ekrana
   `RondaRekto` 1024 × 768 en eldona konstruo); ĝi estas la referenco por la
   buĝetoj de Fazo 6.
7. Neniu haketo de Fazo 1 ŝanĝiĝis.

**Stato (2026-11-22).** Efektivigitaj: `Ordono::RondaRekto` (4 × 4
superprovado kun specimenaj centroj `(2i + 1)/16`, R04–R06; radiuso limigita
al duono de la pli mallonga flanko; `radiuso = 0` egala al `Plenigi`),
`Ordono::Masko` (densa 8-bita kovrado, R03, L22; tro mallonga `kovradoj`
ignorata), `Ordono::Translokigo`/`Maltranslokigo` (stako 8, R12; klipo restas
en aparat-piksela spaco, L21 en testo per `debug_assert!`) kaj la komunranda
testo por `Plenigi`, `RondaRekto` kaj `Masko`. La oraj haketoj
`kadro-malhela-*` kaj `kadro-hela-*` (ses statoj) estas registritaj en
`docs/FIKSAĴOJ.md` kaj reproduktataj de `tools/haketi.sh --kontroli`; la du
haketoj de Fazo 1 restas senŝanĝaj. La nova pruvo `rondigo_kovrado_en_limoj`
(R04–R06) estas en `docs/PRUVOJ.md` kaj en la DI-matrico. La mezuroj de la
kanonika vojo estas en `docs/RENDIMENTO.md`.

### 10.3. Fazo 3 — 0.3.0 «BitmAPA teksto»

**Celo.** Aldoni la titolon al la titolstrio per la bitmAPA tiparo de
MiKerno, sen `alloc` kaj sen tipara kaŝmemoro.

**Enhavo.**

1. `Tiparo` kun `larĝo`, `alto`, `paŝo`, `baza_linio`, glifoj kaj la
   Esperanta indeksa tabelo de la MiKerno-formato (117 glifoj: ASCII,
   `ĉ Ĉ ĝ Ĝ ĥ Ĥ ĵ Ĵ ŝ Ŝ ŭ Ŭ`, citiloj kaj simboloj).
2. Importo de la 8 × 16 kaj 12 × 24 tiparaj datumoj el MiKerno (MIT, sama
   aŭtoro) kun `third_party/dejavu/LICENSE.txt`, nur en la testan fiksaĵaron
   (ne en la publikigatan kraton); neniuj generiloj bezonataj en 1.0.
3. `Glifoj` kun `origino`, `bazo`, `koloro`, `tiparo`, `tekstero` (T01,
   T06); mankanta glifo avancas sen desegni (T03).
4. La ses titolstriaj fiksaĵoj de D02, aldone al la ses kadraj fiksaĵoj de
   Fazo 2, nun kun la titolo.
5. Kani-pruvo `glifo_indekso_en_limoj` kaj unu-testo pri la plena Esperanta
   kovrado.

**Evitindaj eraroj.**

- Konfuzi la supran maldekstran angulon kun la baza linio; uzu T06 kaj
  testu per la deklinaj literoj.
- Legi glifan vicon per la glifa `paŝo` anstataŭ la fiksa `(larĝo + 7) / 8`.
- Preni la plej maldekstran biton kiel la plej malaltan biton de la bajto;
  la formato estas plej-supra-bito = plej-maldekstra.
- Uzi `?` por mankanta glifo; Lumo avancas sen desegni, la FluoU-tiparo
  devas esti konforma.
- Trakti kombinajn signojn kiel subtenatajn; 1.0 subtenas nur antaŭkunmetitajn
  Esperantajn literojn.
- Nombrigi tekstojn per `char` en `u8`; uzu `char_indices`.
- Forgesi avanci glifojn, kiuj estas tute klipitaj.

**Akcepto.**

1. `just kontroli` kaj la tuta DI verda; la nova pruvo en la matrico.
2. La ses kadraj haketoj de Fazo 2 restas senŝanĝaj; ses novaj titolstriaj
   haketoj registriĝas en `docs/FIKSAĴOJ.md`.
3. Kovrada testo: ĉiu Esperanta litero en ambaŭ uzkazoj havas glifon;
   mankanta signo avancas sen skribo.
4. Testoj por `bazo`, `baza_linio` kaj antaŭeniro kun konataj koordinatoj.
5. Neniu `alloc`, glitkomo aŭ `unsafe` en `src/`.

### 10.4. Fazo 4 — 0.4.0 «Damaĝo»

**Celo.** Raporti al la kompozitoro la ekzaktan union de skribitaj pikseloj,
por ke ĝi kopiu al la frambufo nur la necesan regionon.

**Enhavo.**

1. `Damaĝo` kun fiksa kapacito 16 ortanguloj, `aldoni`, kunfandi
   interkovrantajn kaj apudajn ortangulojn, kaj saturiĝo.
2. `desegnu_damagon(surfaco, ordonoj, &mut Damaĝo)`; ĉiu ordono raportas, ĉu
   ĝi skribis almenaŭ unu pikselon; neskribintaj ordonoj ne damaĝas.
3. Satura regulo: super 16 disaj ortanguloj la damaĝo fariĝas la klipo
   (aŭ la surfaco), kun flago `plena`.
4. Du-pasa testo: la sama ordonaro sur du surfacoj kun antaŭa bildo komparas
   la ŝanĝitajn pikselojn kun la raportita damaĝo.

**Evitindaj eraroj.**

- Inkludi ordonojn, kiuj estas plene klipitaj aŭ havas kovradon 0; tio
  malplenigos la damaĝon.
- Kalkuli damaĝon antaŭ la fina klipo kaj translokigo.
- Uzi `u32` por la unio; `i32::MIN` kaj `i32::MAX` troigas.
- Forgesi la kovradon de randoj; specimeno kun `c = 0` ne estas skribo.
- Silente perdi ortangulojn ĉe saturiĝo; la flago `plena` `DEVI` esti
  videbla por la kompozitoro.
- Ŝanĝi la bildon dum aldono de damaĝo; la du pasaĵoj `DEVI` esti identaj.

**Akcepto.**

1. `just kontroli` kaj la tuta DI verda; almenaŭ unu nova pruvo aŭ unu
   limigita Kani-pruvo pri la saturiĝo.
2. Determinitaj hazardaj testoj (fiksa semo, sen ekstera kesto): por 1000
   ordonaroj sur 32 × 32, ĉiu ŝanĝita pikselo kuŝas en la damaĝo, kaj ĉiu
   damaĝa ortangulo enhavas almenaŭ unu ŝanĝitan pikselon.
3. Satura testo: pli ol 16 disaj skriboj donas `plena = true` kaj la
   konservativan damaĝon.
4. Ordonoj sen skribo produktas malplenan damaĝon.
5. Ĉiuj haketoj de Fazo 3 restas senŝanĝaj (damaĝo ne rajtas ŝanĝi bildon).
6. Rendimenta buĝeto: la damaĝa kalkulo ne pli ol 10 % pli malrapidas ol la
   baza desegno por la fiksaĵoj.

### 10.5. Fazo 5 — 0.5.0 «Paritateco kaj plena pordego»

**Celo.** Fermi la unuan duonon: la sama kodo donas la samajn bajtojn sur
ambaŭ arkoj, kaj la tuta kvalita pordego estas matrica kaj deviga.

**Enhavo.**

1. `tools/haketi.sh` kun du reĝimoj: `x86_64` sur la gastiganto kaj
   `arm64` per `qemu-aarch64` aŭ la MiKerno-QEMU-harness.
2. CI-tasko `paritateco`, kiu malsukcesas ĉe iu ajn diferenca bajto.
3. `docs/PRUVOJ.md` kompleta; `tools/pruvoj.sh --kontroli` komparas ĝin kun
   la `#[kani::proof]`-atributoj.
4. `tools/versio.sh`, `deny.toml` kaj la ses DI-taskoj de §8.5.
5. Dokumentado de la haketa formo: nur videblaj pikseloj, eksplicita kanala
   ordo, neniu padro.

**Evitindaj eraroj.**

- Haketi la tutan bufron kun `paŝo`; la padro malsamas inter alvokoj.
- Uzi `usize` en la geometrio; la larĝo povas malsami inter celoj. Uzu
  `u32`/`u64` kaj `usize` nur por indeksoj.
- Fidi hazardan testan ordon aŭ `HashMap`-ordigadon; ne ekzistas `HashMap`,
  sed la principo restas.
- Lasu la Kani-matricon «verda» per malplena listo aŭ per `--harness`
  kongruanta nenion; `tools/pruvoj.sh` `DEVI` kapti tion.
- Uzi alian Rust-version ol la fiksitan; la pordego `DEVI` esti
  reproduktebla.
- Miksi la kanalan ordon en la haketo (ARGB kontraŭ XRGB).

**Akcepto.**

1. Ĉiuj kvin DI-taskoj verdaj; unu matrica tasko po pruvo; la katalogo
   kongruas.
2. Ĉiuj fiksaĵoj havas identan SHA-256 sur `x86_64` kaj `arm64` (D01, D03).
3. `cargo deny check` verda laŭ la permeslisto de Q04.
4. Miri verda; neniu `#[allow]` aŭ malfortigo en `src/`.
5. `tools/versio.sh --kontroli` verda; versio `0.5.0` ĉie kongrua.
6. La pordego estas dokumentita en `README.md` kaj ĉi tiu dokumento.

### 10.6. Fazo 6 — 0.6.0 «Kovrado 8 × 8 kaj optimumigo»

**Celo.** Pli glataj randoj kaj pli rapida strio-laboro, sen rompi la
paritatecon aŭ la komunrandan garantion.

**Enhavo.**

1. Superprovado 4 × 4 → 8 × 8 (`c = (trafoj × 255 + 32) / 64`) por
   `RondaRekto`, kun la sama specimena aranĝo (R06).
2. Bildaj diferencoj kaj novaj haketoj dokumentitaj laŭ D07.
3. Strio-bazita optimumigo: kovrado kalkuliĝas unufoje po strio, ne po
   pikselo, kie eble; neniuj `unsafe` aŭ SIMD.
4. Rendimenta buĝeto: la pli alta kovrado ne pli ol 1,5-oble pli malrapida
   ol Fazo 5, kun dokumentita mezuro.
5. Komunrandaj testoj ripetitaj kun la 8 × 8-formulo.

**Evitindaj eraroj.**

- Ŝanĝi la specimenan formulon aŭ la duonfermitan apartenon; tio kaŝe
  duobligas randojn.
- Forgesi dokumenti la haketŝanĝon; la pordego malakceptos ĝin (D07).
- «Optimumigi» per platforma `cfg`; tio rompas L13 kaj D01.
- Uzi glitkomon por kalkuli la specimenan centron; ĉio restas entjera.
- Superprovadi ankaŭ la internon de la ortangulo; nur la rando bezonas
  specimenojn.
- Malpliigi la kovradon por «gajni rapidon»; la bildo ne rajtas malamiĝi
  sen decido de la uzanto.

**Akcepto.**

1. `just kontroli` kaj la tuta DI verda; paritateco verda.
2. Novaj haketoj dokumentitaj kun bilda diferenca raporto kaj aprobo de la
   uzanto.
3. Komunrandaj testoj verdaj; `radiuso = 0` ankoraŭ egala al `Plenigi`.
4. Rendimento mezurita kaj registrita; la buĝeto de Fazo 6 respektata.
5. Neniu `unsafe`, SIMD, glitkomo aŭ platforma `cfg` en `src/`.
6. Ĉiuj pruvoj de Fazo 5 ankoraŭ verdaj.

### 10.7. Fazo 7 — 0.7.0 «Mika-malklarigo»

**Celo.** Subteni la travideblan materialon Mika per entjera kesta
malklarigo de la suba surfaco.

**Enhavo.**

1. `Malklarigi { rektangulo, radiuso }` kun limigo `radiuso ≤ 64` (K06).
2. Du-pasa kesta malklarigo (horizontalo, poste vertikalo), antaŭmultiplika,
   kun randa limigo kaj duon-alta divido.
3. Alvokanto-provizita skrapa bufro; se ĝi mankas aŭ estas tro malgranda,
   la ordono estas ignorata sen skribo.
4. Oraj malklarigaj fiksaĵoj kaj damaĝo de la malklariga regiono.
5. Kani-pruvoj: suma limo `≤ 255 × n`, sen troo, kaj radiusa limo.

**Evitindaj eraroj.**

- Malklarigi senpremultipikitajn kanalojn; travideblo ŝanĝiĝas kaj «brilas».
- Uzi glitkomon aŭ dividi per `n` sen rondigo; la formulo `DEVI` esti
  `(sumo + n / 2) / n`.
- Legi ekster la regiono aŭ la surfaco; la rando `DEVI` esti limigita al la
  randa pikselo, ne al nulo.
- Reuzi la saman skrapbufron por la du pasaĵoj samtempe; la dua legas la
  rezulton de la unua.
- Forgesi la damaĝon; la kompozitoro ne vidos la malklaron.
- Permesi senfinan radiuson; la buĝeto kaj la limo de §3.3 `DEVI` resti.

**Akcepto.**

1. `just kontroli` kaj la tuta DI verda; paritateco verda.
2. `radiuso = 0` estas senskriba; `radiuso > 64` estas limigata aŭ ignorata
   laŭ dokumentita regulo.
3. Malklarigaj haketoj registritaj; la bilda diferenco montras neniun
   alian ŝanĝon.
4. Skrapa bufro tro malgranda: neniu skribo, neniu paniko.
5. Kani-pruvoj pri la suma limo kaj la radiuso verdaj.
6. Rendimenta buĝeto: malklarigo de 256 × 256 kun radiuso 8 sub la
   registrita limo.

### 10.8. Fazo 8 — 0.8.0 «Eldonkandidato»

**Celo.** Frostigi la konduton kaj la dokumentaron sen aldoni funkciojn;
fari la eldonon ripeteblan.

**Enhavo.**

1. Publika API-revizio: nomoj, vivoj, `Option`-kondutoj, `#[non_exhaustive]`
   kaj dokumentado de ĉiu ero.
2. `docs/PRUVOJ.md`, `docs/FIKSAĴOJ.md` kaj `docs/RENDIMENTO.md` kompletaj;
   historia tabelo de `docs/LUMO.md` ĝisdatigita.
3. Determinitaj fuzaj testoj por malbonaj ordonaroj (troaj profundoj,
   malplenaj klipoj, tro mallongaj maskoj, tro grandaj radiusoj); neniuj
   eksteraj fuzaj iloj.
4. Buĝetoj: grandeco de la publikaj tipoj, nombro da pruvoj, rendimentaj
   limoj; neniuj `TODO`, `unimplemented!` aŭ `panic!` en la desegna vojo.
5. Reproduktebla konstruo: fiksitaj Rust kaj Kani; `Cargo.lock` ne
   publikigata por biblioteko, sed la pordego uzas fiksitan ilarĉenon.
6. Neniu nova funkcio; nur riparoj kaj dokumentaro.

**Evitindaj eraroj.**

- Aldoni «etajn» funkciojn dum la kandidato; la Fazo 9 frostigus ilin sen
  revizio.
- Forgesi ekzemplojn aŭ dokumentadon de la vivodaŭroj en `Ordono`.
- Uzi hazardajn semojn en testoj; la pordego `DEVI` esti determinita.
- Paniki ĉe malbona enigo; L08 kaj la fuzaj testoj `DEVI` kapti tion.
- Ŝanĝi haketojn por «purigi» la kodon; la bildo estas la normo.

**Akcepto.**

1. `just kontroli` kaj la tuta DI verda; ĉiuj pruvoj kaj paritateco verdaj.
2. Ĉiuj publikaj eroj havas dokumentaron; `cargo doc` sen mankaj avertoj.
3. La fuzaj testoj kaj la negativaj kazoj verdaj; neniu paniko.
4. La buĝetoj registritaj kaj respektataj.
5. Neniu funkcia ŝanĝo kompare kun Fazo 7 krom riparoj; ĉiuj haketoj
   senŝanĝaj.
6. La eldonnotoj kaj la akcepta protokolo pretaj por Fazo 9.

### 10.9. Fazo 9 — 1.0.0 «Frostigo kaj unua stabila eldono»

**Celo.** Publikigi la unuan stabilan version kaj frostigi la publikan API-on.

**Enhavo.**

1. Versio `1.0.0` en `Cargo.toml`, `README.md` kaj ĉi tiu dokumento.
2. Frostigaj deklaroj: `Surfaco`, `Koloro`, `Ordono`, `Tiparo`, `Damaĝo`,
   `desegnu` kaj `desegnu_damagon`; rompa ŝanĝo post 1.0 postulas
   `2.0.0`.
3. `docs/HISTORIO.md` kun la naŭ fazoj, iliaj haketoj kaj la pruvokatalogo.
4. Etikedo `v1.0.0` kaj eldonnotoj (nur kun permeso de la uzanto, Q11).
5. Fina pordego: `just kontroli`, Miri, `cargo deny`, la tuta Kani-matrico
   kaj paritateco.

**Evitindaj eraroj.**

- Etikedi antaŭ ol la DI estas verda aŭ antaŭ ol la haketoj estas
  registritaj.
- Aldoni funkcion dum la frostigo; nur dokumentaj kaj riparaj ŝanĝoj.
- Forgesi la historian tabelon aŭ la frostigan deklaron.
- Publikigi ISO aŭ bildon el nepruvita kodo; Lumo ne eldonas ISO-ojn, sed la
  kompozitoro de Horizonto uzos ĝin nur post verda 1.0.
- Promesi skalajn tiparojn en 1.0; ili apartenas al estonta tipara tavolo
  (T07).

**Akcepto.**

1. `just kontroli` verda; la tuta DI verda, inkluzive de la matrico kaj
   paritateco.
2. Ĉiuj haketoj de Fazo 8 senŝanĝaj; neniu bilda diferenco.
3. La API estas dokumentita kaj frostigita; la katalogo de pruvoj estas
   kompleta.
4. `bash tools/versio.sh --kontroli` verda kun `1.0.0`.
5. La eldonnotoj listigas la limojn (bitmAPA teksto, neniu TrueType, neniu
   SIMD, neniu GPU) kaj la post-1.0-planon.
6. La uzanto aprobis la etikedon kaj la eldonon antaŭ ol ili okazas.

### 10.10. Post 1.0

La jenaj temoj estas ekster la frostigo kaj bezonas apartajn dokumentojn:

- tipara tavolo (TrueType/OpenType → kovradaj maskoj);
- SIMD-vojoj, nur se bit-ekzaktaj;
- GPU-malantaŭo, nur kiam ekzistas dua malantaŭaĵo (P03);
- analiza kovrado, se ĝi pruviĝas identa (R05);
- pli fajna damaĝo kaj plurbuĝeta optimumigo.

---

## 11. Rilato al MiKerno, FluoU kaj Horizonto

- Lumo estas aparta projekto kaj `DEVI NE` dependi de MiKerno, FluoU nek
  Horizonto.
- La dependo iras unu-direkte: la kompozitoro de Horizonto dependos de Lumo.
- FluoU restas la **dezajna normo**; Lumo estas nur la **rastrumilo**. La
  estonta kratro `fluou` produktos `Ordono`-aron el la kompona arbo.
- Ĵetonoj, materialoj kaj movo restas en FluoU; Lumo ricevas nur pretajn
  kolorojn kaj geometrion (L11).
- La kvalita pordego sekvas MiKernon (§8), sed la versioj kaj la fazoj de
  Lumo estas sendependaj.

---

## 12. Vortareto

| Esperanto | Angla | Rimarko |
| --- | --- | --- |
| rastrumilo | rasterizer | Lumo mem |
| surfaco | surface | alvokanto-provizita piksela bufro |
| ordonaro | display list | fiksa listo de `Ordono` |
| kovrado | coverage | `u8` por pikselo |
| kunfandado | blending | normiga formulo de §4.3 |
| antaŭmultiplika | premultiplied | kanaloj jam multiplikitaj per `a` |
| fiksa komo | fixed point | 16.16, sen glitkomo |
| oraj fiksaĵoj | golden fixtures | haketitaj bildoj por D02–D03 |
| damaĝo | damage | regiono, kiun la kompozitoro kopias |
| malklarigo | blur | entjera kesta malklarigo |
| kvalita konstitucio | quality constitution | §8 de ĉi tiu dokumento |

---

## Aldono A. Historio de la dokumento

| Dato | Ŝanĝo |
| --- | --- |
| 2026-11-21 | Unua versio: celoj, principoj, objektoj, rastrumado, teksto, kompozicio, determinismo, vojmapo |
| 2026-11-21 | Aldonitaj la kvalita konstitucio (§8) kaj la naŭ detalaj fazoj (§10); maskoj kaj translokigo en §3.3; kovrado 8 × 8 ekde 0.6; TrueType movita al post-1.0 (T07) |
| 2026-11-22 | Fazo 1 efektivigita (0.1.0): `Surfaco`, `Koloro`, `Rektangulo`, `Ordono` kaj `desegnu`; la loka pordego kaj la DI-matrico; la du Kani-pruvoj kaj la oraj haketoj |
| 2026-11-22 | Fazo 2 efektivigita (0.2.0): `RondaRekto` kun 4 × 4 superprovado, `Masko`, `Translokigo`/`Maltranslokigo`; ses kadraj haketoj, la pruvo `rondigo_kovrado_en_limoj` kaj `docs/RENDIMENTO.md` |

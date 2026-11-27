# Rendimento de Lumo

Ĉi tiu dokumento registras la mezurojn de la kanonika skalara vojo. Ili estas
la referenco por la buĝetoj de Fazo 6 (0.6.0) kaj ne estas pordego: la pordego
estas la oraj haketoj kaj la pruvoj.

## Metodo

`examples/rendimento.rs` mezuras plen-ekranan surfacon de 1024 × 768 pikseloj
kun densa paŝo (`paŝo = larĝo`), en eldona konstruo:

```sh
cargo run --release --example rendimento
```

Ĉiu mezuro konsistas el tri varmigaj plenumoj kaj 20 mezuritaj plenumoj; la
eligita nombro estas la mezo. La bufro estas hazarde legata (`black_box`) post
ĉiu plenumo, por ke la optimumigilo ne forigu la skribojn. Du ordonoj estas
mezurataj sur la sama surfaco:

- `Plenigi` de la tuta surfaco (unu ordono);
- `RondaRekto` de la tuta surfaco kun `radiuso = 32` (unu ordono).

## Mezurita aparataro

| Ero | Valoro |
| --- | --- |
| CPU | Intel Core i5-8365U @ 1.60 GHz (4 kernoj / 8 fadenoj) |
| Sistemvokoj | WSL2, Linux 6.18, x86_64 |
| Ilĉeno | rustc 1.100.0 (2026-11-09), eldona profilo |

## Rezultoj (2026-11-22, Lumo 0.2.0)

| Ordono | µs/plenumo (kvar ruloj) | Mezo |
| --- | --- | --- |
| `Plenigi` (plena surfaco) | 3537, 3610, 4750, 5836 | ≈ 4,4 ms |
| `RondaRekto` (radiuso 32) | 10773, 11092, 12504, 18235 | ≈ 13,2 ms |

La disvastiĝo venas de WSL2 kaj de la planilo de la gastiganto; la rilato
inter la du ordonoj estas stabila. `RondaRekto` kostas proksimume trioble pli
ol `Plenigi`, ĉar ĝi superprovadas nur la angulajn regionojn (4 × 4 specimenoj
po angula pikselo) kaj traktas la ceterajn pikselojn per la rapida vojo de
`Plenigi`.

## Buĝeto de damaĝo (2026-11-26, Lumo 0.3.0)

La Fazo 4-buĝeto komparas la saman ordonaron per `desegnu` kaj
`desegnu_damagon` sur 1024 × 768. La bufro estas restarigita antaŭ ĉiu
mezurado, por ke ambaŭ vojoj skribu la saman bildon. Ĉiu rezulto estas la
mezo de 20 mezuroj post tri varmigoj; la kvar ruloj estas apartaj lanĉoj de
`cargo run --release --example rendimento`.

| Ordonaro | Bazaj ruloj (µs) | Kun damaĝo (µs) | Mediano baza | Mediano kun damaĝo |
| --- | --- | --- | --- | --- |
| `Plenigi` | 6500, 7167, 7816, 8305 | 6132, 6836, 7583, 8284 | ≈ 7,5 ms | ≈ 7,2 ms |
| `RondaRekto` (radiuso 32) | 10773, 17132, 19155, 14995 | 10952, 16355, 18694, 15913 | ≈ 16,1 ms | ≈ 16,1 ms |

La plej granda malrapidiĝo inter la parigitaj mezuroj estis 6,2 % por
`RondaRekto`, sub la Fazo 4-limo de 10 %. La WSL2-planilo kaŭzas rimarkindan
disvastiĝon; ĉi tiuj nombroj estas referenco, ne aparta pordego.

## Interpretado

- La normiga kunfandado (R07–R08) kaj la klipo (R10) estas la samaj por ambaŭ
  ordonoj; la diferenco estas la kovrada kalkulo.
- La mezuroj estas por la skalara vojo. Fazo 6 rajtas optimumigi ĝin, sed nur
  se la oraj haketoj kaj la piksela paritateco restas senŝanĝaj (Q01, D02–D03).
- La mezuro de Fazo 4 komparas la bazan desegnon kaj la kolektadon de damaĝo;
  la buĝeto estas maksimume 10 % da malrapidiĝo por la mezuritaj ordonaroj.
- La buĝeto estos fiksita en Fazo 6 surbaze de ĉi tiuj nombroj, sur la sama
  aparataro aŭ sur pli forta, kun la sama metodo.

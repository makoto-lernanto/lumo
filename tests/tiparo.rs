//! Testoj pri la realaj MiKerno-tiparaj datumoj (T01–T03).
//!
//! La datumoj estas kopioj de `mikerno/src/ekrano/fonto` (MIT, sama aŭtoro)
//! kaj restas nur en la testa fiksaĵaro; ili ne eniras la publikigatan kraton.
//! Oni enigas ilin per `include_bytes!`, do la testoj ne legas dosierojn kaj
//! ruliĝas ankaŭ sub Miri kun izolado.

use lumo::{Koloro, Ordono, Surfaco, Tiparo, desegnu};

const TIPARO_8X16: &[u8] = include_bytes!("fiksaĵoj/tiparo-8x16.bin");
const TIPARO_12X24: &[u8] = include_bytes!("fiksaĵoj/tiparo-12x24.bin");

fn kun_tiparo<F: FnOnce(&Tiparo<'_>)>(
    datumoj: &[u8],
    larĝo: u32,
    alto: u32,
    baza_linio: u32,
    testo: F,
) {
    let tiparo = Tiparo::nova(larĝo, alto, baza_linio, datumoj).unwrap();
    testo(&tiparo);
}

fn desegni(tiparo: &Tiparo<'_>, origino: i32, bazo: i32, tekstero: &str) -> [u32; 1024] {
    let koloro = Koloro::nova(255, 255, 255, 255).unwrap();
    let mut bufro = [0u32; 1024];
    {
        let mut surfaco = Surfaco::nova(&mut bufro, 32, 32, 32).unwrap();
        desegnu(
            &mut surfaco,
            &[Ordono::Glifoj {
                origino,
                bazo,
                koloro,
                tiparo,
                tekstero,
            }],
        );
    }
    bufro
}

fn havas_inkon(tiparo: &Tiparo<'_>, baza_linio: u32, signo: char) -> bool {
    let teksto = signo.to_string();
    desegni(tiparo, 0, baza_linio as i32, &teksto)
        .iter()
        .any(|pikselo| *pikselo != 0)
}

#[test]
fn la_tiparoj_havas_la_konatajn_metrikojn() {
    kun_tiparo(TIPARO_8X16, 8, 16, 12, |tiparo| {
        assert_eq!((tiparo.larĝo(), tiparo.alto()), (8, 16));
        assert_eq!(tiparo.paŝo(), 1);
        assert_eq!(tiparo.baza_linio(), 12);
    });
    kun_tiparo(TIPARO_12X24, 12, 24, 18, |tiparo| {
        assert_eq!((tiparo.larĝo(), tiparo.alto()), (12, 24));
        assert_eq!(tiparo.paŝo(), 2);
        assert_eq!(tiparo.baza_linio(), 18);
    });
}

#[test]
fn ĉiu_esperanta_litero_havas_inkon_en_ambaŭ_tiparoj() {
    for (nomo, datumoj, larĝo, alto, baza_linio) in [
        ("tiparo-8x16", TIPARO_8X16, 8, 16, 12),
        ("tiparo-12x24", TIPARO_12X24, 12, 24, 18),
    ] {
        kun_tiparo(datumoj, larĝo, alto, baza_linio, |tiparo| {
            for signo in "ĉĈĝĜĥĤĵĴŝŜŭŬAa".chars() {
                assert!(
                    havas_inkon(tiparo, baza_linio, signo),
                    "{nomo}: {signo:?} havu inkarkon"
                );
            }
        });
    }
}

#[test]
fn mankanta_glifo_avancas_sen_skribi() {
    kun_tiparo(TIPARO_8X16, 8, 16, 12, |tiparo| {
        assert!(!havas_inkon(tiparo, 12, '€'));
        assert_eq!(desegni(tiparo, 0, 12, "A A"), desegni(tiparo, 0, 12, "AéA"));
    });
}

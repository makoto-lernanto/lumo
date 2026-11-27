//! Mezuras la kanonikan skalaran vojon por `docs/RENDIMENTO.md`.
//!
//! Uzo: `cargo run --release --example rendimento`.

use std::hint::black_box;
use std::time::Instant;

use lumo::{Damaĝo, Koloro, Ordono, Rektangulo, Surfaco, desegnu, desegnu_damagon};

const LARĜO: u32 = 1024;
const ALTO: u32 = 768;
const VARMIGO: u32 = 3;
const RIPETOJ: u32 = 20;

fn mezuri(ordonoj: &[Ordono<'_>], bufro: &mut [u32]) -> u128 {
    for _ in 0..VARMIGO {
        let mut surfaco = Surfaco::nova(&mut bufro[..], LARĜO, ALTO, LARĜO).unwrap();
        desegnu(&mut surfaco, ordonoj);
    }
    let start = Instant::now();
    for _ in 0..RIPETOJ {
        let mut surfaco = Surfaco::nova(&mut bufro[..], LARĜO, ALTO, LARĜO).unwrap();
        desegnu(&mut surfaco, ordonoj);
        black_box(&*bufro);
    }
    start.elapsed().as_micros() / u128::from(RIPETOJ)
}

fn mezuri_bazan_desegnon(ordonoj: &[Ordono<'_>], bufro: &mut [u32]) -> u128 {
    bufro.fill(0);
    let start = Instant::now();
    {
        let mut surfaco = Surfaco::nova(&mut bufro[..], LARĜO, ALTO, LARĜO).unwrap();
        desegnu(&mut surfaco, ordonoj);
    }
    let daŭro = start.elapsed().as_micros();
    black_box(&*bufro);
    daŭro
}

fn mezuri_desegnon_kun_damaĝo(ordonoj: &[Ordono<'_>], bufro: &mut [u32]) -> u128 {
    bufro.fill(0);
    let mut damaĝo = Damaĝo::nova();
    let start = Instant::now();
    {
        let mut surfaco = Surfaco::nova(&mut bufro[..], LARĜO, ALTO, LARĜO).unwrap();
        desegnu_damagon(&mut surfaco, ordonoj, &mut damaĝo);
    }
    let daŭro = start.elapsed().as_micros();
    black_box((&*bufro, &damaĝo));
    daŭro
}

fn mezuri_damagxon(ordonoj: &[Ordono<'_>], bufro: &mut [u32]) -> (u128, u128) {
    let mut baza_tempo = 0u128;
    let mut damaĝa_tempo = 0u128;
    for ripeto in 0..VARMIGO + RIPETOJ {
        let (baza, kun_damaĝo) = if ripeto % 2 == 0 {
            (
                mezuri_bazan_desegnon(ordonoj, bufro),
                mezuri_desegnon_kun_damaĝo(ordonoj, bufro),
            )
        } else {
            let kun_damaĝo = mezuri_desegnon_kun_damaĝo(ordonoj, bufro);
            (mezuri_bazan_desegnon(ordonoj, bufro), kun_damaĝo)
        };

        if ripeto >= VARMIGO {
            baza_tempo += baza;
            damaĝa_tempo += kun_damaĝo;
        }
    }
    (
        baza_tempo / u128::from(RIPETOJ),
        damaĝa_tempo / u128::from(RIPETOJ),
    )
}

fn main() {
    let mut bufro = vec![0u32; (LARĜO * ALTO) as usize];
    let koloro = Koloro::nova(255, 200, 100, 50).unwrap();
    let plena = [Ordono::Plenigi {
        rektangulo: Rektangulo::nova(0, 0, LARĜO, ALTO),
        koloro,
    }];
    let ronda = [Ordono::RondaRekto {
        rektangulo: Rektangulo::nova(0, 0, LARĜO, ALTO),
        radiuso: 32,
        koloro,
    }];

    let plenigi = mezuri(&plena, &mut bufro);
    let rondan = mezuri(&ronda, &mut bufro);
    let (baza_damaĝo, kun_damaĝo) = mezuri_damagxon(&plena, &mut bufro);
    let (baza_ronda, kun_damaĝa_ronda) = mezuri_damagxon(&ronda, &mut bufro);

    println!("Lumo {} — rendimento", lumo::VERSIO);
    println!("surfaco: {LARĜO} × {ALTO}, densa paŝo");
    println!("Plenigi: {plenigi} µs/plenumo ({VARMIGO} varmigaj, {RIPETOJ} mezuroj)");
    println!(
        "RondaRekto (radiuso 32): {rondan} µs/plenumo ({VARMIGO} varmigaj, {RIPETOJ} mezuroj)"
    );
    println!("Plenigi kun damaĝo: baza {baza_damaĝo} µs, kun damaĝo {kun_damaĝo} µs");
    println!("RondaRekto kun damaĝo: baza {baza_ronda} µs, kun damaĝo {kun_damaĝa_ronda} µs");
}

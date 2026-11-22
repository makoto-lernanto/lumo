//! Mezuras la kanonikan skalaran vojon por `docs/RENDIMENTO.md`.
//!
//! Uzo: `cargo run --release --example rendimento`.

use std::hint::black_box;
use std::time::Instant;

use lumo::{Koloro, Ordono, Rektangulo, Surfaco, desegnu};

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

    println!("Lumo {} — rendimento", lumo::VERSIO);
    println!("surfaco: {LARĜO} × {ALTO}, densa paŝo");
    println!("Plenigi: {plenigi} µs/plenumo ({VARMIGO} varmigaj, {RIPETOJ} mezuroj)");
    println!(
        "RondaRekto (radiuso 32): {rondan} µs/plenumo ({VARMIGO} varmigaj, {RIPETOJ} mezuroj)"
    );
}

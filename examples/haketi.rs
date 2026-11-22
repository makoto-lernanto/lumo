//! Generas la orajn fiksaĵojn kiel krudajn ARGB-bajtojn.
//!
//! Uzo: `cargo run --quiet --example haketi -- <nomo>`. La eligon oni haketas
//! per `sha256sum`; `tools/haketi.sh` faras tion kaj komparas la rezulton kun
//! `docs/FIKSAĴOJ.md`. Oni eligas nur la videblajn pikselojn, vicon post vico,
//! kun la kanaloj `a, r, g, b` po unu bajto.

use std::io::{self, Write};

use lumo::{Koloro, Ordono, Rektangulo, Surfaco, desegnu};

const LARĜO: u32 = 32;
const ALTO: u32 = 24;
const PAŜO: u32 = 40;

fn koloro(alfa: u8, ruĝa: u8, verda: u8, blua: u8) -> Koloro {
    Koloro::nova(alfa, ruĝa, verda, blua).expect("valida premultiplika koloro")
}

fn ordonoj(nomo: &str) -> Vec<Ordono> {
    let fono = koloro(255, 10, 20, 30);
    let emfazo = koloro(255, 200, 120, 60);
    let plena = Rektangulo::nova(0, 0, LARĜO, ALTO);

    match nomo {
        "plena-ortangulo" => vec![
            Ordono::Plenigi {
                rektangulo: plena,
                koloro: fono,
            },
            Ordono::Plenigi {
                rektangulo: plena,
                koloro: emfazo,
            },
        ],
        "klipita-ortangulo" => vec![
            Ordono::Plenigi {
                rektangulo: plena,
                koloro: fono,
            },
            Ordono::Klipo {
                rektangulo: Rektangulo::nova(5, 4, 18, 12),
            },
            Ordono::Plenigi {
                rektangulo: plena,
                koloro: emfazo,
            },
            Ordono::Malŝlipo,
        ],
        _ => {
            eprintln!("nekonata fiksaĵo: {nomo}");
            std::process::exit(2);
        }
    }
}

fn main() {
    let nomo = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("uzo: haketi <plena-ortangulo|klipita-ortangulo>");
        std::process::exit(2);
    });

    let mut bufro = vec![0u32; (PAŜO * ALTO) as usize];
    {
        let mut surfaco =
            Surfaco::nova(&mut bufro, LARĜO, ALTO, PAŜO).expect("valida geometrio de surfaco");
        desegnu(&mut surfaco, &ordonoj(&nomo));
    }

    let mut eligo = Vec::with_capacity((LARĜO * ALTO * 4) as usize);
    for y in 0..ALTO {
        for x in 0..LARĜO {
            let pikselo = bufro[(y * PAŜO + x) as usize];
            eligo.push((pikselo >> 24) as u8);
            eligo.push((pikselo >> 16) as u8);
            eligo.push((pikselo >> 8) as u8);
            eligo.push(pikselo as u8);
        }
    }
    io::stdout().write_all(&eligo).expect("skribo al stdout");
}

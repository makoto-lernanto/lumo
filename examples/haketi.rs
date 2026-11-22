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

const KADRA_LARĜO: u32 = 160;
const KADRA_ALTO: u32 = 32;
const KADRA_PAŜO: u32 = 176;

static IKONO_MALSUPRE: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 255, 255, 255, 255, 255, 255, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, //
];

static IKONO_MAKSIMUMU: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 255, 255, 255, 255, 255, 255, 0, //
    0, 255, 0, 0, 0, 0, 255, 0, //
    0, 255, 0, 0, 0, 0, 255, 0, //
    0, 255, 0, 0, 0, 0, 255, 0, //
    0, 255, 0, 0, 0, 0, 255, 0, //
    0, 255, 255, 255, 255, 255, 255, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, //
];

static IKONO_FERMU: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 255, 255, 0, 0, 255, 255, 0, //
    0, 0, 255, 255, 255, 255, 0, 0, //
    0, 0, 0, 255, 255, 0, 0, 0, //
    0, 0, 0, 255, 255, 0, 0, 0, //
    0, 0, 255, 255, 255, 255, 0, 0, //
    0, 255, 255, 0, 0, 255, 255, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, //
];

#[derive(Clone, Copy)]
enum Stato {
    Normala,
    Ŝveba,
    Premata,
}

struct Temo {
    fono: Koloro,
    rando: Koloro,
    kadro: Koloro,
    regilo: Koloro,
    ikono: Koloro,
}

fn koloro(alfa: u8, ruĝa: u8, verda: u8, blua: u8) -> Koloro {
    Koloro::nova(alfa, ruĝa, verda, blua).expect("valida premultiplika koloro")
}

fn temo(hela: bool, stato: Stato) -> Temo {
    let (fono, rando, kadro, ikono) = if hela {
        (
            koloro(255, 232, 234, 240),
            koloro(255, 190, 194, 204),
            koloro(255, 248, 249, 252),
            koloro(255, 40, 44, 52),
        )
    } else {
        (
            koloro(255, 16, 16, 20),
            koloro(255, 64, 68, 80),
            koloro(255, 32, 34, 40),
            koloro(255, 220, 224, 232),
        )
    };
    let regilo = match (hela, stato) {
        (false, Stato::Normala) => koloro(255, 54, 58, 68),
        (false, Stato::Ŝveba) => koloro(255, 80, 86, 100),
        (false, Stato::Premata) => koloro(255, 38, 40, 48),
        (true, Stato::Normala) => koloro(255, 222, 225, 232),
        (true, Stato::Ŝveba) => koloro(255, 204, 208, 218),
        (true, Stato::Premata) => koloro(255, 184, 188, 198),
    };
    Temo {
        fono,
        rando,
        kadro,
        regilo,
        ikono,
    }
}

fn kadraj_ordonoj(nomo: &str) -> Option<Vec<Ordono<'static>>> {
    let (hela, stato) = match nomo {
        "kadro-malhela-normala" => (false, Stato::Normala),
        "kadro-malhela-ŝveba" => (false, Stato::Ŝveba),
        "kadro-malhela-premata" => (false, Stato::Premata),
        "kadro-hela-normala" => (true, Stato::Normala),
        "kadro-hela-ŝveba" => (true, Stato::Ŝveba),
        "kadro-hela-premata" => (true, Stato::Premata),
        _ => return None,
    };
    let t = temo(hela, stato);

    let ordonoj = vec![
        Ordono::Plenigi {
            rektangulo: Rektangulo::nova(0, 0, KADRA_LARĜO, KADRA_ALTO),
            koloro: t.fono,
        },
        Ordono::RondaRekto {
            rektangulo: Rektangulo::nova(0, 0, KADRA_LARĜO, KADRA_ALTO),
            radiuso: 8,
            koloro: t.rando,
        },
        Ordono::RondaRekto {
            rektangulo: Rektangulo::nova(1, 1, KADRA_LARĜO - 2, KADRA_ALTO - 2),
            radiuso: 7,
            koloro: t.kadro,
        },
        Ordono::Klipo {
            rektangulo: Rektangulo::nova(4, 4, 64, 24),
        },
        Ordono::RondaRekto {
            rektangulo: Rektangulo::nova(12, 8, 16, 16),
            radiuso: 4,
            koloro: t.regilo,
        },
        Ordono::Masko {
            origino: (16, 12),
            amplekso: (8, 8),
            koloro: t.ikono,
            kovradoj: &IKONO_MALSUPRE,
        },
        Ordono::Translokigo { dx: 20, dy: 0 },
        Ordono::RondaRekto {
            rektangulo: Rektangulo::nova(12, 8, 16, 16),
            radiuso: 4,
            koloro: t.regilo,
        },
        Ordono::Masko {
            origino: (16, 12),
            amplekso: (8, 8),
            koloro: t.ikono,
            kovradoj: &IKONO_MAKSIMUMU,
        },
        Ordono::Translokigo { dx: 20, dy: 0 },
        Ordono::RondaRekto {
            rektangulo: Rektangulo::nova(12, 8, 16, 16),
            radiuso: 4,
            koloro: t.regilo,
        },
        Ordono::Masko {
            origino: (16, 12),
            amplekso: (8, 8),
            koloro: t.ikono,
            kovradoj: &IKONO_FERMU,
        },
        Ordono::Maltranslokigo,
        Ordono::Maltranslokigo,
        Ordono::Malŝlipo,
    ];
    Some(ordonoj)
}

fn ordonoj(nomo: &str) -> Vec<Ordono<'static>> {
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
        _ => match kadraj_ordonoj(nomo) {
            Some(ordonoj) => ordonoj,
            None => {
                eprintln!("nekonata fiksaĵo: {nomo}");
                std::process::exit(2);
            }
        },
    }
}

fn dimensioj(nomo: &str) -> (u32, u32, u32) {
    if nomo.starts_with("kadro-") {
        (KADRA_LARĜO, KADRA_ALTO, KADRA_PAŜO)
    } else {
        (LARĜO, ALTO, PAŜO)
    }
}

fn main() {
    let nomo = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("uzo: haketi <plena-ortangulo|klipita-ortangulo|kadro-*>");
        std::process::exit(2);
    });

    let (larĝo, alto, paŝo) = dimensioj(&nomo);
    let mut bufro = vec![0u32; (paŝo * alto) as usize];
    {
        let mut surfaco =
            Surfaco::nova(&mut bufro, larĝo, alto, paŝo).expect("valida geometrio de surfaco");
        desegnu(&mut surfaco, &ordonoj(&nomo));
    }

    let mut eligo = Vec::with_capacity((larĝo * alto * 4) as usize);
    for y in 0..alto {
        for x in 0..larĝo {
            let pikselo = bufro[(y * paŝo + x) as usize];
            eligo.push((pikselo >> 24) as u8);
            eligo.push((pikselo >> 16) as u8);
            eligo.push((pikselo >> 8) as u8);
            eligo.push(pikselo as u8);
        }
    }
    io::stdout().write_all(&eligo).expect("skribo al stdout");
}

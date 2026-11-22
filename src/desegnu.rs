//! La desegna vojo: klipo, kunfandado kaj ortangula plenigo.

use crate::{Koloro, Ordono, Rektangulo, Surfaco};

const MAKSIMUMAJ_ORDONOJ: usize = 256;
const MAKSIMUMA_KLIPA_PROFUNDO: usize = 8;

/// Unu pikselo en 16.16 fiksa komo (L23).
const FIKSITA_BAZO: i64 = 1 << 16;

/// Klipa ortangulo en 16.16 fiksa komo, kalkulita per `i64` (R11, L23).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Limoj {
    pub(crate) x0: i64,
    pub(crate) y0: i64,
    pub(crate) x1: i64,
    pub(crate) y1: i64,
}

impl Limoj {
    pub(crate) const fn el_surfaco(larĝo: u32, alto: u32) -> Self {
        Self {
            x0: 0,
            y0: 0,
            x1: (larĝo as i64) * FIKSITA_BAZO,
            y1: (alto as i64) * FIKSITA_BAZO,
        }
    }

    pub(crate) const fn el_rektangulo(rektangulo: Rektangulo) -> Self {
        let x = rektangulo.x as i64;
        let y = rektangulo.y as i64;
        Self {
            x0: x * FIKSITA_BAZO,
            y0: y * FIKSITA_BAZO,
            x1: (x + rektangulo.larĝo as i64) * FIKSITA_BAZO,
            y1: (y + rektangulo.alto as i64) * FIKSITA_BAZO,
        }
    }

    pub(crate) const fn intersekci(self, alia: Self) -> Self {
        Self {
            x0: max_i64(self.x0, alia.x0),
            y0: max_i64(self.y0, alia.y0),
            x1: min_i64(self.x1, alia.x1),
            y1: min_i64(self.y1, alia.y1),
        }
    }
}

/// Videbla regiono ene de la surfaco, kun ekskluzivaj finoj.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Regiono {
    pub(crate) x0: u32,
    pub(crate) y0: u32,
    pub(crate) x1: u32,
    pub(crate) y1: u32,
}

const fn max_i64(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

const fn min_i64(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}

/// Kalkulado de la videbla regiono; neniuj skriboj okazas ekster `larĝo × alto`.
pub(crate) fn videbla_regiono(
    larĝo: u32,
    alto: u32,
    limoj: Limoj,
    rektangulo: Rektangulo,
) -> Option<Regiono> {
    let r = Limoj::el_rektangulo(rektangulo);
    let x0 = max_i64(max_i64(limoj.x0, r.x0), 0) >> 16;
    let y0 = max_i64(max_i64(limoj.y0, r.y0), 0) >> 16;
    let x1 = min_i64(min_i64(limoj.x1, r.x1), i64::from(larĝo) * FIKSITA_BAZO) >> 16;
    let y1 = min_i64(min_i64(limoj.y1, r.y1), i64::from(alto) * FIKSITA_BAZO) >> 16;
    let x0 = u32::try_from(x0).ok()?;
    let y0 = u32::try_from(y0).ok()?;
    let x1 = u32::try_from(x1).ok()?;
    let y1 = u32::try_from(y1).ok()?;
    if x0 >= x1 || y0 >= y1 {
        return None;
    }
    Some(Regiono { x0, y0, x1, y1 })
}

/// La normiga kunfandada formulo de §4.3 (R07).
///
/// Rezultas `(alfa, koloro)` en `u32`; la pruvo `miksi_neniam_troas` montras,
/// ke ambaŭ restas `≤ 255`.
pub(crate) fn miksi_kanalojn(as_: u8, cs: u8, ad: u8, cd: u8, kovrado: u8) -> (u32, u32) {
    let as_ = u32::from(as_);
    let cs = u32::from(cs);
    let ad = u32::from(ad);
    let cd = u32::from(cd);
    let k = u32::from(kovrado);

    let as_k = (as_ * k + 127) / 255;
    // R08: post la rondigo ni tenas Cs' ≤ As'.
    let cs_k = ((cs * k + 127) / 255).min(as_k);
    let alfa = as_k + (ad * (255 - as_k) + 127) / 255;
    // La premultiplika invarianto de L16 restas valida ankaŭ por la eligo.
    let koloro = (cs_k + (cd * (255 - as_k) + 127) / 255).min(alfa);
    (alfa.min(255), koloro.min(255))
}

fn miksi_pikselon(fonto: Koloro, celo: u32, kovrado: u8) -> u32 {
    if kovrado == 0 || fonto.alfa() == 0 {
        return celo;
    }
    let (as_, cs_r, cs_g, cs_b) = fonto.kanaloj();
    let ad = (celo >> 24) as u8;
    let cd_r = (celo >> 16) as u8;
    let cd_g = (celo >> 8) as u8;
    let cd_b = celo as u8;

    let (na, nr) = miksi_kanalojn(as_, cs_r, ad, cd_r, kovrado);
    let (_, ng) = miksi_kanalojn(as_, cs_g, ad, cd_g, kovrado);
    let (_, nb) = miksi_kanalojn(as_, cs_b, ad, cd_b, kovrado);
    (na << 24) | (nr << 16) | (ng << 8) | nb
}

fn plenigi(surfaco: &mut Surfaco<'_>, klipo: Limoj, rektangulo: Rektangulo, koloro: Koloro) {
    // L17: travidebla koloro ne efikas.
    if koloro.alfa() == 0 {
        return;
    }
    let Some(regiono) = videbla_regiono(surfaco.larĝo(), surfaco.alto(), klipo, rektangulo) else {
        return;
    };
    let paŝo = u64::from(surfaco.paŝo());
    for y in regiono.y0..regiono.y1 {
        let vico = u64::from(y) * paŝo;
        for x in regiono.x0..regiono.x1 {
            let indekso = (vico + u64::from(x)) as usize;
            let celo = surfaco.pikseloj[indekso];
            surfaco.pikseloj[indekso] = miksi_pikselon(koloro, celo, 255);
        }
    }
}

/// Faru la ordonaron sur la surfaco.
///
/// La ordonoj plenumiĝas en la donita ordo (L20); la klipo estas stako kun
/// profundo 8 (L19). Ordonoj post la 256-a estas ignorataj (L25), kaj troa
/// klipa profundo aŭ `Malŝlipo` sen antaŭa `Klipo` estas alvokanta eraro:
/// en testa konstruo `debug_assert!` ĝin montras, en eldona konstruo la
/// ordono estas ignorata (L21).
pub fn desegnu(surfaco: &mut Surfaco<'_>, ordonoj: &[Ordono]) {
    let mut klipo = Limoj::el_surfaco(surfaco.larĝo(), surfaco.alto());
    let mut stako = [Limoj::el_surfaco(0, 0); MAKSIMUMA_KLIPA_PROFUNDO];
    let mut profundo = 0usize;

    for ordono in ordonoj.iter().take(MAKSIMUMAJ_ORDONOJ) {
        match *ordono {
            Ordono::Plenigi { rektangulo, koloro } => plenigi(surfaco, klipo, rektangulo, koloro),
            Ordono::Klipo { rektangulo } => {
                debug_assert!(
                    profundo < MAKSIMUMA_KLIPA_PROFUNDO,
                    "klipa profundo superas 8"
                );
                if profundo < MAKSIMUMA_KLIPA_PROFUNDO {
                    stako[profundo] = klipo;
                    profundo += 1;
                    klipo = klipo.intersekci(Limoj::el_rektangulo(rektangulo));
                }
            }
            Ordono::Malŝlipo => {
                debug_assert!(profundo > 0, "Malŝlipo sen antaŭa Klipo");
                if profundo > 0 {
                    profundo -= 1;
                    klipo = stako[profundo];
                }
            }
        }
    }
}

#[cfg(test)]
mod testoj {
    use super::{Limoj, desegnu, miksi_kanalojn, videbla_regiono};
    use crate::{Koloro, Ordono, Rektangulo, Surfaco};

    const RUĜA: u32 = 0xFFC8_6432;
    const VERDA: u32 = 0xFF32_64C8;

    fn paka(alfa: u8, ruĝa: u8, verda: u8, blua: u8) -> u32 {
        (u32::from(alfa) << 24)
            | (u32::from(ruĝa) << 16)
            | (u32::from(verda) << 8)
            | u32::from(blua)
    }

    fn plena(larĝo: u32, alto: u32) -> Rektangulo {
        Rektangulo::nova(0, 0, larĝo, alto)
    }

    #[test]
    fn plena_ortangulo_skribas_ĉiujn_pikselojn() {
        let mut bufro = [0u32; 4];
        let mut surfaco = Surfaco::nova(&mut bufro, 2, 2, 2).unwrap();
        let koloro = Koloro::nova(255, 200, 100, 50).unwrap();
        desegnu(
            &mut surfaco,
            &[Ordono::Plenigi {
                rektangulo: plena(2, 2),
                koloro,
            }],
        );
        assert_eq!(surfaco.pikseloj, [paka(255, 200, 100, 50); 4]);
    }

    #[test]
    fn travidebla_koloro_lasas_celon_senŝanĝa() {
        let mut bufro = [RUĜA; 4];
        let mut surfaco = Surfaco::nova(&mut bufro, 2, 2, 2).unwrap();
        let koloro = Koloro::nova(0, 0, 0, 0).unwrap();
        desegnu(
            &mut surfaco,
            &[Ordono::Plenigi {
                rektangulo: plena(2, 2),
                koloro,
            }],
        );
        assert_eq!(surfaco.pikseloj, [RUĜA; 4]);
    }

    #[test]
    fn klipo_limigas_la_skribon() {
        let mut bufro = [0u32; 16];
        let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
        let fono = Koloro::nova(255, 10, 20, 30).unwrap();
        let koloro = Koloro::nova(255, 200, 100, 50).unwrap();
        desegnu(
            &mut surfaco,
            &[
                Ordono::Plenigi {
                    rektangulo: plena(4, 4),
                    koloro: fono,
                },
                Ordono::Klipo {
                    rektangulo: Rektangulo::nova(1, 1, 2, 2),
                },
                Ordono::Plenigi {
                    rektangulo: plena(4, 4),
                    koloro,
                },
            ],
        );
        let fona_pikselo = paka(255, 10, 20, 30);
        let klipa_pikselo = paka(255, 200, 100, 50);
        for y in 0..4usize {
            for x in 0..4usize {
                let atendu = if (1..3).contains(&x) && (1..3).contains(&y) {
                    klipa_pikselo
                } else {
                    fona_pikselo
                };
                assert_eq!(surfaco.pikseloj[y * 4 + x], atendu);
            }
        }
    }

    #[test]
    fn malŝlipo_restarigas_la_antaŭan_klipon() {
        let mut bufro = [0u32; 16];
        let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
        let koloro = Koloro::nova(255, 200, 100, 50).unwrap();
        desegnu(
            &mut surfaco,
            &[
                Ordono::Klipo {
                    rektangulo: Rektangulo::nova(1, 1, 2, 2),
                },
                Ordono::Malŝlipo,
                Ordono::Plenigi {
                    rektangulo: plena(4, 4),
                    koloro,
                },
            ],
        );
        assert_eq!(surfaco.pikseloj, [paka(255, 200, 100, 50); 16]);
    }

    #[test]
    fn nestita_klipo_restarigas_laŭvice() {
        let mut bufro = [0u32; 16];
        let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
        let koloro = Koloro::nova(255, 200, 100, 50).unwrap();
        desegnu(
            &mut surfaco,
            &[
                Ordono::Klipo {
                    rektangulo: Rektangulo::nova(0, 0, 3, 3),
                },
                Ordono::Klipo {
                    rektangulo: Rektangulo::nova(1, 1, 3, 3),
                },
                Ordono::Malŝlipo,
                Ordono::Plenigi {
                    rektangulo: plena(4, 4),
                    koloro,
                },
            ],
        );
        for y in 0..4usize {
            for x in 0..4usize {
                let atendu = if x < 3 && y < 3 {
                    paka(255, 200, 100, 50)
                } else {
                    0
                };
                assert_eq!(surfaco.pikseloj[y * 4 + x], atendu);
            }
        }
    }

    #[test]
    fn klipo_ok_fojojn_funkcias() {
        let mut bufro = [0u32; 16];
        let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
        let koloro = Koloro::nova(255, 200, 100, 50).unwrap();
        let mut ordonoj = [Ordono::Klipo {
            rektangulo: Rektangulo::nova(0, 0, 4, 4),
        }; 9];
        ordonoj[8] = Ordono::Plenigi {
            rektangulo: plena(4, 4),
            koloro,
        };
        desegnu(&mut surfaco, &ordonoj);
        assert_eq!(surfaco.pikseloj, [paka(255, 200, 100, 50); 16]);
    }

    #[test]
    fn klipo_ekster_surfaco_nenion_skribas() {
        let mut bufro = [0u32; 16];
        let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
        let koloro = Koloro::nova(255, 200, 100, 50).unwrap();
        desegnu(
            &mut surfaco,
            &[
                Ordono::Klipo {
                    rektangulo: Rektangulo::nova(10, 10, 2, 2),
                },
                Ordono::Plenigi {
                    rektangulo: plena(4, 4),
                    koloro,
                },
                Ordono::Malŝlipo,
            ],
        );
        assert_eq!(surfaco.pikseloj, [0u32; 16]);
    }

    #[test]
    fn negativaj_koordinatoj_estas_klipitaj() {
        let mut bufro = [0u32; 4];
        let mut surfaco = Surfaco::nova(&mut bufro, 2, 2, 2).unwrap();
        let koloro = Koloro::nova(255, 200, 100, 50).unwrap();
        desegnu(
            &mut surfaco,
            &[Ordono::Plenigi {
                rektangulo: Rektangulo::nova(-2, -2, 4, 4),
                koloro,
            }],
        );
        assert_eq!(surfaco.pikseloj, [paka(255, 200, 100, 50); 4]);
    }

    #[test]
    fn paŝo_kaŝita_padrono_restas_senŝanĝa() {
        let mut bufro = [0u32; 8];
        {
            let mut surfaco = Surfaco::nova(&mut bufro, 2, 2, 4).unwrap();
            let koloro = Koloro::nova(255, 200, 100, 50).unwrap();
            desegnu(
                &mut surfaco,
                &[Ordono::Plenigi {
                    rektangulo: plena(2, 2),
                    koloro,
                }],
            );
        }
        let pikselo = paka(255, 200, 100, 50);
        assert_eq!(bufro, [pikselo, pikselo, 0, 0, pikselo, pikselo, 0, 0]);
    }

    #[test]
    fn ordo_de_la_ordonoj_estas_respektata() {
        let mut bufro = [0u32; 4];
        let mut surfaco = Surfaco::nova(&mut bufro, 2, 2, 2).unwrap();
        let ruĝa = Koloro::nova(255, 200, 100, 50).unwrap();
        let verda = Koloro::nova(255, 50, 100, 200).unwrap();
        desegnu(
            &mut surfaco,
            &[
                Ordono::Plenigi {
                    rektangulo: plena(2, 2),
                    koloro: ruĝa,
                },
                Ordono::Plenigi {
                    rektangulo: plena(2, 2),
                    koloro: verda,
                },
            ],
        );
        assert_eq!(surfaco.pikseloj, [VERDA; 4]);
    }

    #[test]
    fn duon_travidebla_kunfandado_kongruas_la_formulon() {
        let mut bufro = [paka(255, 100, 50, 25); 1];
        let mut surfaco = Surfaco::nova(&mut bufro, 1, 1, 1).unwrap();
        let koloro = Koloro::nova(128, 64, 32, 16).unwrap();
        desegnu(
            &mut surfaco,
            &[Ordono::Plenigi {
                rektangulo: Rektangulo::nova(0, 0, 1, 1),
                koloro,
            }],
        );
        assert_eq!(surfaco.pikseloj[0], paka(255, 114, 57, 28));
    }

    #[test]
    fn kovrado_nul_nenion_ŝanĝas() {
        assert_eq!(miksi_kanalojn(128, 64, 255, 100, 0), (255, 100));
    }

    #[test]
    fn kovrado_plena_aplikas_la_fonton() {
        assert_eq!(miksi_kanalojn(128, 64, 255, 100, 255), (255, 114));
    }

    #[test]
    fn troa_kolora_kanalo_estas_limigita() {
        // L16-malobservo ne povas veni el `Koloro`, sed R08 tenas la eliron
        // premultiplika ankaŭ por la interna funkcio.
        assert_eq!(miksi_kanalojn(0, 255, 0, 0, 255), (0, 0));
    }

    #[test]
    fn ordoj_post_la_256a_estas_ignorataj() {
        let mut bufro = [0u32; 1];
        let mut surfaco = Surfaco::nova(&mut bufro, 1, 1, 1).unwrap();
        let ruĝa = Koloro::nova(255, 200, 100, 50).unwrap();
        let blua = Koloro::nova(255, 50, 100, 200).unwrap();
        let mut ordonoj = [Ordono::Plenigi {
            rektangulo: Rektangulo::nova(0, 0, 1, 1),
            koloro: ruĝa,
        }; 257];
        ordonoj[256] = Ordono::Plenigi {
            rektangulo: Rektangulo::nova(0, 0, 1, 1),
            koloro: blua,
        };
        desegnu(&mut surfaco, &ordonoj);
        assert_eq!(surfaco.pikseloj[0], RUĜA);
    }

    #[test]
    fn videbla_regiono_estas_ene_de_la_surfaco() {
        let regiono = videbla_regiono(
            4,
            4,
            Limoj::el_surfaco(4, 4),
            Rektangulo::nova(-1, -1, 3, 3),
        )
        .unwrap();
        assert_eq!(regiono.x0, 0);
        assert_eq!(regiono.y0, 0);
        assert_eq!(regiono.x1, 2);
        assert_eq!(regiono.y1, 2);
    }
}

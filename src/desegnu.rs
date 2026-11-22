//! La desegna vojo: klipo, kunfandado, ortangula plenigo, rondoj, maskoj kaj
//! translokigo.

use crate::{Koloro, Ordono, Rektangulo, Surfaco};

const MAKSIMUMAJ_ORDONOJ: usize = 256;
const MAKSIMUMA_KLIPA_PROFUNDO: usize = 8;
const MAKSIMUMA_TRANSLOKIGA_PROFUNDO: usize = 8;

/// Unu pikselo en 16.16 fiksa komo (L23).
const FIKSITA_BAZO: i64 = 1 << 16;

/// Centroj de la 4 × 4 subpikseloj en 16.16: `(2i + 1) / 8` (R06).
const SPECIMENAJ_CENTROJ: [i64; 4] = [8192, 24576, 40960, 57344];

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
        Self::el_rektangulo_translokita(rektangulo, 0, 0)
    }

    /// La sama kiel `el_rektangulo`, sed kun entjera translokigo (R12).
    ///
    /// La translokiga profundo estas 8 kaj ĉiu delokiĝo estas `i32`; la sumo
    /// restas malproksime de la limoj de `i64`, eĉ post la multipliko per
    /// `FIKSITA_BAZO`.
    pub(crate) const fn el_rektangulo_translokita(
        rektangulo: Rektangulo,
        dx: i64,
        dy: i64,
    ) -> Self {
        let x = rektangulo.x as i64 + dx;
        let y = rektangulo.y as i64 + dy;
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
    klipo: Limoj,
    geometrio: Limoj,
) -> Option<Regiono> {
    let x0 = max_i64(max_i64(klipo.x0, geometrio.x0), 0) >> 16;
    let y0 = max_i64(max_i64(klipo.y0, geometrio.y0), 0) >> 16;
    let x1 = min_i64(
        min_i64(klipo.x1, geometrio.x1),
        i64::from(larĝo) * FIKSITA_BAZO,
    ) >> 16;
    let y1 = min_i64(
        min_i64(klipo.y1, geometrio.y1),
        i64::from(alto) * FIKSITA_BAZO,
    ) >> 16;
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

fn kovrado_el_specimenoj(specimenoj: [bool; 16]) -> u32 {
    let trafoj = u32::from(specimenoj[0])
        + u32::from(specimenoj[1])
        + u32::from(specimenoj[2])
        + u32::from(specimenoj[3])
        + u32::from(specimenoj[4])
        + u32::from(specimenoj[5])
        + u32::from(specimenoj[6])
        + u32::from(specimenoj[7])
        + u32::from(specimenoj[8])
        + u32::from(specimenoj[9])
        + u32::from(specimenoj[10])
        + u32::from(specimenoj[11])
        + u32::from(specimenoj[12])
        + u32::from(specimenoj[13])
        + u32::from(specimenoj[14])
        + u32::from(specimenoj[15]);
    (trafoj * 255 + 8) / 16
}

fn en_cirklo(dx: i64, dy: i64, radiuso: i64) -> bool {
    let dx = u128::from(dx.unsigned_abs());
    let dy = u128::from(dy.unsigned_abs());
    let radiuso = radiuso as u128;
    dx * dx + dy * dy < radiuso * radiuso
}

/// Ĉu la specimeno apartenas al la rondigita ortangulo? La geometrio estas
/// duonfermita: maldekstra kaj supra randoj apartenas, dekstra kaj malsupra
/// ne (R06).
fn specimeno_en_ronda(x: i64, y: i64, geometrio: Limoj, radiuso: i64) -> bool {
    if x < geometrio.x0 || x >= geometrio.x1 || y < geometrio.y0 || y >= geometrio.y1 {
        return false;
    }
    if radiuso == 0 {
        return true;
    }
    if x < geometrio.x0 + radiuso {
        let centro_x = geometrio.x0 + radiuso;
        if y < geometrio.y0 + radiuso {
            return en_cirklo(x - centro_x, y - (geometrio.y0 + radiuso), radiuso);
        }
        if y >= geometrio.y1 - radiuso {
            return en_cirklo(x - centro_x, y - (geometrio.y1 - radiuso), radiuso);
        }
    } else if x >= geometrio.x1 - radiuso {
        let centro_x = geometrio.x1 - radiuso;
        if y < geometrio.y0 + radiuso {
            return en_cirklo(x - centro_x, y - (geometrio.y0 + radiuso), radiuso);
        }
        if y >= geometrio.y1 - radiuso {
            return en_cirklo(x - centro_x, y - (geometrio.y1 - radiuso), radiuso);
        }
    }
    true
}

/// Ĉu la pikselo tuŝas iun el la kvar angulaj regionoj? Nur tie la superprovado
/// povas doni kovradon inter 0 kaj 255; alie la kovrado estas plena (R04).
fn en_angula_regiono(x: i64, y: i64, geometrio: Limoj, radiuso: i64) -> bool {
    let horizontale = x < geometrio.x0 + radiuso || x + FIKSITA_BAZO > geometrio.x1 - radiuso;
    let vertikale = y < geometrio.y0 + radiuso || y + FIKSITA_BAZO > geometrio.y1 - radiuso;
    horizontale && vertikale
}

fn kovrado_de_ronda_pikselo(x: i64, y: i64, geometrio: Limoj, radiuso: i64) -> u8 {
    let mut specimenoj = [false; 16];
    let mut indekso = 0;
    for oy in SPECIMENAJ_CENTROJ {
        for ox in SPECIMENAJ_CENTROJ {
            specimenoj[indekso] = specimeno_en_ronda(x + ox, y + oy, geometrio, radiuso);
            indekso += 1;
        }
    }
    let kovrado = kovrado_el_specimenoj(specimenoj);
    debug_assert!(kovrado <= 255);
    kovrado as u8
}

fn vico_mut<'b>(surfaco: &'b mut Surfaco<'_>, y: u32) -> Option<&'b mut [u32]> {
    let paŝo = surfaco.paŝo() as usize;
    // `get_mut` gardas la vican komencon; la kolumnan limon pruvas
    // `klipo_restas_en_surfaco` (L15).
    surfaco
        .pikseloj
        .get_mut((y as usize).saturating_mul(paŝo)..)
        .and_then(|resto| resto.get_mut(..paŝo))
}

fn plenigi(
    surfaco: &mut Surfaco<'_>,
    klipo: Limoj,
    rektangulo: Rektangulo,
    koloro: Koloro,
    dx: i64,
    dy: i64,
) {
    // L17: travidebla koloro ne efikas.
    if koloro.alfa() == 0 {
        return;
    }
    let geometrio = Limoj::el_rektangulo_translokita(rektangulo, dx, dy);
    let Some(regiono) = videbla_regiono(surfaco.larĝo(), surfaco.alto(), klipo, geometrio) else {
        return;
    };
    for y in regiono.y0..regiono.y1 {
        let Some(vico) = vico_mut(surfaco, y) else {
            break;
        };
        for x in regiono.x0..regiono.x1 {
            let indekso = x as usize;
            let celo = vico[indekso];
            vico[indekso] = miksi_pikselon(koloro, celo, 255);
        }
    }
}

fn plenigi_rondan(
    surfaco: &mut Surfaco<'_>,
    klipo: Limoj,
    rektangulo: Rektangulo,
    radiuso: u32,
    koloro: Koloro,
    dx: i64,
    dy: i64,
) {
    if koloro.alfa() == 0 {
        return;
    }
    let geometrio = Limoj::el_rektangulo_translokita(rektangulo, dx, dy);
    let Some(regiono) = videbla_regiono(surfaco.larĝo(), surfaco.alto(), klipo, geometrio) else {
        return;
    };
    let radiuso = radiuso.min(rektangulo.larĝo.min(rektangulo.alto) / 2);
    let radiuso = i64::from(radiuso) * FIKSITA_BAZO;

    for y in regiono.y0..regiono.y1 {
        let Some(vico) = vico_mut(surfaco, y) else {
            break;
        };
        let y_fiksita = i64::from(y) * FIKSITA_BAZO;
        for x in regiono.x0..regiono.x1 {
            let x_fiksita = i64::from(x) * FIKSITA_BAZO;
            let kovrado = if en_angula_regiono(x_fiksita, y_fiksita, geometrio, radiuso) {
                kovrado_de_ronda_pikselo(x_fiksita, y_fiksita, geometrio, radiuso)
            } else {
                255
            };
            if kovrado == 0 {
                continue;
            }
            let indekso = x as usize;
            let celo = vico[indekso];
            vico[indekso] = miksi_pikselon(koloro, celo, kovrado);
        }
    }
}

fn apliki_maskon(
    surfaco: &mut Surfaco<'_>,
    klipo: Limoj,
    origino: (i32, i32),
    amplekso: (u32, u32),
    koloro: Koloro,
    kovradoj: &[u8],
    translokigo: (i64, i64),
) {
    let (dx, dy) = translokigo;
    if koloro.alfa() == 0 {
        return;
    }
    let Some(bezonata) = u64::from(amplekso.0).checked_mul(u64::from(amplekso.1)) else {
        return;
    };
    let Ok(disponebla) = u64::try_from(kovradoj.len()) else {
        return;
    };
    if disponebla < bezonata {
        return;
    }

    let rektangulo = Rektangulo::nova(origino.0, origino.1, amplekso.0, amplekso.1);
    let geometrio = Limoj::el_rektangulo_translokita(rektangulo, dx, dy);
    let Some(regiono) = videbla_regiono(surfaco.larĝo(), surfaco.alto(), klipo, geometrio) else {
        return;
    };
    let origino_x = i64::from(origino.0) + dx;
    let origino_y = i64::from(origino.1) + dy;
    let larĝo = amplekso.0 as usize;

    for y in regiono.y0..regiono.y1 {
        let Some(vico) = vico_mut(surfaco, y) else {
            break;
        };
        let maska_y = i64::from(y) - origino_y;
        let Ok(maska_y) = usize::try_from(maska_y) else {
            continue;
        };
        for x in regiono.x0..regiono.x1 {
            let maska_x = i64::from(x) - origino_x;
            let Ok(maska_x) = usize::try_from(maska_x) else {
                continue;
            };
            let Some(indekso) = maska_y
                .checked_mul(larĝo)
                .and_then(|vico| vico.checked_add(maska_x))
            else {
                continue;
            };
            let Some(&kovrado) = kovradoj.get(indekso) else {
                continue;
            };
            if kovrado == 0 {
                continue;
            }
            let indekso = x as usize;
            let celo = vico[indekso];
            vico[indekso] = miksi_pikselon(koloro, celo, kovrado);
        }
    }
}

/// Faru la ordonaron sur la surfaco.
///
/// La ordonoj plenumiĝas en la donita ordo (L20); la klipo kaj la translokigo
/// estas stakoj kun profundo 8 (L19). Ordonoj post la 256-a estas ignorataj
/// (L25), kaj troa profundo aŭ malŝlipo sen antaŭa ŝlipo estas alvokanta
/// eraro: en testa konstruo `debug_assert!` ĝin montras, en eldona konstruo la
/// ordono estas ignorata (L21).
pub fn desegnu(surfaco: &mut Surfaco<'_>, ordonoj: &[Ordono<'_>]) {
    let mut klipo = Limoj::el_surfaco(surfaco.larĝo(), surfaco.alto());
    let mut stako = [Limoj::el_surfaco(0, 0); MAKSIMUMA_KLIPA_PROFUNDO];
    let mut profundo = 0usize;
    let mut translokigo = (0i64, 0i64);
    let mut translokiga_stako = [(0i64, 0i64); MAKSIMUMA_TRANSLOKIGA_PROFUNDO];
    let mut translokiga_profundo = 0usize;

    for ordono in ordonoj.iter().take(MAKSIMUMAJ_ORDONOJ) {
        match *ordono {
            Ordono::Plenigi { rektangulo, koloro } => {
                plenigi(
                    surfaco,
                    klipo,
                    rektangulo,
                    koloro,
                    translokigo.0,
                    translokigo.1,
                );
            }
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
            Ordono::RondaRekto {
                rektangulo,
                radiuso,
                koloro,
            } => {
                plenigi_rondan(
                    surfaco,
                    klipo,
                    rektangulo,
                    radiuso,
                    koloro,
                    translokigo.0,
                    translokigo.1,
                );
            }
            Ordono::Masko {
                origino,
                amplekso,
                koloro,
                kovradoj,
            } => {
                apliki_maskon(
                    surfaco,
                    klipo,
                    origino,
                    amplekso,
                    koloro,
                    kovradoj,
                    translokigo,
                );
            }
            Ordono::Translokigo { dx, dy } => {
                debug_assert!(
                    translokiga_profundo < MAKSIMUMA_TRANSLOKIGA_PROFUNDO,
                    "translokiga profundo superas 8"
                );
                if translokiga_profundo < MAKSIMUMA_TRANSLOKIGA_PROFUNDO {
                    translokiga_stako[translokiga_profundo] = translokigo;
                    translokiga_profundo += 1;
                    translokigo.0 += i64::from(dx);
                    translokigo.1 += i64::from(dy);
                }
            }
            Ordono::Maltranslokigo => {
                debug_assert!(
                    translokiga_profundo > 0,
                    "Maltranslokigo sen antaŭa Translokigo"
                );
                if translokiga_profundo > 0 {
                    translokiga_profundo -= 1;
                    translokigo = translokiga_stako[translokiga_profundo];
                }
            }
        }
    }
}

#[cfg(test)]
mod testoj {
    use super::{
        FIKSITA_BAZO, Limoj, SPECIMENAJ_CENTROJ, desegnu, kovrado_el_specimenoj, miksi_kanalojn,
        specimeno_en_ronda, videbla_regiono,
    };
    use crate::{Koloro, Ordono, Rektangulo, Surfaco};

    const RUĜA: u32 = 0xFFC8_6432;
    const VERDA: u32 = 0xFF32_64C8;

    fn paka(alfa: u8, ruĝa: u8, verda: u8, blua: u8) -> u32 {
        (u32::from(alfa) << 24)
            | (u32::from(ruĝa) << 16)
            | (u32::from(verda) << 8)
            | u32::from(blua)
    }

    fn koloro(alfa: u8, ruĝa: u8, verda: u8, blua: u8) -> Koloro {
        Koloro::nova(alfa, ruĝa, verda, blua).unwrap()
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
            Limoj::el_rektangulo(Rektangulo::nova(-1, -1, 3, 3)),
        )
        .unwrap();
        assert_eq!(regiono.x0, 0);
        assert_eq!(regiono.y0, 0);
        assert_eq!(regiono.x1, 2);
        assert_eq!(regiono.y1, 2);
    }

    #[test]
    fn kovrado_el_specimenoj_kongruas_la_formulon() {
        assert_eq!(kovrado_el_specimenoj([false; 16]), 0);
        assert_eq!(kovrado_el_specimenoj([true; 16]), 255);

        let mut unu = [false; 16];
        unu[0] = true;
        assert_eq!(kovrado_el_specimenoj(unu), 16);

        let mut dek_kvin = [true; 16];
        dek_kvin[15] = false;
        assert_eq!(kovrado_el_specimenoj(dek_kvin), 239);
    }

    #[test]
    fn ronda_rekto_radiuso_nul_egalas_plenigi() {
        let fonto = koloro(160, 80, 40, 20);
        for rektangulo in [
            Rektangulo::nova(0, 0, 4, 4),
            Rektangulo::nova(1, 1, 3, 2),
            Rektangulo::nova(-2, -1, 4, 3),
            Rektangulo::nova(10, 10, 2, 2),
            Rektangulo::nova(2, 1, 1, 4),
        ] {
            let mut unua = [0u32; 36];
            let mut dua = [0u32; 36];
            {
                let mut surfaco = Surfaco::nova(&mut unua, 5, 5, 6).unwrap();
                desegnu(
                    &mut surfaco,
                    &[Ordono::Plenigi {
                        rektangulo,
                        koloro: fonto,
                    }],
                );
            }
            {
                let mut surfaco = Surfaco::nova(&mut dua, 5, 5, 6).unwrap();
                desegnu(
                    &mut surfaco,
                    &[Ordono::RondaRekto {
                        rektangulo,
                        radiuso: 0,
                        koloro: fonto,
                    }],
                );
            }
            assert_eq!(unua, dua);
        }
    }

    #[test]
    fn ronda_rekto_kovras_angulon_per_superprovado() {
        let mut bufro = [0u32; 16];
        let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
        let fonto = koloro(255, 200, 100, 50);
        desegnu(
            &mut surfaco,
            &[Ordono::RondaRekto {
                rektangulo: plena(4, 4),
                radiuso: 2,
                koloro: fonto,
            }],
        );
        // La angulaj pikseloj havas ses specimenojn el dek ses.
        for &indekso in &[0usize, 3, 12, 15] {
            assert_eq!(surfaco.pikseloj[indekso], paka(96, 75, 38, 19));
        }
        for &indekso in &[5usize, 6, 9, 10] {
            assert_eq!(surfaco.pikseloj[indekso], paka(255, 200, 100, 50));
        }
    }

    #[test]
    fn tro_granda_radiuso_estas_limigita() {
        let fonto = koloro(128, 64, 32, 16);
        let mut limigita = [0u32; 16];
        let mut troa = [0u32; 16];
        {
            let mut surfaco = Surfaco::nova(&mut limigita, 4, 4, 4).unwrap();
            desegnu(
                &mut surfaco,
                &[Ordono::RondaRekto {
                    rektangulo: plena(4, 4),
                    radiuso: 2,
                    koloro: fonto,
                }],
            );
        }
        {
            let mut surfaco = Surfaco::nova(&mut troa, 4, 4, 4).unwrap();
            desegnu(
                &mut surfaco,
                &[Ordono::RondaRekto {
                    rektangulo: plena(4, 4),
                    radiuso: 1000,
                    koloro: fonto,
                }],
            );
        }
        assert_eq!(limigita, troa);
    }

    #[test]
    fn ronda_rekto_respektas_klipon() {
        let fonto = koloro(255, 200, 100, 50);
        let mut sen_klipo = [0u32; 16];
        let mut kun_klipo = [0u32; 16];
        {
            let mut surfaco = Surfaco::nova(&mut sen_klipo, 4, 4, 4).unwrap();
            desegnu(
                &mut surfaco,
                &[Ordono::RondaRekto {
                    rektangulo: plena(4, 4),
                    radiuso: 2,
                    koloro: fonto,
                }],
            );
        }
        {
            let mut surfaco = Surfaco::nova(&mut kun_klipo, 4, 4, 4).unwrap();
            desegnu(
                &mut surfaco,
                &[
                    Ordono::RondaRekto {
                        rektangulo: plena(4, 4),
                        radiuso: 2,
                        koloro: fonto,
                    },
                    Ordono::Klipo {
                        rektangulo: Rektangulo::nova(1, 1, 2, 2),
                    },
                    Ordono::Plenigi {
                        rektangulo: plena(4, 4),
                        koloro: koloro(255, 10, 20, 30),
                    },
                    Ordono::Malŝlipo,
                ],
            );
        }
        for (x, y) in [(0usize, 0usize), (1, 0), (0, 1), (3, 3)] {
            assert_eq!(kun_klipo[y * 4 + x], sen_klipo[y * 4 + x]);
        }
        for y in 1..3usize {
            for x in 1..3usize {
                assert_eq!(kun_klipo[y * 4 + x], paka(255, 10, 20, 30));
            }
        }
    }

    #[test]
    fn masko_aplikas_kovradon() {
        let mut bufro = [0u32; 4];
        let mut surfaco = Surfaco::nova(&mut bufro, 2, 2, 2).unwrap();
        let kovradoj = [255u8, 128, 0, 64];
        desegnu(
            &mut surfaco,
            &[Ordono::Masko {
                origino: (0, 0),
                amplekso: (2, 2),
                koloro: koloro(255, 200, 100, 50),
                kovradoj: &kovradoj,
            }],
        );
        assert_eq!(surfaco.pikseloj[0], paka(255, 200, 100, 50));
        assert_eq!(surfaco.pikseloj[1], paka(128, 100, 50, 25));
        assert_eq!(surfaco.pikseloj[2], 0);
        assert_eq!(surfaco.pikseloj[3], paka(64, 50, 25, 13));
    }

    #[test]
    fn tro_mallonga_masko_estas_ignorata() {
        let mut bufro = [0u32; 4];
        let mut surfaco = Surfaco::nova(&mut bufro, 2, 2, 2).unwrap();
        let kovradoj = [255u8; 3];
        desegnu(
            &mut surfaco,
            &[Ordono::Masko {
                origino: (0, 0),
                amplekso: (2, 2),
                koloro: koloro(255, 200, 100, 50),
                kovradoj: &kovradoj,
            }],
        );
        assert_eq!(surfaco.pikseloj, [0u32; 4]);
    }

    #[test]
    fn masko_kun_paŝo_de_surfaco() {
        let mut bufro = [0u32; 8];
        let mut surfaco = Surfaco::nova(&mut bufro, 2, 2, 4).unwrap();
        let kovradoj = [255u8, 128, 0, 64];
        desegnu(
            &mut surfaco,
            &[Ordono::Masko {
                origino: (0, 0),
                amplekso: (2, 2),
                koloro: koloro(255, 200, 100, 50),
                kovradoj: &kovradoj,
            }],
        );
        let plena = paka(255, 200, 100, 50);
        let duona = paka(128, 100, 50, 25);
        assert_eq!(bufro, [plena, duona, 0, 0, 0, paka(64, 50, 25, 13), 0, 0]);
    }

    #[test]
    fn masko_respektas_klipon() {
        let mut bufro = [0u32; 4];
        let mut surfaco = Surfaco::nova(&mut bufro, 2, 2, 2).unwrap();
        let kovradoj = [255u8; 4];
        desegnu(
            &mut surfaco,
            &[
                Ordono::Klipo {
                    rektangulo: Rektangulo::nova(0, 0, 1, 1),
                },
                Ordono::Masko {
                    origino: (0, 0),
                    amplekso: (2, 2),
                    koloro: koloro(255, 200, 100, 50),
                    kovradoj: &kovradoj,
                },
                Ordono::Malŝlipo,
            ],
        );
        assert_eq!(surfaco.pikseloj[0], paka(255, 200, 100, 50));
        assert_eq!(surfaco.pikseloj[1..], [0u32; 3]);
    }

    #[test]
    fn masko_translokigita() {
        let mut bufro = [0u32; 9];
        let mut surfaco = Surfaco::nova(&mut bufro, 3, 3, 3).unwrap();
        let kovradoj = [255u8, 128, 0, 64];
        desegnu(
            &mut surfaco,
            &[
                Ordono::Translokigo { dx: 1, dy: 1 },
                Ordono::Masko {
                    origino: (0, 0),
                    amplekso: (2, 2),
                    koloro: koloro(255, 200, 100, 50),
                    kovradoj: &kovradoj,
                },
                Ordono::Maltranslokigo,
            ],
        );
        assert_eq!(bufro[4], paka(255, 200, 100, 50));
        assert_eq!(bufro[5], paka(128, 100, 50, 25));
        assert_eq!(bufro[7], 0);
        assert_eq!(bufro[8], paka(64, 50, 25, 13));
        assert_eq!(bufro[0..4], [0u32; 4]);
        assert_eq!(bufro[6], 0);
    }

    #[test]
    fn translokigo_movas_plenigon() {
        let mut bufro = [0u32; 16];
        let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
        desegnu(
            &mut surfaco,
            &[
                Ordono::Translokigo { dx: 2, dy: 1 },
                Ordono::Plenigi {
                    rektangulo: Rektangulo::nova(0, 0, 2, 2),
                    koloro: koloro(255, 200, 100, 50),
                },
                Ordono::Maltranslokigo,
            ],
        );
        for y in 0..4usize {
            for x in 0..4usize {
                let atendu = if (1..3).contains(&y) && (2..4).contains(&x) {
                    paka(255, 200, 100, 50)
                } else {
                    0
                };
                assert_eq!(surfaco.pikseloj[y * 4 + x], atendu);
            }
        }
    }

    #[test]
    fn maltranslokigo_restarigas_ekzakte() {
        let mut bufro = [0u32; 9];
        let mut surfaco = Surfaco::nova(&mut bufro, 3, 3, 3).unwrap();
        desegnu(
            &mut surfaco,
            &[
                Ordono::Translokigo { dx: 1, dy: 1 },
                Ordono::Translokigo { dx: 1, dy: 1 },
                Ordono::Maltranslokigo,
                Ordono::Plenigi {
                    rektangulo: Rektangulo::nova(0, 0, 1, 1),
                    koloro: koloro(255, 200, 100, 50),
                },
                Ordono::Maltranslokigo,
                Ordono::Plenigi {
                    rektangulo: Rektangulo::nova(0, 0, 1, 1),
                    koloro: koloro(255, 10, 20, 30),
                },
            ],
        );
        assert_eq!(bufro[4], paka(255, 200, 100, 50));
        assert_eq!(bufro[0], paka(255, 10, 20, 30));
        assert_eq!(bufro[1], 0);
    }

    #[test]
    fn klipo_restas_sen_translokigo() {
        let mut bufro = [0u32; 16];
        let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
        desegnu(
            &mut surfaco,
            &[
                Ordono::Translokigo { dx: 1, dy: 0 },
                Ordono::Klipo {
                    rektangulo: Rektangulo::nova(0, 0, 2, 2),
                },
                Ordono::Plenigi {
                    rektangulo: Rektangulo::nova(0, 0, 2, 2),
                    koloro: koloro(255, 200, 100, 50),
                },
                Ordono::Malŝlipo,
                Ordono::Maltranslokigo,
            ],
        );
        assert_eq!(surfaco.pikseloj[1], paka(255, 200, 100, 50));
        assert_eq!(surfaco.pikseloj[5], paka(255, 200, 100, 50));
        assert_eq!(surfaco.pikseloj[2], 0);
        assert_eq!(surfaco.pikseloj[6], 0);
    }

    #[test]
    fn translokiga_stako_ok_profundoj() {
        let mut bufro = [0u32; 10];
        let mut surfaco = Surfaco::nova(&mut bufro, 10, 1, 10).unwrap();
        let mut ordonoj = [Ordono::Maltranslokigo; 18];
        for ordono in ordonoj.iter_mut().take(8) {
            *ordono = Ordono::Translokigo { dx: 1, dy: 0 };
        }
        ordonoj[8] = Ordono::Plenigi {
            rektangulo: Rektangulo::nova(0, 0, 1, 1),
            koloro: koloro(255, 200, 100, 50),
        };
        ordonoj[17] = Ordono::Plenigi {
            rektangulo: Rektangulo::nova(0, 0, 1, 1),
            koloro: koloro(255, 10, 20, 30),
        };
        desegnu(&mut surfaco, &ordonoj);
        assert_eq!(bufro[8], paka(255, 200, 100, 50));
        assert_eq!(bufro[0], paka(255, 10, 20, 30));
    }

    #[test]
    fn trogranda_translokigo_sen_troo() {
        let mut bufro = [0u32; 4];
        let mut surfaco = Surfaco::nova(&mut bufro, 2, 2, 2).unwrap();
        let mut ordonoj = [Ordono::Translokigo {
            dx: i32::MIN,
            dy: i32::MIN,
        }; 9];
        ordonoj[8] = Ordono::Plenigi {
            rektangulo: Rektangulo::nova(0, 0, 2, 2),
            koloro: koloro(255, 200, 100, 50),
        };
        desegnu(&mut surfaco, &ordonoj);
        assert_eq!(surfaco.pikseloj, [0u32; 4]);
    }

    #[test]
    fn komuna_rando_de_plenigo_ne_duobligas() {
        let fonto = koloro(128, 64, 32, 16);
        for (larĝo, alto, disigo) in [(2u32, 2u32, 1u32), (3, 4, 1), (4, 3, 2), (5, 4, 3)] {
            let mut du = [0u32; 64];
            let mut unu = [0u32; 64];
            {
                let mut surfaco = Surfaco::nova(&mut du, 8, 8, 8).unwrap();
                desegnu(
                    &mut surfaco,
                    &[
                        Ordono::Plenigi {
                            rektangulo: Rektangulo::nova(0, 0, disigo, alto),
                            koloro: fonto,
                        },
                        Ordono::Plenigi {
                            rektangulo: Rektangulo::nova(disigo as i32, 0, larĝo - disigo, alto),
                            koloro: fonto,
                        },
                    ],
                );
            }
            {
                let mut surfaco = Surfaco::nova(&mut unu, 8, 8, 8).unwrap();
                desegnu(
                    &mut surfaco,
                    &[Ordono::Plenigi {
                        rektangulo: Rektangulo::nova(0, 0, larĝo, alto),
                        koloro: fonto,
                    }],
                );
            }
            assert_eq!(du, unu);

            let mut du = [0u32; 64];
            let mut unu = [0u32; 64];
            {
                let mut surfaco = Surfaco::nova(&mut du, 8, 8, 8).unwrap();
                desegnu(
                    &mut surfaco,
                    &[
                        Ordono::Plenigi {
                            rektangulo: Rektangulo::nova(0, 0, larĝo, disigo),
                            koloro: fonto,
                        },
                        Ordono::Plenigi {
                            rektangulo: Rektangulo::nova(0, disigo as i32, larĝo, alto - disigo),
                            koloro: fonto,
                        },
                    ],
                );
            }
            {
                let mut surfaco = Surfaco::nova(&mut unu, 8, 8, 8).unwrap();
                desegnu(
                    &mut surfaco,
                    &[Ordono::Plenigi {
                        rektangulo: Rektangulo::nova(0, 0, larĝo, alto),
                        koloro: fonto,
                    }],
                );
            }
            assert_eq!(du, unu);
        }
    }

    #[test]
    fn komuna_rando_de_ronda_rekto_nula_ne_duobligas() {
        let fonto = koloro(128, 64, 32, 16);
        for (larĝo, alto, disigo) in [(2u32, 2u32, 1u32), (3, 4, 1), (4, 3, 2), (5, 4, 3)] {
            let mut du = [0u32; 64];
            let mut unu = [0u32; 64];
            {
                let mut surfaco = Surfaco::nova(&mut du, 8, 8, 8).unwrap();
                desegnu(
                    &mut surfaco,
                    &[
                        Ordono::RondaRekto {
                            rektangulo: Rektangulo::nova(0, 0, disigo, alto),
                            radiuso: 0,
                            koloro: fonto,
                        },
                        Ordono::RondaRekto {
                            rektangulo: Rektangulo::nova(disigo as i32, 0, larĝo - disigo, alto),
                            radiuso: 0,
                            koloro: fonto,
                        },
                    ],
                );
            }
            {
                let mut surfaco = Surfaco::nova(&mut unu, 8, 8, 8).unwrap();
                desegnu(
                    &mut surfaco,
                    &[Ordono::RondaRekto {
                        rektangulo: Rektangulo::nova(0, 0, larĝo, alto),
                        radiuso: 0,
                        koloro: fonto,
                    }],
                );
            }
            assert_eq!(du, unu);
        }
    }

    #[test]
    fn komuna_rando_de_masko_ne_duobligas() {
        for (larĝo, alto, disigo) in [(2u32, 2u32, 1u32), (3, 2, 1), (3, 2, 2), (2, 3, 1)] {
            let maldekstra = [128u8; 20];
            let dekstra = [128u8; 20];
            let unu = [128u8; 20];
            let maldekstra_len = (disigo * alto) as usize;
            let dekstra_len = ((larĝo - disigo) * alto) as usize;
            let unu_len = (larĝo * alto) as usize;
            let mut du = [0u32; 64];
            let mut unua = [0u32; 64];
            {
                let mut surfaco = Surfaco::nova(&mut du, 8, 8, 8).unwrap();
                desegnu(
                    &mut surfaco,
                    &[
                        Ordono::Masko {
                            origino: (0, 0),
                            amplekso: (disigo, alto),
                            koloro: koloro(255, 200, 100, 50),
                            kovradoj: &maldekstra[..maldekstra_len],
                        },
                        Ordono::Masko {
                            origino: (disigo as i32, 0),
                            amplekso: (larĝo - disigo, alto),
                            koloro: koloro(255, 200, 100, 50),
                            kovradoj: &dekstra[..dekstra_len],
                        },
                    ],
                );
            }
            {
                let mut surfaco = Surfaco::nova(&mut unua, 8, 8, 8).unwrap();
                desegnu(
                    &mut surfaco,
                    &[Ordono::Masko {
                        origino: (0, 0),
                        amplekso: (larĝo, alto),
                        koloro: koloro(255, 200, 100, 50),
                        kovradoj: &unu[..unu_len],
                    }],
                );
            }
            assert_eq!(du, unua);
        }
    }

    #[test]
    fn ronda_specimenoj_ne_duobligas_ĉe_komuna_rando() {
        for radiuso in 1..=4i64 {
            let maldekstra = Limoj::el_rektangulo(Rektangulo::nova(0, 0, 8, 8));
            let dekstra = Limoj::el_rektangulo(Rektangulo::nova(8, 0, 8, 8));
            let supra = Limoj::el_rektangulo(Rektangulo::nova(0, 0, 8, 8));
            let malsupra = Limoj::el_rektangulo(Rektangulo::nova(0, 8, 8, 8));
            let radiuso = radiuso * FIKSITA_BAZO;

            for y in 0..16u32 {
                let y_fiksita = i64::from(y) * FIKSITA_BAZO;
                for x in 0..16u32 {
                    let x_fiksita = i64::from(x) * FIKSITA_BAZO;
                    for oy in SPECIMENAJ_CENTROJ {
                        for ox in SPECIMENAJ_CENTROJ {
                            let sx = x_fiksita + ox;
                            let sy = y_fiksita + oy;
                            assert!(
                                !(specimeno_en_ronda(sx, sy, maldekstra, radiuso)
                                    && specimeno_en_ronda(sx, sy, dekstra, radiuso))
                            );
                            assert!(
                                !(specimeno_en_ronda(sx, sy, supra, radiuso)
                                    && specimeno_en_ronda(sx, sy, malsupra, radiuso))
                            );
                        }
                    }
                }
            }
        }
    }
}

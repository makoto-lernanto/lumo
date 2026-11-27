use lumo::{Damaĝo, Koloro, Ordono, Rektangulo, Surfaco, Tiparo, desegnu, desegnu_damagon};

// La plena 1000-kaza matrico estas kontrolata per la ordinara testa rultempo.
#[cfg(miri)]
const RIPETOJ: u32 = 8;
#[cfg(not(miri))]
const RIPETOJ: u32 = 1000;

fn sekva(ŝtato: &mut u32) -> u32 {
    *ŝtato = ŝtato.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    *ŝtato
}

fn hazarda_rektangulo(ŝtato: &mut u32) -> Rektangulo {
    let x = (sekva(ŝtato) % 40) as i32 - 4;
    let y = (sekva(ŝtato) % 40) as i32 - 4;
    let larĝo = sekva(ŝtato) % 9;
    let alto = sekva(ŝtato) % 9;
    Rektangulo::nova(x, y, larĝo, alto)
}

fn hazarda_koloro(ŝtato: &mut u32) -> Koloro {
    let alfa = (sekva(ŝtato) % 255 + 1) as u8;
    Koloro::nova(
        alfa,
        (sekva(ŝtato) % u32::from(alfa)) as u8,
        (sekva(ŝtato) % u32::from(alfa)) as u8,
        (sekva(ŝtato) % u32::from(alfa)) as u8,
    )
    .unwrap()
}

#[test]
fn mil_determinitaj_ordonaroj_kongruas_kun_la_reportita_damago() {
    let mut ŝtato = 0x4C55_4D4F;

    for _ in 0..RIPETOJ {
        let mut antaŭa = [0u32; 32 * 32];
        for pikselo in &mut antaŭa {
            *pikselo = sekva(&mut ŝtato);
        }
        let mut post = antaŭa;
        let mut masko = [0u8; 64];
        for kovrado in &mut masko {
            *kovrado = sekva(&mut ŝtato) as u8;
        }

        let klipo = hazarda_rektangulo(&mut ŝtato);
        let translokigo = (
            (sekva(&mut ŝtato) % 9) as i32 - 4,
            (sekva(&mut ŝtato) % 9) as i32 - 4,
        );
        let koloro = hazarda_koloro(&mut ŝtato);
        let mut ordonoj = [Ordono::Plenigi {
            rektangulo: Rektangulo::nova(0, 0, 1, 1),
            koloro,
        }; 7];
        ordonoj[0] = Ordono::Klipo { rektangulo: klipo };
        ordonoj[1] = Ordono::Translokigo {
            dx: translokigo.0,
            dy: translokigo.1,
        };
        for ordono in &mut ordonoj[2..6] {
            let rektangulo = hazarda_rektangulo(&mut ŝtato);
            let koloro = hazarda_koloro(&mut ŝtato);
            *ordono = match sekva(&mut ŝtato) % 3 {
                0 => Ordono::Plenigi { rektangulo, koloro },
                1 => Ordono::RondaRekto {
                    rektangulo,
                    radiuso: sekva(&mut ŝtato) % 6,
                    koloro,
                },
                _ => Ordono::Masko {
                    origino: (rektangulo.x, rektangulo.y),
                    amplekso: (rektangulo.larĝo.min(8), rektangulo.alto.min(8)),
                    koloro,
                    kovradoj: &masko,
                },
            };
        }
        ordonoj[6] = Ordono::Maltranslokigo;

        let mut referenco = antaŭa;
        {
            let mut surfaco = Surfaco::nova(&mut referenco, 32, 32, 32).unwrap();
            desegnu(&mut surfaco, &ordonoj);
        }
        let mut damaĝo = Damaĝo::nova();
        {
            let mut surfaco = Surfaco::nova(&mut post, 32, 32, 32).unwrap();
            desegnu_damagon(&mut surfaco, &ordonoj, &mut damaĝo);
        }
        assert_eq!(referenco, post, "desegnu_damagon ŝanĝis la eligon");

        for y in 0..32usize {
            for x in 0..32usize {
                let indekso = y * 32 + x;
                if antaŭa[indekso] != post[indekso] {
                    assert!(
                        damaĝo.rektanguloj().iter().any(|rektangulo| {
                            x >= rektangulo.x as usize
                                && y >= rektangulo.y as usize
                                && x < rektangulo.x as usize + rektangulo.larĝo as usize
                                && y < rektangulo.y as usize + rektangulo.alto as usize
                        }),
                        "ŝanĝita pikselo ({x}, {y}) mankas en la damaĝo"
                    );
                }
            }
        }

        for rektangulo in damaĝo.rektanguloj() {
            let enhavas_skribon = (rektangulo.y as usize
                ..(rektangulo.y as usize + rektangulo.alto as usize))
                .any(|y| {
                    (rektangulo.x as usize..(rektangulo.x as usize + rektangulo.larĝo as usize))
                        .any(|x| antaŭa[y * 32 + x] != post[y * 32 + x])
                });
            assert!(enhavas_skribon, "damaĝa ortangulo sen ŝanĝita pikselo");
        }
    }
}

#[test]
fn ordonoj_sen_efika_skribo_ne_produktas_damagon() {
    let blanka = Koloro::nova(255, 255, 255, 255).unwrap();
    let mut bufro = [0xFFFF_FFFF; 16];
    let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
    let koloro = Koloro::nova(0, 0, 0, 0).unwrap();
    let kovradoj = [0u8; 4];
    let mut damaĝo = Damaĝo::nova();
    desegnu_damagon(
        &mut surfaco,
        &[
            Ordono::Plenigi {
                rektangulo: Rektangulo::nova(0, 0, 4, 4),
                koloro,
            },
            Ordono::Masko {
                origino: (0, 0),
                amplekso: (2, 2),
                koloro: Koloro::nova(255, 255, 255, 255).unwrap(),
                kovradoj: &kovradoj,
            },
            Ordono::Plenigi {
                rektangulo: Rektangulo::nova(10, 10, 2, 2),
                koloro: blanka,
            },
            Ordono::Plenigi {
                rektangulo: Rektangulo::nova(0, 0, 4, 4),
                koloro: blanka,
            },
        ],
        &mut damaĝo,
    );
    assert!(damaĝo.rektanguloj().is_empty());
    assert!(!damaĝo.plena());
}

#[test]
fn glifa_damago_kovras_nur_la_glifan_enhavon() {
    let mut datumoj = [0u8; 234];
    datumoj[33 * 2] = 0b1000_0000;
    datumoj[33 * 2 + 1] = 0b1100_0000;
    let tiparo = Tiparo::nova(2, 2, 1, &datumoj).unwrap();
    let mut bufro = [0u32; 16];
    let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
    let mut damaĝo = Damaĝo::nova();
    desegnu_damagon(
        &mut surfaco,
        &[Ordono::Glifoj {
            origino: 0,
            bazo: 2,
            koloro: Koloro::nova(255, 255, 255, 255).unwrap(),
            tiparo: &tiparo,
            tekstero: "A",
        }],
        &mut damaĝo,
    );

    assert_eq!(damaĝo.rektanguloj(), &[Rektangulo::nova(0, 1, 2, 2)]);
}

#[test]
fn pli_ol_dek_ses_disaj_skriboj_saturas_al_la_tuta_surfaco() {
    let mut bufro = [0u32; 32 * 32];
    let mut surfaco = Surfaco::nova(&mut bufro, 32, 32, 32).unwrap();
    let koloro = Koloro::nova(255, 255, 255, 255).unwrap();
    let mut ordonoj = [Ordono::Plenigi {
        rektangulo: Rektangulo::nova(0, 0, 1, 1),
        koloro,
    }; 17];
    for (indekso, ordono) in ordonoj.iter_mut().enumerate() {
        *ordono = Ordono::Plenigi {
            rektangulo: Rektangulo::nova((indekso as i32 % 5) * 2, (indekso as i32 / 5) * 2, 1, 1),
            koloro,
        };
    }

    let mut damaĝo = Damaĝo::nova();
    desegnu_damagon(&mut surfaco, &ordonoj, &mut damaĝo);

    assert!(damaĝo.plena());
    assert_eq!(damaĝo.rektanguloj(), &[Rektangulo::nova(0, 0, 32, 32)]);
}

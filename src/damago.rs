//! Damaĝo: la regionoj, en kiuj la surfaco efektive ŝanĝiĝis.

use crate::Rektangulo;

const MAKSIMUMAJ_REKTANGULOJ: usize = 16;

/// Fiksa listo de damaĝaj ortanguloj.
///
/// La listo kunfandas interkovrantajn kaj apudajn ortangulojn. Se pli ol 16
/// disaj ortanguloj necesas, `plena` fariĝas vera kaj la rastrumilo konservas
/// la tutan surfacon kiel konservativan damaĝon.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Damaĝo {
    rektanguloj: [Rektangulo; MAKSIMUMAJ_REKTANGULOJ],
    nombro: usize,
    plena: bool,
}

impl Damaĝo {
    /// Kreu malplenan damaĝliston.
    pub const fn nova() -> Self {
        Self {
            rektanguloj: [Rektangulo::nova(0, 0, 0, 0); MAKSIMUMAJ_REKTANGULOJ],
            nombro: 0,
            plena: false,
        }
    }

    /// Aldonu ortangulon, ignorante malplenajn kaj kunfandante najbarajn.
    ///
    /// Se la listo superus sian kapaciton, `plena()` fariĝas vera. La
    /// rastrumilo tiam anstataŭigas la liston per la tuta surfaco.
    pub fn aldoni(&mut self, rektangulo: Rektangulo) {
        if self.plena || rektangulo.larĝo == 0 || rektangulo.alto == 0 {
            return;
        }

        let mut kunfandita = rektangulo;
        let mut indekso = 0;
        while indekso < self.nombro {
            let ekzistanta = self.rektanguloj[indekso];
            if apudaj(kunfandita, ekzistanta) {
                let Some(unio) = unuigi(kunfandita, ekzistanta) else {
                    self.plena = true;
                    self.nombro = 0;
                    return;
                };
                kunfandita = unio;
                for sekva in indekso..self.nombro - 1 {
                    self.rektanguloj[sekva] = self.rektanguloj[sekva + 1];
                }
                self.nombro -= 1;
                indekso = 0;
            } else {
                indekso += 1;
            }
        }

        if self.nombro == MAKSIMUMAJ_REKTANGULOJ {
            self.plena = true;
            self.nombro = 0;
            return;
        }
        self.rektanguloj[self.nombro] = kunfandita;
        self.nombro += 1;
    }

    /// La damaĝaj ortanguloj en la listo.
    pub fn rektanguloj(&self) -> &[Rektangulo] {
        &self.rektanguloj[..self.nombro]
    }

    /// Ĉu la damaĝo saturiĝis kaj do ampleksas la tutan surfacon.
    pub const fn plena(&self) -> bool {
        self.plena
    }

    pub(crate) fn plenigi_per_surfaco(&mut self, larĝo: u32, alto: u32) {
        self.plena = true;
        self.nombro = 0;
        if larĝo > 0 && alto > 0 {
            self.rektanguloj[0] = Rektangulo::nova(0, 0, larĝo, alto);
            self.nombro = 1;
        }
    }
}

impl Default for Damaĝo {
    fn default() -> Self {
        Self::nova()
    }
}

fn finoj(rektangulo: Rektangulo) -> (i64, i64, i64, i64) {
    let x0 = i64::from(rektangulo.x);
    let y0 = i64::from(rektangulo.y);
    (
        x0,
        y0,
        x0 + i64::from(rektangulo.larĝo),
        y0 + i64::from(rektangulo.alto),
    )
}

fn apudaj(a: Rektangulo, b: Rektangulo) -> bool {
    let (ax0, ay0, ax1, ay1) = finoj(a);
    let (bx0, by0, bx1, by1) = finoj(b);
    ax0 <= bx1 && bx0 <= ax1 && ay0 <= by1 && by0 <= ay1
}

fn unuigi(a: Rektangulo, b: Rektangulo) -> Option<Rektangulo> {
    let (ax0, ay0, ax1, ay1) = finoj(a);
    let (bx0, by0, bx1, by1) = finoj(b);
    let x0 = ax0.min(bx0);
    let y0 = ay0.min(by0);
    let x1 = ax1.max(bx1);
    let y1 = ay1.max(by1);
    Some(Rektangulo::nova(
        i32::try_from(x0).ok()?,
        i32::try_from(y0).ok()?,
        u32::try_from(x1 - x0).ok()?,
        u32::try_from(y1 - y0).ok()?,
    ))
}

#[cfg(test)]
mod testoj {
    use super::Damaĝo;
    use crate::Rektangulo;

    #[test]
    fn kunfandas_interkovrantajn_kaj_apudajn_ortangulojn() {
        let mut damaĝo = Damaĝo::nova();
        damaĝo.aldoni(Rektangulo::nova(2, 2, 2, 2));
        damaĝo.aldoni(Rektangulo::nova(3, 1, 2, 3));
        damaĝo.aldoni(Rektangulo::nova(5, 1, 1, 3));
        assert_eq!(damaĝo.rektanguloj(), &[Rektangulo::nova(2, 1, 4, 3)]);
        assert!(!damaĝo.plena());
    }

    #[test]
    fn malplenaj_ortanguloj_ne_eniras_la_liston() {
        let mut damaĝo = Damaĝo::nova();
        damaĝo.aldoni(Rektangulo::nova(1, 1, 0, 3));
        damaĝo.aldoni(Rektangulo::nova(1, 1, 3, 0));
        assert!(damaĝo.rektanguloj().is_empty());
        assert!(!damaĝo.plena());
    }

    #[test]
    fn nesufiĉe_reprezentebla_unio_saturiĝas() {
        let mut damaĝo = Damaĝo::nova();
        damaĝo.aldoni(Rektangulo::nova(i32::MIN, 0, u32::MAX, 1));
        damaĝo.aldoni(Rektangulo::nova(i32::MAX, 0, 1, 1));
        assert!(damaĝo.plena());
        assert!(damaĝo.rektanguloj().is_empty());
    }
}

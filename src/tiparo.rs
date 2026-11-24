//! Bitmapaj tiparoj en la kerna MiKerno-formato (T01).

/// La nombro da glifoj en unu tiparo de la MiKerno-formato.
pub const GLIFOJ: usize = 117;

/// La signoj post la ASCII-bloko `0x20..=0x7E`, en la fiksa ordo de MiKerno.
const ALDONAJ: [char; 22] = [
    'ĉ', 'Ĉ', 'ĝ', 'Ĝ', 'ĥ', 'Ĥ', 'ĵ', 'Ĵ', 'ŝ', 'Ŝ', 'ŭ', 'Ŭ', '«', '»', '‹', '›', '·', '—', '→',
    '─', '│', '✕',
];

/// Bitmapa tiparo: kruda bitmAPO de 117 glifoj (T01, T02).
///
/// Ĉiu glifo havas `alto` vicojn; ĉiu vico havas `paŝo = (larĝo + 7) / 8`
/// bajtojn, kun la plej supra bito kiel la plej maldekstra pikselo. La datumoj
/// estas pruntitaj de la alvokanto; Lumo ne kopias ilin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tiparo<'a> {
    larĝo: u32,
    alto: u32,
    paŝo: u32,
    baza_linio: u32,
    glifoj: &'a [u8],
}

impl<'a> Tiparo<'a> {
    /// Kreu tiparon el la geometrio kaj la krudaj glifaj datumoj.
    ///
    /// `baza_linio` estas la distanco de la supra rando de la glifa ĉelo al la
    /// baza linio (T06). Rezultas `None`, se `larĝo` aŭ `alto` estas nulo, se
    /// `baza_linio > alto`, aŭ se `glifoj` ne havas ekzakte 117 glifojn.
    pub const fn nova(larĝo: u32, alto: u32, baza_linio: u32, glifoj: &'a [u8]) -> Option<Self> {
        if larĝo == 0 || alto == 0 || baza_linio > alto {
            return None;
        }
        let paŝo = (larĝo as u64).div_ceil(8) as u32;
        let bezonata = match (GLIFOJ as u64).checked_mul(alto as u64) {
            Some(valoro) => match valoro.checked_mul(paŝo as u64) {
                Some(valoro) => valoro,
                None => return None,
            },
            None => return None,
        };
        if glifoj.len() as u64 != bezonata {
            return None;
        }
        Some(Self {
            larĝo,
            alto,
            paŝo,
            baza_linio,
            glifoj,
        })
    }

    /// La larĝo de unu glifa ĉelo en pikseloj.
    pub const fn larĝo(&self) -> u32 {
        self.larĝo
    }

    /// La alto de unu glifa ĉelo en pikseloj.
    pub const fn alto(&self) -> u32 {
        self.alto
    }

    /// La bajtoj por unu glifa vico.
    pub const fn paŝo(&self) -> u32 {
        self.paŝo
    }

    /// La distanco de la supra glifa rando al la baza linio.
    pub const fn baza_linio(&self) -> u32 {
        self.baza_linio
    }

    pub(crate) fn glifo(&self, indekso: usize) -> &[u8] {
        let longo = self.alto as usize * self.paŝo as usize;
        let komenco = indekso * longo;
        &self.glifoj[komenco..komenco + longo]
    }
}

/// La indekso de `signo` en la MiKerno-formato, aŭ `None` se ĝi mankas.
///
/// La unuaj 95 glifoj estas ASCII `0x20..=0x7E`; poste sekvas la Esperantaj
/// literoj, la citiloj kaj la simboloj (T02). Nekonata signo ne estas
/// anstataŭigata (T03).
pub(crate) const fn indekso(signo: char) -> Option<usize> {
    if signo >= ' ' && signo <= '~' {
        return Some(signo as usize - ' ' as usize);
    }
    let mut i = 0;
    while i < ALDONAJ.len() {
        if ALDONAJ[i] == signo {
            return Some(95 + i);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod testoj {
    use super::{GLIFOJ, Tiparo, indekso};

    fn tiparo<'a>(datumoj: &'a [u8; GLIFOJ]) -> Tiparo<'a> {
        Tiparo::nova(1, 1, 0, datumoj).unwrap()
    }

    #[test]
    fn indekso_kongruas_la_ascii_blokon() {
        assert_eq!(indekso(' '), Some(0));
        assert_eq!(indekso('A'), Some(33));
        assert_eq!(indekso('~'), Some(94));
    }

    #[test]
    fn indekso_kongruas_la_aldonajn_signojn() {
        for (i, signo) in "ĉĈĝĜĥĤĵĴŝŜŭŬ«»‹›·—→─│✕".chars().enumerate()
        {
            assert_eq!(indekso(signo), Some(95 + i), "signo: {signo:?}");
        }
    }

    #[test]
    fn indekso_rifuzas_nekonatan_signojn() {
        assert_eq!(indekso('é'), None);
        assert_eq!(indekso('€'), None);
        assert_eq!(indekso('\u{7F}'), None);
    }

    #[test]
    fn nova_akceptas_validan_tiparon() {
        let datumoj = [0u8; GLIFOJ];
        let tiparo = tiparo(&datumoj);
        assert_eq!(tiparo.larĝo(), 1);
        assert_eq!(tiparo.alto(), 1);
        assert_eq!(tiparo.paŝo(), 1);
        assert_eq!(tiparo.baza_linio(), 0);
    }

    #[test]
    fn nova_rifuzas_malvalidan_geometrion() {
        let datumoj = [0u8; GLIFOJ];
        assert!(Tiparo::nova(0, 1, 0, &datumoj).is_none());
        assert!(Tiparo::nova(1, 0, 0, &datumoj).is_none());
        assert!(Tiparo::nova(1, 1, 2, &datumoj).is_none());
    }

    #[test]
    fn nova_rifuzas_malĝustan_datumlongon() {
        let mallonga = [0u8; GLIFOJ - 1];
        assert!(Tiparo::nova(1, 1, 0, &mallonga).is_none());
        let longa = [0u8; GLIFOJ + 1];
        assert!(Tiparo::nova(1, 1, 0, &longa).is_none());
    }
}

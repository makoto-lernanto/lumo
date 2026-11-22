//! Surfaco: alvokanto-provizita piksela bufro.

/// Muta piksela bufro kun geometrio.
///
/// La origino estas supre-maldekstre; la vicoj kreskas malsupren. La bajta
/// ordo estas hosta (`u32` en memoro), la kanala ordo estas kanonike
/// ARGB8888: `(a << 24) | (r << 16) | (g << 8) | b`.
pub struct Surfaco<'a> {
    pub(crate) pikseloj: &'a mut [u32],
    larĝo: u32,
    alto: u32,
    paŝo: u32,
}

impl<'a> Surfaco<'a> {
    /// Kreu surfacon el la bufro kaj ĝia geometrio.
    ///
    /// `paŝo` estas la distanco inter vicoj en **pikseloj** (ne bajtoj).
    /// Rezultas `None`, se `paŝo < larĝo` aŭ se la bufro havas malpli ol
    /// `paŝo × alto` erojn; la kontrolo kalkuliĝas per `u64` (L14).
    pub fn nova(pikseloj: &'a mut [u32], larĝo: u32, alto: u32, paŝo: u32) -> Option<Self> {
        if paŝo < larĝo {
            return None;
        }
        let bezonata = u64::from(paŝo).checked_mul(u64::from(alto))?;
        if u64::try_from(pikseloj.len()).ok()? < bezonata {
            return None;
        }
        Some(Self {
            pikseloj,
            larĝo,
            alto,
            paŝo,
        })
    }

    /// La videbla larĝo en pikseloj.
    pub const fn larĝo(&self) -> u32 {
        self.larĝo
    }

    /// La videbla alto en pikseloj.
    pub const fn alto(&self) -> u32 {
        self.alto
    }

    /// La distanco inter vicoj en pikseloj.
    pub const fn paŝo(&self) -> u32 {
        self.paŝo
    }
}

#[cfg(test)]
mod testoj {
    use super::Surfaco;

    #[test]
    fn akceptas_gustan_bufron() {
        let mut bufro = [0u32; 12];
        let surfaco = Surfaco::nova(&mut bufro, 3, 4, 3).unwrap();
        assert_eq!(surfaco.larĝo(), 3);
        assert_eq!(surfaco.alto(), 4);
        assert_eq!(surfaco.paŝo(), 3);
    }

    #[test]
    fn akceptas_paŝon_pli_grandan_ol_larĝo() {
        let mut bufro = [0u32; 20];
        assert!(Surfaco::nova(&mut bufro, 2, 4, 5).is_some());
    }

    #[test]
    fn rifuzas_tro_mallongan_bufron() {
        let mut bufro = [0u32; 7];
        assert!(Surfaco::nova(&mut bufro, 3, 2, 4).is_none());
    }

    #[test]
    fn rifuzas_paŝon_malpli_ol_larĝo() {
        let mut bufro = [0u32; 16];
        assert!(Surfaco::nova(&mut bufro, 4, 2, 3).is_none());
    }

    #[test]
    fn akceptas_malplenan_surfacon() {
        let mut bufro = [0u32; 0];
        assert!(Surfaco::nova(&mut bufro, 0, 0, 0).is_some());
    }

    #[test]
    fn kontrolo_uzas_u64() {
        let mut bufro = [0u32; 4];
        assert!(Surfaco::nova(&mut bufro, u32::MAX, 2, u32::MAX).is_none());
    }
}

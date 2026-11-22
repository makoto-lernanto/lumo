//! Antaŭmultiplika koloro.

/// Antaŭmultiplika koloro `(a, r, g, b)` kun la invarianto `r, g, b ≤ a`.
///
/// Lumo akceptas nur antaŭmultiplikan formon (L16). La konstruilo rifuzas
/// koloron, kiu malobservas la invarianton, anstataŭ «ripari» ĝin silente
/// (L18).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Koloro {
    alfa: u8,
    ruĝa: u8,
    verda: u8,
    blua: u8,
}

impl Koloro {
    /// Kreu antaŭmultiplikan koloron.
    ///
    /// Rezultas `None`, se iu el `ruĝa`, `verda` kaj `blua` superas `alfa`.
    pub const fn nova(alfa: u8, ruĝa: u8, verda: u8, blua: u8) -> Option<Self> {
        if ruĝa <= alfa && verda <= alfa && blua <= alfa {
            Some(Self {
                alfa,
                ruĝa,
                verda,
                blua,
            })
        } else {
            None
        }
    }

    /// La alfa-kanalo.
    pub const fn alfa(&self) -> u8 {
        self.alfa
    }

    /// La ruĝa kanalo.
    pub const fn ruĝa(&self) -> u8 {
        self.ruĝa
    }

    /// La verda kanalo.
    pub const fn verda(&self) -> u8 {
        self.verda
    }

    /// La blua kanalo.
    pub const fn blua(&self) -> u8 {
        self.blua
    }

    /// Ĉiuj kanaloj en la ordo `(a, r, g, b)`.
    pub(crate) const fn kanaloj(&self) -> (u8, u8, u8, u8) {
        (self.alfa, self.ruĝa, self.verda, self.blua)
    }
}

#[cfg(test)]
mod testoj {
    use super::Koloro;

    #[test]
    fn akceptas_validan_koloron() {
        let koloro = Koloro::nova(128, 64, 32, 16).unwrap();
        assert_eq!(koloro.alfa(), 128);
        assert_eq!(koloro.ruĝa(), 64);
        assert_eq!(koloro.verda(), 32);
        assert_eq!(koloro.blua(), 16);
    }

    #[test]
    fn akceptas_travideblan_koloron() {
        assert!(Koloro::nova(0, 0, 0, 0).is_some());
    }

    #[test]
    fn rifuzas_malvalidan_invarianton() {
        assert!(Koloro::nova(10, 11, 0, 0).is_none());
        assert!(Koloro::nova(10, 0, 11, 0).is_none());
        assert!(Koloro::nova(10, 0, 0, 11).is_none());
    }
}

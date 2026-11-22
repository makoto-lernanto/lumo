//! Ordono: la sola enigo de la rastrumilo.

use crate::{Koloro, Rektangulo};

/// Unu ordono en la ordonaro.
///
/// La enumo estas `#[non_exhaustive]`, por ke novaj variantoj ne rompu
/// eksterajn `match`-ojn. `kovradoj` de `Masko` estas pruntita datumaro de la
/// alvokanto; pro tio la enumo havas vivodaŭron.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Ordono<'a> {
    /// Plenigu ortangulon per unu koloro.
    Plenigi {
        /// La ortangulo en aparat-pikseloj.
        rektangulo: Rektangulo,
        /// La antaŭmultiplika koloro.
        koloro: Koloro,
    },
    /// Premu la klipon al la komunaĵo kun la nuna klipo.
    ///
    /// La klipo restas en aparat-piksela spaco; `Translokigo` ne ŝanĝas ĝin.
    Klipo {
        /// La nova klipa ortangulo.
        rektangulo: Rektangulo,
    },
    /// Restarigu ekzakte la antaŭan klipon.
    Malŝlipo,
    /// Plenigu rondigitan ortangulon per unu koloro.
    ///
    /// La radiuso estas limigata al duono de la pli mallonga flanko. Kun
    /// `radiuso = 0` la rezulto estas bajte identa al `Plenigi`.
    RondaRekto {
        /// La ortangulo en aparat-pikseloj.
        rektangulo: Rektangulo,
        /// La radiuso de la anguloj en aparat-pikseloj.
        radiuso: u32,
        /// La antaŭmultiplika koloro.
        koloro: Koloro,
    },
    /// Apliku 8-bitan kovradan maskon.
    ///
    /// `kovradoj` estas densa (sen paŝo), vico post vico, kaj ampleksas
    /// `amplekso.0 × amplekso.1` bajtojn. Tro mallonga tranĉaĵo estas
    /// ignorata (L22).
    Masko {
        /// La supra-maldekstra angulo de la masko en aparat-pikseloj.
        origino: (i32, i32),
        /// La larĝo kaj alto de la masko en pikseloj.
        amplekso: (u32, u32),
        /// La antaŭmultiplika koloro, kun kiu la masko kunfandiĝas.
        koloro: Koloro,
        /// La 8-bitaj kovradoj, po unu bajto por pikselo.
        kovradoj: &'a [u8],
    },
    /// Premu translokigon sur la stakon por la sekvaj geometrioj.
    Translokigo {
        /// La horizontala delokiĝo en aparat-pikseloj.
        dx: i32,
        /// La vertikala delokiĝo en aparat-pikseloj.
        dy: i32,
    },
    /// Restarigu ekzakte la antaŭan translokigon.
    Maltranslokigo,
}

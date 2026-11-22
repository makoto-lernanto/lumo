//! Ordono: la sola enigo de la rastrumilo.

use crate::{Koloro, Rektangulo};

/// Unu ordono en la ordonaro.
///
/// La enumo estas `#[non_exhaustive]`, por ke novaj variantoj ne rompu
/// eksterajn `match`-ojn.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Ordono {
    /// Plenigu ortangulon per unu koloro.
    Plenigi {
        /// La ortangulo en aparat-pikseloj.
        rektangulo: Rektangulo,
        /// La antaŭmultiplika koloro.
        koloro: Koloro,
    },
    /// Premu la klipon al la komunaĵo kun la nuna klipo.
    Klipo {
        /// La nova klipa ortangulo.
        rektangulo: Rektangulo,
    },
    /// Restarigu ekzakte la antaŭan klipon.
    Malŝlipo,
}

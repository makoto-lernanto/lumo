//! Lumo — pura Rust-rastrumilo por la grafika interfaco FluoU.
//!
//! Lumo transformas ordonaron (sendoliston) en pikselojn sur surfaco, kiun
//! provizas la alvokanto. Ĝi estas biblioteko: neniu sistemvoko, neniu fadeno,
//! neniu propra memoro kaj neniu dosieraliro.
//!
//! La deviga normo estas `docs/LUMO.md` en ĉi tiu deponejo. Kodo, identigiloj
//! kaj dokumentoj estas Esperantaj.
//!
//! # Ekzemplo
//!
//! ```
//! use lumo::{desegnu, Koloro, Ordono, Rektangulo, Surfaco};
//!
//! let mut bufro = [0u32; 16];
//! let mut surfaco = Surfaco::nova(&mut bufro, 4, 4, 4).unwrap();
//! let koloro = Koloro::nova(255, 200, 100, 50).unwrap();
//! desegnu(
//!     &mut surfaco,
//!     &[Ordono::Plenigi {
//!         rektangulo: Rektangulo::nova(0, 0, 4, 4),
//!         koloro,
//!     }],
//! );
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod desegnu;
mod geometrio;
#[cfg(kani)]
mod kani_pruvoj;
mod koloro;
mod ordono;
mod surfaco;

pub use desegnu::desegnu;
pub use geometrio::Rektangulo;
pub use koloro::Koloro;
pub use ordono::Ordono;
pub use surfaco::Surfaco;

/// La versio de Lumo, legata el `Cargo.toml`.
pub const VERSIO: &str = env!("CARGO_PKG_VERSION");

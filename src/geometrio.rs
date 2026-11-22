//! Ortangula geometrio en aparat-pikseloj.

/// Ortangulo kun entjeraj koordinatoj kaj ampleksoj.
///
/// La origino estas supre-maldekstre; `y` kreskas malsupren. La koordinatoj
/// rajtas esti negativaj aŭ ekster la surfaco; la rastrumilo klipas ilin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rektangulo {
    /// La x-koordinato de la maldekstra rando.
    pub x: i32,
    /// La y-koordinato de la supra rando.
    pub y: i32,
    /// La larĝo en aparat-pikseloj.
    pub larĝo: u32,
    /// La alto en aparat-pikseloj.
    pub alto: u32,
}

impl Rektangulo {
    /// Kreu ortangulon el la maldekstra kaj supra koordinatoj kaj la ampleksoj.
    pub const fn nova(x: i32, y: i32, larĝo: u32, alto: u32) -> Self {
        Self { x, y, larĝo, alto }
    }
}

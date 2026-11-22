//! Formalaj pruvoj per Kani; ili ruliĝas nur en DI (Q02).

use crate::desegnu::{Limoj, miksi_kanalojn, videbla_regiono};
use crate::geometrio::Rektangulo;

/// R07: kun premultiplika fonto la kunfandita rezulto neniam troas 255.
#[kani::proof]
fn miksi_neniam_troas() {
    let as_ = kani::any();
    let cs = kani::any();
    kani::assume(cs <= as_);
    let ad = kani::any();
    let cd = kani::any();
    let kovrado = kani::any();

    let (alfa, koloro) = miksi_kanalojn(as_, cs, ad, cd, kovrado);
    assert!(alfa <= 255);
    assert!(koloro <= 255);
}

/// L15: la videbla regiono restas ene de `larĝo × alto`, kaj la plej granda
/// indekso restas ene de la bufro kiam `paŝo ≥ larĝo`.
///
/// La pruvo limigas la dimensiojn al `u16::MAX` — praktika supra limo por
/// FluoU-surfacoj — por ke la indeksa kalkulo restu solvebla en la templimo
/// de Q06. `Surfaco::nova` jam garantias `paŝo ≥ larĝo`. La klipaj
/// limoj konstruiĝas el la surfaco kaj ortangulo, kiel ili fakte konstruiĝas
/// en la klipa stako (L19); ĉiu klipa stako finas kiel unu ortangulo.
#[kani::proof]
fn klipo_restas_en_surfaco() {
    const MAKSIMA_DIMENSIO: u32 = u16::MAX as u32;

    let larĝo: u32 = kani::any();
    let alto: u32 = kani::any();
    let paŝo: u32 = kani::any();
    kani::assume(larĝo > 0 && larĝo <= MAKSIMA_DIMENSIO);
    kani::assume(alto > 0 && alto <= MAKSIMA_DIMENSIO);
    kani::assume(paŝo >= larĝo && paŝo <= MAKSIMA_DIMENSIO);

    let klipo = Rektangulo {
        x: kani::any(),
        y: kani::any(),
        larĝo: kani::any(),
        alto: kani::any(),
    };
    let limoj = Limoj::el_surfaco(larĝo, alto).intersekci(Limoj::el_rektangulo(klipo));
    let rektangulo = Rektangulo {
        x: kani::any(),
        y: kani::any(),
        larĝo: kani::any(),
        alto: kani::any(),
    };

    if let Some(regiono) = videbla_regiono(larĝo, alto, limoj, rektangulo) {
        assert!(regiono.x0 < regiono.x1);
        assert!(regiono.x1 <= larĝo);
        assert!(regiono.y0 < regiono.y1);
        assert!(regiono.y1 <= alto);

        // Sub la dimensia limo la indeksoj restas sub 2^32, do la u32-kalkulo
        // kongruas kun la u64-indekso de `plenigi`; ĝi ankaŭ restas solvebla
        // en la templimo de Q06.
        assert!(regiono.x1 <= paŝo);
        let lasta = (regiono.y1 - 1) * paŝo + (regiono.x1 - 1);
        let buĝeto = paŝo * alto;
        assert!(lasta < buĝeto);
    }
}

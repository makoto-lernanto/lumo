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
#[kani::proof]
fn klipo_restas_en_surfaco() {
    let larĝo: u32 = kani::any();
    let alto: u32 = kani::any();
    let paŝo: u32 = kani::any();
    let limoj = Limoj {
        x0: kani::any(),
        y0: kani::any(),
        x1: kani::any(),
        y1: kani::any(),
    };
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

        if paŝo >= larĝo {
            let lasta = (u64::from(regiono.y1) - 1) * u64::from(paŝo) + (u64::from(regiono.x1) - 1);
            let buĝeto = u64::from(paŝo) * u64::from(alto);
            assert!(lasta < buĝeto);
        }
    }
}

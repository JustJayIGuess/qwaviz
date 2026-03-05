//! A test program decomposing a 1-D quantum state in position basis
//! into an energy eigenbasis and allowing it to evolve.
#![deny(missing_docs)]

use crate::framework::prelude::*;

pub mod framework;
pub(crate) mod frontend;

fn main() {
    let ket = square_in_harmonic_well();
    frontend::run(ket);
}

/* -------------------------------------------------------------------------- */
/*                          Some demo wavefunctions:                          */
/* -------------------------------------------------------------------------- */

#[allow(unused)]
fn square_in_harmonic_well() -> Ket<WF1D> {
    let hw = HarmonicWell::new(10.0, 1.0, 0.001, 1.0, 4.0);
    let ket_0 = Ket::new(
        Arc::new(|_, _| Complex32::ONE),
        SubDomain1D {
            lower: -1.0,
            upper: 1.0,
            step_size: 0.001,
        },
    )
    .translate_space(1.5);
    hw.evolution(&ket_0, 0.0, 1, 128)
}

#[allow(unused)]
fn sudden_isw_expansion() -> Ket<WF1D> {
    let isw = InfiniteSquareWell::new(2.0, 1.0, 2.0, 0.001);
    let ket_0 = isw.expansion_state(1.0, 1);
    isw.evolution(&ket_0, 0.0, 1, 512)
}

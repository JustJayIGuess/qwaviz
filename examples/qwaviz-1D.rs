//! A test program decomposing a 1-D quantum state in position basis
//! into an energy eigenbasis and allowing it to evolve.

use qwaviz::prelude::*;

fn main() {
    let ket = square_in_harmonic_well();
    run_viz_1d(ket);
}

/* -------------------------------------------------------------------------- */
/*                          Some demo wavefunctions:                          */
/* -------------------------------------------------------------------------- */

/// A square wave offset from the centre of a harmonic well
#[allow(unused)]
fn square_in_harmonic_well() -> Ket<Sign1D> {
    let hw = HarmonicWell::new(10.0, 1.0, 1.0, 4.0);
    let ket_0 = Ket::new_static(
        |_, _| Complex32::ONE,
        SubDomain1D {
            lower: -1.0,
            upper: 1.0,
        },
    )
    .translate_space(1.5);
    hw.evolution(&ket_0, 0.0, 0.001, 1, 128)
}

/// The ground state of a small infinite square well in a larger one
#[allow(unused)]
fn sudden_isw_expansion() -> Ket<Sign1D> {
    let isw = InfiniteSquareWell::new(2.0, 1.0, 2.0);
    let ket_0 = isw.expansion_state(1.0, 1);
    isw.evolution(&ket_0, 0.0, 0.001, 1, 512)
}

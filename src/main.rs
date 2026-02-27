//! A test program decomposing a quantum state into an eigenbasis.
#![deny(missing_docs)]

use std::time::Instant;

use crate::infinite_square_well::InfiniteSquareWell;

pub mod braket;
pub mod domains;
pub mod fields;
pub mod infinite_square_well;
pub mod signatures;
pub mod vectorspaces;

fn main() {
    let time_create_ket = Instant::now();
    let isw = InfiniteSquareWell::new(1.0, 1.0, 1.0, 1.0 / 1000.0);
    let ket_0 = isw.expansion_state(0.5, 1);
    let ket_1 = isw.evolve_state_from_t(&ket_0, 0.0, 128);
    let _ = isw.evolve_state_from_t(&ket_1, 0.0, 128);
    println!("Create ket: {:?}", time_create_ket.elapsed());
}

//! For representing solvable, confined time-independent potentials or other systems with discrete states.

mod harmonic_well;
mod infinite_square_well;
mod two_state;

pub use harmonic_well::HarmonicWell;
pub use infinite_square_well::InfiniteSquareWell;
pub use two_state::TwoState;

use super::{
    braket::{Bra, Ket, WFKet},
    core::vectorspace::VectorSpace,
    wavefunction::signature::WFSignature,
};

/// A time-independent potential for which the Schroedinger equation can be solved.
/// Note that the potential must be confining so that eigenstates are discrete.
pub trait DiscreteSystem<S: WFSignature> {
    /// Return the `n`th energy eigenstate of the specified ISW, in the basis specified by `S::Space`
    fn energy_eigenstate(&self, n: i32) -> WFKet<S>;

    /// Return a state which evolves from `initial_state(t=0)` according to the Schrodinger equation
    fn evolution(&self, initial_state: &WFKet<S>, t0: S::Time, min_n: i32, max_n: i32) -> WFKet<S> {
        let coef_eigenkets: Vec<(S::Out, WFKet<S>)> = (min_n..=max_n)
            .map(|i| {
                let basis_state = self.energy_eigenstate(i);
                (
                    WFKet::<S>::adjoint(&basis_state).apply(initial_state, t0),
                    basis_state,
                )
            })
            .collect();

        WFKet::<S>::weighted_sum(coef_eigenkets)
    }
}

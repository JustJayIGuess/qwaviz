//! For representing solvable, confined time-independent potentials.

mod harmonic_well;
mod infinite_square_well;

pub use harmonic_well::HarmonicWell;
pub use infinite_square_well::InfiniteSquareWell;

use super::{
    braket::{Bra, Ket, WFKet},
    core::vectorspace::VectorSpace,
    wavefunction::signature::WFSignature,
};

/// A time-independent potential for which the Schroedinger equation can be solved.
/// Note that the potential must be confining so that eigenstates are discrete.
pub trait ConfinedPotential<S: WFSignature> {
    /// Return the `n`th eigenstate of the specified ISW
    fn eigenstate(&self, n: usize) -> WFKet<S>;

    /// Return a state which evolves from `initial_state(t=0)` according to the Schrodinger equation
    fn evolution(&self, initial_state: &WFKet<S>, t0: S::Time, max_n: usize) -> WFKet<S> {
        let coef_eigenkets: Vec<(S::Out, WFKet<S>)> = (1..=max_n)
            .map(|i| {
                let basis_state = self.eigenstate(i);
                (
                    WFKet::<S>::adjoint(&basis_state).apply(initial_state, t0),
                    basis_state,
                )
            })
            .collect();

        WFKet::<S>::weighted_sum(coef_eigenkets)
    }
}

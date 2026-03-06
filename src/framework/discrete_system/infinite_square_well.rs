//! Functionality for working with infinite square well problems

use std::{f32::consts::PI, sync::Arc};

use num_complex::Complex32;

use crate::framework::prelude::SubDomain1D;

use super::super::{
    braket::{Ket, WFOperation},
    discrete_system::DiscreteSystem,
    wavefunction::signature::{Sign1D, WFSignature},
};

#[derive(Clone)]
/// A struct representing an infinite square well with a particle inside.
pub struct InfiniteSquareWell {
    /// The width of the ISW
    width: f32,
    /// The mass of the particle
    mass: f32,
    /// The value of hbar to use
    hbar: f32,
}

/// Get the value of the `n`th energy eigenfunction at `x`, `t` with given parameters
fn eigenfunction(x: f32, t: f32, width: f32, mass: f32, hbar: f32, n: i32) -> Complex32 {
    let energy = (n as f32 * PI * hbar / width).powi(2) / (2.0 * mass);
    let coef = (2.0 / width).sqrt();
    let phase_x: f32 = (n as f32) * PI * x / width;
    coef * phase_x.sin() * Complex32::cis(-energy * t / hbar)
}

impl DiscreteSystem<Sign1D> for InfiniteSquareWell {
    fn energy_eigenstate(&self, n: i32) -> Ket<Sign1D> {
        let width = self.width;
        let mass = self.mass;
        let hbar = self.hbar;
        Ket::<Sign1D>::new(
            Arc::new(move |x, t| eigenfunction(x, t, width, mass, hbar, n)),
            SubDomain1D {
                lower: 0.0,
                upper: width,
            },
        )
    }
}

impl InfiniteSquareWell {
    /// Create an infinite square well
    #[must_use]
    pub fn new(width: f32, mass: f32, hbar: f32, step_size: f32) -> InfiniteSquareWell {
        InfiniteSquareWell {
            width,
            mass,
            hbar,
        }
    }

    /// Return a the state resulting from suddenly expanding an ISW from width `initial_width` to `final_width`
    #[must_use]
    pub fn expansion_state(&self, initial_width: f32, n: i32) -> Ket<Sign1D> {
        let mass = self.mass;
        let hbar = self.hbar;
        Ket::<Sign1D>::new(
            Arc::new(move |x, t| eigenfunction(x, t, initial_width, mass, hbar, n)),
            SubDomain1D {
                lower: 0.0,
                upper: initial_width,
            },
        )
    }
}

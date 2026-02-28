//! Functionality for working with infinite square well problems

use std::{f32::consts::PI, sync::Arc};

use num_complex::Complex32;

use crate::{
    braket::{Bra, Ket, WFKet, WFOperation},
    signatures::{WF1Space1Time, WFSignature},
    vectorspaces::VectorSpace,
};

type Ket1D = WFKet<WF1Space1Time>;
type SubDom = <WF1Space1Time as WFSignature>::SubDom;

fn eigenfunction(x: f32, t: f32, width: f32, mass: f32, hbar: f32, n: usize) -> Complex32 {
    let energy = (n as f32 * PI * hbar / width).powi(2) / (2.0 * mass);
    let coef = (2.0 / width).sqrt();
    let phase_x: f32 = (n as f32) * PI * x / width;
    coef * phase_x.sin() * Complex32::cis(-energy * t / hbar)
}

#[derive(Clone)]
/// A struct representing an infinite square well with a particle inside.
pub struct InfiniteSquareWell {
    /// The width of the ISW
    width: f32,
    /// The mass of the particle
    mass: f32,
    /// The value of hbar to use
    hbar: f32,
    /// The step size to use when doing calculations
    step_size: f32,
}

impl InfiniteSquareWell {
    /// Create an infinite square well
    pub fn new(width: f32, mass: f32, hbar: f32, step_size: f32) -> InfiniteSquareWell {
        InfiniteSquareWell {
            width,
            mass,
            hbar,
            step_size,
        }
    }

    /// Return the `n`th eigenstate of the specified ISW
    pub fn eigenstate(&self, n: usize) -> Ket1D {
        let width = self.width;
        let mass = self.mass;
        let hbar = self.hbar;
        Ket1D {
            wavefunction: WFOperation::func(Arc::new(move |x, t| {
                eigenfunction(x, t, width, mass, hbar, n)
            })),
            subdomain: SubDom {
                lower: 0.0,
                upper: width,
                step_size: self.step_size,
            },
        }
    }

    /// Return a the state resulting from suddenly expanding an ISW from width `initial_width` to `final_width`
    pub fn expansion_state(&self, initial_width: f32, n: usize) -> Ket1D {
        let mass = self.mass;
        let hbar = self.hbar;
        Ket1D {
            wavefunction: WFOperation::func(Arc::new(move |x, t| {
                eigenfunction(x, t, initial_width, mass, hbar, n)
            })),
            subdomain: SubDom {
                lower: 0.0,
                upper: initial_width,
                step_size: self.step_size,
            },
        }
    }

    /// Return a state which evolves from `initial_state(t=0)` according to the Schrodinger equation
    pub fn evolution(&self, initial_state: &Ket1D, t0: f32, max_n: usize) -> Ket1D {
        let coef_eigenkets: Vec<(Complex32, Ket1D)> = (1..=max_n)
            .map(|i| {
                let basis_state = self.eigenstate(i);
                (
                    Ket1D::adjoint(&basis_state).apply(initial_state, t0),
                    basis_state,
                )
            })
            .collect();

        Ket1D::weighted_sum(coef_eigenkets)
    }
}

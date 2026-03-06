//! Harmonic well potential

use std::{
    f32::consts::{PI, SQRT_2},
    sync::{Arc, LazyLock},
};

use num_complex::Complex32;

use super::super::{
    braket::{Ket, WFOperation},
    discrete_system::DiscreteSystem,
    wavefunction::signature::{WF1D, WFSignature},
};

type Ket1D = Ket<WF1D>;
type SubDom = <WF1D as WFSignature>::SubDom;

/// A struct representing a harmonic well
pub struct HarmonicWell {
    omega: f32,
    mass: f32,
    step_size: f32,
    hbar: f32,
    half_width: f32,
}

static PI_FTH_RT: LazyLock<f32> = LazyLock::new(|| 1.0 / PI.sqrt().sqrt());
static PI_SQRT: LazyLock<f32> = LazyLock::new(|| PI.sqrt());

/// Courtesy of `ChatGPT`!
/// Seriously, how is it so hard to find good implementations of
/// normalised hermite polynomials?
fn norm_hermite(n: i32, x: f32) -> f32 {
    let psi0 = *PI_FTH_RT;
    if n == 0 {
        return psi0;
    }

    let psi1 = SQRT_2 * x * psi0;
    if n == 1 {
        return psi1;
    }

    let mut psi_nm1 = psi0;
    let mut psi_n = psi1;

    for k in 1..n {
        let kf = k as f32;
        let psi_np1 = (2.0 / (kf + 1.0)).sqrt() * x * psi_n - (kf / (kf + 1.0)).sqrt() * psi_nm1;

        psi_nm1 = psi_n;
        psi_n = psi_np1;
    }

    psi_n
}

fn eigenfunction(x: f32, t: f32, omega: f32, mass: f32, hbar: f32, n: i32) -> Complex32 {
    let scale = (mass * omega / hbar).sqrt();
    let y = scale * x;
    let psi = norm_hermite(n, y);
    let prefactor = scale.sqrt();
    let exp = (-0.5 * y * y).exp();
    let energy = hbar * omega * (n as f32 + 0.5);
    prefactor * exp * psi * Complex32::cis(-energy * t / hbar)
}

impl HarmonicWell {
    /// Create a harmonic well
    #[must_use]
    pub fn new(omega: f32, mass: f32, step_size: f32, hbar: f32, half_width: f32) -> HarmonicWell {
        HarmonicWell {
            omega,
            mass,
            step_size,
            hbar,
            half_width,
        }
    }
}

impl DiscreteSystem<WF1D> for HarmonicWell {
    fn energy_eigenstate(&self, n: i32) -> Ket1D {
        let (omega, mass, hbar) = (self.omega, self.mass, self.hbar);
        // let width = (hbar / (mass * omega)).sqrt();
        Ket1D::new(
            Arc::new(move |x, t| eigenfunction(x, t, omega, mass, hbar, n - 1)),
            SubDom {
                lower: -self.half_width,
                upper: self.half_width,
            },
        )
    }
}

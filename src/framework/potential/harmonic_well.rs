//! Harmonic well potential

use std::{f32::consts::PI, sync::Arc};

use lazy_static::lazy_static;
use num_complex::Complex32;

use super::super::{
    braket::{WFKet, WFOperation},
    potential::ConfinedPotential,
    wavefunction::signature::{WF1Space1Time, WFSignature},
};

type Ket1D = WFKet<WF1Space1Time>;
type SubDom = <WF1Space1Time as WFSignature>::SubDom;

/// A struct representing a harmonic well
pub struct HarmonicWell {
    omega: f32,
    mass: f32,
    step_size: f32,
    hbar: f32,
    half_width: f32,
}

lazy_static! {
    static ref PI_FTH_RT: f32 = 1.0 / PI.sqrt().sqrt();
    static ref PI_SQRT: f32 = PI.sqrt();
}

/// Courtesy of ChatGPT!
/// Seriously, how is it so hard to find good implementations of
/// normalised hermite polynomials?
fn norm_hermite(n: usize, x: f32) -> f32 {
    match n {
        0 => return *PI_FTH_RT, // 1 / π^(1/4)
        1 => return x * *PI_FTH_RT,
        _ => {}
    }

    // unnormalized recurrence
    let mut h_nm1 = 1.0; // H_0
    let mut h_n = 2.0 * x; // H_1

    for k in 1..n {
        let h_np1 = 2.0 * x * h_n - 2.0 * (k as f32) * h_nm1;
        h_nm1 = h_n;
        h_n = h_np1;
    }

    // compute normalization incrementally: sqrt(2^n n! sqrt(pi))
    let mut norm_sq = *PI_SQRT; // sqrt(pi)
    for k in 0..n {
        norm_sq *= 2.0 * (k as f32 + 1.0);
    }

    h_n / norm_sq.sqrt()
}

fn eigenfunction(x: f32, t: f32, omega: f32, mass: f32, hbar: f32, n: usize) -> Complex32 {
    let coef = (mass * omega / hbar).sqrt().sqrt();
    let herm = norm_hermite(n, (mass * omega / hbar).sqrt() * x);
    let energy = hbar * omega * (n as f32 + 0.5);
    let exp = (-mass * omega * x * x / (2.0 * hbar)).exp();
    coef * exp * herm * Complex32::cis(-energy * t / hbar)
}

impl HarmonicWell {
    /// Create a harmonic well
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

impl ConfinedPotential<WF1Space1Time> for HarmonicWell {
    fn eigenstate(&self, n: usize) -> Ket1D {
        let (omega, mass, hbar) = (self.omega, self.mass, self.hbar);
        // let width = (hbar / (mass * omega)).sqrt();
        Ket1D {
            wavefunction: WFOperation::func(Arc::new(move |x, t| {
                eigenfunction(x, t, omega, mass, hbar, n - 1)
            })),
            subdomain: SubDom {
                lower: -self.half_width,
                upper: self.half_width,
                step_size: self.step_size,
            },
        }
    }
}

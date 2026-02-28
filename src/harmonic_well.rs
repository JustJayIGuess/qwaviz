//! Harmonic well potential

use std::{f32::consts::PI, sync::Arc};

use num_complex::Complex32;

use crate::{
    braket::{WFKet, WFOperation},
    potential::ConfinedPotential,
    signatures::{WF1Space1Time, WFSignature},
};

type Ket1D = WFKet<WF1Space1Time>;
type SubDom = <WF1Space1Time as WFSignature>::SubDom;

/// A struct representing a harmonic well
pub struct HarmonicWell {
    omega: f32,
    mass: f32,
    step_size: f32,
    hbar: f32,
}

/// Courtesy of ChatGPT
fn hermite(n: usize, x: f32) -> f32 {
    match n {
        0 => 1.0,
        1 => 2.0 * x,
        _ => {
            let mut h_nm2 = 1.0; // H_0
            let mut h_nm1 = 2.0 * x; // H_1
            let mut h_n = 0.0;

            for k in 1..n {
                h_n = 2.0 * x * h_nm1 - 2.0 * (k as f32) * h_nm2;
                h_nm2 = h_nm1;
                h_nm1 = h_n;
            }

            h_n
        }
    }
}

fn factorial(n: usize) -> usize {
    match n {
        0 => 1,
        n => n * factorial(n - 1),
    }
}

fn eigenfunction(x: f32, t: f32, omega: f32, mass: f32, hbar: f32, n: usize) -> Complex32 {
    let coef = 1.0 / (2.0f32.powi(n as i32) * factorial(n) as f32)
        * (mass * omega / (PI * hbar)).sqrt().sqrt();
    let herm = hermite(n, (mass * omega / hbar).sqrt() * x);
    let energy = hbar * omega * (n as f32 + 0.5);
    coef * (-mass * omega * x * x / (2.0 * hbar)).exp() * herm * Complex32::cis(-energy * t / hbar)
}

impl HarmonicWell {
    /// Create a harmonic well
    pub fn new(omega: f32, mass: f32, step_size: f32, hbar: f32) -> HarmonicWell {
        HarmonicWell {
            omega,
            mass,
            step_size,
            hbar,
        }
    }
}

impl ConfinedPotential<WF1Space1Time> for HarmonicWell {
    fn eigenstate(&self, n: usize) -> Ket1D {
        let (omega, mass, hbar) = (self.omega, self.mass, self.hbar);
        let width = (hbar / (mass * omega)).sqrt();
        Ket1D {
            wavefunction: WFOperation::func(Arc::new(move |x, t| {
                eigenfunction(x, t, omega, mass, hbar, n - 1)
            })),
            subdomain: SubDom {
                lower: -5.0 * width,
                upper: 5.0 * width,
                step_size: self.step_size,
            },
        }
    }
}

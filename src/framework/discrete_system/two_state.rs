use std::sync::Arc;

use num_complex::Complex32;

use crate::framework::{
    braket::{WFKet, WFOperation},
    core::{domain::finite_domains::FiniteSubDomain, field::Field},
    discrete_system::DiscreteSystem,
    wavefunction::signature::WFFinite,
};

/// A two-state quantum system
pub struct TwoState {
    level_1: f32,
    level_2: f32,
    coupling: Complex32,
    hbar: f32,
}

impl TwoState {
    /// Return a new TwoState system with given parameters.
    pub fn new(level_1: f32, level_2: f32, coupling: Complex32, hbar: f32) -> Self {
        Self {
            level_1,
            level_2,
            coupling,
            hbar,
        }
    }
}

impl DiscreteSystem<WFFinite> for TwoState {
    fn energy_eigenstate(&self, n: i32) -> WFKet<WFFinite> {
        if n < 0 || n > 1 {
            panic!("Index of TwoState eigenstate invalid. Only states 0,1 allowed.")
        }
        let (level_1, level_2, coupling, hbar) =
            (self.level_1, self.level_2, self.coupling, self.hbar);

        let delta = level_1 - level_2;
        let v = coupling.norm();
        let theta = 0.5 * (2.0 * v / delta).atan();
        let phase = if v == 0.0 {
            Complex32::ONE
        } else {
            coupling / v
        };
        let eigenstate = match n {
            0 => (Complex32::new(-theta.sin(), 0.0), phase * theta.cos()),
            _ => (Complex32::new(theta.cos(), 0.0), phase * theta.sin()),
        };
        let split = ((delta / 2.0).powi(2) + coupling.norm_sqr()).sqrt()
            * match n {
                0 => -1.0,
                _ => 1.0,
            };
        let mean_level = 0.5 * (level_1 + level_2);
        let energy: f32 = mean_level + split;
        WFKet::new(
            Arc::new(move |x: i32, t: f32| {
                Complex32::cis(-energy * t / hbar)
                    * match x {
                        0 => eigenstate.0,
                        _ => eigenstate.1,
                    }
            }),
            FiniteSubDomain {
                min_idx: 0,
                max_idx: 1,
            },
        )
    }
}

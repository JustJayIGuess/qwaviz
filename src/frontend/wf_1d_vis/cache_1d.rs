use num_complex::{Complex32, ComplexFloat};
use thiserror::Error;

use crate::framework::{
    braket::Ket,
    wavefunction::{Wavefunction, signature::WF1D},
};

#[derive(Clone)]
pub struct Cache1D {
    min: f32,
    range: f32,
    step_size: f32,
    n: usize,
    pub cache_value: Vec<Complex32>,
    pub cache_density: Vec<f32>,
}

impl Default for Cache1D {
    fn default() -> Self {
        Self::new(0.0, 1.0, 0.1).unwrap()
    }
}

#[derive(Debug, Error)]
pub enum Cache1DError {
    #[error("Cache1D.min must be less than Cache1D.max")]
    InvalidMinMax,
    #[error("Cache1D step size must be positive")]
    NegativeStepSize,
}

impl Cache1D {
    pub fn new(min: f32, max: f32, step_size: f32) -> Result<Cache1D, Cache1DError> {
        if min > max {
            return Err(Cache1DError::InvalidMinMax);
        }
        if step_size <= 0.0 {
            return Err(Cache1DError::NegativeStepSize);
        }
        let num = ((max - min) / step_size).ceil() as usize;
        Ok(Cache1D {
            min,
            range: max - min,
            step_size,
            cache_value: vec![Complex32::ZERO; num],
            cache_density: vec![0.0; num],
            n: num,
        })
    }

    pub fn from_ket(ket: &Ket<WF1D>, step_size: f32) -> Result<Cache1D, Cache1DError> {
        Self::new(ket.subdomain.lower, ket.subdomain.upper, step_size)
    }

    pub fn update(&mut self, wf: &Ket<WF1D>, t: f32) {
        // let mut x = self.min;
        for i in 0..self.cache_value.len() {
            let x = self.min + i as f32 * self.step_size;
            self.cache_value[i] = wf.f(x, t);
            self.cache_density[i] = wf.p(x, t).abs();
            // x += self.step_size;
        }
    }

    pub fn get_value(&self, x: f32) -> Complex32 {
        if x <= self.min {
            return self.cache_value[0];
        }
        if x >= self.min + self.range {
            return self.cache_value[self.n - 1];
        }

        let idx = ((x - self.min) / self.step_size).round() as usize;
        self.cache_value[idx.clamp(0, self.n - 1)]
    }

    pub fn get_density(&self, x: f32) -> f32 {
        if x <= self.min {
            return self.cache_density[0];
        }
        if x >= self.min + self.range {
            return self.cache_density[self.n - 1];
        }

        let idx = ((x - self.min) / self.step_size).round() as usize;
        self.cache_density[idx.clamp(0, self.n - 1)]
    }
}

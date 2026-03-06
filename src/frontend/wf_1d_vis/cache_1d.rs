use num_complex::Complex32;
use splines::{Interpolation, Key, Spline};
use thiserror::Error;

use crate::framework::{
    braket::Ket,
    wavefunction::{Wavefunction, signature::WF1D},
};

#[derive(Clone)]
pub struct Cache1D {
    spline_re: Spline<f32, f32>,
    spline_im: Spline<f32, f32>,
    spline_p: Spline<f32, f32>,
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

        let mut keys: Vec<Key<f32, f32>> = vec![];

        let mut x = min;
        keys.push(Key::new(
            x - 2.0 * step_size,
            0.0,
            Interpolation::CatmullRom,
        ));
        keys.push(Key::new(x - step_size, 0.0, Interpolation::CatmullRom));
        while x <= max {
            keys.push(Key::new(x, 0.0, Interpolation::CatmullRom));
            x += step_size;
        }
        keys.push(Key::new(x, 0.0, Interpolation::CatmullRom));
        keys.push(Key::new(x + step_size, 0.0, Interpolation::CatmullRom));

        Ok(Cache1D {
            spline_re: Spline::from_vec(keys.clone()),
            spline_im: Spline::from_vec(keys.clone()),
            spline_p: Spline::from_vec(keys),
        })
    }

    pub fn from_ket(ket: &Ket<WF1D>, step_size: f32) -> Result<Cache1D, Cache1DError> {
        Self::new(ket.subdomain.lower, ket.subdomain.upper, step_size)
    }

    pub fn update(&mut self, wf: &Ket<WF1D>, t: f32) {
        for i in 0..self.spline_re.len() {
            let x = self.spline_re.get(i).unwrap().t;
            if let (Some(re), Some(im), Some(p)) = (
                self.spline_re.get_mut(i),
                self.spline_im.get_mut(i),
                self.spline_p.get_mut(i),
            ) {
                let value = wf.f(x, t);
                *re.value = value.re;
                *im.value = value.im;
                *p.value = value.norm_sqr();
            }
        }
    }

    pub fn get_value(&self, x: f32) -> Complex32 {
        if let (Some(re), Some(im)) = (self.spline_re.sample(x), self.spline_im.sample(x)) {
            Complex32::new(re, im)
        } else {
            Complex32::ZERO
        }
    }

    pub fn get_density(&self, x: f32) -> f32 {
        self.spline_p.sample(x).unwrap_or(0.0)
    }
}

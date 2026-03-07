//! Caching for 1D wavefunction domains with Catmull-Rom interpolation
//! between sampled points.

use num_complex::Complex32;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use splines::{Interpolation, Key, Spline};
use thiserror::Error;

use crate::framework::{
    braket::Ket,
    wavefunction::{Wavefunction, signature::Sign1D},
};

/// A cache holding values of a wavefunction at a point in time.
/// This is updated each frame in `PreUpdate`, and is kept to prevent
/// repeat calculations and allow interpolation between sampled points via
/// a Catmull-Rom spline.
///
/// Note that the probability density can be inferred from the wavefunction,
/// so no cache is kept of the evaluation of `WFKet<WF1D>::p(&self, x, t)`.
#[derive(Clone)]
pub struct Cache1D {
    /// A cache of the real part of the wavefunction
    spline_re: Spline<f32, f32>,
    /// A cache of the imaginary part of the wavefunction
    spline_im: Spline<f32, f32>,
}

impl Default for Cache1D {
    fn default() -> Self {
        Self::new(0.0, 1.0, 0.1).unwrap()
    }
}

#[derive(Debug, Error)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum Cache1DError {
    #[error("Cache1D.min must be less than Cache1D.max")]
    InvalidMinMax,
    #[error("Cache1D step size must be positive")]
    NegativeStepSize,
}

impl Cache1D {
    /// Create a new cache ranging from min to max with the given `step_size`
    /// for sampling values.
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
        })
    }

    /// Create a cache matching the subdomain of a 1D ket.
    pub fn from_ket(ket: &Ket<Sign1D>, step_size: f32) -> Result<Cache1D, Cache1DError> {
        Self::new(ket.subdomain.lower, ket.subdomain.upper, step_size)
    }

    /// Update the cache with values at time `t`.
    pub fn update(&mut self, wf: &Ket<Sign1D>, t: f32) {
        let xs: Vec<_> = self.spline_re.into_iter().map(|k| k.t).collect();

        #[cfg(feature = "par_braket")]
        let values: Vec<_> = xs.par_iter().map(|x| wf.f(*x, t)).collect();
        #[cfg(not(feature = "par_braket"))]
        let values: Vec<_> = xs.iter().map(|x| wf.f(*x, t)).collect();

        for (i, value) in values.iter().enumerate() {
            if let (Some(re), Some(im)) = (self.spline_re.get_mut(i), self.spline_im.get_mut(i)) {
                *re.value = value.re;
                *im.value = value.im;
            }
        }
    }

    /// Get the value at the given point. This is interpolated from sampled
    /// points via Catmull-Rom.
    pub fn at(&self, x: f32) -> Complex32 {
        if let (Some(re), Some(im)) = (self.spline_re.sample(x), self.spline_im.sample(x)) {
            Complex32::new(re, im)
        } else {
            Complex32::ZERO
        }
    }
}

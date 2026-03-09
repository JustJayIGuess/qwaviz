//! Bevy components to be associated with wavefunctions being visualised.

use std::sync::Arc;

use bevy::{
    color::{Color, LinearRgba},
    ecs::component::Component,
    pbr::StandardMaterial,
};
use bevy_polyline::prelude::PolylineMaterial;
use num_complex::Complex32;
use thiserror::Error;

use super::super::{
    framework::{braket::Ket, wavefunction::signature::Sign1D},
    frontend::wf_1d_vis::{Cache1D, Cache1DError},
};

#[derive(Debug, Error)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum WFComponentError {
    #[error("Unable to create cache.")]
    CacheError(#[from] Cache1DError),
}

/// A component holding information for one wavefunction.
/// There should only be one `WFComponent` per wavefunction - additional
/// instances of this component will carry with them additional caches and
/// cache updates, which could significantly degrade performance.
#[derive(Component, Default, Clone)]
pub(in crate::frontend) struct WFComponent {
    /// A reference to the wavefunction
    ket: Arc<Ket<Sign1D>>,
    /// The wavefunction cache. This may be mutated by bevy systems.
    cache: Cache1D,
    /// The step size at which to render the wavefunction each frame. This
    /// should be lower than `eval_step_size`, as points between wavefunction
    /// samples will be interpolated via Catmull-Rom.
    render_step: f32,
    /// The time scale at which to render the wavefunction. Lower values are
    /// slower.
    pub time_scale: f32,
    /// Whether the wavefunction evolution is paused. This may be mutated by
    /// bevy systems.
    pub paused: bool,
    /// The current time value associated with the wavefunction. This may be
    /// mutated by bevy systems.
    pub time: f32,
}

impl WFComponent {
    /// Create a wavefunction component for a wavefunction
    pub fn new(
        ket: Ket<Sign1D>,
        cache_step_size: f32,
        render_step_size: f32,
        time_scale: f32,
    ) -> Result<Self, WFComponentError> {
        let cache = Cache1D::from_ket(&ket, cache_step_size)?;
        Ok(Self {
            ket: Arc::new(ket),
            cache,
            render_step: render_step_size,
            time_scale,
            paused: false,
            time: 0.0,
        })
    }

    /// Iterate over the wavefunction domain with rendering step size
    pub fn iter_render_points(&self) -> impl Iterator<Item = f32> {
        self.ket.iter_with_step_size(self.render_step)
    }

    /// Update the wavefunction value cache
    pub fn update_cache(&mut self) {
        self.cache.update(&self.ket, self.time);
    }

    /// Get the value at the given point. This is interpolated from sampled
    /// points via Catmull-Rom.
    pub fn cache_at(&self, x: f32) -> Complex32 {
        self.cache.at(x)
    }
}

/// The type of wavefunction visualisation attached to an entity. `Real` and
/// `Imag` each visualise the real and imaginary parts of the wavefunction,
/// while `Full`
#[derive(Component, Default)]
pub(in crate::frontend) enum WFType {
    #[default]
    /// For the full wavefunction, assigning the real and imaginary parts
    /// to perpendicular axes
    Full,
    /// For the real part of the wavefunction
    Real,
    /// For the imaginary part of the wavefunction
    Imag,
    /// For the probability density of the wavefunction
    Density,
}

#[derive(Debug, Error)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum FilledWaveMatError {
    #[error("FilledWave cannot be applied to WFType::Full wavefunctions")]
    AppliedToFullWF,
}

impl WFType {
    /// Get the standard polyline material for a particular `WFType`
    pub fn polyline_mat(&self) -> PolylineMaterial {
        match self {
            WFType::Full => PolylineMaterial {
                width: 50.0,
                color: LinearRgba::rgb(1.0, 2.0, 1.0),
                perspective: true,
                ..Default::default()
            },
            WFType::Real => PolylineMaterial {
                width: 15.0,
                color: LinearRgba::rgb(15.0, 0.0, 0.0),
                perspective: true,
                ..Default::default()
            },
            WFType::Imag => PolylineMaterial {
                width: 15.0,
                color: LinearRgba::rgb(0.0, 0.0, 15.0),
                perspective: true,
                ..Default::default()
            },
            WFType::Density => PolylineMaterial {
                width: 50.0,
                color: LinearRgba::rgb(10.0, 10.0, 10.0),
                perspective: true,
                ..Default::default()
            },
        }
    }

    /// Get the standard `FilledWave` material for a particular `WFType`
    pub fn filled_mat(&self) -> Result<StandardMaterial, FilledWaveMatError> {
        match self {
            WFType::Full => Err(FilledWaveMatError::AppliedToFullWF),
            WFType::Real => Ok(StandardMaterial {
                base_color: Color::srgba(1.0, 0.2, 0.2, 0.7),
                cull_mode: None,
                alpha_mode: bevy::render::alpha::AlphaMode::Add,
                ..Default::default()
            }),
            WFType::Imag => Ok(StandardMaterial {
                base_color: Color::srgba(0.3, 0.3, 1.0, 0.7),
                cull_mode: None,
                alpha_mode: bevy::render::alpha::AlphaMode::Add,
                ..Default::default()
            }),
            WFType::Density => Ok(StandardMaterial {
                base_color: Color::srgba(1.0, 1.0, 1.0, 0.7),
                cull_mode: None,
                alpha_mode: bevy::render::alpha::AlphaMode::Add,
                ..Default::default()
            }),
        }
    }
}

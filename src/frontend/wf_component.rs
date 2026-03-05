use std::sync::Arc;

use bevy::{
    color::{Color, LinearRgba},
    ecs::component::Component,
    pbr::StandardMaterial,
};
use bevy_polyline::prelude::PolylineMaterial;
use thiserror::Error;

use crate::{
    framework::{braket::Ket, wavefunction::signature::WF1D},
    frontend::wf_1d_vis::Cache1D,
};

#[derive(Component, Default, Clone)]
pub(in crate::frontend) struct WFComponent {
    pub wf: Arc<Ket<WF1D>>,
    pub time_scale: f32,
    pub render_step_size: f32,
    pub wf_cache: Cache1D,
}

#[derive(Component, Default)]
pub(in crate::frontend) enum WFType {
    #[default]
    Full,
    Real,
    Imag,
    Density,
}

#[derive(Debug, Error)]
pub enum FilledWaveMatError {
    #[error("FilledWave cannot be applied to WFType::Full wavefunctions")]
    AppliedToFullWF,
}

impl WFType {
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

    pub fn filled_mat(&self) -> Result<StandardMaterial, FilledWaveMatError> {
        match self {
            WFType::Full => Err(FilledWaveMatError::AppliedToFullWF),
            WFType::Real => Ok(StandardMaterial {
                base_color: Color::srgba(1.0, 0.2, 0.2, 0.7),
                cull_mode: None,
                alpha_mode: bevy::render::alpha::AlphaMode::Blend,
                ..Default::default()
            }),
            WFType::Imag => Ok(StandardMaterial {
                base_color: Color::srgba(0.3, 0.3, 1.0, 0.7),
                cull_mode: None,
                alpha_mode: bevy::render::alpha::AlphaMode::Blend,
                ..Default::default()
            }),
            WFType::Density => Ok(StandardMaterial {
                base_color: Color::srgba(1.0, 1.0, 1.0, 0.7),
                cull_mode: None,
                alpha_mode: bevy::render::alpha::AlphaMode::Blend,
                ..Default::default()
            }),
        }
    }
}

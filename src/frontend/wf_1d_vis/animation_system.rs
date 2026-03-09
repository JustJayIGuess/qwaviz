//! Animation logic for wavefunction polylines and fill meshes.

use bevy::{mesh::VertexAttributeValues, prelude::*};
use bevy_polyline::prelude::{Polyline, PolylineHandle};
use thiserror::Error;

use crate::frontend::wf_1d_vis::filled_wave::FilledWave;

use super::super::wf_component::{WFComponent, WFType};

#[derive(Debug, Error)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum FilledWaveMeshError {
    #[error("Unable to get mutable reference to vertex positions")]
    VertexPositions,
    #[error("Unable to get mutable reference to vertex colors")]
    VertexColors,
    #[error("No mesh found in FilledWave")]
    NoMesh,
    #[error("Vertices in FilledWave mesh in invalid format")]
    InvalidVertices,
    #[error("FilledWave cannot be applied to WFType::Full wavefunctions")]
    AppliedToFullWF,
}

#[derive(Error, Debug)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum WFPolylineError {
    #[error("Unable to find polyline using given handle.")]
    MissingPolyline,
}

/// Animate 1D wavefunction polylines and fill
pub fn wf_animation_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut polylines: ResMut<Assets<Polyline>>,
    poly_query: Query<(&PolylineHandle, &WFType)>,
    filled_query: Query<(&FilledWave, &WFType)>,
    wf_component_query: Query<(&WFComponent, &Children)>,
) -> Result<(), BevyError> {
    for (wf, children) in wf_component_query.iter() {
        for child in children {
            if let Ok((PolylineHandle(handle), wf_type)) = poly_query.get(*child) {
                let polyline = polylines
                    .get_mut(handle)
                    .ok_or(WFPolylineError::MissingPolyline)?;
                polyline.vertices = wf
                    .iter_render_points()
                    .map(|x| {
                        let value = wf.cache_at(x);
                        match wf_type {
                            WFType::Full => vec3(x, value.re, value.im),
                            WFType::Real => vec3(x, value.re, 0.0),
                            WFType::Imag => vec3(x, 0.0, value.im),
                            WFType::Density => vec3(x, value.norm_sqr().abs(), 0.0),
                        }
                    })
                    .collect();
            }

            if let Ok((fill, wf_type)) = filled_query.get(*child) {
                let mesh = meshes
                    .get_mut(fill.mesh_handle())
                    .ok_or(FilledWaveMeshError::NoMesh)?;

                let mut positions_opt: Option<&mut Vec<[f32; 3]>> = None;
                let mut colors_opt: Option<&mut Vec<[f32; 4]>> = None;
                for (&attr, value) in mesh.attributes_mut() {
                    match (attr, value) {
                        (Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float32x3(ps)) => {
                            positions_opt = Some(ps);
                        }
                        (Mesh::ATTRIBUTE_COLOR, VertexAttributeValues::Float32x4(cs)) => {
                            colors_opt = Some(cs);
                        }
                        _ => {}
                    }
                }
                let positions = positions_opt.ok_or(FilledWaveMeshError::VertexPositions)?;
                let colors = colors_opt.ok_or(FilledWaveMeshError::VertexColors)?;

                for (pos_chunk, color_chunk) in positions.chunks_mut(2).zip(colors.chunks_mut(2)) {
                    match (pos_chunk, color_chunk) {
                        ([domain_p, val_p], [domain_c, val_c]) => {
                            let x = domain_p[0];
                            let y = match wf_type {
                                WFType::Full => {
                                    return Err(FilledWaveMeshError::AppliedToFullWF.into());
                                }
                                WFType::Real => wf.cache_at(x).re,
                                WFType::Imag => wf.cache_at(x).im,
                                WFType::Density => wf.cache_at(x).norm_sqr().abs(),
                            };
                            val_p[1] = y;
                            *domain_c = [y * fill.intensity(); 4];
                            *val_c = [y * fill.intensity(); 4];
                        }
                        _ => return Err(FilledWaveMeshError::InvalidVertices.into()),
                    }
                }
            }
        }
    }

    Ok(())
}

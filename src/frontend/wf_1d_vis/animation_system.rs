use bevy::{mesh::VertexAttributeValues, prelude::*};
use bevy_polyline::prelude::{Polyline, PolylineHandle};
use num_complex::ComplexFloat;
use thiserror::Error;

use crate::{framework::wavefunction::Wavefunction, frontend::wf_1d_vis::filled_wave::FilledWave};

use super::super::wf_component::{WFComponent, WFType};

#[derive(Debug, Error)]
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
pub enum WFPolylineError {
    #[error("Unable to find polyline using given handle.")]
    MissingPolyline,
}

pub fn wf_animation_system(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut polylines: ResMut<Assets<Polyline>>,
    poly_query: Query<(&PolylineHandle, &WFComponent, &WFType)>,
    filled_query: Query<(&FilledWave, &WFComponent, &WFType)>,
) -> Result<(), BevyError> {
    // Polyline
    for (
        PolylineHandle(handle),
        WFComponent {
            wf,
            time_scale,
            render_step_size,
        },
        wf_type,
    ) in poly_query.iter()
    {
        let t = time_scale * time.elapsed_secs();
        let polyline = polylines
            .get_mut(handle)
            .ok_or(WFPolylineError::MissingPolyline)?;
        polyline.vertices = wf
            .iter_with_step_size(*render_step_size)
            .map(|x| match wf_type {
                WFType::Full => {
                    let value = wf.f(x, t);
                    vec3(x, value.re, value.im)
                }
                WFType::Real => vec3(x, wf.f(x, t).re, 0.0),
                WFType::Imag => vec3(x, 0.0, wf.f(x, t).im),
                WFType::Density => vec3(x, wf.p(x, t).abs(), 0.0),
            })
            .collect();
    }

    // FilledWave
    for (
        FilledWave {
            mesh_handle,
            fill_intensity,
        },
        WFComponent {
            wf,
            time_scale,
            render_step_size: _,
        },
        wf_type,
    ) in filled_query.iter()
    {
        let t = time_scale * time.elapsed_secs();
        let mesh = meshes
            .get_mut(mesh_handle)
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
                        WFType::Full => return Err(FilledWaveMeshError::AppliedToFullWF.into()),
                        WFType::Real => wf.f(x, t).re,
                        WFType::Imag => wf.f(x, t).im,
                        WFType::Density => wf.p(x, t).abs(),
                    };
                    val_p[1] = y;
                    *domain_c = [y * fill_intensity; 4];
                    *val_c = [y * fill_intensity; 4];
                }
                _ => return Err(FilledWaveMeshError::InvalidVertices.into()),
            }
        }
    }

    Ok(())
}

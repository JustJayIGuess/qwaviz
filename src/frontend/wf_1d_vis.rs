//! Frontend functionality for visualising 1D wavefunctions.

mod animation_system;
mod bundle;
mod cache_1d;
mod cache_1d_system;
mod filled_wave;

use std::f32::consts::PI;

pub(in crate::frontend) use animation_system::wf_animation_system;
pub(in crate::frontend) use bundle::{WFFilledWaveBundle, WFPolylineBundle};
pub(in crate::frontend) use cache_1d::Cache1D;
pub(in crate::frontend) use cache_1d_system::update_cache_system;
pub(in crate::frontend) use filled_wave::FilledWave;

use bevy::{
    asset::Assets,
    camera::visibility::Visibility,
    ecs::system::{Commands, ResMut},
    math::Quat,
    mesh::{Mesh, Mesh3d},
    pbr::StandardMaterial,
    transform::components::Transform,
};
use bevy_polyline::prelude::{
    Polyline, PolylineBundle, PolylineHandle, PolylineMaterial, PolylineMaterialHandle,
};

use super::wf_component::{WFComponent, WFType};

/// Spawn a 1D wavefunction visualiser.
/// Spawns two polylines with fill for the real and imaginary parts, a polyline for the full wavefunction,
/// and a polyline for the probability density.
pub fn spawn_wavefunction(
    wf_component: WFComponent,
    transform: Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    standard_materials: &mut ResMut<Assets<StandardMaterial>>,
    polyline_materials: &mut ResMut<Assets<PolylineMaterial>>,
    polylines: &mut ResMut<Assets<Polyline>>,
) {
    let fill_re = FilledWave::from_wf_component(&wf_component, 4.0, meshes);
    let fill_im = FilledWave::from_wf_component(&wf_component, 4.0, meshes);
    let fill_p = FilledWave::from_wf_component(&wf_component, 2.0, meshes);
    commands
        .spawn((wf_component, transform, Visibility::default()))
        .with_children(|parent| {
            parent.spawn(WFPolylineBundle {
                polyline: PolylineBundle {
                    polyline: PolylineHandle(polylines.add(Polyline::default())),
                    material: PolylineMaterialHandle(
                        polyline_materials.add(WFType::Full.polyline_mat()),
                    ),
                    ..Default::default()
                },
                wf_type: WFType::Full,
            });

            parent.spawn(WFPolylineBundle {
                polyline: PolylineBundle {
                    polyline: PolylineHandle(polylines.add(Polyline::default())),
                    material: PolylineMaterialHandle(
                        polyline_materials.add(WFType::Real.polyline_mat()),
                    ),
                    ..Default::default()
                },
                wf_type: WFType::Real,
            });
            parent.spawn(WFFilledWaveBundle {
                mesh: Mesh3d(fill_re.mesh_handle().clone()),
                wave: fill_re,
                material: bevy::pbr::MeshMaterial3d(
                    standard_materials.add(WFType::Real.filled_mat().unwrap()),
                ),
                wf_type: WFType::Real,
                ..Default::default()
            });

            parent.spawn(WFPolylineBundle {
                polyline: PolylineBundle {
                    polyline: PolylineHandle(polylines.add(Polyline::default())),
                    material: PolylineMaterialHandle(
                        polyline_materials.add(WFType::Imag.polyline_mat()),
                    ),
                    ..Default::default()
                },
                wf_type: WFType::Imag,
            });
            parent.spawn(WFFilledWaveBundle {
                mesh: Mesh3d(fill_im.mesh_handle().clone()),
                wave: fill_im,
                material: bevy::pbr::MeshMaterial3d(
                    standard_materials.add(WFType::Imag.filled_mat().unwrap()),
                ),
                wf_type: WFType::Imag,
                transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
                ..Default::default()
            });

            parent.spawn(WFPolylineBundle {
                polyline: PolylineBundle {
                    polyline: PolylineHandle(polylines.add(Polyline::default())),
                    material: PolylineMaterialHandle(
                        polyline_materials.add(WFType::Density.polyline_mat()),
                    ),
                    transform: Transform::from_xyz(0.0, 0.0, -2.0),
                    ..Default::default()
                },
                wf_type: WFType::Density,
            });
            parent.spawn(WFFilledWaveBundle {
                mesh: Mesh3d(fill_p.mesh_handle().clone()),
                wave: fill_p,
                material: bevy::pbr::MeshMaterial3d(
                    standard_materials.add(WFType::Density.filled_mat().unwrap()),
                ),
                // wf_component,
                wf_type: WFType::Density,
                transform: Transform::from_xyz(0.0, 0.0, -2.0),
                ..Default::default()
            });
        });
}

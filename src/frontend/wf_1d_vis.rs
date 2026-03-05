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
    asset::Assets, camera::visibility::Visibility, ecs::system::{Commands, ResMut}, math::Quat, mesh::{Mesh, Mesh3d}, pbr::StandardMaterial, transform::components::Transform
};
use bevy_polyline::prelude::{
    Polyline, PolylineBundle, PolylineHandle, PolylineMaterial, PolylineMaterialHandle,
};

use super::wf_component::{WFComponent, WFType};

pub fn spawn_wavefunction(
    wf_component: WFComponent,
    transform: Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    standard_materials: &mut ResMut<Assets<StandardMaterial>>,
    polyline_materials: &mut ResMut<Assets<PolylineMaterial>>,
    polylines: &mut ResMut<Assets<Polyline>>,
) {
    let real_part_mesh = meshes.add(FilledWave::mesh_from_wf_component(&wf_component));
    let imag_part_mesh = meshes.add(FilledWave::mesh_from_wf_component(&wf_component));
    let density_part_mesh = meshes.add(FilledWave::mesh_from_wf_component(&wf_component));
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
                ..Default::default()
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
                ..Default::default()
            });

            parent.spawn(WFFilledWaveBundle {
                wave: FilledWave {
                    mesh_handle: real_part_mesh.clone(),
                    fill_intensity: 4.0,
                },
                mesh: Mesh3d(real_part_mesh),
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
                ..Default::default()
            });
            parent.spawn(WFFilledWaveBundle {
                wave: FilledWave {
                    mesh_handle: imag_part_mesh.clone(),
                    fill_intensity: 4.0,
                },
                mesh: Mesh3d(imag_part_mesh),
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
                ..Default::default()
            });
            parent.spawn(WFFilledWaveBundle {
                wave: FilledWave {
                    mesh_handle: density_part_mesh.clone(),
                    fill_intensity: 2.0,
                },
                mesh: Mesh3d(density_part_mesh),
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

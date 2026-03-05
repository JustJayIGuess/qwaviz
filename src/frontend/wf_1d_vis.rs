mod animation_system;
mod bundle;
mod filled_wave;

use std::f32::consts::PI;

pub(in crate::frontend) use animation_system::wf_animation_system;
pub(in crate::frontend) use bundle::{WFFilledWaveBundle, WFPolylineBundle};
pub(in crate::frontend) use filled_wave::FilledWave;

use bevy::{
    asset::Assets,
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

pub fn spawn_wavefunction(
    wf_component: WFComponent,
    transform: Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    standard_materials: &mut ResMut<Assets<StandardMaterial>>,
    polyline_materials: &mut ResMut<Assets<PolylineMaterial>>,
    polylines: &mut ResMut<Assets<Polyline>>,
) {
    commands.spawn(WFPolylineBundle {
        polyline: PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline::default())),
            material: PolylineMaterialHandle(polyline_materials.add(WFType::Full.polyline_mat())),
            transform,
            ..Default::default()
        },
        wf_component: wf_component.clone(),
        wf_type: WFType::Full,
        ..Default::default()
    });

    let real_part_mesh = meshes.add(FilledWave::mesh_from_wf_component(&wf_component));
    commands
        .spawn(WFPolylineBundle {
            polyline: PolylineBundle {
                polyline: PolylineHandle(polylines.add(Polyline::default())),
                material: PolylineMaterialHandle(
                    polyline_materials.add(WFType::Real.polyline_mat()),
                ),
                transform,
                ..Default::default()
            },
            wf_component: wf_component.clone(),
            wf_type: WFType::Real,
            ..Default::default()
        })
        .with_child(WFFilledWaveBundle {
            wave: FilledWave {
                mesh_handle: real_part_mesh.clone(),
                fill_intensity: 4.0,
            },
            mesh: Mesh3d(real_part_mesh),
            material: bevy::pbr::MeshMaterial3d(
                standard_materials.add(WFType::Real.filled_mat().unwrap()),
            ),
            wf_component: wf_component.clone(),
            wf_type: WFType::Real,
            ..Default::default()
        });

    let imag_part_mesh = meshes.add(FilledWave::mesh_from_wf_component(&wf_component));
    commands
        .spawn(WFPolylineBundle {
            polyline: PolylineBundle {
                polyline: PolylineHandle(polylines.add(Polyline::default())),
                material: PolylineMaterialHandle(
                    polyline_materials.add(WFType::Imag.polyline_mat()),
                ),
                transform,
                ..Default::default()
            },
            wf_component: wf_component.clone(),
            wf_type: WFType::Imag,
            ..Default::default()
        })
        .with_child(WFFilledWaveBundle {
            wave: FilledWave {
                mesh_handle: imag_part_mesh.clone(),
                fill_intensity: 4.0,
            },
            mesh: Mesh3d(imag_part_mesh),
            material: bevy::pbr::MeshMaterial3d(
                standard_materials.add(WFType::Imag.filled_mat().unwrap()),
            ),
            wf_component: wf_component.clone(),
            wf_type: WFType::Imag,
            transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
            ..Default::default()
        });

    let density_part_mesh = meshes.add(FilledWave::mesh_from_wf_component(&wf_component));
    commands
        .spawn(WFPolylineBundle {
            polyline: PolylineBundle {
                polyline: PolylineHandle(polylines.add(Polyline::default())),
                material: PolylineMaterialHandle(
                    polyline_materials.add(WFType::Density.polyline_mat()),
                ),
                transform: transform * Transform::from_xyz(0.0, 0.0, -2.0),
                ..Default::default()
            },
            wf_component: wf_component.clone(),
            wf_type: WFType::Density,
            ..Default::default()
        })
        .with_child(WFFilledWaveBundle {
            wave: FilledWave {
                mesh_handle: density_part_mesh.clone(),
                fill_intensity: 2.0,
            },
            mesh: Mesh3d(density_part_mesh),
            material: bevy::pbr::MeshMaterial3d(
                standard_materials.add(WFType::Density.filled_mat().unwrap()),
            ),
            wf_component,
            wf_type: WFType::Density,
            ..Default::default()
        });
}

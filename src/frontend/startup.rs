//! Startup functionality

use std::{f32::consts::PI, sync::Arc};

use bevy::{
    asset::AssetServer,
    color::palettes,
    ecs::system::Res,
    light::DirectionalLight,
    math::Quat,
    mesh::Mesh3d,
    prelude::{
        Assets, Color, Commands, Mesh, PointLight, ResMut, StandardMaterial, Transform, Vec3,
    },
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridSettings};
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_polyline::prelude::{
    Polyline, PolylineBundle, PolylineHandle, PolylineMaterial, PolylineMaterialHandle,
};
use num_complex::{Complex32, ComplexFloat};

use super::wf_1d_vis::WFPolylineBundle;
use super::wf_component::{WFComponent, WFType};
use crate::{
    framework::{
        braket::WFKet,
        core::{domain::SubDomain1D, vectorspace::VectorSpace},
        discrete_system::{DiscreteSystem, HarmonicWell, InfiniteSquareWell},
        wavefunction::Wavefunction,
    },
    frontend::wf_1d_vis::{FilledWave, WFFilledWaveBundle},
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    // let hw = HarmonicWell::new(10.0, 1.0, 0.001, 1.0, 3.0);
    // let ket_0 = WFKet::new(
    //     Arc::new(|_, _| Complex32::ONE),
    //     SubDomain1D {
    //         lower: -1.0,
    //         upper: 1.0,
    //         step_size: 0.001,
    //     },
    // )
    // .translate_space(1.5);
    // let ket_1 = Arc::new(hw.evolution(&ket_0, 0.0, 1, 30));

    // let isw = InfiniteSquareWell::new(2.0, 1.0, 2.0, 0.001);
    // let ket = Arc::new((isw.energy_eigenstate(1) + isw.energy_eigenstate(2)).scale(Complex32::new(1.0/2.0.sqrt(), 0.0)));

    let isw = InfiniteSquareWell::new(2.0, 1.0, 2.0, 0.001);
    let ket_0 = isw.expansion_state(1.0, 1);
    let ket_1 = Arc::new(isw.evolution(&ket_0, 0.0, 1, 512));

    let wf_component = WFComponent {
        wf: ket_1,
        time_scale: 0.1,
        render_step_size: 0.01,
    };

    // commands.spawn((SceneBu {
    //     scene: arrow,
    //     ..Default::default()
    // },));

    commands.spawn(WFPolylineBundle {
        polyline: PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline::default())),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 50.0,
                color: palettes::css::GRAY.into(),
                perspective: true,
                ..Default::default()
            })),
            ..Default::default()
        },
        wf_component: wf_component.clone(),
        wf_type: WFType::Full,
        ..Default::default()
    });

    commands.spawn(WFPolylineBundle {
        polyline: PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline::default())),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 15.0,
                color: palettes::css::RED.into(),
                perspective: true,
                ..Default::default()
            })),
            ..Default::default()
        },
        wf_component: wf_component.clone(),
        wf_type: WFType::Real,
        ..Default::default()
    });

    let real_part_mesh = meshes.add(FilledWave::mesh_from_wf_component(&wf_component));
    let real_part_mat = standard_materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.2, 0.2, 0.7),
        cull_mode: None,
        alpha_mode: bevy::render::alpha::AlphaMode::Blend,
        ..Default::default()
    });

    commands.spawn(WFFilledWaveBundle {
        wave: FilledWave {
            mesh: real_part_mesh.clone(),
        },
        mesh: Mesh3d(real_part_mesh),
        material: bevy::pbr::MeshMaterial3d(real_part_mat),
        wf_component: wf_component.clone(),
        wf_type: WFType::Real,
        ..Default::default()
    });

    commands.spawn(WFPolylineBundle {
        polyline: PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline::default())),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 15.0,
                color: palettes::css::BLUE.into(),
                perspective: true,
                ..Default::default()
            })),
            ..Default::default()
        },
        wf_component: wf_component.clone(),
        wf_type: WFType::Imag,
        ..Default::default()
    });

    let imag_part_mesh = meshes.add(FilledWave::mesh_from_wf_component(&wf_component));
    let imag_part_mat = standard_materials.add(StandardMaterial {
        base_color: Color::srgba(0.3, 0.3, 1.0, 0.7),
        cull_mode: None,
        alpha_mode: bevy::render::alpha::AlphaMode::Blend,
        ..Default::default()
    });

    commands.spawn(WFFilledWaveBundle {
        wave: FilledWave {
            mesh: imag_part_mesh.clone(),
        },
        mesh: Mesh3d(imag_part_mesh),
        material: bevy::pbr::MeshMaterial3d(imag_part_mat),
        wf_component: wf_component.clone(),
        wf_type: WFType::Imag,
        transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
        ..Default::default()
    });

    commands.spawn(WFPolylineBundle {
        polyline: PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline::default())),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 50.0,
                color: palettes::css::WHITE.into(),
                perspective: true,
                ..Default::default()
            })),
            transform: Transform::from_xyz(0.0, 0.0, -2.0),
            ..Default::default()
        },
        wf_component: wf_component.clone(),
        wf_type: WFType::Density,
        ..Default::default()
    });

    let density_part_mesh = meshes.add(FilledWave::mesh_from_wf_component(&wf_component));
    let density_part_mat = standard_materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.7),
        cull_mode: None,
        alpha_mode: bevy::render::alpha::AlphaMode::Blend,
        ..Default::default()
    });

    commands.spawn(WFFilledWaveBundle {
        wave: FilledWave {
            mesh: density_part_mesh.clone(),
        },
        mesh: Mesh3d(density_part_mesh),
        material: bevy::pbr::MeshMaterial3d(density_part_mat),
        wf_component: wf_component.clone(),
        wf_type: WFType::Density,
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..Default::default()
    });

    commands.spawn(InfiniteGridBundle {
        settings: InfiniteGridSettings {
            x_axis_color: Color::WHITE,
            ..Default::default()
        },
        ..Default::default()
    });

    // light
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 1.0,
            ..Default::default()
        },
        Transform::from_rotation(Quat::from_rotation_x(PI / 4.0)),
    ));

    // camera
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}

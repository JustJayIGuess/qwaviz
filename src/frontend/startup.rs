//! Startup functionality

use std::{f32::consts::PI, sync::Arc};

use bevy::{
    camera::{Camera, Camera3d},
    core_pipeline::tonemapping::Tonemapping,
    light::DirectionalLight,
    math::Quat,
    post_process::bloom::Bloom,
    prelude::{Assets, Color, Commands, Mesh, ResMut, StandardMaterial, Transform, Vec3},
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridSettings};
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_polyline::prelude::{Polyline, PolylineMaterial};
use num_complex::Complex32;

use super::wf_component::WFComponent;
use crate::{
    framework::{
        braket::WFKet,
        core::domain::SubDomain1D,
        discrete_system::{DiscreteSystem, HarmonicWell},
        wavefunction::Wavefunction,
    },
    frontend::wf_1d_vis::spawn_wavefunction,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    // create wavefunction
    let hw = HarmonicWell::new(10.0, 1.0, 0.001, 1.0, 3.0);
    let ket_0 = WFKet::new(
        Arc::new(|_, _| Complex32::ONE),
        SubDomain1D {
            lower: -1.0,
            upper: 1.0,
            step_size: 0.001,
        },
    )
    .translate_space(1.5);
    let ket_1 = Arc::new(hw.evolution(&ket_0, 0.0, 1, 30));

    // let isw = InfiniteSquareWell::new(2.0, 1.0, 2.0, 0.001);
    // let ket = Arc::new((isw.energy_eigenstate(1) + isw.energy_eigenstate(2)).scale(Complex32::new(1.0/2.0.sqrt(), 0.0)));

    // let isw = InfiniteSquareWell::new(2.0, 1.0, 2.0, 0.001);
    // let ket_0 = isw.expansion_state(1.0, 1);
    // let ket_1 = Arc::new(isw.evolution(&ket_0, 0.0, 1, 512));

    // wavefunction visualiser spec
    let wf_component = WFComponent {
        wf: ket_1,
        time_scale: 0.1,
        render_step_size: 0.01,
    };

    // wavefunction group
    spawn_wavefunction(
        wf_component,
        Transform::IDENTITY,
        &mut commands,
        &mut meshes,
        &mut standard_materials,
        &mut polyline_materials,
        &mut polylines,
    );

    // grid
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
        Camera3d::default(),
        Camera {
            clear_color: bevy::camera::ClearColorConfig::Custom(Color::srgb(0.05, 0.05, 0.05)),
            ..Default::default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera {
            orbit_smoothness: 0.08,
            pan_smoothness: 0.1,
            zoom_smoothness: 0.2,
            ..Default::default()
        },
    ));

    // action!
}

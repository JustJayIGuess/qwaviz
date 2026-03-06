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

use super::wf_component::WFComponent;
use crate::{
    framework::{braket::Ket, wavefunction::signature::WF1D},
    frontend::wf_1d_vis::{Cache1D, spawn_wavefunction},
};

pub fn get_setup(
    ket: Ket<WF1D>,
) -> impl FnMut(
    Commands,
    ResMut<Assets<Mesh>>,
    ResMut<Assets<StandardMaterial>>,
    ResMut<Assets<PolylineMaterial>>,
    ResMut<Assets<Polyline>>,
) {
    let ket_arc = Arc::new(ket);
    move |mut commands: Commands,
          mut meshes: ResMut<Assets<Mesh>>,
          mut standard_materials: ResMut<Assets<StandardMaterial>>,
          mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
          mut polylines: ResMut<Assets<Polyline>>| {
        // wavefunction visualiser spec
        let wf_component = WFComponent {
            wf_cache: Cache1D::from_ket(&ket_arc, 0.02).unwrap(),
            wf: ket_arc.clone(),
            time_scale: 0.1,
            eval_step_size: 0.05,
            render_step_size: 0.01,
            paused: false,
            time: 0.0,
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
}

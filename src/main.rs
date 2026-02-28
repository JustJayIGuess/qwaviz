//! A test program decomposing a quantum state into an eigenbasis.
#![deny(missing_docs)]

use std::sync::Arc;

use bevy::prelude::*;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_polyline::prelude::*;
use num_complex::ComplexFloat;

use crate::{
    braket::{WFKet, Wavefunction},
    domains::SubDomain,
    infinite_square_well::InfiniteSquareWell,
    signatures::WF1Space1Time,
};

pub mod braket;
pub mod domains;
pub mod fields;
pub mod infinite_square_well;
pub mod signatures;
pub mod vectorspaces;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PolylinePlugin)
        .add_plugins(InfiniteGridPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                wf_animation_system,
                rotator_system,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _standard_materials: ResMut<Assets<StandardMaterial>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let isw = InfiniteSquareWell::new(1.0, 1.0, 1.0, 1.0 / 1000.0);
    let ket_0 = isw.expansion_state(0.8, 1);
    let ket_1 = Arc::new(isw.evolution(&ket_0, 0.0, 128));

    commands.spawn((
        PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline {
                ..Default::default()
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 50.0,
                color: bevy::color::palettes::css::GRAY.into(),
                perspective: true,
                depth_bias: -0.0002,
            })),
            transform: Transform::from_xyz(-2.5, 0.0, 0.0).with_scale(vec3(5.0, 0.5, 0.5)),
            ..Default::default()
        },
        AnimateVertices,
        Wavefunction1D {
            wf: ket_1.clone(),
            time_scale: 0.2,
        },
        WavefunctionType::Full,
    ));

    commands.spawn((
        PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline {
                ..Default::default()
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 15.0,
                color: bevy::color::palettes::css::RED.into(),
                perspective: true,
                depth_bias: -0.0002,
            })),
            transform: Transform::from_xyz(-2.5, 0.0, 0.0).with_scale(vec3(5.0, 0.5, 0.5)),
            ..Default::default()
        },
        AnimateVertices,
        Wavefunction1D {
            wf: ket_1.clone(),
            time_scale: 0.2,
        },
        WavefunctionType::Real,
    ));

    commands.spawn((
        PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline {
                ..Default::default()
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 15.0,
                color: bevy::color::palettes::css::BLUE.into(),
                perspective: true,
                depth_bias: -0.0002,
            })),
            transform: Transform::from_xyz(-2.5, 0.0, 0.0).with_scale(vec3(5.0, 0.5, 0.5)),
            ..Default::default()
        },
        AnimateVertices,
        Wavefunction1D {
            wf: ket_1.clone(),
            time_scale: 0.2,
        },
        WavefunctionType::Imag,
    ));



    commands.spawn((
        PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline {
                ..Default::default()
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 15.0,
                color: bevy::color::palettes::css::WHITE.into(),
                perspective: true,
                depth_bias: -0.0002,
            })),
            transform: Transform::from_xyz(-2.5, 0.0, -2.5).with_scale(vec3(5.0, 0.5, 0.5)),
            ..Default::default()
        },
        AnimateVertices,
        Wavefunction1D {
            wf: ket_1,
            time_scale: 0.2,
        },
        WavefunctionType::Density,
    ));

    commands.spawn(InfiniteGridBundle {
        settings: InfiniteGridSettings {
            x_axis_color: Color::WHITE,
            ..Default::default()
        },
        ..Default::default()
    });

    // light
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));

    // camera
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}

/// this component indicates what entities should rotate
#[derive(Component)]
struct Rotates;

#[derive(Component)]
struct AnimateVertices;

#[derive(Component)]
struct Wavefunction1D {
    wf: Arc<WFKet<WF1Space1Time>>,
    time_scale: f32,
}

#[derive(Component)]
enum WavefunctionType {
    Full,
    Real,
    Imag,
    Density,
}

fn wf_animation_system(
    time: Res<Time>,
    mut polylines: ResMut<Assets<Polyline>>,
    query: Query<(&PolylineHandle, &Wavefunction1D, &WavefunctionType)>,
) {
    for (handle, Wavefunction1D { wf, time_scale }, wf_type) in query.iter() {
        polylines.get_mut(&handle.0).unwrap().vertices = wf
            .subdomain
            .iter()
            .map(|x| {
                let t = time_scale * time.elapsed_secs();
                match wf_type {
                    WavefunctionType::Full => {
                        let value = wf.f(x, t);
                        vec3(x, value.re, value.im)
                    },
                    WavefunctionType::Real => vec3(x, wf.f(x, t).re, 0.0),
                    WavefunctionType::Imag => vec3(x, 0.0, wf.f(x, t).im),
                    WavefunctionType::Density => vec3(x, wf.p(x, t).abs(), 0.0),
                }
            })
            .collect();
    }
}

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_secs(),
        )) * *transform;
    }
}

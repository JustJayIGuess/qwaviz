//! Startup functionality

use std::sync::Arc;

use bevy::{
    color::palettes,
    prelude::{
        Assets, Color, Commands, Mesh, PointLight, ResMut, StandardMaterial, Transform, Vec3,
    },
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridSettings};
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_polyline::prelude::{
    Polyline, PolylineBundle, PolylineHandle, PolylineMaterial, PolylineMaterialHandle,
};
use num_complex::Complex32;

use super::wf_polyline::{WFComponent, WFPolylineBundle, WFType};
use crate::framework::{
    braket::{WFKet, WFOperation},
    core::domain::DomainSection1D,
    potential::{ConfinedPotential, HarmonicWell},
    wavefunction::Wavefunction,
};

pub fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _standard_materials: ResMut<Assets<StandardMaterial>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let hw = HarmonicWell::new(10.0, 1.0, 0.001, 1.0, 3.0);
    let ket_0 = WFKet {
        wavefunction: WFOperation::func(Arc::new(|_, _| Complex32::ONE)),
        subdomain: DomainSection1D {
            lower: -1.0,
            upper: 1.0,
            step_size: 0.001,
        },
    }
    .translate_space(1.5);
    let ket_1 = Arc::new(hw.evolution(&ket_0, 0.0, 30));
    println!("Computation done. Domain: {:?}", ket_1.subdomain);

    let wf_component = WFComponent {
        wf: ket_1,
        time_scale: 0.1,
        render_step_size: 0.01,
    };

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
        wf_component,
        wf_type: WFType::Density,
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
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));

    // camera
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}

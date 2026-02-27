//! A test program decomposing a quantum state into an eigenbasis.

#![deny(missing_docs)]

use std::{f32::consts::PI, sync::Arc, time::Instant};

use num_complex::Complex32;
use three_d::*;

use crate::{
    braket::{Ket, WFKet, WFOperation, Wavefunction},
    signatures::{WF1Space1Time, WFSignature},
    vectorspaces::VectorSpace,
};

pub mod braket;
pub mod domains;
pub mod fields;
pub mod signatures;
pub mod vectorspaces;

type Ket1D = WFKet<WF1Space1Time>;
type SubDom = <WF1Space1Time as WFSignature>::SubDom;

fn get_isw_eigenstate(width: f32, mass: f32, hbar: f32, n: usize) -> Ket1D {
    Ket1D {
        wavefunction: WFOperation::Function(Arc::new(move |x, t| {
            let energy = (n as f32 * PI * hbar / width).powi(2) / (2.0 * mass);
            let coef = (2.0 / width).sqrt();
            let phase_x: f32 = (n as f32) * PI * x / width;
            coef * phase_x.sin() * Complex32::cis(-energy * t / hbar)
        })),
        subdomain: SubDom {
            lower: 0.0,
            upper: width,
            step_size: width / 1000.0,
        },
    }
}

fn get_expansion_state(width: f32) -> Ket1D {
    let psi0 = get_isw_eigenstate(width / 2.0, 1.0, 1.0, 1);
    Ket1D {
        wavefunction: WFOperation::Function(Arc::new(move |x, t| psi0.f(x, t))),
        subdomain: SubDom {
            lower: 0.0,
            upper: width,
            step_size: width / 1000.0,
        },
    }
}

fn main() {
    // let window = Window::new(WindowSettings {
    //     title: "qwaviz".into(),
    //     max_size: Some((1280, 720)),
    //     ..Default::default()
    // })
    // .unwrap();
    // let context = window.gl();
    // let mut camera = Camera::new_perspective(
    //     window.viewport(),
    //     vec3(5.0, 5.0, 5.0),
    //     vec3(0.0, 0.0, 0.0),
    //     vec3(0.0, 1.0, 0.0),
    //     degrees(45.0),
    //     0.1,
    //     1000.0,
    // );
    // let mut control = OrbitControl::new(camera.target(), 1.0, 100.0);
    // let mut sphere = Gm::new(
    //     Mesh::new(&context, &CpuMesh::cylinder(8)),
    //     PhysicalMaterial::new_opaque(
    //         &context,
    //         &CpuMaterial {
    //             albedo: Srgba {
    //                 r: 255,
    //                 g: 100,
    //                 b: 50,
    //                 a: 255,
    //             },
    //             ..Default::default()
    //         },
    //     ),
    // );
    // let mut sphere2 = Gm::new(
    //     Mesh::new(&context, &CpuMesh::sphere(16)),
    //     PhysicalMaterial::new_opaque(
    //         &context,
    //         &CpuMaterial {
    //             albedo: Srgba {
    //                 r: 128,
    //                 g: 255,
    //                 b: 51,
    //                 a: 255,
    //             },
    //             ..Default::default()
    //         },
    //     ),
    // );

    // sphere.set_transformation(Mat4::from_translation(vec3(0.1, 0.0, 0.0)) * Mat4::from_scale(0.5));
    // sphere2.set_transformation(Mat4::from_translation(vec3(0.0, 1.5, 0.0)) * Mat4::from_scale(0.7));

    // let light = DirectionalLight::new(&context, 1.0, Srgba::RED, vec3(0.5, -1.0, 0.0));
    // let light2 = DirectionalLight::new(&context, 0.5, Srgba::BLUE, vec3(-1.0, -0.5, 0.0));

    // window.render_loop(move |mut frame_input| {
    //     camera.set_viewport(frame_input.viewport);
    //     control.handle_events(&mut camera, &mut frame_input.events);

    //     frame_input
    //         .screen()
    //         .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
    //         .render(
    //             &camera,
    //             sphere.into_iter().chain(&sphere2),
    //             &[&light, &light2],
    //         );

    //     FrameOutput::default()
    // });

    const MAX_N: usize = 128;

    let ket_0 = get_expansion_state(1.0);
    let eigenkets: [Ket1D; MAX_N] =
        std::array::from_fn(|i| get_isw_eigenstate(1.0, 1.0, 1.0, i + 1));

    let time_find_coefs = Instant::now();
    let coefs: Vec<Complex32> = eigenkets
        .iter()
        .map(|ket| ket.clone().adjoint() * ket_0.clone())
        .enumerate()
        .inspect(|(i, c)| println!("C_{}: {}", i + 1, c))
        .map(|(_, c)| c)
        .collect();
    println!("\nFind coefs: {:?}\n", time_find_coefs.elapsed());

    let time_construct_eigenket = Instant::now();
    let eigenbasis_ket = eigenkets
        .iter()
        .zip(coefs)
        .map(|(ket, c)| ket.clone().scale(c))
        .reduce(|a, b| a + b)
        .unwrap();
    println!(
        "Construct from eigenkets: {:?}\n",
        time_construct_eigenket.elapsed()
    );

    let time_recompute_coefs = Instant::now();
    eigenkets.iter().enumerate().for_each(|(i, ket)| {
        println!(
            "C_{}: {}",
            i + 1,
            ket.clone().adjoint() * eigenbasis_ket.clone()
        )
    });
    println!("\nRecompute coefs: {:?}\n", time_recompute_coefs.elapsed());
}

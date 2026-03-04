use bevy::{mesh::VertexAttributeValues, prelude::*};
use bevy_polyline::prelude::{Polyline, PolylineHandle};
use num_complex::ComplexFloat;

use crate::{framework::wavefunction::Wavefunction, frontend::wf_1d_vis::filled_wave::FilledWave};

use super::super::wf_component::{WFComponent, WFType};

pub fn wf_animation_system(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut polylines: ResMut<Assets<Polyline>>,
    poly_query: Query<(&PolylineHandle, &WFComponent, &WFType)>,
    filled_query: Query<(&FilledWave, &WFComponent, &WFType)>,
) {
    for (
        handle,
        WFComponent {
            wf,
            time_scale,
            render_step_size,
        },
        wf_type,
    ) in poly_query.iter()
    {
        let t = time_scale * time.elapsed_secs();
        polylines.get_mut(&handle.0).unwrap().vertices = wf
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

    for (
        FilledWave { mesh: mesh_handle },
        WFComponent {
            wf,
            time_scale,
            render_step_size: _,
        },
        wf_type,
    ) in filled_query.iter()
    {
        let t = time_scale * time.elapsed_secs();
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            let positions = match mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                Some(VertexAttributeValues::Float32x3(positions)) => positions,
                _ => {
                    panic!(
                        "FilledWave Mesh Error: Unable to get mutable reference to mesh positions."
                    )
                }
            };
            positions.chunks_mut(2).for_each(|chunk| {
                match chunk {
                    [domain, val] => {
                        let x = domain[0];
                        val[1] = match wf_type {
                            WFType::Full => 0.0, // Don't do this
                            WFType::Real => wf.f(x, t).re,
                            WFType::Imag => wf.f(x, t).im,
                            WFType::Density => wf.p(x, t).abs(),
                        }
                    }
                    [_] => panic!("FilledWave Mesh Error: Odd Number of Vertices!"),
                    _ => unreachable!(),
                }
            });
        }
    }
}

// pub fn update_mesh(mut meshes: ResMut<Assets<Mesh>>, mut query: Query<(&mut FilledWave, &WFComponent, &WFType)>) {
//     if self.mesh.is_none() {
//         self.mesh = Some(Mesh::new(
//             PrimitiveTopology::TriangleStrip,
//             Default::default(),
//         ));
//     }
//     let mesh = self.mesh.as_mut().unwrap();
//     let positions = match mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
//         Some(VertexAttributeValues::Float32x3(positions)) => positions,
//         _ => {
//             panic!("FilledWave Mesh Error: Unable to get mutable reference to mesh positions.")
//         }
//     };

//     if points.len() == positions.len() {
//         for (p, chunk) in points.iter().zip(positions.chunks_mut(2)) {
//             match chunk {
//                 [domain, val] => {
//                     domain[0] = p[0];
//                     val[0] = p[0];
//                     val[1] = p[1];
//                 }
//                 [_] => panic!("FilledWave Mesh Error: Odd Number of Vertices!"),
//                 _ => unreachable!(),
//             }
//         }
//     } else {
//         let mut verts: Vec<[f32; 3]> = Vec::with_capacity(points.len() * 2);

//         for [x, y] in points {
//             verts.push([*x, 0.0, 0.0]);
//             verts.push([*x, *y, 0.0]);
//         }
//         *positions = verts;
//         mesh.compute_normals();
//     }
// }

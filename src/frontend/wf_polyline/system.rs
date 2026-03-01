use bevy::prelude::*;
use bevy_polyline::prelude::{Polyline, PolylineHandle};
use num_complex::ComplexFloat;

use crate::framework::{core::domain::SubDomain, wavefunction::Wavefunction};

use super::{WFComponent, WFType};

pub fn wf_animation_system(
    time: Res<Time>,
    mut polylines: ResMut<Assets<Polyline>>,
    query: Query<(&PolylineHandle, &WFComponent, &WFType)>,
) {
    for (
        handle,
        WFComponent {
            wf,
            time_scale,
            render_step_size,
        },
        wf_type,
    ) in query.iter()
    {
        polylines.get_mut(&handle.0).unwrap().vertices = wf
            .subdomain
            .clone()
            .with_step_size(*render_step_size)
            .iter()
            .map(|x| {
                let t = time_scale * time.elapsed_secs();
                match wf_type {
                    WFType::Full => {
                        let value = wf.f(x, t);
                        vec3(x, value.re, value.im)
                    }
                    WFType::Real => vec3(x, wf.f(x, t).re, 0.0),
                    WFType::Imag => vec3(x, 0.0, wf.f(x, t).im),
                    WFType::Density => vec3(x, wf.p(x, t).abs(), 0.0),
                }
            })
            .collect();
    }
}

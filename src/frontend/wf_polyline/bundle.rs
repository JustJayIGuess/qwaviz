use std::sync::Arc;

use bevy::prelude::*;
use bevy_polyline::prelude::PolylineBundle;

use crate::framework::{braket::WFKet, wavefunction::signature::WF1Space1Time};

#[derive(Component, Default)]
pub(in crate::frontend) struct AnimateVertices;

#[derive(Component, Default, Clone)]
pub(in crate::frontend) struct WFComponent {
    pub wf: Arc<WFKet<WF1Space1Time>>,
    pub time_scale: f32,
    pub render_step_size: f32,
}

#[derive(Component, Default)]
pub(in crate::frontend) enum WFType {
    #[default]
    Full,
    Real,
    Imag,
    Density,
}

/// A polyline that visualises a wavefunction
#[derive(Bundle, Default)]
pub(in crate::frontend) struct WFPolylineBundle {
    pub polyline: PolylineBundle,
    pub wf_component: WFComponent,
    pub wf_type: WFType,
    pub animate_marker: AnimateVertices,
}

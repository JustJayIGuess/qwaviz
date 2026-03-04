use bevy::prelude::*;
use bevy_polyline::prelude::PolylineBundle;

use crate::frontend::wf_1d_vis::filled_wave::FilledWave;

use super::super::wf_component::{WFComponent, WFType};

#[derive(Component, Default)]
pub(in crate::frontend) struct AnimateVertices;

/// A polyline that visualises a wavefunction
#[derive(Bundle, Default)]
pub(in crate::frontend) struct WFPolylineBundle {
    pub polyline: PolylineBundle,
    pub wf_component: WFComponent,
    pub wf_type: WFType,
    pub animate_marker: AnimateVertices,
}

#[derive(Bundle, Default)]
pub struct WFFilledWaveBundle {
    pub wave: FilledWave,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub wf_component: WFComponent,
    pub wf_type: WFType,
    pub transform: Transform,
    pub visibility: Visibility,
}

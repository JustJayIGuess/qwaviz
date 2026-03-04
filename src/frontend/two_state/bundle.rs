use bevy::ecs::{bundle::Bundle, component::Component};

use super::super::wf_component::{WFComponent, WFType};

#[derive(Component, Default)]
pub(in crate::frontend) struct AnimateVertices;

// #[derive(Componenet)]

/// A polyline that visualises a wavefunction
#[derive(Bundle, Default)]
pub(in crate::frontend) struct WFPolylineBundle {
    pub arrow: (),
    pub wf_component: WFComponent,
    pub wf_type: WFType,
    pub animate_marker: AnimateVertices,
}

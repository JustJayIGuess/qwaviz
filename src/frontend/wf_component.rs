use std::sync::Arc;

use bevy::ecs::component::Component;

use crate::framework::{braket::WFKet, wavefunction::signature::WF1D};

#[derive(Component, Default, Clone)]
pub(in crate::frontend) struct WFComponent {
    pub wf: Arc<WFKet<WF1D>>,
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

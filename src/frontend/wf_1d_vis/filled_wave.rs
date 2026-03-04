use bevy::{asset::Handle, ecs::component::Component, mesh::Mesh};

use crate::frontend::wf_component::WFComponent;

#[derive(Component, Default)]
pub struct FilledWave {
    pub mesh: Handle<Mesh>,
}

impl FilledWave {
    pub fn mesh_from_wf_component(wf_component: &WFComponent) -> Mesh {
        let positions: Vec<_> = wf_component
            .wf
            .iter_with_step_size(wf_component.render_step_size)
            .flat_map(|x| [[x, 0.0, 0.0], [x, 0.0, 0.0]])
            .collect();
        Mesh::new(
            bevy::mesh::PrimitiveTopology::TriangleStrip,
            Default::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    }
}

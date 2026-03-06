//! A realtime updated mesh that fills between planar polylines and axes. Not
//! applicable to `WFType::Full`. Mesh is transparent and vertex colours are
//! updated to reflect the magnitude of the attached wavefunction.

use bevy::{
    asset::{Assets, Handle, RenderAssetUsages},
    ecs::{component::Component, system::ResMut},
    mesh::Mesh,
};

use crate::frontend::wf_component::WFComponent;

/// A realtime updated mesh to fill in planar polylines.
/// Vertex colouring is also updated to match the value of the attached wavefunction.
#[derive(Component, Default)]
pub struct FilledWave {
    /// A handle to the mesh to update realtime
    mesh_handle: Handle<Mesh>,
    /// The intensity of the fill colour
    fill_intensity: f32,
}

impl FilledWave {
    /// Get a valid mesh 
    pub fn from_wf_component(wf_component: &WFComponent, fill_intensity: f32, meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let positions: Vec<_> = wf_component
            .wf
            .iter_with_step_size(wf_component.eval_step_size)
            .flat_map(|x| [[x, 0.0, 0.0], [x, 0.0, 0.0]])
            .collect();
        let colors = vec![[1.0, 1.0, 1.0, 1.0]; positions.len()];
        let mesh = Mesh::new(
            bevy::mesh::PrimitiveTopology::TriangleStrip,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        Self {
            mesh_handle: meshes.add(mesh),
            fill_intensity,
        }
    }

    /// Get a handle to the mesh.
    pub fn mesh_handle(&self) -> &Handle<Mesh> {
        &self.mesh_handle
    }

    /// Get the intensity of the mesh.
    pub fn intensity(&self) -> f32 {
        self.fill_intensity
    }
}

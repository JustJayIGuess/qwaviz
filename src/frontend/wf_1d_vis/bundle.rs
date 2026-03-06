//! Bevy `Bundles` associated with visualising 1D wavefunctions.

use bevy::prelude::*;
use bevy_polyline::prelude::PolylineBundle;

use crate::frontend::wf_1d_vis::filled_wave::FilledWave;

use super::super::wf_component::WFType;

/// A polyline that visualises a wavefunction
#[derive(Bundle, Default)]
pub(in crate::frontend) struct WFPolylineBundle {
    /// The polyline component
    pub polyline: PolylineBundle,
    /// The type of wavefunction. All types are valid.
    pub wf_type: WFType,
}

/// A realtime updated mesh that goes with a `WFPolylineBundle`
#[derive(Bundle, Default)]
pub struct WFFilledWaveBundle {
    /// The filled wave mesh component
    pub wave: FilledWave,
    /// Handle to the mesh
    pub mesh: Mesh3d,
    /// Material of the `FilledWave`
    pub material: MeshMaterial3d<StandardMaterial>,
    /// The type of wavefunction. Cannot be `WFType::Full`
    pub wf_type: WFType,
    /// The transform of the bundle
    pub transform: Transform,
    /// The visibility of the bundle
    pub visibility: Visibility,
}

use bevy::prelude::{App, DefaultPlugins, Startup, Update};
use bevy_infinite_grid::InfiniteGridPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_polyline::PolylinePlugin;

use super::{rotator::rotator_system, startup::setup, wf_polyline::wf_animation_system};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PolylinePlugin)
        .add_plugins(InfiniteGridPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (wf_animation_system, rotator_system))
        .run();
}

use bevy::{
    app::PreUpdate,
    color::Color,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::{App, DefaultPlugins, Startup, Update},
    text::TextFont,
};
use bevy_infinite_grid::InfiniteGridPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_polyline::PolylinePlugin;

use crate::frontend::wf_1d_vis::update_cache_system;

use super::{startup::setup, wf_1d_vis::wf_animation_system};

/// Run the application.
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PolylinePlugin)
        .add_plugins(InfiniteGridPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 20.0,
                    ..Default::default()
                },
                text_color: Color::WHITE,
                refresh_interval: core::time::Duration::from_millis(500),
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .add_systems(Startup, (setup,))
        .add_systems(PreUpdate, (update_cache_system,))
        .add_systems(Update, (wf_animation_system,))
        .run();
}

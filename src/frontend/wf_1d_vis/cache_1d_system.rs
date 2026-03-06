use bevy::{
    ecs::system::{Query, Res},
    input::{ButtonInput, keyboard::KeyCode},
    time::Time,
};

use crate::frontend::wf_component::WFComponent;

pub fn update_cache_system(
    time: Res<Time>,
    mut query: Query<&mut WFComponent>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    for mut wf_component in &mut query {
        if key_input.just_pressed(KeyCode::Space) {
            wf_component.paused = !wf_component.paused;
        }
        if !wf_component.paused {
            wf_component.time += wf_component.time_scale * time.delta_secs();
        }

        let t = wf_component.time;
        let f = wf_component.wf.clone();
        wf_component.wf_cache.update(&f, t);
    }
}

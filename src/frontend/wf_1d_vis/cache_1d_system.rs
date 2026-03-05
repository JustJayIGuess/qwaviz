use bevy::{
    ecs::system::{Query, Res},
    time::Time,
};

use crate::frontend::wf_component::WFComponent;

pub fn update_cache_system(time: Res<Time>, mut query: Query<&mut WFComponent>) {
    for mut wf_component in &mut query {
        let f = wf_component.wf.clone();
        let t = wf_component.time_scale * time.elapsed_secs();
        wf_component.wf_cache.update(&f, t);
    }
}

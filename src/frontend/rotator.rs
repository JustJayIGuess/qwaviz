use bevy::prelude::*;

/// this component indicates what entities should rotate
#[derive(Component)]
pub struct Rotates;

pub fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_secs(),
        )) * *transform;
    }
}

use bevy::prelude::*;

use crate::client::graphics::components::LocalPlayer;

pub fn camera(
    player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let player_transform = match player_query.get_single() {
        Ok(player_transform) => player_transform,
        _ => return,
    };
    let mut camera_transform = camera_query.single_mut();
    let position = player_transform.translation;
    camera_transform.translation = Vec3::new(position.x - 5.0, 15.0, position.z);
}

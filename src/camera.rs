use bevy::prelude::*;
use crate::player;

#[derive(Component)]
pub struct CameraMaker;

pub fn update_camera(
    mut query: Query<&Transform, With<player::Player>>,
    mut cameras: Query<&mut Transform, (With<CameraMaker>, Without<player::Player>)>
) {
    if let Ok(player_transform) = query.get_single_mut() {
        for mut camera in &mut cameras {
            camera.translation.x = player_transform.translation.x;
            camera.translation.y = player_transform.translation.y;
        }
    }

}
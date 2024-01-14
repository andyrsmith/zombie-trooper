use bevy::prelude::*;
use crate::movement;
use crate::player;

pub const BULLET_RADIUS: f32 = 1.;

#[derive(Component)]
pub struct Bullet;

pub fn move_bullet(mut query: Query<(&mut Transform, &mut movement::Movement, &mut player::Distance), With<Bullet>>) {
    for (mut transform, movement, mut distance_query) in &mut query {
        distance_query.distance_travel += 1;
        if movement.last_movement == movement::Direction::DOWN {
            transform.translation.y -= 1.0;
        } else if movement.last_movement == movement::Direction::LEFT {
            transform.translation.x -= 1.0;
        } else if movement.last_movement == movement::Direction::UP {
            transform.translation.y += 1.0;
        } else {
            transform.translation.x += 1.0;
        }
    }
}

pub fn despawn_bullet(
    mut commands: Commands,
    bullet_query: Query<(Entity, &player::Distance), With<Bullet>>
) {
    for(bullet_entity, bullet_distance) in &bullet_query {
        if(bullet_distance.distance_travel > bullet_distance.distance_despawn) {
            commands.entity(bullet_entity).despawn();
        }
    }

}
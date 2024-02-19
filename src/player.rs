use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::f32::consts::PI;
use crate::movement;
use crate::bullet;

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Distance {
    pub distance_travel: i32,
    pub distance_despawn: i32
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>, 
    mut query: Query<(&mut Transform, &mut movement::Movement), With<Player>>
) {
    if let Ok((mut player_transform, mut player_movement)) = query.get_single_mut() {
        let mut xdirection = 0.0;
        let mut ydirection = 0.0;
        let mut rotation = 0.0;
        
        if keyboard_input.pressed(KeyCode::A) {
            xdirection -= 1.0;
            player_movement.last_movement = movement::Direction::LEFT;
            rotation = 0.0;
            player_transform.rotation = Quat::from_rotation_z(rotation);
        }
    
        if keyboard_input.pressed(KeyCode::D) {
            xdirection += 1.0;
            player_movement.last_movement = movement::Direction::RIGHT;
            rotation = PI;
            player_transform.rotation = Quat::from_rotation_z(rotation);
        }
    
        if keyboard_input.pressed(KeyCode::S) {
            ydirection -= 1.0;
            player_movement.last_movement = movement::Direction::DOWN;
            rotation = PI / 2.;
            player_transform.rotation = Quat::from_rotation_z(rotation);
        }
    
        if keyboard_input.pressed(KeyCode::W) {
            ydirection += 1.0;
            player_movement.last_movement = movement::Direction::UP;
            rotation = (3. * PI) / 2.;
            player_transform.rotation = Quat::from_rotation_z(rotation);
        }
        player_transform.translation.y = player_transform.translation.y + ydirection;
        player_transform.translation.x = player_transform.translation.x + xdirection;
    }
}


pub fn player_shoot(mut commands: Commands, 
    keyboard_input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Transform, &mut movement::Movement), With<Player>>) {
    if let Ok((player_transform, player_movement)) = query.get_single_mut() {
        if keyboard_input.just_released(KeyCode::Space) {
            commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(bullet::BULLET_RADIUS).into()).into(),
                material: material.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(player_transform.translation),
                ..default()
            },
            bullet::Bullet,
            movement::Movement {
                last_movement: player_movement.last_movement,
            },
            Distance {
                distance_travel: 0,
                distance_despawn: 100
            }
        ));
        }
    }

}
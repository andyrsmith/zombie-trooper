use bevy::prelude::*;
use std::f32::consts::PI;
use rand::Rng;
use crate::ImageCache;
use crate::ZombieCounter;
use crate::ZombieScore;
use crate::ZombieWave;
use crate::player;
use crate::bullet;
use crate::GameState;

const ZOMBIE_RADIUS: f32 = 15.;

#[derive(Component)]
pub struct Zombie;

pub fn move_zombies(mut player_query: Query<&Transform, (With<player::Player>, Without<Zombie>)>, mut zombie_query: Query<&mut Transform, (With<Zombie>, Without<player::Player>)>) {

    if let Ok(player_transform) = player_query.get_single_mut() {
        for mut transform in &mut zombie_query {
            const ZOMBIE_SPEED: f32 = 0.5;
            if player_transform.translation.x > transform.translation.x {
                transform.translation.x += ZOMBIE_SPEED;
            } else {
                transform.translation.x -= ZOMBIE_SPEED;

            }
    
            if player_transform.translation.y > transform.translation.y {
                transform.translation.y += ZOMBIE_SPEED;


            } else {
                transform.translation.y -= ZOMBIE_SPEED;
            }

            let diff_x = transform.translation.x - player_transform.translation.x;
            let diff_y = transform.translation.y - player_transform.translation.y;
            let mut zombie_rotation = 0.0;
            if diff_x.abs() > diff_y.abs() {
                if diff_x > 0.0 {
                    zombie_rotation = PI;
                } else {
                    zombie_rotation = 0.0;
                }
            } else {
                if diff_y > 0.0 {
                    zombie_rotation = (3. * PI) / 2.;
                } else {
                    zombie_rotation = PI / 2.;
                }
            }

            transform.rotation = Quat::from_rotation_z(zombie_rotation);
        }
          
    }
}

pub fn zombie_player_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<player::Player>>,
    zombie_query: Query<&Transform, With<Zombie>>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for transform in &zombie_query {
            let distance = player_transform.translation.distance(transform.translation);
            let player_radius = 30. / 2.0;
            if distance < player_radius + ZOMBIE_RADIUS {
                println!("Game Over");
                commands.entity(player_entity).despawn();
                app_state.set(GameState::GameOver);
            }
        }
    }
}

pub fn zombie_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<bullet::Bullet>>,
    zombie_query: Query<(Entity, &Transform), With<Zombie>>,
    mut zombie_score_text: Query<&mut Text, With<ZombieCounter>>,
    mut zombie_score: ResMut<ZombieScore>
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        for (zombie_entity, zombie_transform) in &zombie_query {
            let distance = bullet_transform.translation.distance(zombie_transform.translation);
            if distance < ZOMBIE_RADIUS + bullet::BULLET_RADIUS {
                commands.entity(bullet_entity).despawn();
                commands.entity(zombie_entity).despawn();
                zombie_score.0 += 1;

                if let Ok(mut text) = zombie_score_text.get_single_mut() {
                    text.sections[0].value = format!("Zombies: {0}", zombie_score.0);
                }
            }

        }
    }

}

pub fn next_zombie_wave(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<player::Player>>,
    zombie_query: Query<Entity, With<Zombie>>,
    image_cache: Res<ImageCache>,
    mut zombie_wave: ResMut<ZombieWave>
) {

    let no_zombies = zombie_query.is_empty();

    if let Ok(player_transform) = player_query.get_single_mut() {
        let player_x = player_transform.translation.x;
        let player_y = player_transform.translation.y;

        if no_zombies {
            zombie_wave.0 += 1;
            //zombie spawns but disapears.  Entity is still there just not graphic
            for _n in 0..zombie_wave.0 {
                let x_pos = rand::thread_rng().gen_range(-275..=275) as f32;
                let y_pos = rand::thread_rng().gen_range(-275..=275) as f32; 
                commands.spawn((SpriteBundle {
                    texture: image_cache.zombie.clone_weak(),
                    transform: Transform::from_translation(Vec3::new(x_pos + player_x, y_pos + player_y, 1.)),
                    ..default()
                },
                Zombie));
            }

        }
    }

}
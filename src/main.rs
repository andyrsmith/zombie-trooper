use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::f32::consts::PI;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Zombie;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Movement {
    last_movement: Direction,
}

#[derive(Component)]
struct Distance {
    distance_travel: i32,
    distance_despawn: i32
}

const PLAYER_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const ZOMBIE_RADIUS: f32 = 15.;
const BULLET_RADIUS: f32 = 1.;
const TIME_STEP: f32 = 1.0/60.0;
const PLAYER_SPEED: f32 = 100.0;

// ResMut is a mutable resource

fn setup_game(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/manRed_gun.png"),
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            ..default()
        },
        Player,
        Movement {
            last_movement: Direction::UP,
        },
        Collider,
    ));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("sprites/zoimbie1_hold.png"),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    },
    Zombie));

}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &mut Movement), With<Player>>,) {
    if let Ok((mut player_transform, mut player_movement)) = query.get_single_mut() {
        let mut xdirection = 0.0;
        let mut ydirection = 0.0;
        let mut rotation = 0.0;
        
        if keyboard_input.pressed(KeyCode::A) {
            xdirection -= 1.0;
            player_movement.last_movement = Direction::LEFT;
            rotation = 0.0;
            player_transform.rotation = Quat::from_rotation_z(rotation);
        }
    
        if keyboard_input.pressed(KeyCode::D) {
            xdirection += 1.0;
            player_movement.last_movement = Direction::RIGHT;
            rotation = PI;
            player_transform.rotation = Quat::from_rotation_z(rotation);
        }
    
        if keyboard_input.pressed(KeyCode::S) {
            ydirection -= 1.0;
            player_movement.last_movement = Direction::DOWN;
            rotation = PI / 2.;
            player_transform.rotation = Quat::from_rotation_z(rotation);
        }
    
        if keyboard_input.pressed(KeyCode::W) {
            ydirection += 1.0;
            player_movement.last_movement = Direction::UP;
            rotation = (3. * PI) / 2.;
            player_transform.rotation = Quat::from_rotation_z(rotation);
        }
        player_transform.translation.y = player_transform.translation.y + ydirection;
        player_transform.translation.x = player_transform.translation.x + xdirection;

    }

}

fn player_shoot(mut commands: Commands, 
    keyboard_input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Transform, &mut Movement), With<Player>>) {
    if let Ok((player_transform, player_movement)) = query.get_single_mut() {
        if keyboard_input.just_released(KeyCode::Space) {
            commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(BULLET_RADIUS).into()).into(),
                material: material.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(player_transform.translation),
                ..default()
            },
            Bullet,
            Movement {
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

fn move_bullet(mut query: Query<(&mut Transform, &mut Movement, &mut Distance), With<Bullet>>) {
    for (mut transform, movement, mut distance_query) in &mut query {
        distance_query.distance_travel += 1;
        if movement.last_movement == Direction::DOWN {
            transform.translation.y -= 1.0;
        } else if movement.last_movement == Direction::LEFT {
            transform.translation.x -= 1.0;
        } else if movement.last_movement == Direction::UP {
            transform.translation.y += 1.0;
        } else {
            transform.translation.x += 1.0;
        }
    }
}

fn move_zombies(mut player_query: Query<(&Transform, &Movement), (With<Player>, Without<Zombie>)>, mut zombie_query: Query<&mut Transform, (With<Zombie>, Without<Player>)>) {

    if let Ok((player_transform, player_movement)) = player_query.get_single_mut() {
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

fn zombie_player_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    zombie_query: Query<&Transform, With<Zombie>>
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for transform in &zombie_query {
            let distance = player_transform.translation.distance(transform.translation);
            let player_radius = PLAYER_SIZE.x / 2.0;
            if distance < player_radius + ZOMBIE_RADIUS {
                println!("Game Over");
                commands.entity(player_entity).despawn();
            }
        }
    }
}

fn zombie_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    zombie_query: Query<(Entity, &Transform), With<Zombie>>
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        for (zombie_entity, zombie_transform) in &zombie_query {
            let distance = bullet_transform.translation.distance(zombie_transform.translation);
            if distance < ZOMBIE_RADIUS + BULLET_RADIUS {
                println!("Zombie Shot!");
                commands.entity(bullet_entity).despawn();
                commands.entity(zombie_entity).despawn();
            }

        }
    }

}

fn despawn_bullet(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Distance), With<Bullet>>
) {
    for(bullet_entity, bullet_distance) in &bullet_query {
        if(bullet_distance.distance_travel > bullet_distance.distance_despawn) {
            commands.entity(bullet_entity).despawn();
        }
    }

}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_game)
        .add_systems(Update, (move_player, player_shoot, move_bullet, move_zombies, zombie_player_collision, zombie_bullet_collision, despawn_bullet))
        .run();
}


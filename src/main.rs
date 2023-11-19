use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
struct Collider;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Movement {
    last_movement: Direction
}

const PLAYER_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const TIME_STEP: f32 = 1.0/60.0;
const PLAYER_SPEED: f32 = 100.0;

// ResMut is a mutable resource

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform { 
                translation: Vec3::new(0.0,0.0,10.0),
                 scale:PLAYER_SIZE,
                ..default() 
            },
            sprite: Sprite { 
                color: PLAYER_COLOR, 
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

}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &mut Movement), With<Player>>,) {
    let (mut player_transform, mut player_movement) = query.single_mut();
    let mut xdirection = 0.0;
    let mut ydirection = 0.0;
    
    if keyboard_input.pressed(KeyCode::A) {
        xdirection -= 1.0;
        player_movement.last_movement = Direction::LEFT;
    }

    if keyboard_input.pressed(KeyCode::D) {
        xdirection += 1.0;
        player_movement.last_movement = Direction::RIGHT;
    }

    if keyboard_input.pressed(KeyCode::S) {
        ydirection -= 1.0;
        player_movement.last_movement = Direction::DOWN;
    }

    if keyboard_input.pressed(KeyCode::W) {
        ydirection += 1.0;
        player_movement.last_movement = Direction::UP;
    }

    player_transform.translation.y = player_transform.translation.y + ydirection;
    player_transform.translation.x = player_transform.translation.x + xdirection;
}

fn player_shoot(mut commands: Commands, 
    keyboard_input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Transform, &mut Movement), With<Player>>) {
    let (player_transform, player_movement) = query.single_mut();

    if keyboard_input.just_released(KeyCode::Space) {
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(1.).into()).into(),
            material: material.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(player_transform.translation),
            ..default()
        },
        Bullet,
        Movement {
            last_movement: player_movement.last_movement
        }
    ));
    }

}

fn move_bullet(mut query: Query<(&mut Transform, &mut Movement), With<Bullet>>) {
    for (mut transform, movement) in &mut query {
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
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_game)
        .add_systems(Update, (move_player, player_shoot, move_bullet))
        .run();
}


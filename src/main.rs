use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Name(String);

const PLAYER_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const TIME_STEP: f32 = 1.0/60.0;
const PLAYER_SPEED: f32 = 100.0;

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform { 
                translation: Vec3::new(-125.0,35.0,0.0),
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
        Collider,
    ));
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>,) {
    let mut player_transform = query.single_mut();
    let mut xdirection = 0.0;
    let mut ydirection = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        xdirection -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        xdirection += 1.0;

    }

    if keyboard_input.pressed(KeyCode::S) {
        ydirection -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        ydirection += 1.0;
    }
    let new_player_position_x = player_transform.translation.x + xdirection*PLAYER_SPEED*TIME_STEP;
    let new_player_position_y = player_transform.translation.y + ydirection*PLAYER_SPEED*TIME_STEP;
    player_transform.translation.y = new_player_position_y;
    player_transform.translation.x = new_player_position_x;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_game)
        .add_systems(Update, move_player)
        .run();
}

use bevy::{prelude::*, window::close_on_esc};
use bevy_ecs_tilemap::prelude::*;

mod player;
mod movement;
mod zombies;
mod camera;
mod bullet;

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Start,
    Playing,
    GameOver,
}

#[derive(Resource)]
struct ImageCache {
    zombie: Handle<Image>,
}

#[derive(Resource, Debug)]
struct ZombieWave(i32);

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct GameOverMessage;

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    commands.spawn((Camera2dBundle::default(),
    camera::CameraMaker));
    // add setting up spritesheet_tiles
    let texture_handle: Handle<Image> = asset_server.load("spritesheet/spritesheet_tiles.png");

    let map_size = TilemapSize { x: 64, y: 64 };
    
    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size:map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    //add setting up graphics
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/manRed_gun.png"),
            transform: Transform::from_translation(Vec3::new(0.,0.,1.)),
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            ..default()
        },
        player::Player,
        movement::Movement {
            last_movement: movement::Direction::UP,
        }
    ));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("sprites/zoimbie1_hold.png").clone_weak(),
        transform: Transform::from_translation(Vec3::new(150., 0., 1.)),
        ..default()
    },
    zombies::Zombie));

}


fn load_and_cache_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    println!("Adding Resource");
    commands.insert_resource(ImageCache {
        zombie: asset_server.load("sprites/zoimbie1_hold.png")
    });
}

fn main_menu(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: 20.,
        ..default()
    };
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(50.),
            ..default()
        },
        ..default()
    }))
    .with_children(|parent| {
        parent.spawn((TextBundle::from_sections([
            TextSection::new("Zombie Trooper", text_style.clone()),
        ]), MainMenu));
        parent.spawn((TextBundle::from_sections([
            TextSection::new("AWSD to move, spacebar to shoot", text_style.clone()),
        ]), MainMenu));
        parent.spawn((TextBundle::from_sections([
            TextSection::new("Press Enter to Start", text_style.clone()),
        ]), MainMenu));
    });
}

fn check_start(keyboard_input: Res<Input<KeyCode>>, 
    query: Query<Entity, With<MainMenu>>, 
    mut commands: Commands,
    mut app_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.pressed(KeyCode::Return) {
        for text in &query {
            commands.entity(text).despawn_recursive();
        }
        app_state.set(GameState::Playing);
    }
}

fn game_over(mut commands: Commands,
    query: Query<Entity, With<GameOverMessage>>) {
    let text_style = TextStyle {
        font_size: 20.,
        ..default()
    };

    if query.is_empty() {
        commands.spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(50.),
                ..default()
            },
            ..default()
        }))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_sections([
                TextSection::new("Game Over", text_style.clone()),
            ]), GameOverMessage));
            parent.spawn((TextBundle::from_sections([
                TextSection::new("Press Enter to restart", text_style.clone()),
            ]), GameOverMessage));
            parent.spawn((TextBundle::from_sections([
                TextSection::new("Press ESC to quit", text_style.clone()),
            ]), GameOverMessage));
        });
    }
}

fn listen_for_restart(mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,   
    query: Query<Entity, With<GameOverMessage>>,
    mut app_state: ResMut<NextState<GameState>>,
    zombie_query: Query<(Entity), With<zombies::Zombie>>,
    asset_server: Res<AssetServer>,
    mut zombie_wave: ResMut<ZombieWave>
) {
    if keyboard_input.pressed(KeyCode::Return) {
        for text in &query {
            commands.entity(text).despawn_recursive();
        }

        for zombie_entity in &zombie_query {
            commands.entity(zombie_entity).despawn();
        }
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/manRed_gun.png"),
                transform: Transform::from_translation(Vec3::new(0.,0.,1.)),
                sprite: Sprite {
                    flip_x: true,
                    ..default()
                },
                ..default()
            },
            player::Player,
            movement::Movement {
                last_movement: movement::Direction::UP,
            }
        ));
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("sprites/zoimbie1_hold.png").clone_weak(),
            transform: Transform::from_translation(Vec3::new(150., 0., 1.)),
            ..default()
        },
        zombies::Zombie));
        zombie_wave.0 = 1;
        app_state.set(GameState::Playing);
    }
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .insert_resource(ZombieWave(1))
        .add_systems(Startup, load_and_cache_images)
        .add_systems(Startup, setup_game)
        .add_systems(Startup, (main_menu).run_if(in_state(GameState::Start)))
        .add_systems(Update, (check_start).run_if(in_state(GameState::Start)))
        .add_systems(Update, (player::move_player, camera::update_camera, player::player_shoot, bullet::move_bullet, zombies::move_zombies, zombies::zombie_player_collision, zombies::zombie_bullet_collision, bullet::despawn_bullet, zombies::next_zombie_wave).run_if(in_state(GameState::Playing)))
        .add_systems(Update, (game_over, listen_for_restart).run_if(in_state(GameState::GameOver)))
        .add_systems(Update, close_on_esc)
        .run();
}



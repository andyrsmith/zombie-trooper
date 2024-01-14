use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod player;
pub mod movement;
pub mod zombies;
pub mod camera;
pub mod bullet;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Name(String);

const TIME_STEP: f32 = 1.0/60.0;
const PLAYER_SPEED: f32 = 100.0;

// ResMut is a mutable resource

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
        },
        Collider,
    ));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("sprites/zoimbie1_hold.png"),
        transform: Transform::from_translation(Vec3::new(150., 0., 1.)),
        ..default()
    },
    zombies::Zombie));

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, setup_game)
        .add_systems(Update, (player::move_player, camera::update_camera, player::player_shoot, bullet::move_bullet, zombies::move_zombies, zombies::zombie_player_collision, zombies::zombie_bullet_collision, bullet::despawn_bullet))
        .run();
}


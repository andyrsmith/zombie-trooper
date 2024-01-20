use bevy::{prelude::*, ui::debug};
use bevy_ecs_tilemap::prelude::*;

mod player;
mod movement;
mod zombies;
mod camera;
mod bullet;

#[derive(Resource)]
struct ImageCache {
    zombie: Handle<Image>,
}

#[derive(Resource, Debug)]
struct ZombieWave(i32);


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


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .insert_resource(ZombieWave(1))
        .add_systems(Startup, load_and_cache_images)
        .add_systems(Startup, setup_game)
        .add_systems(Update, (player::move_player, camera::update_camera, player::player_shoot, bullet::move_bullet, zombies::move_zombies, zombies::zombie_player_collision, zombies::zombie_bullet_collision, bullet::despawn_bullet, zombies::next_zombie_wave))
        .run();
}


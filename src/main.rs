mod components;
mod map;
mod prefab;
mod systems;

use simplelog::{Config, LevelFilter, TermLogger};

use bevy::{app::Events, prelude::*, text::Text2dSize, window::WindowResized};
use map::{Generator, TileMap};

const TILESIZE: u32 = 16;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).expect("error");

    App::build()
        .insert_resource(WindowDescriptor {
            title: "Blob".to_owned(),
            width: WIDTH as f32,
            height: HEIGHT as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(systems::read_input.system())
        .add_system(systems::move_sprite.system())
        .add_system(window_resize.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture = asset_server.load("dungeon.png");
    let atlas =
        TextureAtlas::from_grid(texture, Vec2::new(TILESIZE as f32, TILESIZE as f32), 16, 16);
    let handle = texture_atlas.add(atlas);

    let world_width = WIDTH / TILESIZE;
    let world_height = HEIGHT / TILESIZE;

    let mut tilemap = TileMap::new(world_width, world_height);

    let mut generator = Generator::new(world_width, world_height, 5, 10, 50);

    let rooms = generator.generate();

    for y in 0..generator.height() {
        for x in 0..generator.width() {
            log::debug!("New tile: ({},{})", x, y);
            prefab::create_tile(
                &mut commands,
                x as u32,
                y as u32,
                generator.tile(x, y),
                handle.clone()
            );
            tilemap.add_tile(generator.tile(x, y));
        }
    }

    commands.insert_resource(tilemap);

    let (x, y) = rooms[0].center();

    prefab::create_player(&mut commands, x, y, handle.clone());
}

fn window_resize(resize_event: Res<Events<WindowResized>>) {
    for event in resize_event.get_reader().iter(&resize_event) {
        log::debug!("Resize {}/{}", event.width, event.height);
    }
}

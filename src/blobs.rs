use amethyst::{GameData, SimpleState, StateData, StateEvent, SimpleTrans, Trans};
use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::Transform;
use amethyst::core::math::Vector3;
use amethyst::ecs::{Builder, EntityBuilder, World};
use amethyst::input::{VirtualKeyCode, is_key_down};
use amethyst::renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};

use crate::config::MapConfig;
use crate::components::{Player, Tile};
use crate::map::{Generator, Map, TileType};

pub struct Blobs {
    sprite_sheet_handle_char: Option<Handle<SpriteSheet>>,
    sprite_sheet_handle_map: Option<Handle<SpriteSheet>>
}

impl Default for Blobs {
    fn default() -> Self {
        Blobs {
            sprite_sheet_handle_char: None,
            sprite_sheet_handle_map: None
        }
    }
}

impl SimpleState for Blobs {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let the_world = data.world;

        the_world.register::<Tile>();

        let handle_char = load_sprite_sheet(the_world, "sprites.png", "sprites.ron");
        let handle_map = load_sprite_sheet(the_world, "dungeon.png", "dungeon.ron");

        self.sprite_sheet_handle_char.replace(handle_char.clone());
        self.sprite_sheet_handle_map.replace(handle_map.clone());

        let (player_x, player_y) = init_map(the_world, handle_map);
        init_player(the_world, player_x, player_y, handle_char);
        init_camera(the_world);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>,
                    event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit
            }
        }

        Trans::None
    }
}

fn create_tile(the_world: &mut World, x: f32, y: f32, z: f32,
               handle: Option<Handle<SpriteSheet>>, idx: usize) -> EntityBuilder {
    let transform = {
        let map = the_world.read_resource::<Map>();
        let mut transform = Transform::default();
        transform.set_scale(Vector3::from_element(map.ratio()));
        transform.set_translation_xyz(x, y, z);

        transform
    };

    let builder = the_world.create_entity()
        .with(transform);

    if let Some(handle) = handle {
        let sprite_render = SpriteRender {
            sprite_sheet: handle,
            sprite_number: idx
        };

        builder.with(sprite_render)
    } else {
        builder
    }
}

fn init_map(the_world: &mut World, handle: Handle<SpriteSheet>) -> (usize, usize) {
    let (mut generator, map) = {
        let config = &the_world.read_resource::<MapConfig>();
        let generator = Generator::new(config);
        let map = Map::new(config);

        (generator, map)
    };

    the_world.add_resource(map);

    let start = generator.generate();

    let floor = Tile::new(false, true);
    let wall = Tile::new(true, false);

    for y in 0..generator.height() {
        for x in 0..generator.width() {
            let tile = match generator.tile(x, y) {
                TileType::None => {
                    create_tile(the_world, x as f32, y as f32, 0.,
                                None, 0)
                        .with(floor.clone())
                        .build()
                },
                TileType::Wall => {
                    create_tile(the_world, x as f32, y as f32, 0.,
                                Some(handle.clone()), 1)
                        .with(wall.clone())
                        .build()
                },
                TileType::Full => {
                    create_tile(the_world, x as f32, y as f32, 0.,
                                Some(handle.clone()), 0)
                        .with(wall.clone())
                        .build()
                }

            };

            let mut map = the_world.write_resource::<Map>();
            map.add_tile(tile);
        }
    }

    start
}

fn init_player(the_world: &mut World, player_x: usize, player_y: usize,
               handle: Handle<SpriteSheet>) {
    create_tile(the_world, player_x as f32, player_y as f32, 1., Some(handle), 1)
        .with(Player)
        .build();
}

fn init_camera(the_world: &mut World) {
    let (transform, camera) = {
        let mut transform = Transform::default();
        let map = the_world.read_resource::<Map>();

        transform.set_translation_xyz(map.width() as f32 / 2.0 - 0.5,
                                      map.height() as f32 / 2.0 - 0.5,
                                      10.0);

        let camera = Camera::standard_2d(map.width() as f32, map.height() as f32);

        (transform, camera)
    };

   the_world.create_entity()
        .with(camera)
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World, file: &str, config: &str)
    -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            file,
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        config,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}


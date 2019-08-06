use amethyst::{GameData, SimpleState, StateData, StateEvent, SimpleTrans, Trans};
use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::Transform;
use amethyst::core::math::Vector3;
use amethyst::ecs::{Builder, World};
use amethyst::input::{VirtualKeyCode, is_key_down};
use amethyst::renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};

use crate::config::MapConfig;
use crate::components::{Player, Tile};
use crate::map::Map;

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
        init_map(the_world, handle_map.clone());
        init_player(the_world, handle_char);
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

fn init_map(the_world: &mut World, handle: Handle<SpriteSheet>) {
    let map = {
        let config = &the_world.read_resource::<MapConfig>();
        Map::new(config)
    };

    let mut transform = Transform::default();
    transform.set_scale(Vector3::from_element(map.ratio()));
    transform.set_translation_xyz(5., 5., 0.);

    let sprite_render = SpriteRender {
        sprite_sheet: handle,
        sprite_number: 0
    };

    let tile = the_world.create_entity()
        .with(Tile::default())
        .with (transform)
        .with(sprite_render)
        .build();

   the_world.add_resource(map);
}

fn init_player(the_world: &mut World, handle: Handle<SpriteSheet>) {
    let transform = {
        let map = the_world.read_resource::<Map>();
        let mut transform = Transform::default();
        transform.set_scale(Vector3::from_element(map.ratio()));

        transform.set_translation_xyz(map.width() as f32 / 2.0,
                                      map.height() as f32 / 2.0, 0.0);

        transform
    };

    let sprite_render = SpriteRender {
        sprite_sheet: handle,
        sprite_number: 1
    };

    the_world.create_entity()
        .with(transform)
        .with(Player)
        .with(sprite_render)
        .build();
}

fn init_camera(the_world: &mut World) {
    let (transform, camera) = {
        let mut transform = Transform::default();
        let map = the_world.read_resource::<Map>();

        transform.set_translation_xyz(map.width() as f32 / 2.0,
                                      map.height() as f32 / 2.0, 1.0);

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


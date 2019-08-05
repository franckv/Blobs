use amethyst::{GameData, SimpleState, StateData, StateEvent, SimpleTrans, Trans};
use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::Transform;
use amethyst::ecs::{Builder, Component, DenseVecStorage, World};
use amethyst::input::{VirtualKeyCode, is_key_down};
use amethyst::renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};

use crate::map::Map;

pub struct Blobs {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>
}

impl Default for Blobs {
    fn default() -> Self {
        Blobs {
            sprite_sheet_handle: None
        }
    }
}

impl SimpleState for Blobs {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let the_world = data.world;

        the_world.add_resource(Map::default());

        let handle = load_sprite_sheet(the_world, "dwarves.png", "dwarves.ron");

        self.sprite_sheet_handle.replace(handle.clone());
        init_player(the_world, handle);
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

pub struct Player;
impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn init_player(the_world: &mut World, handle: Handle<SpriteSheet>) {
    let transform = {
        let mut transform = Transform::default();
        let map = the_world.read_resource::<Map>();

        transform.set_translation_xyz(map.width_px() / 2.0,
                                      map.height_px() / 2.0, 0.0);

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

        transform.set_translation_xyz(map.width_px() / 2.0,
                                      map.height_px() / 2.0, 1.0);

        let camera = Camera::standard_2d(map.width_px(), map.height_px());

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


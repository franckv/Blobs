use amethyst::{GameData, SimpleState, StateData};
use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::Transform;
use amethyst::ecs::{Builder, Component, DenseVecStorage, World};
use amethyst::renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};

pub const MAP_WIDTH: f32 = 512.;
pub const MAP_HEIGHT: f32 = 512.;

#[derive(Default)]
pub struct Blobs {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>
}

pub struct Player;
impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for Blobs {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let the_world = data.world;

        let handle = load_sprite_sheet(the_world, "dwarves.png", "dwarves.ron");

        self.sprite_sheet_handle.replace(handle.clone());
        init_player(the_world, handle);
        init_camera(the_world);
    }
}

fn init_player(the_world: &mut World, handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();

    transform.set_translation_xyz(MAP_WIDTH / 2.0, MAP_HEIGHT / 2.0, 0.0);

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
    let mut transform = Transform::default();
    transform.set_translation_xyz(MAP_WIDTH / 2.0, MAP_HEIGHT / 2.0, 1.0);

    the_world.create_entity()
        .with(Camera::standard_2d(MAP_WIDTH, MAP_HEIGHT))
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


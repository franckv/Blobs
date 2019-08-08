use amethyst::{GameData, SimpleState, StateData, StateEvent, SimpleTrans, Trans};
use amethyst::core::Transform;
use amethyst::ecs::{Builder, World};
use amethyst::input::{VirtualKeyCode, is_key_down};
use amethyst::renderer::Camera;

use crate::config::MapConfig;
use crate::components::Init;
use crate::map::{Generator, Map};
use crate::prefab;
use crate::sprite::{SpriteHandler, SpriteSheets};

#[derive(Default)]
pub struct Blobs;

impl SimpleState for Blobs {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let the_world = data.world;

        let mut sprite_handler = SpriteHandler::default();

        sprite_handler.add_sprite_sheet(the_world, SpriteSheets::Character,
                                             "sprites.png", "sprites.ron");
        sprite_handler.add_sprite_sheet(the_world, SpriteSheets::Dungeon,
                                             "dungeon.png", "dungeon.ron");

        the_world.add_resource(sprite_handler);

        the_world.create_entity().with(Init).build();

        let (player_x, player_y) = init_map(the_world);
        prefab::create_player(the_world, player_x, player_y);
        init_camera(the_world);
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>,
                    event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit
            }
        }

        Trans::None
    }
}

fn init_map(the_world: &mut World) -> (usize, usize) {
    let (mut generator, map) = {
        let config = &the_world.read_resource::<MapConfig>();
        let generator = Generator::new(config);
        let map = Map::new(config);

        (generator, map)
    };

    the_world.add_resource(map);

    let start = generator.generate();

    for y in 0..generator.height() {
        for x in 0..generator.width() {
            let tile = prefab::create_tile(the_world, x, y, generator.tile(x, y));

            let mut map = the_world.write_resource::<Map>();
            map.add_tile(tile);
        }
    }

    start
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

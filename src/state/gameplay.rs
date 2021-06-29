use amethyst::{GameData, SimpleState, StateData, StateEvent, SimpleTrans, Trans};
use amethyst::assets::Loader;
use amethyst::core::Transform;
use amethyst::ecs::{Builder, World};
use amethyst::input::{VirtualKeyCode, is_key_down};
use amethyst::renderer::Camera;
use amethyst::ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform};
use amethyst::prelude::WorldExt;

use crate::config::BlobsConfig;
use crate::components::Init;
use crate::map::{Generator, Map};
use crate::utils::prefab;
use crate::ui;
use crate::utils::sprite::{SpriteHandler, SpriteSheets};

#[derive(Default)]
pub struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let the_world = data.world;

        let mut sprite_handler = SpriteHandler::default();

        sprite_handler.add_sprite_sheet(the_world, SpriteSheets::Character,
                                             "sprites.png", "sprites.ron");
        sprite_handler.add_sprite_sheet(the_world, SpriteSheets::Dungeon,
                                             "dungeon.png", "dungeon.ron");
        sprite_handler.add_sprite_sheet(the_world, SpriteSheets::Mobs,
                                             "mobs.png", "mobs.ron");

        the_world.insert(sprite_handler);

        the_world.create_entity().with(Init).build();

        let (player_x, player_y) = init_map(the_world);

        prefab::create_player(the_world, player_x, player_y);
        init_camera(the_world);
        init_ui(the_world);
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
        let config = &the_world.read_resource::<BlobsConfig>();
        let generator = Generator::new(&config.map);
        let map = Map::new(&config.map);

        (generator, map)
    };

    the_world.insert(map);

    let rooms = generator.generate();

    for y in 0..generator.height() {
        for x in 0..generator.width() {
            let tile = prefab::create_tile(the_world, x, y, generator.tile(x, y));

            let mut map = the_world.write_resource::<Map>();
            map.add_tile(tile);
        }
    }

    for room in rooms.iter().skip(1) {
        let c = room.center();
        prefab::create_mob(the_world, c.0, c.1);
    }

    rooms[0].center()
}

fn init_camera(the_world: &mut World) {
    let (transform, camera) = {
        let mut transform = Transform::default();
        let config = the_world.read_resource::<BlobsConfig>();

        let screen_width = (config.map.width + config.panel.right) as f32;
        let screen_height = (config.map.height + config.panel.bottom) as f32;

        transform.set_translation_xyz(
            screen_width / 2.0 - 0.5,
            screen_height / 2.0 - config.panel.bottom as f32 - 0.5,
            10.0);

        let camera = Camera::standard_2d(screen_width, screen_height as f32);

        (transform, camera)
    };

   the_world.create_entity()
        .with(camera)
        .with(transform)
        .build();
}

fn init_ui(the_world: &mut World) {
    let (log_height, log_lines, font_size) = {
        let config = the_world.read_resource::<BlobsConfig>();

        (config.panel.bottom as f32 * config.map.tile_size as f32,
         config.panel.bottom_lines, config.panel.font_size as f32)
    };

    let font = the_world.read_resource::<Loader>().load(
        "font.ttf",
        TtfFormat,
        (),
        &the_world.read_resource()
    );

    let transform = UiTransform::new(
        "hp".to_string(),
        Anchor::TopLeft,
        Anchor::Middle,
        260., -20., 5.,
        500., 50.);

    let text = UiText::new(
        font.clone(),
        "HP:".to_string(),
        [1., 1., 1., 1.],
        font_size,
        LineMode::Single,
        Anchor::MiddleLeft);

    let label = the_world.create_entity()
        .with(transform)
        .with(text)
        .build();

    let hp = ui::Hp::new(label);

    the_world.insert(hp);

    let transform = UiTransform::new(
        "log".to_string(),
        Anchor::BottomLeft,
        Anchor::Middle,
        410., log_height / 2., 5.,
        800., log_height);

    let text = UiText::new(
        font.clone(),
        "".to_string(),
        [1., 1., 1., 1.],
        font_size,
        LineMode::Wrap,
        Anchor::TopLeft);

    let label = the_world.create_entity()
        .with(transform)
        .with(text)
        .build();

    let message_log = ui::MessageLog::new(label, log_lines);

    the_world.insert(message_log);
}

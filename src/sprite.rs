use std::collections::HashMap;

use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::ecs::World;
use amethyst::renderer::{
    ImageFormat, SpriteRender, SpriteSheet,
    SpriteSheetFormat, Texture};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum SpriteSheets {
    Character,
    Dungeon,
    Mobs
}

#[derive(Copy, Clone)]
pub enum Sprite {
    Player,
    Blob,
    Wall,
    Floor,
    Full
}

#[derive(Default)]
pub struct SpriteHandler {
    sprite_sheets: HashMap<SpriteSheets, Handle<SpriteSheet>>
}

impl SpriteHandler {
    pub fn add_sprite_sheet(&mut self, the_world: &mut World,
                            sheet: SpriteSheets,
                            file: &str, config: &str) {
        let handle = load_sprite_sheet(the_world, file, config);

        self.sprite_sheets.insert(sheet, handle);
    }

    pub fn get_sprite(&self, sprite: Sprite) -> SpriteRender {
        match sprite {
            Sprite::Player => {
                SpriteRender {
                    sprite_sheet: self.sprite_sheets[&SpriteSheets::Character].clone(),
                    sprite_number: 1
                }
            },
            Sprite::Blob => {
                SpriteRender {
                    sprite_sheet: self.sprite_sheets[&SpriteSheets::Mobs].clone(),
                    sprite_number: 0
                }
            },
            Sprite::Wall => {
                SpriteRender {
                    sprite_sheet: self.sprite_sheets[&SpriteSheets::Dungeon].clone(),
                    sprite_number: 1
                }
            },
            Sprite::Floor => {
                SpriteRender {
                    sprite_sheet: self.sprite_sheets[&SpriteSheets::Dungeon].clone(),
                    sprite_number: 2
                }
            },
            Sprite::Full => {
                SpriteRender {
                    sprite_sheet: self.sprite_sheets[&SpriteSheets::Dungeon].clone(),
                    sprite_number: 0
                }
            }
        }
    }
}

fn load_sprite_sheet(world: &mut World, file: &str, config: &str)
    -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            file,
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        config,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}

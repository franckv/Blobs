use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::ecs::World;
use amethyst::renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture};

#[derive(Default)]
pub struct SpriteHandler {
    sprite_sheets: Vec<Handle<SpriteSheet>>
}

impl SpriteHandler {
    pub fn add_sprite_sheet(&mut self, the_world: &mut World,
                            file: &str, config: &str) {
        let handle = load_sprite_sheet(the_world, file, config);

        self.sprite_sheets.push(handle);
    }

    pub fn get_sprite_sheet(&self, idx: usize) -> Handle<SpriteSheet> {
        self.sprite_sheets[idx].clone()
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

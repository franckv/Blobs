use amethyst::core::{Hidden, Transform};
use amethyst::core::math::Vector3;
use amethyst::ecs::{Builder, Entity, EntityBuilder, World};
use amethyst::prelude::WorldExt;

use crate::components::{Fighter, Health, Mob, Name, Player, Tile};
use crate::map::{Map, TileType};
use crate::utils::sprite::{Sprite, SpriteHandler};

pub fn create_player(the_world: &mut World,
                     player_x: usize, player_y: usize) -> Entity {
    create_entity(the_world, player_x, player_y, 1, Some(Sprite::Player), 1.)
        .with(Health::new(100))
        .with(Fighter::new(5, 2))
        .with(Name::new("Hero"))
        .with(Player)
        .build()
}

pub fn create_mob(the_world: &mut World, x: usize, y: usize) -> Entity {
    create_entity(the_world, x, y, 1, Some(Sprite::Blob), 16./24.)
        .with(Health::new(10))
        .with(Fighter::new(3, 1))
        .with(Name::new("Blob"))
        .with(Mob)
        .with(Hidden)
        .build()
}

pub fn create_tile(the_world: &mut World,
                   x: usize, y: usize, tile_type: TileType) -> Entity {
    let (block, transparent, sprite) = match tile_type {
        TileType::None => (false, true, None),
        TileType::Floor => (false, true, Some(Sprite::Floor)),
        TileType::Wall => (true, false, Some(Sprite::Wall)),
        TileType::Full => (true, false, Some(Sprite::Full))
    };

    create_entity(the_world, x, y, 0, sprite, 1.)
        .with(Tile::new(block, transparent))
        .build()
}

fn create_entity(the_world: &mut World, x: usize, y: usize, z: usize,
               sprite: Option<Sprite>, ratio: f32) -> EntityBuilder {
    let transform = {
        let map = the_world.read_resource::<Map>();
        let mut transform = Transform::default();
        transform.set_scale(Vector3::from_element(ratio * map.ratio()));
        transform.set_translation_xyz(x as f32,
                                      y as f32,
                                      z as f32);

        transform
    };

    let sprite_render = match sprite {
        Some(sprite) => {
            let handler = the_world.read_resource::<SpriteHandler>();
            Some(handler.get_sprite(sprite))
        },
        None => None
    };

    let builder = the_world.create_entity()
        .with(transform);

    if let Some(sprite_render) = sprite_render {
        builder.with(sprite_render)
    } else {
        builder
    }
}



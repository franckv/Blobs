use bevy::prelude::*;

use crate::{components::{Name, Player, Position}, map::Tile};

pub fn create_player(commands: &mut Commands, x: u32, y: u32, atlas: Handle<TextureAtlas>) {
    let player = TextureAtlasSprite::new(64);

    let mut transform = Transform::from_translation(Vec3::new(x as f32 * 16., y as f32 *  16., 0.));

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas,
            sprite: player,
            ..Default::default()
        })
        .insert(Player)
        .insert(Position::new(x, y))
        .insert(Name::new("Player"))
        .insert(transform);
}

pub fn _create_label(commands: &mut Commands, label: &str, font: Handle<Font>) {
    let text_align = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Left
    };

    let text_style = TextStyle {
        font,
        font_size: 16.,
        color: Color::GREEN
    };

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(label.to_owned(), text_style, text_align),        
        transform: Transform::from_translation(Vec3::new(0., 250., 0.)),
        ..Default::default()
    });
}

pub fn create_tile(commands: &mut Commands, x: u32, y: u32, tile: Tile, atlas: Handle<TextureAtlas>) {
    
    let idx = match tile {
        Tile::Floor => TextureAtlasSprite::new(206),
        Tile::Full => TextureAtlasSprite::new(0),
        Tile::Wall => TextureAtlasSprite::new(8),
        Tile::None => TextureAtlasSprite::new(0)
    };

    let mut transform = Transform::from_translation(Vec3::new(x as f32 * 16., y as f32 *  16., 0.));

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas,
        sprite: idx,
        ..Default::default()
    })
    .insert(Position::new(x, y))
    .insert(transform);
}
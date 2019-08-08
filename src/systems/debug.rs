use amethyst::core::{Hidden, Transform};
use amethyst::ecs::{Entities, Join, ReadStorage, System};
use amethyst::renderer::{Camera, SpriteRender};

use crate::components::{Explored, Init, Intent, Mob, Player, Tile};

#[derive(Default)]
pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Explored>,
        ReadStorage<'s, Hidden>,
        ReadStorage<'s, Init>,
        ReadStorage<'s, Intent>,
        ReadStorage<'s, Mob>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, SpriteRender>,
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Transform>,
        Entities<'s>
    );

    fn run(&mut self, (camera, explored, hidden, init, intent, mob, player,
                       sprite, tile, transform, entities): Self::SystemData) {
        for entity in entities.join() {
            debug!("{:?}: [", entity);
            if let Some(c) = camera.get(entity) {
                debug!("  {:?}, ", c);
            }
            if let Some(c) = explored.get(entity) {
                debug!("  {:?}, ", c);
            }
            if let Some(c) = hidden.get(entity) {
                debug!("  {:?}, ", c);
            }
            if let Some(c) = init.get(entity) {
                debug!("  {:?}, ", c);
            }
            if let Some(c) = intent.get(entity) {
                debug!("  {:?}, ", c);
            }
            if let Some(c) = mob.get(entity) {
                debug!("  {:?}, ", c);
            }
            if let Some(c) = player.get(entity) {
                debug!("  {:?}, ", c);
            }
            if let Some(c) = sprite.get(entity) {
                debug!("  {:?}, ", c);
            }
            if let Some(c) = tile.get(entity) {
                debug!("  {:?}, ", c);
            }
            if let Some(c) = transform.get(entity) {
                debug!("  {:?}", c);
            }

            debug!("]");
        }
    }
}

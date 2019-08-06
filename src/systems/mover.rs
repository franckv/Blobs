use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

use crate::map::Map;
use crate::components::{Direction, Action, Intent, Tile};

#[derive(Default)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Intent>,
        ReadStorage<'s, Tile>,
        Read<'s, LazyUpdate>,
        Read<'s, Map>,
        Entities<'s>
    );

    fn run(&mut self, (mut transform, intents, tiles, update, map, entities):
           Self::SystemData) {
        for (transform, intent, entity) in
            (&mut transform, &intents, &entities).join() {
            let (x, y) = (transform.translation().x, transform.translation().y);

            if let Action::Move(dir) = intent.action() {
                let (dx, dy) = match dir {
                    Direction::Up => {
                        (0., 1.)
                    },
                    Direction::Down => {
                        (0., -1.)
                    },
                    Direction::Left => {
                        (-1., 0.)
                    },
                    Direction::Right => {
                        (1., 0.)
                    },
                };

                if map.in_bound(x + dx, y + dy) &&
                    !Self::is_blocked(x + dx, y + dy, &map, &tiles) {
                    transform.set_translation_xyz(x + dx, y + dy, 0.);
                }

            }

            update.remove::<Intent>(entity);
        }
    }
}

impl<'s> MoveSystem {
    fn is_blocked(x: f32, y: f32, map: &Map, tiles: &ReadStorage<'s, Tile>) -> bool {
        let entity = map.tile(x as usize, y as usize);
        let tile = tiles.get(entity).unwrap();

        tile.is_block()
    }
}

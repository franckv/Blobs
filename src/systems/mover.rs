use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

use crate::components::{Direction, Action, Intent};

#[derive(Default)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Intent>,
        Read<'s, LazyUpdate>,
        Entities<'s>
    );

    fn run(&mut self, (mut transform, intents, update, entities): Self::SystemData) {
        for (transform, intent, entity) in (&mut transform, &intents, &entities).join() {
            match intent.action() {
                Action::Move(Direction::Up) => {
                    let y = transform.translation().y;
                    transform.set_translation_y(y + 16.);
                },
                Action::Move(Direction::Down) => {
                    let y = transform.translation().y;
                    transform.set_translation_y(y - 16.);
                },
                Action::Move(Direction::Left) => {
                    let x = transform.translation().x;
                    transform.set_translation_x(x - 16.);
                },
                Action::Move(Direction::Right) => {
                    let x = transform.translation().x;
                    transform.set_translation_x(x + 16.);
                },
                _ => ()
            }

            update.remove::<Intent>(entity);
        }
    }
}

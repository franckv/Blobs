use amethyst::ecs::{Entities, Join, ReadStorage, System};

use crate::components::Init;

#[derive(Default)]
pub struct InitSystem;

impl<'s> System<'s> for InitSystem {
    type SystemData = (
        ReadStorage<'s, Init>,
        Entities<'s>
    );

    fn run(&mut self, (init, entities): Self::SystemData) {
        for (_, entity) in (&init, &entities).join() {
            entities.delete(entity).unwrap();
        }
    }
}

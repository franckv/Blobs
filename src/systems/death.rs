use amethyst::ecs::{Entities, Join, ReadStorage, System};

use crate::components::Dead;

#[derive(Default)]
pub struct DeathSystem;

impl<'s> System<'s> for DeathSystem {
    type SystemData = (
        ReadStorage<'s, Dead>,
        Entities<'s>
    );

    fn run(&mut self, (dead, entities): Self::SystemData) {
        for (_, entity) in (&dead, &entities).join() {
            println!("Dead!");
            entities.delete(entity).unwrap();
        }
    }
}

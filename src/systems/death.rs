use amethyst::ecs::{Entities, Join, ReadStorage, System};

use crate::components::{Dead, Name};

#[derive(Default)]
pub struct DeathSystem;

impl<'s> System<'s> for DeathSystem {
    type SystemData = (
        ReadStorage<'s, Dead>,
        ReadStorage<'s, Name>,
        Entities<'s>
    );

    fn run(&mut self, (dead, name, entities): Self::SystemData) {
        for (_, entity) in (&dead, &entities).join() {
            let name = match name.get(entity) {
                Some(name) => name.name(),
                _ => "Entity"
            };
            println!("{} is dead !", name);
            entities.delete(entity).unwrap();
        }
    }
}

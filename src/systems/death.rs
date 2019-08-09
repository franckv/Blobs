use amethyst::ecs::{Entities, Join, ReadStorage, System, WriteExpect};

use super::utils;
use crate::components::{Dead, Name};
use crate::ui::MessageLog;

#[derive(Default)]
pub struct DeathSystem;

impl<'s> System<'s> for DeathSystem {
    type SystemData = (
        ReadStorage<'s, Dead>,
        ReadStorage<'s, Name>,
        WriteExpect<'s, MessageLog>,
        Entities<'s>
    );

    fn run(&mut self, (dead, name, mut logs, entities): Self::SystemData) {
        for (_, entity) in (&dead, &entities).join() {
            let name = utils::get_name(entity, "Entity", &name);
            logs.push(format!("{} is dead !", name));
            entities.delete(entity).unwrap();
        }
    }
}

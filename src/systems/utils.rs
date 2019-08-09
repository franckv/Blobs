use amethyst::ecs::{Entity, ReadStorage};

use crate::components::Name;

pub(crate) fn get_name<'s>(entity: Entity, default: &'s str,
                name_storage: &'s ReadStorage<'s, Name>) -> &'s str{
    match name_storage.get(entity) {
        Some(name) => name.name(),
        _ => default
    }
}

use crate::components::{Action, Direction, Intent};
use bevy::prelude::*;

pub fn move_sprite(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Intent)>
) {
    for (entity, mut transform, intent) in query.iter_mut() {
        let step_x = 16.;
        let step_y = 16.;

        if let Action::Move(dir) = intent.action() {
            let (dx, dy) = match dir {
                Direction::Up => (0., step_y),
                Direction::Down => (0., -step_y),
                Direction::Left => (-step_x, 0.),
                Direction::Right => (step_x, 0.)
            };

            transform.translation.x += dx;
            transform.translation.y += dy;
        }

        commands.entity(entity).remove::<Intent>();
    }
}

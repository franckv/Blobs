use bevy::prelude::*;

use crate::components::Player;
use crate::components::{Action, Direction, Intent};

pub fn read_input(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    for player in query.iter() {
        let action = {
            if keys.just_pressed(KeyCode::Z) {
                Action::Move(Direction::Up)
            } else if keys.just_pressed(KeyCode::S) {
                Action::Move(Direction::Down)
            } else if keys.just_pressed(KeyCode::Q) {
                Action::Move(Direction::Left)
            } else if keys.just_pressed(KeyCode::D) {
                Action::Move(Direction::Right)
            } else {
                Action::None
            }
        };

        if let Action::Move(_) = action {
            commands.entity(player).insert(Intent::new(action));
        }
    }
}

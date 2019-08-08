use amethyst::core::{Hidden, Transform};
use amethyst::ecs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::{Action, Direction, Intent, Mob, Player};

#[derive(Default)]
pub struct AttackSystem;

impl<'s> System<'s> for AttackSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Hidden>,
        WriteStorage<'s, Intent>,
        ReadStorage<'s, Mob>,
        ReadStorage<'s, Player>,
        Entities<'s>
    );

    fn run(&mut self, (transform, hidden, mut intents, mob, player, entities):
           Self::SystemData) {

        let mut remove_intent = None;

        for (player_trans, intent, player_entity) in (
            &transform, &intents, &entities).join() {
            let (x, y) = (
                player_trans.translation().x, player_trans.translation().y);

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
                    }
                };

                for (mob_trans, _, _, _) in (
                    &transform, !&hidden, &mob, &entities).join() {
                    let (mob_x, mob_y) = (
                        mob_trans.translation().x, mob_trans.translation().y);

                    if x + dx == mob_x && y + dy == mob_y {
                        println!("Attack!");
                        remove_intent = Some(player_entity);
                        break;
                    }
                }
            }
        }

        if let Some(entity) = remove_intent {
            intents.remove(entity);
        }
    }
}

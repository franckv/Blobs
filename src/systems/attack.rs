use amethyst::core::{Hidden, Transform};
use amethyst::ecs::{Entity, Entities, Join, ReadStorage, System, WriteStorage};

use super::utils;
use crate::components::{
    Action, Dead, Direction, Fighter,
    Health, Intent, Mob, Name, Player};

#[derive(Default)]
pub struct AttackSystem;

impl<'s> System<'s> for AttackSystem {
    type SystemData = (
        WriteStorage<'s, Dead>,
        ReadStorage<'s, Fighter>,
        WriteStorage<'s, Health>,
        ReadStorage<'s, Hidden>,
        WriteStorage<'s, Intent>,
        ReadStorage<'s, Mob>,
        ReadStorage<'s, Name>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        Entities<'s>
    );

    fn run(&mut self, (mut dead, fighter, mut health, hidden,
                       mut intents, mob, name, player, transform,
                       entities): Self::SystemData) {
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

                for (mob_trans, _, _, mob_entity) in (
                    &transform, !&hidden, &mob, &entities).join() {
                    let (mob_x, mob_y) = (
                        mob_trans.translation().x, mob_trans.translation().y);

                    if x + dx == mob_x && y + dy == mob_y {
                        remove_intent = Some(player_entity);
                        attack(&player_entity, &mob_entity,
                               &fighter, &mut health, &name, &mut dead);
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

fn attack<'s>(attacker: &Entity, defender: &Entity,
              fighter: &ReadStorage<'s, Fighter>,
              health: &mut WriteStorage<'s, Health>,
              name: &ReadStorage<'s, Name>,
              dead: &mut WriteStorage<'s, Dead>) {
    if let (Some(attack), Some(defense)) =
        (fighter.get(*attacker), fighter.get(*defender)) {
            println!("Attack!");
            let damage_d = attack.attack() - defense.defense();
            let damage_a = defense.attack() - attack.defense();

            if let Some(health) = health.get_mut(*defender) {
                health.damage(damage_d);
                let name = utils::get_name(*attacker, "Defender", &name);
                println!("{} take {} damage ({}/{})",
                name, damage_d, health.current(), health.max());
                if health.current() == 0 {
                    dead.insert(*defender, Dead).unwrap();
                }
            }

            if let Some(health) = health.get_mut(*attacker) {
                health.damage(damage_a);
                let name = utils::get_name(*attacker, "Attacker", &name);
                println!("{} take {} damage ({}/{})",
                name, damage_a, health.current(), health.max());
                if health.current() == 0 {
                    dead.insert(*attacker, Dead).unwrap();
                }
            }
        }
}

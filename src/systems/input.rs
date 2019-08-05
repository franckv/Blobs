use amethyst::core::shrev::{EventChannel, ReaderId};
use amethyst::ecs::{Entities, Join, Read, ReadStorage, Resources, System, SystemData, WriteStorage};
use amethyst::input::{InputEvent, InputEvent::ActionPressed, StringBindings};

use crate::components::{Direction, Action, Intent, Player};

#[derive(Default)]
pub struct InputSystem {
    reader: Option<ReaderId<InputEvent<StringBindings>>>
}

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        WriteStorage<'s, Intent>,
        ReadStorage<'s, Player>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
        Entities<'s>
    );

    fn run(&mut self, (mut intent, player, channel, entities): Self::SystemData) {
        let action = {
            let mut action = Action::None;

            for event in channel.read(self.reader.as_mut().unwrap()) {
                debug!("Event: {:?}", event);
                if let ActionPressed(key) = event {
                    match key.as_ref() {
                        "up" => {
                            action = Action::Move(Direction::Up)
                        },
                        "down" => {
                            action = Action::Move(Direction::Down)
                        },
                        "left" => {
                            action = Action::Move(Direction::Left)
                        },
                        "right" => {
                            action = Action::Move(Direction::Right)
                        },
                        _ => ()
                    }
                }
            }

            action
        };

        if action != Action::None {
            for (_, entity) in (&player, &entities).join() {
                intent.insert(entity, Intent::new(action)).unwrap();
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<InputEvent<StringBindings>>>().register_reader());
    }
}

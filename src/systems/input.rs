use amethyst::core::Transform;
use amethyst::core::shrev::{EventChannel, ReaderId};
use amethyst::ecs::{Join, Read, ReadStorage, Resources, System, SystemData, WriteStorage};
use amethyst::input::{InputEvent, InputEvent::ActionPressed, StringBindings};

use crate::blobs::Player;

#[derive(Default)]
pub struct InputSystem {
    reader: Option<ReaderId<InputEvent<StringBindings>>>
}

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>
    );

    fn run(&mut self, (mut transform, player, channel): Self::SystemData) {
        let (hmove, vmove) = {
            let mut hmove = 0.;
            let mut vmove = 0.;

            for event in channel.read(self.reader.as_mut().unwrap()) {
                debug!("Event: {:?}", event);
                if let ActionPressed(key) = event {
                    match key.as_ref() {
                        "up" => {
                            vmove = 1.;
                        },
                        "down" => {
                            vmove = -1.;
                        },
                        "left" => {
                            hmove = -1.;
                        },
                        "right" => {
                            hmove = 1.;
                        },
                        _ => ()
                    }
                }
            }

            (hmove, vmove)
        };

        for (transform, _) in (&mut transform, &player).join() {
            if hmove != 0.0 {
                let x = transform.translation().x;
                transform.set_translation_x(x + hmove * 16.);
            }

            if vmove != 0.0 {
                let y = transform.translation().y;
                transform.set_translation_y(y + vmove * 16.);
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<InputEvent<StringBindings>>>().register_reader());
    }
}

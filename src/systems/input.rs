use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::blobs::Player;

pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut transform, player, input): Self::SystemData) {
        for (transform, _) in (&mut transform, &player).join() {
            if let Some(hmove) = input.axis_value("left_right") {
                if hmove != 0.0 {
                    println!("move lr: {}", hmove);
                    let x = transform.translation().x;
                    transform.set_translation_x(x + hmove * 16.);
                }
            }

            if let Some(vmove) = input.axis_value("up_down") {
                if vmove != 0.0 {
                    println!("move ud: {}", vmove);
                    let y = transform.translation().y;
                    transform.set_translation_y(y + vmove * 16.);
                }
            }
        }
    }
}

use amethyst::ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::ui::UiText;

use crate::components::{Health, Player};
use crate::ui::{Hp, MessageLog};

#[derive(Default)]
pub struct UiSystem;

impl<'s> System<'s> for UiSystem {
    type SystemData = (
        ReadStorage<'s, Health>,
        ReadStorage<'s, Player>,
        ReadExpect<'s, Hp>,
        ReadExpect<'s, MessageLog>,
        WriteStorage<'s, UiText>
    );

    fn run(&mut self, (health, player, hp, logs, mut ui_text): Self::SystemData) {
        for (health, _) in (&health, &player).join() {
            let label = ui_text.get_mut(hp.label()).unwrap();
            label.text = format!("HP: {}/{}", health.current(), health.max());
        }

        let label = ui_text.get_mut(logs.label()).unwrap();
        label.text = logs.to_string();
    }
}

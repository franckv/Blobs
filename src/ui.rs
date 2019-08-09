use amethyst::ecs::Entity;

pub struct Hp {
    label: Entity
}

impl Hp {
    pub fn new(label: Entity) -> Self {
        Hp {
            label
        }
    }

    pub fn label(&self) -> Entity {
        self.label
    }
}

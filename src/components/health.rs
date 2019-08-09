use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone, Debug)]
pub struct Health {
    current: u32,
    max: u32
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}

impl Health {
    pub fn new(max: u32) -> Self {
        Health {
            current: max,
            max
        }
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn damage(&mut self, damage: u32) {
        if damage > self.current {
            self.current = 0
        } else {
            self.current = self.current - damage
        }
    }

    pub fn heal(&mut self, health: u32) {
        self.current = self.max.min(self.current + health);
    }
}

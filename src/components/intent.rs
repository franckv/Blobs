use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    None,
    Move(Direction)
}

#[derive(Debug)]
pub struct Intent {
    action: Action
}

impl Intent {
    pub fn action(&self) -> Action {
        self.action
    }

    pub fn new(action: Action) -> Self {
        Intent {
            action
        }
    }
}

impl Component for Intent {
    type Storage = DenseVecStorage<Self>;
}



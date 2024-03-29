use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone, Debug)]
pub struct Name {
    name: String
}

impl Component for Name {
    type Storage = DenseVecStorage<Self>;
}

impl Name {
    pub fn new(name: &str) -> Self {
        Name {
            name: name.to_string()
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

use std::collections::VecDeque;

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

pub struct MessageLog {
    max_size: usize,
    logs: VecDeque<String>,
    label: Entity
}

impl MessageLog {
    pub fn new(label: Entity, max_size: usize) -> Self {
        MessageLog {
            max_size,
            logs: VecDeque::new(),
            label
        }
    }

    pub fn label(&self) -> Entity {
        self.label
    }

    pub fn push(&mut self, line: String) {
        if self.logs.len() >= self.max_size {
            self.logs.pop_front();
        }
        self.logs.push_back(line);
    }

    pub fn to_string(&self) -> String {
        let mut log = String::new();
        for l in &self.logs {
            log = format!("{}\n{}", log, l);
        }

        log
    }
}

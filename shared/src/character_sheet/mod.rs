mod resource;

use resource::{Damage, Resource};

pub struct CharacterSheet {
    resources: Vec<Box<dyn Resource>>,
}

pub struct Health {
    current: i32,
    max: i32,
}

impl Health {}

impl Resource for Health {
    fn current(&self) -> i32 {
        self.current
    }
    fn max(&self) -> i32 {
        self.max
    }
}

impl Damage for Health {
    fn damage(mut self, amount: i32) {
        self.current -= amount;
    }
}

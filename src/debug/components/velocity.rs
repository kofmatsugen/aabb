use crate::types::Vector;
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Velocity {
    pub velocity: Vector,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity {
            velocity: Vector::new(x, y),
        }
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

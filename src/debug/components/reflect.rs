use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Reflect;

impl Component for Reflect {
    type Storage = NullStorage<Self>;
}

use crate::debug::components::Velocity;
use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

pub struct VelocitySystem;

impl VelocitySystem {
    pub fn new() -> Self {
        VelocitySystem
    }
}

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (ReadStorage<'s, Velocity>, WriteStorage<'s, Transform>);

    fn run(&mut self, (velocities, mut transforms): Self::SystemData) {
        for (v, t) in (&velocities, &mut transforms).join() {
            t.translation_mut().x += v.velocity.x;
            t.translation_mut().y += v.velocity.y;
        }
    }
}

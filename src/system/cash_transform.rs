use crate::LastTransform;
use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

pub(crate) struct CashTransformSystem;

impl CashTransformSystem {
    pub(crate) fn new() -> Self {
        CashTransformSystem
    }
}

impl<'s> System<'s> for CashTransformSystem {
    type SystemData = (ReadStorage<'s, Transform>, WriteStorage<'s, LastTransform>);

    fn run(&mut self, (transforms, mut lasts): Self::SystemData) {
        for (t, l) in (&transforms, &mut lasts).join() {
            l.set_last(t.clone());
        }
    }
}

use crate::LastTransform;
use amethyst::{
    core::Transform,
    ecs::{rayon::iter::ParallelIterator, ParJoin, ReadStorage, System, WriteStorage},
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
        (&transforms, &mut lasts)
            .par_join()
            .for_each(cash_transform);
    }
}

fn cash_transform((transform, cashed): (&Transform, &mut LastTransform)) {
    cashed.set_last(transform.clone());
}

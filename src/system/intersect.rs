use crate::{types::Vector, Collisions, ContactEvent, ContactEventChannel, LastTransform};
use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, Write},
};
use ncollide2d::{math::Isometry, query};
use std::marker::PhantomData;

pub(crate) struct IntersectSystem<T>
where
    T: 'static + Send + Sync,
{
    paramater: PhantomData<T>,
}

impl<T> IntersectSystem<T>
where
    T: 'static + Send + Sync,
{
    pub(crate) fn new() -> Self {
        IntersectSystem {
            paramater: PhantomData,
        }
    }
}

impl<'s, T> System<'s> for IntersectSystem<T>
where
    T: 'static + Send + Sync + Copy,
{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collisions<T>>,
        ReadStorage<'s, Transform>,
        Write<'s, ContactEventChannel<T>>,
        ReadStorage<'s, LastTransform>,
    );

    fn run(&mut self, (entities, collisions, transforms, mut channel, lasts): Self::SystemData) {
        for (entity1, t1, c1, _last1) in (&*entities, &transforms, &collisions, &lasts).join() {
            for (entity2, t2, c2, _last2) in (&*entities, &transforms, &collisions, &lasts)
                .join()
                .filter(|(e, _, _, _)| *e > entity1)
            {
                for c1 in &c1.collisions {
                    for c2 in &c2.collisions {
                        let position1 = Vector::new(t1.translation().x, t1.translation().y);
                        let position2 = Vector::new(t2.translation().x, t2.translation().y);
                        let position1 = Isometry::new(position1 + c1.position, 0.);
                        let position2 = Isometry::new(position2 + c2.position, 0.);

                        if let Some(contact) =
                            query::contact(&position1, &c1.aabb, &position2, &c2.aabb, 0.)
                        {
                            channel.single_write(ContactEvent {
                                contact,
                                entity1,
                                entity2,
                                args1: c1.paramater,
                                args2: c2.paramater,
                            });
                        }
                    }
                }
            }
        }
    }
}

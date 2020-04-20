use crate::{
    event::{ContactEvent, ContactEventChannel},
    traits::CollisionObject,
    Collisions,
};
use amethyst::ecs::{Entities, Join, ReadStorage, System, Write};
use itertools::iproduct;
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
    T: 'static + Send + Sync + Copy + CollisionObject<'s>,
{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collisions<T>>,
        Write<'s, ContactEventChannel<T>>,
        T::SystemData,
    );

    fn run(&mut self, (entities, collisions, mut channel, paramater_data): Self::SystemData) {
        let obj1_iter = (&*entities, &collisions).join();
        let obj2_iter = (&*entities, &collisions).join();
        for ((entity1, c1), (entity2, c2)) in
            iproduct!(obj1_iter, obj2_iter).filter(|((e1, _), (e2, _))| e1 < e2)
        {
            let collision1 = c1.collisions.iter();
            let collision2 = c2.collisions.iter();
            for (c1, c2) in iproduct!(collision1, collision2).filter(|(c1, c2)| {
                T::pair_filter(
                    entity1,
                    &c1.paramater,
                    entity2,
                    &c2.paramater,
                    &paramater_data,
                )
            }) {
                let position1 = Isometry::new(c1.position, 0.);
                let position2 = Isometry::new(c2.position, 0.);

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

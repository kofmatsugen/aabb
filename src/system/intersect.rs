use crate::{
    event::{ContactEvent, ContactEventChannel},
    traits::CollisionObject,
    types::{Aabb, Vector},
    Collisions,
};
use amethyst::ecs::{Entities, Join, ReadStorage, System, Write};
use itertools::iproduct;
use std::marker::PhantomData;

pub(crate) struct IntersectSystem<T> {
    paramater: PhantomData<T>,
}

impl<T> IntersectSystem<T> {
    pub(crate) fn new() -> Self {
        IntersectSystem {
            paramater: PhantomData,
        }
    }
}

impl<'s, T> System<'s> for IntersectSystem<T>
where
    T: CollisionObject,
{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collisions<T>>,
        Write<'s, ContactEventChannel<T>>,
    );

    fn run(&mut self, (entities, collisions, mut channel): Self::SystemData) {
        let joined1 = (&*entities, &collisions).join();
        let joined2 = (&*entities, &collisions).join();
        for ((e1, c1), (e2, c2)) in iproduct!(joined1, joined2).filter(|((e1, _), (e2, _))| e1 < e2)
        {
            for c1 in &c1.collisions {
                for c2 in &c2.collisions {
                    if T::pair_filter(&c1.paramater, &c2.paramater) == false {
                        continue;
                    }
                    if let Some(hit) =
                        intersect_aabb(&c1.aabb, &c1.position, &c2.aabb, &c2.position)
                    {
                        channel.single_write(ContactEvent {
                            entity1: e1,
                            entity2: e2,
                            args1: c1.paramater.clone(),
                            args2: c2.paramater.clone(),
                            point: hit.point,
                            delta: hit.delta,
                            hit_center: (c1.position + c2.position) / 2.,
                        });
                    }
                }
            }
        }
    }
}

struct Hit {
    point: Vector,
    delta: Vector,
}

fn intersect_aabb(aabb1: &Aabb, pos1: &Vector, aabb2: &Aabb, pos2: &Vector) -> Option<Hit> {
    let pivot_distance = pos2 - pos1;
    let diff_half = aabb1.half_extents() + aabb2.half_extents() - pivot_distance.abs();

    if diff_half.x <= 0. || diff_half.y <= 0. {
        return None;
    }

    if diff_half.x < diff_half.y {
        let sign_x = pivot_distance.x.signum();
        let delta = Vector::new(diff_half.x * sign_x, 0.);
        let point = Vector::new(pos1.x + aabb1.half_extents().x * sign_x, pos2.y);

        Some(Hit { delta, point })
    } else {
        let sign_y = pivot_distance.y.signum();
        let delta = Vector::new(0., diff_half.y * sign_y);
        let point = Vector::new(pos2.x, pos1.y + aabb1.half_extents().y * sign_y);

        Some(Hit { delta, point })
    }
}

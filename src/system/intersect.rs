use crate::{
    event::{ContactEvent, ContactEventChannel},
    traits::CollisionObject,
    Collisions,
};
use amethyst::ecs::{Entities, Entity, Join, ReadStorage, System, Write};
use ncollide2d::{
    query::{Contact, TrackedContact},
    shape::ShapeHandle,
    {
        math::{Isometry, Point},
        pipeline::{
            object::{CollisionGroups, GeometricQueryType},
            world::CollisionWorld,
        },
    },
};
use std::marker::PhantomData;

pub(crate) struct IntersectSystem<T>
where
    T: CollisionObject,
{
    paramater: PhantomData<T>,
    world: CollisionWorld<f32, (Entity, T)>,
}

impl<T> IntersectSystem<T>
where
    T: CollisionObject,
{
    pub(crate) fn new() -> Self {
        let mut world = CollisionWorld::new(0.);
        // フィルタを設定
        world.set_broad_phase_pair_filter(Some(T::PairFilter::default()));
        IntersectSystem {
            paramater: PhantomData,
            world,
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
        // 登録されている判定をすべて登録
        // 判定はこのフレーム上のものだけで行うので，後で削除する
        let mut registered_handles = vec![];

        for (e, collision) in (&*entities, &collisions).join() {
            for c in &collision.collisions {
                let position = Isometry::new(c.position, 0.);
                let shape = ShapeHandle::new(c.aabb.clone());
                let group = CollisionGroups::new();
                let query_type = GeometricQueryType::Contacts(0., 0.);
                let data = (e, c.paramater.clone());

                let (handle, _object) = self.world.add(position, shape, group, query_type, data);
                registered_handles.push(handle);
            }
        }

        self.world.update();

        // ここで処理した判定をすべて取得する
        for (h1, h2, _, manifold) in self.world.contact_pairs(false) {
            match (
                self.world.collision_object(h1),
                self.world.collision_object(h2),
            ) {
                (Some(o1), Some(o2)) => {
                    let (entity1, args1) = o1.data();
                    let (entity2, args2) = o2.data();

                    if let Some(deepest) = manifold.deepest_contact() {
                        channel.single_write(ContactEvent {
                            entity1: *entity1,
                            entity2: *entity2,
                            args1: args1.clone(),
                            args2: args2.clone(),
                            contact: deepest.contact,
                        });
                    }
                }
                _ => {}
            }
        }

        // 次のフレームに持ち越さないためにすべて削除
        self.world.remove(&registered_handles[..]);
    }
}

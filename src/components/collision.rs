use crate::types::{Aabb, Vector};
use amethyst::ecs::{Component, DenseVecStorage};
use ncollide2d::pipeline::object::CollisionObjectSlabHandle;
use std::collections::BTreeMap;

pub(crate) struct Collision<T> {
    pub(crate) paramater: T,
    pub(crate) position: Vector,
    pub(crate) aabb: Aabb,
    pub(crate) handle: Option<CollisionObjectSlabHandle>,
    pub(crate) dirty: bool, // 判定は常に更新されるはず．更新されないものは削除
}

pub struct Collisions<T> {
    pub(crate) collisions: BTreeMap<u64, Collision<T>>,
    pub(crate) removed_objects: Vec<CollisionObjectSlabHandle>,
}

impl<T> Collisions<T> {
    pub fn new() -> Self {
        Collisions {
            collisions: BTreeMap::new(),
            removed_objects: Vec::with_capacity(128),
        }
    }

    pub fn update_aabb(
        &mut self,
        id: u64,
        (x, y): (f32, f32),
        width: f32,
        height: f32,
        paramater: T,
    ) -> &mut Self {
        if let Some(collision) = self.collisions.get_mut(&id) {
            collision.position = Vector::new(x, y);
            collision.aabb = Aabb::new(Vector::new(width.abs() / 2., height.abs() / 2.));
            collision.paramater = paramater;
            collision.dirty = true;
        } else {
            self.collisions.insert(
                id,
                Collision {
                    position: Vector::new(x, y),
                    aabb: Aabb::new(Vector::new(width.abs() / 2., height.abs() / 2.)),
                    paramater: paramater,
                    handle: None,
                    dirty: true,
                },
            );
        }
        self
    }

    fn remove(&mut self, id: u64) {
        if let Some(Collision {
            handle: Some(handle),
            ..
        }) = self.collisions.remove(&id)
        {
            self.removed_objects.push(handle);
        }
    }

    fn remove_iter<I>(&mut self, ids: I)
    where
        I: Iterator<Item = u64>,
    {
        for id in ids {
            self.remove(id);
        }
    }

    pub(crate) fn remove_non_dirty(&mut self) {
        let removed = self
            .collisions
            .iter()
            .filter_map(|(id, c)| if c.dirty == false { Some(*id) } else { None })
            .collect::<Vec<_>>();

        self.remove_iter(removed.into_iter());
    }
}

impl<T> Component for Collisions<T>
where
    T: 'static + Send + Sync,
{
    type Storage = DenseVecStorage<Self>;
}

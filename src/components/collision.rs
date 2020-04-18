use crate::types::{Aabb, Vector};
use amethyst::ecs::{Component, DenseVecStorage};

pub(crate) struct Collision<T> {
    pub(crate) paramater: T,
    pub(crate) position: Vector,
    pub(crate) aabb: Aabb,
}

pub struct Collisions<T> {
    pub(crate) collisions: Vec<Collision<T>>,
}

impl<T> Collisions<T> {
    pub fn new() -> Self {
        Collisions {
            collisions: Vec::with_capacity(128),
        }
    }

    pub fn clear(&mut self) {
        self.collisions.clear();
    }

    pub fn add_aabb(
        &mut self,
        (x, y): (f32, f32),
        width: f32,
        height: f32,
        paramater: T,
    ) -> &mut Self {
        self.collisions.push(Collision {
            position: Vector::new(x, y),
            aabb: Aabb::new(Vector::new(width.abs() / 2., height.abs() / 2.)),
            paramater: paramater,
        });
        self
    }
}

impl<T> Component for Collisions<T>
where
    T: 'static + Send + Sync,
{
    type Storage = DenseVecStorage<Self>;
}

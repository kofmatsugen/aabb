use crate::types::{Aabb, Vector};
use amethyst::ecs::{Component, DenseVecStorage};

pub(crate) struct Collision<T> {
    pub(crate) paramater: Option<T>,
    pub(crate) position: Vector,
    pub(crate) aabb: Aabb,
}

pub struct Collisions<T> {
    pub(crate) collisions: Vec<Collision<T>>,
}

impl<T> Collisions<T> {
    pub fn new() -> Self {
        Collisions { collisions: vec![] }
    }

    pub fn add_aabb<P>(
        &mut self,
        (x, y): (f32, f32),
        width: f32,
        height: f32,
        paramater: P,
    ) -> &mut Self
    where
        P: Into<Option<T>>,
    {
        self.collisions.push(Collision {
            position: Vector::new(x, y),
            aabb: Aabb::new(Vector::new(width / 2., height / 2.)),
            paramater: paramater.into(),
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

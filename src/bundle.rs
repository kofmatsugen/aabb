#[cfg(feature = "debug")]
use crate::debug::{system::CollisionViewSystem, traits::CollisionColor};
use crate::{system::IntersectSystem, traits::CollisionObject};
use amethyst::{
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
};
use std::marker::PhantomData;

pub struct AabbCollisionBundle<T>
where
    T: 'static + Send + Sync,
{
    paramater: PhantomData<T>,
}

impl<T> AabbCollisionBundle<T>
where
    T: 'static + Send + Sync,
{
    pub fn new() -> Self {
        AabbCollisionBundle {
            paramater: PhantomData,
        }
    }
}
#[cfg(not(feature = "debug"))]
impl<'a, 'b, T> SystemBundle<'a, 'b> for AabbCollisionBundle<T>
where
    T: CollisionObject,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        builder.add(IntersectSystem::<T>::new(), "intersect_aabb", &[]);
        Ok(())
    }
}

#[cfg(feature = "debug")]
impl<'a, 'b, T> SystemBundle<'a, 'b> for AabbCollisionBundle<T>
where
    T: CollisionObject + CollisionColor,
{
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        builder.add(IntersectSystem::<T>::new(), "intersect_aabb", &[]);

        builder.add(
            CollisionViewSystem::<T>::new(world),
            "collision_view",
            &["intersect_aabb"],
        );

        Ok(())
    }
}

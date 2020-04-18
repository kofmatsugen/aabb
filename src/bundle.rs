#[cfg(feature = "debug")]
use crate::debug::system::{CollisionViewSystem, ReflectSystem, VelocitySystem};
use crate::{
    system::{CashTransformSystem, IntersectSystem},
    traits::CollisionObject,
};
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

impl<'a, 'b, T> SystemBundle<'a, 'b> for AabbCollisionBundle<T>
where
    T: 'static + Send + Sync + Copy + for<'c> CollisionObject<'c>,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        #[cfg(feature = "debug")]
        builder.add(VelocitySystem::new(), "velocity_system", &[]);

        builder.add(IntersectSystem::<T>::new(), "intersect_aabb", &[]);

        #[cfg(feature = "debug")]
        builder.add(
            ReflectSystem::<T>::new(),
            "collision_reflect",
            &["transform_system", "intersect_aabb"],
        );

        #[cfg(feature = "debug")]
        builder.add(
            CollisionViewSystem::<T>::new(_world),
            "collision_view",
            &["intersect_aabb"],
        );

        builder.add(
            CashTransformSystem::new(),
            "cash_transform",
            &["transform_system", "intersect_aabb"],
        );

        Ok(())
    }
}

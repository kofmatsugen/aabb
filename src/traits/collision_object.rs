use amethyst::ecs::Entity;
use ncollide2d::pipeline::{
    broad_phase::BroadPhasePairFilter,
    object::{CollisionObject as NCollideObject, CollisionObjectSlabHandle},
};
use std::marker::PhantomData;

// 判定処理を行うためのトレイトエイリアス
// ペアフィルターになる型を実装していればOK
pub trait CollisionObject: 'static + Send + Sync + Clone {
    fn pair_filter(paramater: &Self, paramater: &Self) -> bool;
}

pub struct PairFilter<T> {
    marker: PhantomData<T>,
}

impl<T> BroadPhasePairFilter<f32, NCollideObject<f32, (Entity, T)>, CollisionObjectSlabHandle>
    for PairFilter<T>
where
    T: CollisionObject,
{
    fn is_pair_valid(
        &self,
        b1: &NCollideObject<f32, (Entity, T)>,
        b2: &NCollideObject<f32, (Entity, T)>,
        _: CollisionObjectSlabHandle,
        _: CollisionObjectSlabHandle,
    ) -> bool {
        let (_, args1) = b1.data();
        let (_, args2) = b2.data();

        T::pair_filter(args1, args2)
    }
}

impl<T> Default for PairFilter<T> {
    fn default() -> Self {
        PairFilter {
            marker: PhantomData,
        }
    }
}

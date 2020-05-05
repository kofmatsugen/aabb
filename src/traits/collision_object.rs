use amethyst::ecs::Entity;
use ncollide2d::pipeline::{
    broad_phase::BroadPhasePairFilter,
    object::{CollisionObject as NCollideObject, CollisionObjectSlabHandle},
};

// 判定処理を行うためのトレイトエイリアス
// ペアフィルターになる型を実装していればOK
pub trait CollisionObject: 'static + Send + Sync + Clone {
    type PairFilter: Default
        + BroadPhasePairFilter<f32, NCollideObject<f32, (Entity, Self)>, CollisionObjectSlabHandle>;
}

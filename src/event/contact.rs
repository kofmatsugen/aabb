use crate::types::Vector;
use amethyst::{ecs::Entity, shrev::EventChannel};

pub type ContactEventChannel<T> = EventChannel<ContactEvent<T>>;

pub struct ContactEvent<T> {
    pub entity1: Entity,
    pub entity2: Entity,
    pub args1: T,           // 衝突時パラメータ
    pub args2: T,           // 衝突時パラメータ
    pub point: Vector,      // 衝突点
    pub delta: Vector,      // 反発ベクトル
    pub hit_center: Vector, // 衝突した判定の中間点
}

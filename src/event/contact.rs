use amethyst::{ecs::Entity, shrev::EventChannel};
use ncollide2d::query::Contact;

pub type ContactEventChannel<T> = EventChannel<ContactEvent<T>>;

pub struct ContactEvent<T> {
    pub entity1: Entity,
    pub entity2: Entity,
    pub args1: Option<T>, // 衝突時パラメータ
    pub args2: Option<T>, // 衝突時パラメータ
    pub contact: Contact<f32>,
}

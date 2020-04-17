use amethyst::core::math::Vector2;
use ncollide2d::{query::Contact as ContactGenerics, shape::Cuboid};

pub(crate) type Vector = Vector2<f32>;
pub(crate) type Contact = ContactGenerics<f32>;
pub(crate) type Aabb = Cuboid<f32>;

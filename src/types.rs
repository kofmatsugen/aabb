use amethyst::core::math::Vector2;
use ncollide2d::{query::Contact as ContactGenerics, shape::Cuboid};

pub type Vector = Vector2<f32>;
pub type Contact = ContactGenerics<f32>;
pub type Aabb = Cuboid<f32>;

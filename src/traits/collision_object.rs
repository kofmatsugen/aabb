use crate::traits::PairFilter;

// 判定処理を行うためのトレイトエイリアス
pub trait CollisionObject<'s>: PairFilter<'s> {}

impl<'s, T> CollisionObject<'s> for T where T: PairFilter<'s> {}

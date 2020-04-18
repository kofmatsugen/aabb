use amethyst::ecs::{Entity, SystemData};

pub trait PairFilter<'s> {
    type SystemData: SystemData<'s>;

    // 判定処理を行うフィルタ
    fn pair_filter(
        _entity1: Entity,
        _paramater1: &Self,
        _entity2: Entity,
        _paramater2: &Self,
        _data: &Self::SystemData,
    ) -> bool {
        //デフォルトは特に制限なし
        true
    }
}

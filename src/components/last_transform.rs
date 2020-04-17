use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage},
};

#[derive(Default)]
pub struct LastTransform {
    last: Option<Transform>,
}

impl LastTransform {
    pub fn set_last(&mut self, tranform: Transform) {
        self.last = tranform.into();
    }

    pub fn last(&self) -> Option<&Transform> {
        self.last.as_ref()
    }
}

impl Component for LastTransform {
    type Storage = DenseVecStorage<Self>;
}

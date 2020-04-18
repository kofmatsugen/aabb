pub trait CollisionColor {
    fn collision_color(&self) -> (f32, f32, f32, f32) {
        (1., 0., 0., 1.)
    }
}

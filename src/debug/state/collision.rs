use crate::{
    debug::components::{Reflect, Velocity},
    Collisions, LastTransform,
};
use amethyst::{
    core::Transform,
    ecs::{Builder, Component, Join, NullStorage, Read, ReadStorage, WorldExt, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{camera::Camera, ActiveCamera},
    window::ScreenDimensions,
    GameData, SimpleState, SimpleTrans, StateData,
};

#[derive(Default)]
struct Tag;
impl Component for Tag {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct CollisionState {
    width: f32,
    height: f32,
}
impl SimpleState for CollisionState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        world.register::<Tag>();
        let mut collisions = Collisions::<()>::new();
        let transform = Transform::default();

        collisions.add_aabb((0., 0.), 100., 100., None);

        world
            .create_entity()
            .with(transform)
            .with(collisions)
            .with(Tag)
            .with(LastTransform::default())
            .build();

        let mut collisions = Collisions::<()>::new();
        let transform = Transform::default();
        collisions.add_aabb((-45., -45.), 100., 100., None);
        world
            .create_entity()
            .with(transform)
            .with(collisions)
            .with(Reflect)
            .with(Velocity::new(10., 10.))
            .with(LastTransform::default())
            .build();

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };
        self.width = width;
        self.height = height;

        let mut camera_transform = Transform::default();
        camera_transform.set_translation_z(1024.0);

        let camera = world
            .create_entity()
            .with(camera_transform)
            .with(Camera::standard_2d(width, height))
            .build();

        world.insert(ActiveCamera {
            entity: Some(camera),
        });

        // 壁
        let transforms = vec![
            (-width / 2., 0., 10., height),
            (width / 2., 0., 10., height),
            (0., -height / 2., width, 10.),
            (0., height / 2., width, 10.),
        ];

        for (x, y, w, h) in transforms {
            let mut collisions = Collisions::<()>::new();
            let mut transform = Transform::default();
            transform.set_translation_xyz(x, y, 0.);
            collisions.add_aabb((0., 0.), w, h, None);
            world
                .create_entity()
                .with(transform)
                .with(collisions)
                .with(LastTransform::default())
                .build();
        }

        log::info!("collision state initialize");
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        let world = &mut data.world;
        world.exec(
            |(tags, mut transforms, handler): (
                ReadStorage<Tag>,
                WriteStorage<Transform>,
                Read<InputHandler<StringBindings>>,
            )| {
                if let Some((x, y)) = handler.mouse_position() {
                    for (_, t) in (&tags, &mut transforms).join() {
                        t.set_translation_x(x - self.width / 2.);
                        t.set_translation_y(-y + self.height / 2.);
                    }
                }
            },
        );
        SimpleTrans::None
    }
}

use crate::{
    debug::components::{Reflect, Velocity},
    types::Contact,
    ContactEvent, ContactEventChannel,
};
use amethyst::{
    core::Transform,
    ecs::{ReadStorage, ReaderId, System, Write, WriteStorage},
};
use std::marker::PhantomData;

pub struct ReflectSystem<T>
where
    T: 'static + Send + Sync,
{
    paramater: PhantomData<T>,
    reader: Option<ReaderId<ContactEvent<T>>>,
}

impl<T> ReflectSystem<T>
where
    T: 'static + Send + Sync,
{
    pub fn new() -> Self {
        ReflectSystem {
            paramater: PhantomData,
            reader: None,
        }
    }
}

impl<'s, T> System<'s> for ReflectSystem<T>
where
    T: 'static + Send + Sync,
{
    type SystemData = (
        Write<'s, ContactEventChannel<T>>,
        ReadStorage<'s, Reflect>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut channel, reflects, mut velocties, mut transforms): Self::SystemData) {
        if self.reader.is_none() == true {
            self.reader = channel.register_reader().into();
        }

        for ContactEvent {
            entity1,
            entity2,
            contact: Contact { normal, depth, .. },
            ..
        } in channel.read(self.reader.as_mut().unwrap())
        {
            let delta = normal.into_inner() * *depth;
            if let (Some(_), Some(v), Some(t)) = (
                reflects.get(*entity1),
                velocties.get_mut(*entity1),
                transforms.get_mut(*entity1),
            ) {
                t.translation_mut().x -= delta.x;
                t.translation_mut().y -= delta.y;
                v.velocity.x *= if -delta.x * v.velocity.x < 0. {
                    -1.
                } else {
                    1.
                };
                v.velocity.y *= if -delta.y * v.velocity.y < 0. {
                    -1.
                } else {
                    1.
                };
            }
            if let (Some(_), Some(v), Some(t)) = (
                reflects.get(*entity2),
                velocties.get_mut(*entity2),
                transforms.get_mut(*entity2),
            ) {
                t.translation_mut().x += delta.x;
                t.translation_mut().y += delta.y;
                v.velocity.x *= if delta.x * v.velocity.x < 0. { -1. } else { 1. };
                v.velocity.y *= if delta.y * v.velocity.y < 0. { -1. } else { 1. };
            }
        }
    }
}

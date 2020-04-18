use crate::{
    debug::traits::CollisionColor,
    event::{ContactEvent, ContactEventChannel},
    types::{Aabb, Vector},
    Collision, Collisions,
};
use amethyst::{
    core::{
        math::{Point2, Point3},
        Transform,
    },
    ecs::{
        Builder, Entity, Join, Read, ReadStorage, ReaderId, System, World, WorldExt, Write,
        WriteStorage,
    },
    renderer::{debug_drawing::DebugLinesComponent, palette::rgb::Srgba, ActiveCamera},
};
use ncollide2d::query::Contact;
use std::marker::PhantomData;

pub(crate) struct CollisionViewSystem<T>
where
    T: 'static + Send + Sync,
{
    debug_collisions: Entity,
    contact_collisions: Entity,
    reader: Option<ReaderId<ContactEvent<T>>>,
    paramater: PhantomData<T>,
}

impl<T> CollisionViewSystem<T>
where
    T: 'static + Send + Sync,
{
    pub(crate) fn new(world: &mut World) -> Self {
        CollisionViewSystem {
            debug_collisions: world.create_entity().build(),
            contact_collisions: world.create_entity().build(),
            reader: None,
            paramater: PhantomData,
        }
    }
}

impl<'s, T> System<'s> for CollisionViewSystem<T>
where
    T: 'static + Send + Sync + CollisionColor,
{
    type SystemData = (
        WriteStorage<'s, DebugLinesComponent>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Collisions<T>>,
        Read<'s, ActiveCamera>,
        Write<'s, ContactEventChannel<T>>,
    );

    fn run(
        &mut self,
        (mut debug_lines, transforms, collisions, camera, mut channel): Self::SystemData,
    ) {
        if self.reader.is_none() == true {
            self.reader = channel.register_reader().into();
        }

        let camera_z = camera
            .entity
            .and_then(|entity| transforms.get(entity))
            .map(|transform| transform.translation().z - 1.);
        if camera_z.is_none() == true {
            return;
        }

        let position_z = camera_z.unwrap();

        // 既存コリジョン
        if let Ok(entry) = debug_lines.entry(self.debug_collisions) {
            let debug = entry.or_insert(DebugLinesComponent::with_capacity(2048));
            debug.clear();
            for (t, c) in (&transforms, &collisions).join() {
                for Collision {
                    aabb,
                    position,
                    paramater,
                } in &c.collisions
                {
                    let (r, g, b, a) = paramater.collision_color();
                    draw_aabb(debug, aabb, position, t, position_z, Srgba::new(r, g, b, a));
                }
            }
        }

        // 衝突判定検出点
        let reader = self.reader.as_mut().unwrap();
        if let Ok(entry) = debug_lines.entry(self.contact_collisions) {
            let debug = entry.or_insert(DebugLinesComponent::with_capacity(2048));
            debug.clear();
            let color = Srgba::new(0., 1., 1., 1.);
            for ContactEvent {
                contact:
                    Contact {
                        world1,
                        world2,
                        normal,
                        depth,
                    },
                entity1,
                entity2,
                ..
            } in channel.read(reader)
            {
                let delta = normal.into_inner() * *depth;
                let radius = 2.;
                // entity1
                {
                    let position = Point3::new(world1.x, world1.y, position_z);
                    debug.add_circle_2d(position, radius, 4, color);

                    let normal_3d =
                        Point3::new(position.x - delta.x, position.y - delta.y, position_z);
                    debug.add_line(position, normal_3d, color);

                    match (collisions.get(*entity1), transforms.get(*entity1)) {
                        (Some(collisions), Some(t)) => {
                            let mut t = t.clone();
                            t.translation_mut().x -= delta.x / 2.;
                            t.translation_mut().y -= delta.y / 2.;
                            for Collision { aabb, position, .. } in &collisions.collisions {
                                draw_aabb(debug, aabb, position, &t, position_z, color);
                            }
                        }
                        _ => {}
                    }
                }

                // entity2
                {
                    let position = Point3::new(world2.x, world2.y, position_z);
                    debug.add_circle_2d(position, radius, 4, color);

                    let normal_3d =
                        Point3::new(position.x + delta.x, position.y + delta.y, position_z);
                    debug.add_line(position, normal_3d, color);

                    match (collisions.get(*entity2), transforms.get(*entity2)) {
                        (Some(collisions), Some(t)) => {
                            let mut t = t.clone();
                            t.translation_mut().x += delta.x / 2.;
                            t.translation_mut().y += delta.y / 2.;
                            for Collision { aabb, position, .. } in &collisions.collisions {
                                draw_aabb(debug, aabb, position, &t, position_z, color);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn draw_aabb(
    debug: &mut DebugLinesComponent,
    aabb: &Aabb,
    position: &Vector,
    t: &Transform,
    position_z: f32,
    color: Srgba,
) {
    let position = Vector::new(t.translation().x, t.translation().y) + position;
    let left_top = Point2::new(
        position.x - aabb.half_extents().x,
        position.y + aabb.half_extents().y,
    );
    let right_down = Point2::new(
        position.x + aabb.half_extents().x,
        position.y - aabb.half_extents().y,
    );

    debug.add_rectangle_2d(left_top, right_down, position_z, color);
}

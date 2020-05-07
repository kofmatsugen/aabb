use crate::{
    debug::traits::CollisionColor,
    event::{ContactEvent, ContactEventChannel},
    types::{Aabb, Vector},
    Collision, Collisions,
};
use amethyst::{
    core::{
        math::{Point2, Point3, Vector3},
        Transform,
    },
    ecs::{
        Builder, Entity, Join, Read, ReadStorage, ReaderId, System, World, WorldExt, Write,
        WriteStorage,
    },
    renderer::{debug_drawing::DebugLinesComponent, palette::rgb::Srgba, ActiveCamera},
};
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
            for (c,) in (&collisions,).join() {
                for Collision {
                    aabb,
                    position,
                    paramater,
                    ..
                } in c.collisions.iter()
                {
                    let (r, g, b, a) = paramater.collision_color();
                    draw_aabb(debug, aabb, position, position_z, Srgba::new(r, g, b, a));
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
                hit_center, delta, ..
            } in channel.read(reader)
            {
                let radius = 30.;
                // entity1
                let position = Point3::new(hit_center.x, hit_center.y, position_z);
                debug.add_circle_2d(position, radius, 100, color);

                let delta = Vector3::new(delta.x, delta.y, position_z);
                debug.add_direction(position, delta, color);
            }
        }
    }
}

fn draw_aabb(
    debug: &mut DebugLinesComponent,
    aabb: &Aabb,
    position: &Vector,
    position_z: f32,
    color: Srgba,
) {
    let left_top = Point2::new(
        position.x - aabb.half_extents().x,
        position.y + aabb.half_extents().y,
    );
    let right_down = Point2::new(
        position.x + aabb.half_extents().x,
        position.y - aabb.half_extents().y,
    );

    debug.add_line(
        Point3::new(position.x, position.y - 5., position_z),
        Point3::new(position.x, position.y + 5., position_z),
        color,
    );
    debug.add_line(
        Point3::new(position.x - 5., position.y, position_z),
        Point3::new(position.x + 5., position.y, position_z),
        color,
    );

    debug.add_rectangle_2d(left_top, right_down, position_z, color);
}

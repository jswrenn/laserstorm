use ::components::{Identity, Position, Shape};
use specs::{FetchMut, System, Join, ReadStorage};
use ncollide::world::CollisionWorld2;
use ncollide::world::CollisionGroups;
use ncollide::world::GeometricQueryType;
use ncollide::shape::ShapeHandle;

pub struct Collision;

impl<'a> System<'a> for Collision {
  type SystemData =
    ( FetchMut<'a, CollisionWorld2<f64,()>>
    , ReadStorage<'a, Identity>
    , ReadStorage<'a, Position>
    , ReadStorage<'a, Shape>);

  fn run(&mut self, (mut world, identities, positions, shapes) : Self::SystemData) {
    for (identity, position, shape) in (&identities, &positions, &shapes).join() {
      if let None = world.collision_object(**identity) {
        world.deferred_add(
          **identity,
          **position,
          shape.0.clone(),
          CollisionGroups::new(),
          GeometricQueryType::Contacts(0.0),
          ());
      }
      world.deferred_set_position(**identity, **position);
    }
    world.update();
  }
}
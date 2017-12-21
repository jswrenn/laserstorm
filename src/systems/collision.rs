use types::*;
use specs::*;
use components::*;
use ncollide::world::CollisionGroups;
use ncollide::world::GeometricQueryType;

pub struct Collision;

impl<'a> System<'a> for Collision {
  type SystemData =
    ( FetchMut<'a, CollisionWorld>
    , Entities<'a>
    , ReadStorage<'a, Position>
    , ReadStorage<'a, Shape>);

  fn run(&mut self, (mut world, entities, positions, shapes) : Self::SystemData) {
    for (entity, position, shape) in (&*entities, &positions, &shapes).join() {
      if let None = world.collision_object(entity.id() as usize) {
        world.deferred_add(
          entity.id() as usize,
          **position,
          shape.0.clone(),
          CollisionGroups::new(),
          GeometricQueryType::Contacts(0.0),
          entity);
      }
      world.deferred_set_position(entity.id() as usize, **position);
    }
    world.update();
  }
}
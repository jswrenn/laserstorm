use ::components::{Identity, Position};
use specs::{FetchMut, System, Join, ReadStorage};
use ncollide::world::CollisionWorld2;

pub struct Collision;

impl<'a> System<'a> for Collision {
  type SystemData =
    ( FetchMut<'a, CollisionWorld2<f64,()>>
    , ReadStorage<'a, Identity>
    , ReadStorage<'a, Position>);

  fn run(&mut self, (mut world, identities, positions) : Self::SystemData) {
    for (identity, position) in (&identities, &positions).join() {
      world.deferred_set_position(**identity, **position);
    }
    world.update();
  }
}
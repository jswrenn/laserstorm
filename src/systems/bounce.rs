use types::*;
use specs::*;
use components::*;
use nalgebra;

pub struct Bounce;

impl<'a> System<'a> for Bounce {
  type SystemData =
    ( FetchMut<'a, CollisionWorld>
    , Entities<'a>
    , WriteStorage<'a, LinearVelocity>);

  fn run(&mut self, (world, entities, mut linear_velocities) : Self::SystemData) {
    for (a, b, contact) in world.contacts() {
      if let Some(mut linear_velocity) = linear_velocities.get_mut(a.data) {
        **linear_velocity -= 2. * nalgebra::dot(&**linear_velocity, &contact.normal) * contact.normal;
      }
      if let Some(mut linear_velocity) = linear_velocities.get_mut(b.data) {
        **linear_velocity -= 2. * nalgebra::dot(&**linear_velocity, &contact.normal) * contact.normal;
      }
    }
  }
}
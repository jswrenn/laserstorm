use ::components::{Position, Velocity};
use specs::{System, Join, ReadStorage, WriteStorage};
use nalgebra::geometry::Translation2;

const TIMESTEP : f64 = 0.016;

pub struct Motion;

impl<'a> System<'a> for Motion {
  type SystemData =
    ( WriteStorage<'a, Position>
    , ReadStorage<'a,  Velocity> );

  fn run(&mut self, (mut positions, velocities): Self::SystemData) {
    for (mut position, &velocity) in (&mut positions, &velocities).join() {
      **position *= Translation2::from_vector(*velocity * TIMESTEP);
    }
  }
}
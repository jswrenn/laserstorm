use ::components::{Identity, Mass, Force, Position, Velocity, Acceleration};
use specs::{FetchMut, System, ParJoin, ReadStorage, WriteStorage};
use nalgebra::geometry::Translation2;
use std::time::{Duration, Instant};
use rayon::iter::ParallelIterator;

pub struct Motion;

impl<'a> System<'a> for Motion {
  type SystemData =
    ( FetchMut<'a, Option<Instant>>
    , ReadStorage<'a,  Identity>
    , ReadStorage<'a,  Mass>
    , ReadStorage<'a,  Force>
    , WriteStorage<'a, Acceleration>
    , WriteStorage<'a, Velocity>
    , WriteStorage<'a, Position>);

  fn run(&mut self, ( mut last_update
                    , identities
                    , masses
                    , forces
                    , mut accelerations
                    , mut velocities
                    , mut positions) : Self::SystemData)
  {
    let now   = Instant::now();
    let delta = last_update.map(|time|
      { let duration = now.duration_since(time);
        duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
      }).unwrap_or(0.);

    (&identities, &masses, &forces, &mut accelerations, &mut velocities, &mut positions).par_join()
      .for_each(|(identity, mass, force, acceleration, velocity, position)|
        { **velocity += 0.5 * delta * **acceleration;
          **position *= Translation2::from_vector(**velocity * delta);
          **acceleration = **force / **mass;
          **velocity += 0.5 * delta * **acceleration; });

    *last_update = Some(now);
  }
}

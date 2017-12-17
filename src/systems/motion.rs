use ::components::{Identity, Mass, Force, Position, Velocity, Acceleration};
use specs::{FetchMut, System, ParJoin, ReadStorage, WriteStorage};
use nalgebra::geometry::Translation2;
use std::time::{Duration, Instant};
use rayon::iter::ParallelIterator;

pub struct Motion;

impl<'a> System<'a> for Motion {
  type SystemData =
    ( FetchMut<'a, Option<Instant>>
    , ReadStorage<'a,  Acceleration>
    , WriteStorage<'a, Velocity>
    , WriteStorage<'a, Position>);

  fn run(&mut self, ( mut last_update
                    , accelerations
                    , mut velocities
                    , mut positions) : Self::SystemData)
  {
    let now   = Instant::now();
    let delta = last_update.map(|time|
      { let duration = now.duration_since(time);
        duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
      }).unwrap_or(0.);

    (&accelerations, &mut velocities, &mut positions).par_join()
      .for_each(|(acceleration, velocity, position)|
        { **velocity += **acceleration * delta;
          **position *= Translation2::from_vector(**velocity * delta); });

    *last_update = Some(now);
  }
}
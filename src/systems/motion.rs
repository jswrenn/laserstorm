use types::*;
use components::*;
use alga::general::Real;
use alga::linear::Transformation;
use specs::{FetchMut, System, Join, ParJoin, ReadStorage, WriteStorage};
use nalgebra;
use nalgebra::geometry::Point;
use nalgebra::geometry::Translation2;
use std::time::{Duration, Instant};
use rayon::iter::ParallelIterator;
use num_complex::Complex;

pub struct Motion;

impl<'a> System<'a> for Motion {
  type SystemData =
    ( FetchMut<'a, Option<Instant>>
    , ReadStorage<'a,  CenterOfMass>
    , ReadStorage<'a,  LinearAcceleration>
    , ReadStorage<'a,  AngularAcceleration>
    , WriteStorage<'a, LinearVelocity>
    , WriteStorage<'a, AngularVelocity>
    , WriteStorage<'a, Position>);

  fn run(&mut self, ( mut last_update
                    , center_of_mass
                    , linear_accelerations
                    , angular_accelerations
                    , mut linear_velocities
                    , mut angular_velocities
                    , mut positions ) : Self::SystemData)
  {
    let now   = Instant::now();
    let delta_time = last_update.map(|time|
      { let duration = now.duration_since(time);
        duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
      }).unwrap_or(0.);

    ( &center_of_mass
    , &linear_accelerations
    , &angular_accelerations
    , &mut linear_velocities
    , &mut angular_velocities
    , &mut positions).par_join()
      .for_each(|( center_of_mass
                 , linear_acceleration
                 , angular_acceleration
                 , linear_velocity
                 , angular_velocity
                 , position )|
        { **linear_velocity   += **linear_acceleration * delta_time;
          **angular_velocity  += **angular_acceleration * delta_time;
          let rotation         = Rotation::from_scaled_axis(**angular_velocity * delta_time);
          let translation      =   Translation::from_vector(**linear_velocity  * delta_time);
          let pivot = position.translation.transform_point(center_of_mass);
          position.append_rotation_wrt_point_mut(&rotation, &pivot);
          position.append_translation_mut(&translation);
        });
    *last_update = Some(now);
  }
}
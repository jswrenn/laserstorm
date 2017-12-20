use types::*;
use components::*;
use events;
use alga::general::Real;
use alga::linear::Transformation;
use specs::{Fetch, System, Join, ParJoin, ReadStorage, WriteStorage};
use nalgebra;
use nalgebra::geometry::Point;
use nalgebra::geometry::Translation2;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use rayon::iter::ParallelIterator;
use alga::linear::Similarity;
use nalgebra::geometry::Rotation2;
use nalgebra::core::Unit;
pub struct Control;

impl<'a> System<'a> for Control {
  type SystemData =
    ( Fetch<'a, events::ControlState>
    , ReadStorage<'a,  Identity>
    , WriteStorage<'a, Position>
    , WriteStorage<'a, LinearVelocity>
    , WriteStorage<'a, AngularVelocity>);

  fn run(&mut self, ( state
                    , identities
                    , mut positions
                    , mut linear_velocities
                    , mut angular_velocities ) : Self::SystemData)
  {
    ( &identities
    , &mut positions
    , &mut linear_velocities
    , &mut angular_velocities ).par_join()
      .for_each(|( identity
                 , position
                 , linear_velocity
                 , angular_velocity )|
        { if state.forward {
            **linear_velocity = position.rotation.rotate_vector(&Vector::new(0.0, 0.1));
          } else if state.backward {
            **linear_velocity = position.rotation.rotate_vector(&Vector::new(0.0, 0.1));
          } else { **linear_velocity = Vector::new(0.0, 0.0); }
          if state.left {
            **angular_velocity -= Orientation::new(0.1);
          } else if state.right {
            **angular_velocity += Orientation::new(0.1);
          } else { **angular_velocity = Orientation::new(0.) }
        });
  }
}
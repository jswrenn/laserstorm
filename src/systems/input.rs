use ::components::*;
use types::*;
use specs::{Fetch, System, Join, ParJoin, ReadStorage, WriteStorage};
use nalgebra::core::{Matrix1,Vector1,Vector2};
use nalgebra::core::Unit;
use nalgebra;
use nalgebra::geometry::Translation2;
use std::time::{Duration, Instant};
use rayon::iter::ParallelIterator;
use amethyst::core::LocalTransform;
use amethyst::core::cgmath::Vector3;
use  nalgebra::Rotation2;
use amethyst::input::InputHandler;
use alga::linear::Similarity;
use num_complex::Complex;

/// This system is responsible for moving all the paddles according to the user
/// provided input.
pub struct Input;

impl<'a> System<'a> for Input {
  type SystemData =
    ( Fetch<'a, InputHandler<String, String>>
    , ReadStorage<'a, Position>
    , WriteStorage<'a, LinearVelocity>
    , WriteStorage<'a, AngularVelocity>);

    fn run(&mut self, (input, positions, mut linear_velocities, mut angular_velocities): Self::SystemData) {

        for (position, linear_velocity, angular_velocity)
            in (&positions, &mut linear_velocities, &mut angular_velocities).join() {
          if let Some(turn) = input.axis_value("turn") {
            **angular_velocity = Orientation::new(1. * turn);
          } else {
            **angular_velocity = nalgebra::zero();
          }
          
          if let Some(accl) = input.axis_value("accelerate") {
            **linear_velocity = accl * position.rotation.rotate_vector(&Vector::new(0.5, 0.0));
          } else {
            **linear_velocity = nalgebra::zero();
          }
        }
    }
}
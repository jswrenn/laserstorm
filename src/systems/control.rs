use types::*;
use specs::*;
use components;
use components::*;
use events;
use rayon::iter::ParallelIterator;
use alga::linear::Similarity;

pub struct Control;

impl<'a> System<'a> for Control {
  type SystemData =
    ( ReadStorage<'a,  components::Control>
    , WriteStorage<'a, Position>
    , WriteStorage<'a, LinearVelocity>
    , WriteStorage<'a, AngularVelocity>);

  fn run(&mut self, ( controls
                    , mut positions
                    , mut linear_velocities
                    , mut angular_velocities ) : Self::SystemData)
  {
    ( &controls
    , &mut positions
    , &mut linear_velocities
    , &mut angular_velocities ).par_join()
      .for_each(|( control
                 , position
                 , linear_velocity
                 , angular_velocity )|
        { if control.forward {
            **linear_velocity = position.rotation.rotate_vector(&Vector::new(0.0, 0.5));
          } else if control.backward {
            **linear_velocity = position.rotation.rotate_vector(&Vector::new(0.0, -0.5));
          }
          if control.turn_left {
            **angular_velocity -= Orientation::new(0.1);
          } else if control.turn_right {
            **angular_velocity += Orientation::new(0.1);
          } else { **angular_velocity = Orientation::new(0.) }
        });
  }
}
use types::*;
use specs::*;
use components::*;
use events;
use rayon::iter::ParallelIterator;
use alga::linear::Similarity;

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
            **linear_velocity = position.rotation.rotate_vector(&Vector::new(0.0, 0.5));
          } else if state.backward {
            **linear_velocity = position.rotation.rotate_vector(&Vector::new(0.0, -0.5));
          } else { **linear_velocity = Vector::new(0.0, 0.0); }
          if state.left {
            **angular_velocity -= Orientation::new(0.1);
          } else if state.right {
            **angular_velocity += Orientation::new(0.1);
          } else { **angular_velocity = Orientation::new(0.) }
        });
  }
}
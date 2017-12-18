use ::components::*;
use alga::general::Real;
use alga::general::SubsetOf;
use specs::{FetchMut, System, Join, ParJoin, ReadStorage, WriteStorage};
use nalgebra::geometry::{Translation2, Isometry3};
use nalgebra::core::{Vector3};
use nalgebra;
use rayon::iter::ParallelIterator;
use amethyst::core::Transform;
use amethyst::core::cgmath::{Matrix4,Rotation3,InnerSpace,Angle,Deg,Rad,Quaternion};
use num_complex::Complex;
use nalgebra::geometry::UnitComplex;

pub struct AmethystMotion;

impl<'a> System<'a> for AmethystMotion {
  type SystemData =
    ( ReadStorage<'a,  Position>
    , WriteStorage<'a, Transform>);

  fn run(&mut self, ( positions
                    , mut amethyst_positions) : Self::SystemData)
  {
    ( &positions
    , &mut amethyst_positions).join()
      .for_each(|(position
                , amethyst_position)|
        { println!("{}", position.translation.vector.x);
          let r : nalgebra::Matrix4<f32> =
            Isometry3::new(
              Vector3::new(position.translation.vector.x as f32,
                           position.translation.vector.y as f32, 0.),
              Vector3::new(0., 0., position.rotation.angle() as f32))
              .to_superset();
          //amethyst_position.rotation
          //  = Quaternion::from_angle_z(Rad(position.rotation.angle() as f32));
          //amethyst_position.translation =
          //  Vector3::new(position.translation.vector.x as f32,
          //               position.translation.vector.y as f32,
          //               0.);
          //let m : Rotation3<f64> = position.rotation.to_rotation_matrix().matrix();
          amethyst_position.0 =
            Matrix4::new(
              r[(0,0)] as f32, r[(1,0)] as f32, r[(2,0)] as f32, r[(3,0)] as f32,
              r[(0,1)] as f32, r[(1,1)] as f32, r[(2,1)] as f32, r[(3,1)] as f32,
              r[(0,2)] as f32, r[(1,2)] as f32, r[(2,2)] as f32, r[(3,2)] as f32,
              r[(0,3)] as f32, r[(1,3)] as f32, r[(2,3)] as f32, r[(3,3)] as f32);
          //let theta = position.rotation.angle() / 2.;
          //amethyst_position.rotation =
          //  Quaternion::new(theta.cos() as f32, 0.0, 0., theta.sin() as f32);
        });
  }
}
//! Rendering system.
//!

use std::mem;
use types::*;
use specs::*;
use components;

use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;
use kiss3d::window::Window;
use alga::general::SubsetOf;
use ncollide::shape;
use nalgebra::convert;
use alga::general::Real;
use alga::linear::ProjectiveTransformation;
use nalgebra::{self, Point2, Point3, Translation2};
use ncollide::world::{CollisionWorld2, CollisionGroups};
use nalgebra::geometry::{Isometry2, Isometry3};
use nalgebra::core::Vector3;
use ncollide::shape::{ShapeHandle, Ball, Ball2, Ball3, Cuboid2, Cone2, Cylinder2, Segment2, Triangle2};
use ncollide::transformation::ToPolyline;
use ncollide::transformation::ToTriMesh;
use kiss3d::resource::Mesh;
use ncollide::procedural::*;
use kiss3d::scene::SceneNode;
use std::collections::HashMap;
use std::f32::consts::PI;
use nalgebra::geometry::UnitQuaternion;

pub struct Render<'w> {
  target: Rc<RefCell<&'w mut Window>>,
}

impl<'w> Render<'w> {
  pub fn new(window : Rc<RefCell<&'w mut Window>>,) -> Render {
    Render {
      target: window
    }
  }

  fn draw_polyline(&mut self, polyline: &Polyline<Point2<f64>>) {
    let mut window = self.target.borrow_mut();
    for pt in polyline.coords().windows(2) {
      window.draw_line(&Point3::new(pt[0].x as f32, pt[0].y as f32, 0.0), &Point3::new(pt[1].x as f32, pt[1].y as f32, 0.0), &Point3::new(0.0, 1.0, 0.0));
    }

    let last = polyline.coords().len() - 1;
    window.draw_line(&Point3::new(polyline.coords()[0].x as f32, polyline.coords()[0].y as f32, 0.0),
                     &Point3::new(polyline.coords()[last].x as f32, polyline.coords()[last].y as f32, 0.0),
                     &Point3::new(0.0, 1.0, 0.0));
  }
}

impl<'a, 'w> System<'a> for Render<'w> {
  type SystemData =
    ( Fetch<'a, CollisionWorld2<f64,()>>);

  fn run(&mut self, world: Self::SystemData) {
    for object in world.collision_objects() {
      let id = object.uid;
      let position = object.position;
      let shape = &object.shape;
      let mut polyline =
            shape.as_shape::<Ball2<f64>>().map(|s|    s.to_polyline(10))
        .or(shape.as_shape::<Cone2<f64>>().map(|s|    s.to_polyline(())))
        .or(shape.as_shape::<Cuboid2<f64>>().map(|s|  s.to_polyline(())))
        .or(shape.as_shape::<Segment2<f64>>().map(|s| s.to_polyline(())))
        .or(shape.as_shape::<Triangle2<f64>>().map(|s| s.to_polyline(()))).unwrap();
      polyline.transform_by(&position);
      self.draw_polyline(&polyline);
      // todo: figure out what to do with 2d & compound shapes
      //self.scene.entry(id).or_insert_with(|| {
      //  let mut n = window.add_cone(0.01,0.1);
      //  n.set_color(1.0, 0.0, 0.0);
      //  n }).set_local_transformation(
      //    convert(Isometry3::new(
      //      Vector3::new(position.translation.vector.x,
      //                   position.translation.vector.y, 0.),
      //      position.rotation.angle() * Vector3::z_axis().to_superset())));
    }
  }
}
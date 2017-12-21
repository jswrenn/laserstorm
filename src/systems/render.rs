use types::*;
use specs::*;
use std::rc::Rc;
use std::cell::RefCell;
use kiss3d::window::Window;
use alga::linear::Similarity;
use nalgebra::{Point2, Point3};
use nalgebra::geometry::Rotation2;
use ncollide::shape::{Plane2, Ball2, Cuboid2, Cone2, Segment2, Triangle2};
use ncollide::transformation::ToPolyline;
use ncollide::procedural::*;

pub struct Render<'w> {
  target: Rc<RefCell<&'w mut Window>>,
}

impl<'w> Render<'w> {
  pub fn new(window : Rc<RefCell<&'w mut Window>>,) -> Render {
    Render {
      target: window
    }
  }

  fn draw_polyline(&mut self, polyline: &Polyline<Point2<Precision>>) {
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
    ( Fetch<'a, CollisionWorld>);

  fn run(&mut self, world: Self::SystemData) {
    for object in world.collision_objects() {
      let id = object.uid;
      let position = object.position;
      let shape = &object.shape;
      if let Some(mut polyline) =
            shape.as_shape::<Ball2<Precision>>().map(|s|    s.to_polyline(10))
        .or(shape.as_shape::<Cone2<Precision>>().map(|s|    s.to_polyline(())))
        .or(shape.as_shape::<Cuboid2<Precision>>().map(|s|  s.to_polyline(())))
        .or(shape.as_shape::<Segment2<Precision>>().map(|s| s.to_polyline(())))
        .or(shape.as_shape::<Triangle2<Precision>>().map(|s| s.to_polyline(())))
      {
        polyline.transform_by(&position);
        self.draw_polyline(&polyline);
      } else if let Some(plane) = shape.as_shape::<Plane2<Precision>>() {
          let normal   = Point2::from_coordinates(*plane.normal());
          let rotation = Rotation2::new(normal.x.atan2(normal.y));
          let left     = position.translation.translate_point(&rotation.rotate_point(&Point2::new(-9999999., 0.)));
          let right    = position.translation.translate_point(&rotation.rotate_point(&Point2::new( 9999999., 0.)));
          self.target.borrow_mut().draw_line(
            &Point3::new(left.x as f32, left.y as f32, 0.),
            &Point3::new(right.x as f32, right.y as f32, 0.),
            &Point3::new(0.0, 1.0, 0.0));
      }
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
    for (a, b, contact) in world.contacts() {
      let mut window = self.target.borrow_mut();
      window.draw_point(&Point3::new(contact.world1.x, contact.world1.y, 0.), &Point3::new(1.0, 0.0, 0.0));
      window.draw_point(&Point3::new(contact.world2.x, contact.world2.y, 0.), &Point3::new(1.0, 0.0, 0.0));
      window.draw_line(&Point3::new(contact.world1.x, contact.world1.y, 0.),
                       &Point3::new(contact.world2.x, contact.world2.y, 0.), &Point3::new(1.0, 0.0, 0.0));
    }
  }
}
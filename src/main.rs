//#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate rayon;
extern crate specs;
extern crate shrev;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate specs_derive;
extern crate ncollide;
extern crate nalgebra;
extern crate alga;
extern crate num_complex;
extern crate kiss3d;
extern crate glfw;


mod types;
use types::*;

mod events;
mod components;
mod systems;

use specs::*;
use std::time::Instant;
use std::cell::*;
use std::rc::*;
use ncollide::shape::Cone2;
use ncollide::shape::Ball2;

use kiss3d::scene::SceneNode;
fn main() {
  let mut world = specs::World::new();

  world.register::<components::Identity>();
  world.register::<components::Position>();
  world.register::<components::LinearVelocity>();
  world.register::<components::AngularVelocity>();
  world.register::<components::LinearAcceleration>();
  world.register::<components::AngularAcceleration>();
  world.register::<components::LinearForce>();
  world.register::<components::AngularForce>();
  world.register::<components::CenterOfMass>();
  world.register::<components::Mass>();
  world.register::<components::Identity>();
  world.register::<components::Shape>();
  world.register::<components::Control>();

  let mut window = kiss3d::window::Window::new("nphysics: 3d demo");
  window.set_light(kiss3d::light::Light::StickToCamera);
  let window_wrapper = Rc::new(RefCell::new(&mut window));

  world.add_resource(None::<Instant>);
  world.add_resource(events::ControlState::default());
  world.add_resource(CollisionWorld::new(0.02, true));

  use ncollide::shape::Plane;

  // Create four planes to bound the world
  world.create_entity()
    .with(components::Position(Isometry::new(Vector::new(-1.0, 0.0), nalgebra::zero())))
    .with(components::Shape(ShapeHandle::new(Plane::new(Vector::x()))))
    .build();
  world.create_entity()
    .with(components::Position(Isometry::new(Vector::new(0.0, -1.0), nalgebra::zero())))
    .with(components::Shape(ShapeHandle::new(Plane::new(Vector::y()))))
    .build();
  world.create_entity()
    .with(components::Position(Isometry::new(Vector::new(1.0, 0.0), nalgebra::zero())))
    .with(components::Shape(ShapeHandle::new(Plane::new(-Vector::x()))))
    .build();
  world.create_entity()
    .with(components::Position(Isometry::new(Vector::new(0.0, 1.0), nalgebra::zero())))
    .with(components::Shape(ShapeHandle::new(Plane::new(-Vector::y()))))
    .build();

  // Create a ball
  world.create_entity()
    .with(components::Position(
        nalgebra::Isometry::from_parts(
          nalgebra::Translation2::from_vector(Vector::new(0., 0.)),
          nalgebra::UnitComplex::new(0.001))))
    .with(components::LinearVelocity(Vector::new(0., 0. )))
    .with(components::AngularVelocity(Orientation::new(0.)))
    .with(components::LinearAcceleration(Vector::new(0., 0.)))
    .with(components::AngularAcceleration(nalgebra::zero()))
    .with(components::CenterOfMass(nalgebra::origin()))
    .with(components::Shape(ShapeHandle::new(Ball2::new(0.05f32))))
    .with(components::Control::default())
    .build();

  let mut dispatcher = DispatcherBuilder::new()
    .add(systems::Motion, "motion", &[])
    .add(systems::Control, "control", &[])
    .add(systems::Collision, "collision", &["motion"])
    .add(systems::Resolution, "resolution", &["collision"])
    .add_thread_local(systems::Render::new(window_wrapper.clone()))
    .add_thread_local(systems::Input::new(window_wrapper.clone()))
    .build();

  while window_wrapper.borrow_mut().render() {
    dispatcher.dispatch(&mut world.res);
    world.maintain();
  }
}
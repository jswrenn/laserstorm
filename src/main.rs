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
extern crate piston;
extern crate piston_window;
extern crate elmesque;
extern crate num_traits;
#[macro_use]
extern crate derive_more;


mod components;
use components::*;

mod systems;
use systems::Motion;

use ncollide::world::{CollisionWorld2,CollisionGroups, GeometricQueryType};
use specs::DispatcherBuilder;
use specs::Join;
use std::time::Instant;

use elmesque::{Form, Renderer};
use piston::input::UpdateEvent;
use piston::window::WindowSettings;
use piston_window::*;

fn main() {
  use specs::World;

  let mut window: PistonWindow =
    WindowSettings::new("Hello Piston!", [640, 480])
      .exit_on_esc(true).build().unwrap();

  let mut world = World::new();
  world.register::<components::Position>();
  world.register::<components::Velocity>();
  world.register::<components::Acceleration>();
  world.register::<components::Identity>();

  world.add_resource(None::<Instant>);
  world.add_resource(CollisionWorld2::<f64,()>::new(0.02, true));
  world.add_resource(Vec::<Event>::new());

  // create a box with identity, position, velocity, and acceleration
  world.create_entity()
    .with(Identity(0))
    .with(components::Position(
        nalgebra::Isometry::from_parts(
          nalgebra::Translation2::from_vector(nalgebra::Vector2::new(500., 500.)),
          nalgebra::UnitComplex::new(0.))))
    .with(components::Velocity(nalgebra::Vector2::new(10., 10.)))
    .with(components::Acceleration(nalgebra::Vector2::new(0., 9.8)))
    .build();

  let mut dispatcher = DispatcherBuilder::new()
    .add(Motion, "motion", &[]).build();

  while let Some(event) = window.next() {
    window.draw_2d(&event, |context, graphics| {
      dispatcher.dispatch(&mut world.res);
      world.maintain();
      clear([1.0; 4], graphics);
      for position in world.read::<components::Position>().join() {
        let position = position.translation.vector;
        rectangle(
          [1.0, 0.0, 0.0, 1.0], // red
          [position.x, position.y, 10.0, 10.0],
          context.transform,
          graphics);
      }
    });
  }
}
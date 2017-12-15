extern crate specs;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate ncollide;
extern crate nalgebra;

mod components;
use components::Position;

mod systems;
use systems::Motion;

use specs::DispatcherBuilder;


fn main() {
    use specs::World;
    let mut world = World::new();
    world.register::<Position>();
    let mut dispatcher = DispatcherBuilder::new().add(Motion, "physics", &[]).build();
    println!("Hello, world!");
}
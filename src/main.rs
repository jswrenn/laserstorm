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
extern crate amethyst;
extern crate alga;
extern crate num_complex;

use amethyst::Result;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, RenderBundle, RenderSystem,
                         Stage};
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::assets::Loader;
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{LocalTransform, Transform};
use amethyst::renderer::{Camera, Material, MaterialDefaults, PosTex, MeshHandle, Event,
                         KeyboardInput, VirtualKeyCode, WindowEvent};

mod types;
use types::*;

mod components;
use components::*;

mod systems;
use systems::Motion;
use systems::Input;
use systems::AmethystMotion;

use ncollide::world::{CollisionWorld2,CollisionGroups, GeometricQueryType};
use specs::DispatcherBuilder;
use specs::Join;
use std::time::Instant;

pub struct LaserStorm;

const PADDLE_HEIGHT: f32 = 0.30;
const PADDLE_WIDTH: f32 = 0.05;
const PADDLE_COLOUR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const PADDLE_VELOCITY: f32 = 1.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
    pub velocity: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side: side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            velocity: PADDLE_VELOCITY
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

impl State for LaserStorm {
    fn on_start(&mut self, world: &mut World) {
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

      world.add_resource(None::<Instant>);
      world.add_resource(CollisionWorld2::<f64,()>::new(0.02, true));

      use amethyst::renderer::{ScreenDimensions, Projection};
      let proj = {
        let dim = world.read_resource::<ScreenDimensions>();
        let aspect_ratio = dim.aspect_ratio();
        let eye = [0., 0., 0.1];
        let target = [0., 0., 0.];
        let up = [0., 1., 0.];
        let scale = 100.0;
        Projection::orthographic(
          -1.0 * scale * aspect_ratio,
           1.0 * scale * aspect_ratio,
           1.0 * scale,
          -1.0 * scale)
      };
      world.create_entity()
          .with(Camera::from(proj))
          .build();
      
      // Create right plank entity.

      let mesh = create_mesh(world, generate_rectangle_vertices(-0.1, -0.1, 0.1, 0.1));
      let material = create_colour_material(&world, [-0.0, 0.0, 1.0, 1.0]);
      
      // create a box with identity, position, velocity, and acceleration
      world.create_entity()
        .with(Identity(0))
        .with(components::Position(
            nalgebra::Isometry::from_parts(
              nalgebra::Translation2::from_vector(Vector::new(0., 0.)),
              nalgebra::UnitComplex::new(0.001))))
        .with(components::LinearVelocity(Vector::new(0., 0.)))
        .with(components::AngularVelocity(Orientation::new(0.)))
        .with(components::LinearAcceleration(nalgebra::zero()))
        .with(components::AngularAcceleration(nalgebra::zero()))
        .with(components::CenterOfMass(nalgebra::origin()))
        .with(Transform::default())
        .with(mesh)
        .with(material)
        .build();
    }
    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => Trans::Quit,
                    _ => Trans::None,
                }
            }
            _ => Trans::None,
        }
    }
}

fn run() -> Result<()> {
    let display_config = format!("{}/resources/display_config.ron",
                                 env!("CARGO_MANIFEST_DIR"));
    let key_bindings_path = format!("{}/resources/input.ron",
                                    env!("CARGO_MANIFEST_DIR"));
    let config = DisplayConfig::load(&display_config);

    let pipe = Pipeline::build().with_stage(Stage::with_backbuffer()
        .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
        .with_pass(DrawFlat::<PosTex>::new()));

    let mut game = Application::build("./", LaserStorm)?
        .with_bundle(
            InputBundle::<String, String>::new()
            .with_bindings_from_file(&key_bindings_path)
        )?
        .with::<Input>(Input, "control_system", &["input_system"])
        .with::<Motion>(Motion, "motion_system", &[])
        .with::<AmethystMotion>(AmethystMotion, "amethyst_motion_system", &["motion_system"])
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new())?
        .with_local(RenderSystem::build(pipe, Some(config))?)
        .build()?;

    Ok(game.run())
}

fn main() {
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}


/// Converts a vector of vertices into a mesh.
fn create_mesh(world: &World, vertices: Vec<PosTex>) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

/// Creates a solid material of the specified colour.
fn create_colour_material(world: &World, colour: [f32; 4]) -> Material {

    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(colour.into(), (), &world.read_resource());

    Material { albedo: albedo, ..mat_defaults.0.clone() }
}

/// Generates six vertices forming a rectangle.
fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<PosTex> {
    vec![PosTex {
             position: [left, bottom, 0.],
             tex_coord: [0.0, 0.0],
         },
         PosTex {
             position: [right, bottom, 0.0],
             tex_coord: [1.0, 0.0],
         },
         PosTex {
             position: [left, top, 0.0],
             tex_coord: [1.0, 1.0],
         },
         PosTex {
             position: [right, top, 0.],
             tex_coord: [1.0, 1.0],
         },
         PosTex {
             position: [left, top, 0.],
             tex_coord: [0.0, 1.0],
         },
         PosTex {
             position: [right, bottom, 0.0],
             tex_coord: [0.0, 0.0],
         }]
}

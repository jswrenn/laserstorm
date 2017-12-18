//! Rendering system.
//!

use std::mem;
use specs::*;
use amethyst;


use amethyst::core::Transform;
use amethyst::renderer::*;


pub struct RenderSystem {
  render_system : amethyst::ecs::rendering::systems::RenderSystem
}

impl<'a, 'b> amethyst::ecs::SystemExt<'a, (&'b EventsLoop, PipelineBuilder, Option<DisplayConfig>)>
    for RenderSystem {
    /// Create new `RenderSystem`
    /// It creates window and do render into it
    fn build(
        (events, pipe, config): (&'b EventsLoop, PipelineBuilder, Option<DisplayConfig>),
        world: &mut World,
    ) -> Result<Self> {
        amethyst::ecs::rendering::systems::RenderSystem::build((events, pipe, config))
          .map(|result| RenderSystem{render_system:result});
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (Fetch<'a, Camera>,
     Fetch<'a, Factory>,
     Fetch<'a, AmbientColor>,
     ReadStorage<'a, Transform>,
     ReadStorage<'a, Light>,
     ReadStorage<'a, Material>,
     ReadStorage<'a, Mesh>);

    fn run(
        &mut self,
        (camera, factory, ambient_color, globals, lights, materials, meshes): Self::SystemData,
    ) {
        use std::time::Duration;

        while let Some(job) = factory.jobs.try_pop() {
            job.exec(&mut self.renderer.factory);
        }

        self.scene.clear();

        for (mesh, material, global) in (&meshes, &materials, &globals).join() {
            self.scene.add_model(Model {
                material: material.0.clone(),
                mesh: mesh.as_ref().clone(),
                pos: global.0.into(),
            });
        }

        self.scene.set_ambient_color(ambient_color.0.clone());

        for light in lights.join() {
            self.scene.add_light(light.0.clone());
        }

        self.scene.add_camera(camera.clone());

        self.renderer.draw(
            &self.scene,
            &self.pipe,
            Duration::from_secs(0),
        );
    }
}
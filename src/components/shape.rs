use types::*;
use specs::*;

#[derive(Component, Clone)]
#[component(VecStorage)]
pub struct Shape(pub ShapeHandle);

impl ::std::ops::Deref for Shape {
  type Target = ShapeHandle;
  fn deref(&self) -> &ShapeHandle {
    &self.0
  }
}

impl ::std::ops::DerefMut for Shape {
  fn deref_mut(&mut self) -> &mut ShapeHandle {
    &mut self.0
  }
}
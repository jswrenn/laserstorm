use types::*;
use ncollide::shape::ShapeHandle2;
use specs::{Component, VecStorage};

#[derive(Component, Clone)]
#[component(VecStorage)]
pub struct Shape(pub ShapeHandle2<f64>);

impl ::std::ops::Deref for Shape {
  type Target = ShapeHandle2<f64>;
  fn deref(&self) -> &ShapeHandle2<f64> {
    &self.0
  }
}

impl ::std::ops::DerefMut for Shape {
  fn deref_mut(&mut self) -> &mut ShapeHandle2<f64> {
    &mut self.0
  }
}
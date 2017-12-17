use nalgebra::core::Vector2;
use specs::{Component, VecStorage};

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct Velocity(pub Vector2<f64>);

impl ::std::ops::Deref for Velocity {
  type Target = Vector2<f64>;
  fn deref(&self) -> &Vector2<f64> {
    &self.0
  }
}

impl ::std::ops::DerefMut for Velocity {
  fn deref_mut(&mut self) -> &mut Vector2<f64> {
    &mut self.0
  }
}
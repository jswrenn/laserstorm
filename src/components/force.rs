use nalgebra::core::Vector2;
use specs::{Component, VecStorage};

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct Force(pub Vector2<f64>);

impl ::std::ops::Deref for Force {
  type Target = Vector2<f64>;
  fn deref(&self) -> &Vector2<f64> {
    &self.0
  }
}

impl ::std::ops::DerefMut for Force {
  fn deref_mut(&mut self) -> &mut Vector2<f64> {
    &mut self.0
  }
}
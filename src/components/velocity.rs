use nalgebra::core::Vector2;
use specs::{Component, VecStorage};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Velocity(Vector2<f64>);

impl Component for Velocity {
  type Storage = VecStorage<Self>;
}

impl ::std::ops::Deref for Velocity {
  type Target = Vector2<f64>;
  fn deref(&self) -> &Vector2<f64> {
    &self.0
  }
}


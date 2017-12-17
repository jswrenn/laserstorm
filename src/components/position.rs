use nalgebra::geometry::Isometry2;
use specs::{Component, VecStorage};

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct Position(pub Isometry2<f64>);

impl ::std::ops::Deref for Position {
  type Target = Isometry2<f64>;
  fn deref(&self) -> &Isometry2<f64> {
    &self.0
  }
}

impl ::std::ops::DerefMut for Position {
  fn deref_mut(&mut self) -> &mut Isometry2<f64> {
    &mut self.0
  }
}
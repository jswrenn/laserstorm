use types::Point;
use specs::{Component, VecStorage};

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct CenterOfMass(pub Point<f64>);

impl ::std::ops::Deref for CenterOfMass {
  type Target = Point<f64>;
  fn deref(&self) -> &Point<f64> {
    &self.0
  }
}

impl ::std::ops::DerefMut for CenterOfMass {
  fn deref_mut(&mut self) -> &mut Point<f64> {
    &mut self.0
  }
}
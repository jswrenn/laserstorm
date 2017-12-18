use types::Isometry;
use specs::{Component, VecStorage};

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct Position(pub Isometry<f64>);

impl ::std::ops::Deref for Position {
  type Target = Isometry<f64>;
  fn deref(&self) -> &Isometry<f64> {
    &self.0
  }
}

impl ::std::ops::DerefMut for Position {
  fn deref_mut(&mut self) -> &mut Isometry<f64> {
    &mut self.0
  }
}
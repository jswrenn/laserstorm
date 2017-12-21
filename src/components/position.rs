use types::*;
use specs::*;

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct Position(pub Isometry);

impl ::std::ops::Deref for Position {
  type Target = Isometry;
  fn deref(&self) -> &Isometry {
    &self.0
  }
}

impl ::std::ops::DerefMut for Position {
  fn deref_mut(&mut self) -> &mut Isometry {
    &mut self.0
  }
}
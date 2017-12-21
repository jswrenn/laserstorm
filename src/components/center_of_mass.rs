use types::*;
use specs::*;

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct CenterOfMass(pub Point);

impl ::std::ops::Deref for CenterOfMass {
  type Target = Point;
  fn deref(&self) -> &Point {
    &self.0
  }
}

impl ::std::ops::DerefMut for CenterOfMass {
  fn deref_mut(&mut self) -> &mut Point {
    &mut self.0
  }
}